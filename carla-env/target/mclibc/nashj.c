//h: errors
/* nashj.c -- Nashj (نشج) allocator: a free list and no per-block headers.

   Author:  contato.lucasdwbfff@gmail.com

   This file is part of mclibc.

   mclibc is a MINIMAL package of functions that ships bundled with
   Carla, the programming language.  Everything in mclibc is written to
   be tiny, dependency free (no libc, no malloc) and easy to audit.

   Nashj (نشج) is the Arabic word for "weaving".  The allocator weaves
   live allocations, free ranges and their bookkeeping into one single
   region of memory, and `coalesce' re-weaves it without gaps.

   ---------------------------------------------------------------------
   CODE CONVENTIONS (READ THIS BEFORE WRITING ANY "stdlibcore" LIB)
   ---------------------------------------------------------------------

   The code in this file is the REFERENCE EXAMPLE for every "lib" that
   belongs to Carla's "stdlibcore".  Its style is intentionally
   uniform, so that all libs look and behave the same way.  The rules
   that can be observed here are:

   1. Namespacing.  Every public or internal identifier is spelled
      through the `mangled(id)' macro, defined in the lib's own header
      (here, `nashj.h').  For this lib, `mangled(alloc)' expands to
      `nashj_alloc'.  Changing the prefix means changing ONE macro.

   2. Public API through an X-macro.  The header lists every public
      function once, in a `FUNCTIONS' table, and expands it to emit the
      `extern inline' declarations.  This file provides the matching
      `inline' definitions.

   3. Function layout.  The return type goes on its own line, the
      function name starts at column 0 (so `grep -n "^name"' finds the
      definition), and the body's opening brace is followed by three
      spaces and the first statement:

          inline void
          mangled(name) (arguments)
          {   first_statement;
              ...
          }

   4. Internal helpers are `static inline'.  Only the functions listed
      in the header's `FUNCTIONS' table are visible to the outside.

   5. Platform code is isolated in a single block at the top of the
      file, behind `#if' guards, and exports the same tiny interface on
      every platform.  The rest of the file never mentions an OS.

   6. No hidden allocations, no global state.  Every function works only
      on the state that the caller hands to it.

   7. Comments follow the GNU style: full sentences, two spaces after a
      period, and the closing marker of a multi-line comment on the
      last line of text.

   8. Runtime errors are reported through `errors.h', as
      `runtime_error (LIB) ("message")', with LIB defined as the name of
      the lib.  The code that follows the call still returns a safe
      value, so the function behaves well even if the error handler
      comes back.

   ---------------------------------------------------------------------
   DESIGN OVERVIEW
   ---------------------------------------------------------------------

   The nashj owns TWO kinds of memory:

   * The DATA REGION: one contiguous region obtained from the operating
     system, with exactly the size that was asked from `start'.  It
     holds user data and nothing else, so a nashj started with 128
     bytes can hand out 128 bytes of data:

       low addresses                                    high addresses
       +--------------------------+---------------------------------+
       | user data  ->            |           (free room)           |
       +--------------------------+---------------------------------+
       ^ mem                      ^ mem+offset                      ^ mem+capacity

     `offset' is the top of the data area.

   * The BOOKKEEPING CHUNKS: separate regions, mapped on demand, that
     hold the nodes and the refs.  They are chained through `tab',
     newest first, and they NEVER count against the data capacity.

   Every allocation is described by a NODE, which stores an address and
   a size ("9 until 103", for instance), and by a REF, the small handle
   that `alloc' returns to the user (see `nashj.h').  Both live in the
   bookkeeping chunks, so no external allocator is needed.  Nodes are
   kept in three singly linked lists:

   * `used_list'  - live allocations.  Freeing looks up the pointer
                    here; the address found IS the identity, and the
                    size comes from the same node.  This is why no
                    header is stored before the user's data.
   * `free_list'  - ranges that were freed, sorted by address, and
                    merged with their neighbours whenever they touch.
   * `spare'      - dead nodes, kept for reuse so that the number of
                    nodes is bounded by the PEAK number of simultaneous
                    allocations rather than by the total history.

   Refs are different: a ref must keep telling "this address was
   freed" AFTER the free, so it can never be recycled.  Each `alloc'
   therefore consumes one ref from the bookkeeping chunks until `reset'
   or `destroy'.

   Freed ranges have no "chunk" concept: a range is just an address
   and a length, and may be split or merged freely.

   `coalesce' slides the live allocations down to close the gaps that
   `dump' left between them.  Handles survive it (their address is
   updated); raw addresses obtained from `makeraw' do not.

   Every byte handed out by `alloc' is zero-filled, no matter whether
   it comes from fresh pages, from a reused free range, or from a data
   area that `dump' gave back.  */

#include "nashj.h"
#include <stddef.h>

#define LIB nashj
#include "errors.h"

/* ---------------------------------------------------------------------
   Platform detection.

   Exactly one of NASHJ_OS_WINDOWS or NASHJ_OS_LINUX is defined here.
   `_WIN32' is set by every Windows compiler (MSVC, MinGW, clang-cl) for
   both 32 and 64 bit targets, unlike `WIN32', which usually comes from
   the project files.  Any other system is rejected at compile time
   instead of failing mysteriously at run time.  */

#if defined(_WIN32)
#define NASHJ_OS_WINDOWS 1
/* Trim <windows.h> down to the parts that we really need.  */
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#elif defined(__linux__)
#define NASHJ_OS_LINUX 1
#include "syscall.h"
#else
#error "nashj: os not supported"
#endif

/* =====================================================================
   LINUX BACKEND

   We talk to the kernel directly, bypassing libc entirely.  Each
   supported architecture provides two raw wrappers, `sys_mmap' and
   `sys_munmap', and the common code further below turns them into the
   portable `mmap' and `munmap' used by the rest of the file.  */

#if defined(NASHJ_OS_LINUX)

/* Values from the Linux kernel ABI (<sys/mman.h>).  They are repeated
   here so that this file does not depend on any libc header.

   PROT_READ / PROT_WRITE : the pages may be read and written.
   MAP_PRIVATE            : changes are not shared with other processes.
   MAP_ANONYMOUS          : the pages are not backed by a file; the
                            kernel hands them over zero-filled.  */
#define PROT_READ     0x1
#define PROT_WRITE    0x2
#define MAP_PRIVATE   0x02
#define MAP_ANONYMOUS 0x20

/* ---------------------------------------------------------------------
   x86_64: the `syscall' instruction.

   Calling convention of the Linux x86_64 kernel:

     rax = system call number         (also the return value)
     rdi, rsi, rdx, r10, r8, r9 = arguments 1 to 6

   Note that the 4th argument goes in r10, NOT in rcx, because the
   `syscall' instruction itself clobbers rcx and r11.  */

#if defined(__x86_64__)

/* Perform a raw system call with up to six arguments.

   NUMBER is the system call number; A1..A6 are its arguments.  The
   return value is whatever the kernel left in rax.  On failure, the
   kernel returns a small negative number (-errno) in the range
   -1..-4095; it does NOT touch any `errno' variable, since there is
   no libc here.  */
static inline long
mangled(mclibcsys6) (
    long number,
    long a1, long a2, long a3,
    long a4, long a5, long a6
) {
    /* Arguments 4 to 6 have no single-letter GCC constraint, so they
       are pinned to their registers with explicit register variables.  */
    register long r10 __asm__("r10") = a4;
    register long r8  __asm__("r8")  = a5;
    register long r9  __asm__("r9")  = a6;
    long result;
    __asm__ volatile (
        "syscall"
        : "=a"(result)          /* output: rax                          */
        : "a"(number),          /* input:  rax = system call number     */
          "D"(a1),              /*         rdi = argument 1             */
          "S"(a2),              /*         rsi = argument 2             */
          "d"(a3),              /*         rdx = argument 3             */
          "r"(r10),             /*         r10 = argument 4             */
          "r"(r8),              /*         r8  = argument 5             */
          "r"(r9)               /*         r9  = argument 6             */
        : "rcx", "r11", "memory"    /* clobbered by the instruction     */
    );
    return result;
}

/* Ask the kernel for LENGTH bytes of anonymous, zero-filled,
   read/write memory (system call 9, `mmap').  The kernel chooses the
   address.  Returns the address, or -errno on failure.  */
static inline long
mangled(sys_mmap) (size_t length)
{   return mangled(mclibcsys6)(
        9, 0, (long)length,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1, 0                   /* no file descriptor, no file offset   */
    );
}

/* Give the LENGTH bytes starting at PTR back to the kernel
   (system call 11, `munmap').  Returns 0 on success, -errno on
   failure.  */
static inline long
mangled(sys_munmap) (mangled(addr) ptr, size_t length)
{   return mangled(mclibcsys6)(11, (long)ptr, (long)length, 0, 0, 0, 0);
}

/* ---------------------------------------------------------------------
   x86 (32 bit): the `int $0x80' software interrupt.

   Calling convention of the Linux i386 kernel:

     eax = system call number         (also the return value)
     ebx, ecx, edx, esi, edi, ebp = arguments 1 to 6

   We only need three registers, which keeps the asm simple.  */

#elif defined(__i386__)

/* Perform a raw system call with up to three arguments.

   In position independent code (-fPIC) ebx is reserved as the GOT
   pointer, so GCC refuses the "b" constraint.  To stay portable, ebx
   is saved with `pushl', loaded by hand, and restored with `popl'
   around the interrupt.  */
static inline long
mangled(mclibcsys3) (long number, long a1, long a2, long a3)
{   long result;
    __asm__ volatile (
        "pushl %%ebx\n\t"       /* save the PIC register                */
        "movl  %2, %%ebx\n\t"   /* ebx = argument 1                     */
        "int   $0x80\n\t"       /* enter the kernel                     */
        "popl  %%ebx"           /* restore the PIC register             */
        : "=a"(result)
        : "0"(number),          /* eax = system call number (same reg   */
                                /* as the output)                       */
          "ri"(a1),             /* argument 1: any register or an imm.  */
          "c"(a2),              /* ecx = argument 2                     */
          "d"(a3)               /* edx = argument 3                     */
        : "memory", "cc"
    );
    return result;
}

/* Ask the kernel for LENGTH bytes of anonymous, zero-filled,
   read/write memory.

   We use system call 90 (`old_mmap') on purpose: it takes ONE pointer
   to a block of six arguments, so it needs a single register.  The
   newer `mmap2' (call 192) needs six registers, including ebp, which
   collides with frame pointers.  */
static inline long
mangled(sys_mmap) (size_t length)
{   struct {
        unsigned long addr, len, prot, flags, fd, offset;
    } args = {
        0, (unsigned long)length,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        (unsigned long)-1, 0
    };
    return mangled(mclibcsys3)(90, (long)&args, 0, 0);
}

/* Give the LENGTH bytes starting at PTR back to the kernel
   (system call 91, `munmap').  */
static inline long
mangled(sys_munmap) (mangled(addr) ptr, size_t length)
{   return mangled(mclibcsys3)(91, (long)ptr, (long)length, 0);
}

#else
#error "nashj: Arch Linux not supported"
#endif

/* ---------------------------------------------------------------------
   Portable wrappers shared by every Linux architecture.  */

/* Map LENGTH bytes of fresh memory.  PTR is a placement hint that is
   currently ignored (the kernel always picks the address).

   Returns the address, or NULL on failure.  The kernel reports errors
   as values in the range -4095..-1, which, read as an unsigned number,
   is the highest 4095 values of the address space; no valid mapping
   can live there.  */
static inline mangled(addr)
mangled(mmap) (mangled(addr) ptr, size_t length)
{   (void)ptr;
    long r = mangled(sys_mmap)(length);
    if( ((unsigned long)r) > (unsigned long)-4096L ) return NULL;
    return (mangled(addr)) r;
}

/* Unmap LENGTH bytes at PTR.  Returns 0 on success, non-zero on
   failure.  */
static inline int
mangled(munmap) (mangled(addr) ptr, size_t length)
{   return (int) mangled(sys_munmap)(ptr, length);
}

/* =====================================================================
   WINDOWS BACKEND

   Windows has no raw system call ABI that is stable across versions,
   so the documented Win32 virtual memory API is used instead.  */

#elif defined(NASHJ_OS_WINDOWS)

/* Map LENGTH bytes of fresh memory.  MEM_RESERVE | MEM_COMMIT gets the
   address range and the backing storage in a single call, and the
   pages come zero-filled, just like an anonymous mmap on Linux.

   Returns the address, or NULL on failure.  */
static inline mangled(addr)
mangled(mmap) (mangled(addr) ptr, size_t length)
{   return VirtualAlloc(ptr, length, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
}

/* Release the region that starts at PTR.  Returns 0 on success and -1
   on failure, mirroring the Linux convention.

   LENGTH is unused: with MEM_RELEASE, Windows requires the size to be
   zero and always frees the whole region that was reserved.  */
static inline int
mangled(munmap) (mangled(addr) ptr, size_t length)
{   (void)length;
    return VirtualFree(ptr, 0, MEM_RELEASE) ? 0 : -1;
}

#endif

/* =====================================================================
   INTERNAL HELPERS

   From this point on, the code is completely platform independent.  */

/* Return how many bytes of the data area are still untouched.  Only
   user data lives there, so this is exactly what `alloc' may still
   hand out.  */
static inline size_t
mangled(room) (mangled(t) *a)
{   return a->capacity - a->offset;
}

/* Fill the N bytes starting at P with zeros.

   The stores go through a `volatile' pointer on purpose: otherwise the
   compiler may turn this loop into a call to `memset', which does not
   exist here because there is no libc.  */
static inline void
mangled(zero) (mangled(byte) *p, size_t n)
{   volatile mangled(byte) *v = p;
    while( n-- ) *v++ = 0;
}

/* Copy N bytes from SRC to DST.  The two ranges must not overlap.

   As in `zero', the stores go through a `volatile' pointer so that the
   compiler cannot turn the loop into a call to `memcpy', which does not
   exist here because there is no libc.  */
static inline void
mangled(copy) (mangled(byte) *dst, const mangled(byte) *src, size_t n)
{   volatile mangled(byte) *v = dst;
    while( n-- ) *v++ = *src++;
}

/* Make sure that the current bookkeeping chunk has at least SIZE free
   bytes, mapping a new chunk when it does not.

   Returns 0 on success and -1 if the system refuses the new chunk.
   Nothing is changed on failure.  The old chunk is kept, so nodes and
   refs that already live in it never move.  */
static inline int
mangled(table_reserve) (mangled(t) *a, size_t size)
{   if( a->tab && a->tab->size - a->tab_used >= size ) return 0;
    size_t want = sizeof(mangled(chunk)) + size;
    size_t len = want > NASHJ_CHUNK ? want : NASHJ_CHUNK;
    mangled(chunk) *c = (mangled(chunk)*) mangled(mmap) (NULL, len);
    if( !c ) return -1;
    c->prev = a->tab;
    c->size = len;
    a->tab = c;
    a->tab_used = sizeof(mangled(chunk));
    return 0;
}

/* Take SIZE bytes from the current bookkeeping chunk and return them.
   The caller MUST have called `table_reserve' for at least SIZE bytes
   before; nothing is verified here.  */
static inline mangled(addr)
mangled(table_get) (mangled(t) *a, size_t size)
{   mangled(byte) *p = (mangled(byte)*) a->tab + a->tab_used;
    a->tab_used += size;
    return p;
}

/* Obtain a node to describe an allocation or a free range.

   A recycled node from the `spare' list is preferred.  Only when none
   is available does the bookkeeping grow.  The caller MUST have
   reserved `node_cost' bytes before.  */
static inline mangled(node)*
mangled(node_get) (mangled(t) *a)
{   mangled(node) *n = a->spare;
    if( n ) { a->spare = n->next; return n; }
    return (mangled(node)*) mangled(table_get)(a, sizeof(mangled(node)));
}

/* Return how many bytes of the bookkeeping `node_get' will consume
   right now: zero when a spare node is available.  */
static inline size_t
mangled(node_cost) (mangled(t) *a)
{   return a->spare ? 0 : sizeof(mangled(node));
}

/* Unmap every bookkeeping chunk.  Returns non-zero if the system
   refused any of them.  Every node and ref dies with the chunks.  */
static inline int
mangled(table_free) (mangled(t) *a)
{   int bad = 0;
    mangled(chunk) *c = a->tab;
    while( c ) {
        mangled(chunk) *prev = c->prev;
        if( mangled(munmap)(c, c->size) ) bad = 1;
        c = prev;
    }
    a->tab = NULL;
    a->tab_used = 0;
    return bad;
}

/* Give a node that is no longer needed back to the `spare' list, so
   that `node_get' can reuse it.  The bookkeeping chunks never shrink
   (except through `reset'), which is why recycling matters.  */
static inline void
mangled(node_put) (mangled(t) *a, mangled(node) *n)
{   n->next = a->spare;
    a->spare = n;
}

/* Report that `alloc' ran out of room in the data area and return
   NULL, so that every failure path of `alloc' can say
   `return no_room ()'.  */
static inline mangled(ref)*
mangled(no_room) (void)
{   runtime_error(LIB)("Out of memory: no room left in the data area for the allocation.");
    return NULL;
}

/* =====================================================================
   PUBLIC API

   Every function below is declared in `nashj.h' through the
   `FUNCTIONS' X-macro.  */

/* Create a nashj with BYTES bytes of DATA obtained from the operating
   system.  The nashj is returned BY VALUE; the caller owns it and must
   eventually pass it to `destroy'.

   BYTES is exactly the data capacity: the nodes and refs that
   describe the allocations live in separate bookkeeping chunks, which
   are mapped on demand and never take room from the data area.

   If the system refuses the request, a runtime error is emitted and the
   returned nashj has `capacity' == 0, so every later `alloc' on it
   fails too and the caller needs no special handling.  */
inline mangled(t)
mangled(start) (size_t bytes)
{   mangled(t) nashj = {0};     /* all lists start empty (NULL)         */
    nashj.mem = mangled(mmap) (NULL, bytes);
    if( !nashj.mem )
        runtime_error(LIB)("Could not obtain memory from the operating system.");
    nashj.capacity = nashj.mem ? bytes : 0;
    return nashj;
}

/* Allocate SIZE bytes from the nashj A, all of them set to zero.

   The size is rounded up to a multiple of NASHJ_ALIGN, so every
   returned address is NASHJ_ALIGN-byte aligned.  The strategy is:

     1. First fit over the free list.  A range that is a perfect fit is
        promoted to a live allocation as is.  A larger range is split:
        the front becomes the allocation and the free range shrinks.
     2. Otherwise, the top of the data area is advanced.

   The result is a pointer to a REF: the handle that stores the address
   and the `freed' flag.  Use `makeraw' to get the plain address.

   Returns NULL, after emitting a runtime error, if SIZE is zero, if it
   is absurdly large, if the data area has no room left for the
   (rounded) size, or if the system refuses the memory for the
   bookkeeping.  Nothing is modified when NULL is returned.  */
inline mangled(ref)*
mangled(alloc) (mangled(t) *a, size_t size)
{   /* Reject zero, and sizes that would overflow when rounded up.  */
    if( size == 0 ) {
        runtime_error(LIB)("Can't allocate zero bytes.");
        return NULL;
    }
    if( size > (size_t)-1 - NASHJ_ALIGN ) {
        runtime_error(LIB)("The requested size is too large.");
        return NULL;
    }
    size_t total = NASHJ_NORM(size);
    size_t refs = sizeof(mangled(ref));

    /* Step 1: look for a suitable free range (first fit).  `link'
       points at the pointer that references the current node, which
       lets us unlink a node without keeping a `prev' pointer.  */
    mangled(node) **link = &a->free_list;
    while( *link && (*link)->size < total ) link = &(*link)->next;
    mangled(node) *n = *link;
    int perfect = n && n->size == total;

    /* All the checks happen BEFORE anything is touched, so a failure
       leaves the nashj exactly as it was.  First the data room (only
       when the top of the data area has to grow), then the room for
       the bookkeeping.  A perfect fit reuses the free node itself.  */
    if( !n && total > mangled(room)(a) ) return mangled(no_room)();
    size_t need = refs + (perfect ? 0 : mangled(node_cost)(a));
    if( mangled(table_reserve)(a, need) ) {
        runtime_error(LIB)("Could not obtain memory for the bookkeeping.");
        return NULL;
    }

    mangled(node) *r;
    mangled(ref) *h = (mangled(ref)*) mangled(table_get)(a, refs);
    if( perfect ) {
        /* The free node itself becomes the allocation node.  */
        *link = n->next;                    /* unlink from free list    */
        r = n;
    } else if( n ) {
        /* The range is larger than needed: carve the allocation out
           of its front.  A fresh node describes the allocation, and
           the free range just loses the bytes we took.  */
        r = mangled(node_get)(a);
        r->addr = n->addr;
        n->addr += total;
        n->size -= total;
    } else {
        /* Step 2: no free range fits, so grow the data area.  */
        r = mangled(node_get)(a);
        r->addr = a->mem + a->offset;
        a->offset += total;
    }

    /* Common tail: describe the allocation, zero it (memory coming
       from a free range or from a lowered `offset' is dirty) and
       publish it on the used list.  */
    r->size = total;
    mangled(zero)(r->addr, total);
    h->freed = 0;
    h->ptr = r->addr;
    r->ref = h;
    r->next = a->used_list;
    a->used_list = r;
    return h;
}

/* Return the plain address stored in the handle H.

   Returns NULL, after emitting a runtime error, if H is NULL or if the
   allocation has already been freed, so a stale handle can never be
   used by mistake.  */
inline mangled(addr)
mangled(makeraw) (mangled(ref) *h)
{   if( !h ) {
        runtime_error(LIB)("Null handle. It has no address.");
        return NULL;
    }
    if( h->freed ) {
        runtime_error(LIB)("Address already freed. You can't get its raw address.");
        return NULL;
    }
    return h->ptr;
}

/* Free the allocation described by the handle H.

   The allocation is looked up in the used list by ADDRESS: the node
   whose address equals `H->ptr' (and whose handle is H) holds the size
   of the allocation.  A NULL handle is ignored, like `free (NULL)'.  A
   handle that is already marked as freed (a double free), or that does
   not belong to this nashj, emits a runtime error and leaves the
   nashj untouched.

   The handle is marked as freed.  The freed range is then inserted in
   the free list, keeping it sorted by address, and merged with the
   previous and next ranges whenever they are adjacent in memory.
   Finally, if the resulting range touches the top of the data area, it
   is handed back to the untouched region by lowering `offset'.  */
inline void
mangled(dump) (mangled(t) *a, mangled(ref) *h)
{   if( !h ) return;
    if( h->freed ) {
        runtime_error(LIB)("Address already freed. You can't free it twice.");
        return;
    }

    /* Find the node of this handle and detach it from the used list.
       R stays NULL if there is no such node.  */
    mangled(node) *r = NULL;
    for( mangled(node) **l = &a->used_list; *l; l = &(*l)->next )
    /* -> */ if( (*l)->ref == h && (*l)->addr == (mangled(byte)*)h->ptr ) {
        r = *l;
        *l = r->next;
        break;
    }

    if( !r ) {          /* not ours: nothing to do                      */
        runtime_error(LIB)("This handle does not belong to this allocator.");
        return;
    }

    /* The handle outlives the node, so it is marked here, before the
       node is merged or recycled.  */
    h->freed = 1;

    /* Find the insertion point in the address-sorted free list.  When
       the loop ends, PREV is the last range before R (or NULL) and NEXT
       is the first range after it (or NULL).  */
    mangled(node) *prev = NULL, *next = a->free_list;
    while( next && next->addr < r->addr ) { prev = next; next = next->next; }
    r->next = next;

    /* Merge with the range that follows, if R ends exactly where it
       starts.  The absorbed node is recycled.  */
    if( next && r->addr + r->size == next->addr ) {
        r->size += next->size;
        r->next  = next->next;
        mangled(node_put)(a, next);
    }

    /* Merge with the range that precedes, if it ends exactly where R
       starts.  In that case PREV swallows R, and R is recycled.
       Otherwise, R is linked in as a range of its own.  */
    if( prev && prev->addr + prev->size == r->addr ) {
        prev->size += r->size;
        prev->next  = r->next;
        mangled(node_put)(a, r);
        r = prev;                       /* R is now the merged range    */
    } else if( prev ) prev->next = r;
    else a->free_list = r;              /* R is the new list head       */

    /* If the range reaches the top of the data area, there is nothing
       above it that is still in use, so give it back to the untouched
       region by lowering the top.  Being the highest range, it is
       necessarily the LAST node of the free list, so it is unlinked
       from there and recycled.  The bytes it held are dirty, which is
       fine: `alloc' zeroes everything it hands out.  */
    if( r->addr + r->size == a->mem + a->offset ) {
        a->offset = r->addr - a->mem;
        mangled(node) **l = &a->free_list;
        while(*l != r) l = &(*l)->next;
        *l = NULL;
        mangled(node_put)(a, r);
    }
}

/* Slide every live allocation down, closing all the gaps that `dump'
   left between them.

       memory: |xxxxxx|....|xxxxxxx|.........|
       result: |xxxxxx|xxxxxxx|.............|

   The data area is first copied, byte by byte, to a temporary region
   obtained from the operating system.  Every live allocation is then
   copied back from that snapshot to its new, lower address: the old
   address minus the total size of the free ranges below it.  Because
   the moves always read from the snapshot, they can be done in any
   order and can never overwrite data that is still waiting to be
   moved.  The snapshot is returned to the system before the function
   ends.  Only the data bytes need a snapshot: the nodes and handles
   are updated in place.

   Afterwards the free list is empty (its nodes are recycled) and
   `offset' is lowered by the size of all the gaps.

   Handles stay valid, because their `ptr' is updated.  Every raw
   address obtained before with `makeraw' is INVALID after this call
   and must be fetched again.

   Returns 0 on success, which includes the case of a nashj without
   gaps.  Returns -1, after emitting a runtime error, if the temporary
   region cannot be obtained; in that case nothing has been touched.  */
inline int
mangled(coalesce) (mangled(t) *a)
{   if( !a->free_list ) return 0;           /* no gaps: nothing to do   */

    /* Step 1: snapshot of the whole data area.  Everything below
       `offset' is either live or free, and the free ranges are about
       to be overwritten, so the copy covers all of it.  */
    size_t used = a->offset;
    mangled(byte) *snap = mangled(mmap) (NULL, used);
    if( !snap ) {
        runtime_error(LIB)("Could not obtain the temporary copy. Nothing was moved.");
        return -1;
    }
    mangled(copy)(snap, a->mem, used);

    /* Step 2: move every live allocation.  The free list is sorted by
       address, so the walk stops at the first range above the node.  */
    for( mangled(node) *n = a->used_list; n; n = n->next ) {
        size_t shift = 0;
        for( mangled(node) *f = a->free_list; f && f->addr < n->addr; f = f->next )
            shift += f->size;
        if( shift == 0 ) continue;          /* below every gap: stays   */

        mangled(byte) *old = n->addr;
        n->addr = old - shift;
        mangled(copy)(n->addr, snap + (old - a->mem), n->size);
        n->ref->ptr = n->addr;              /* keep the handle in sync  */
    }

    /* Step 3: the gaps are gone.  Recycle their nodes and lower the
       top of the data area by their total size.  */
    while( a->free_list ) {
        mangled(node) *f = a->free_list;
        a->free_list = f->next;
        a->offset -= f->size;
        mangled(node_put)(a, f);
    }

    if( mangled(munmap)(snap, used) )
        runtime_error(LIB)("Could not release the temporary copy.");
    return 0;
}

/* Forget every allocation at once, keeping the data region.

   The nashj goes back to its just-started state: the data area is
   empty, the bookkeeping chunks are returned to the system, and all
   three lists are dropped.  Every pointer and every handle previously
   returned by `alloc' becomes invalid.  This is the classic "free
   everything" operation of arena allocators, and it takes time
   proportional to the number of bookkeeping chunks (usually one).  */
inline void
mangled(reset) (mangled(t) *a)
{   if( mangled(table_free)(a) )
        runtime_error(LIB)("Could not return the bookkeeping memory to the operating system.");
    a->offset = 0;
    a->free_list = a->used_list = a->spare = NULL;
}

/* Return the whole nashj to the operating system.

   After this call, A is left in a safe, empty state (`capacity' is
   zero and `mem' is NULL), so a repeated `destroy' or a later `alloc'
   is harmless, and any pointer or handle obtained from the nashj is
   invalid.  If the system refuses to take the memory back, a runtime
   error is emitted, but A is cleared anyway.  */
inline void
mangled(destroy) (mangled(t) *a)
{   int bad = mangled(table_free)(a);
    if( a->mem && mangled(munmap)(a->mem, a->capacity) ) bad = 1;
    if( bad )
        runtime_error(LIB)("Could not return the memory to the operating system.");
    a->mem = NULL;
    a->capacity = a->offset = 0;
    a->free_list = a->used_list = a->spare = NULL;
}

#undef mangled
