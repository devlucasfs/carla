//h: errors
/* rmem.c -- Raw allocator: one mapping per block, refs in a global
   bookkeeping region.

   Author:  contato.lucasdwbfff@gmail.com

   This file is part of mclibc.

   mclibc is a MINIMAL package of functions that ships bundled with
   Carla, the programming language.  Everything in mclibc is written to
   be tiny, dependency free (no libc, no malloc) and easy to audit.

   The code conventions are the ones of `nashj.c'.  ONE of them is
   bent on purpose here: rule 6 says "no global state", but this lib
   exists to have a global bookkeeping.  The whole global state is the
   single `static' variable `global' below; nothing else is global.

   DESIGN OVERVIEW

   * `bookkeeping_set (N)' maps one region of N bytes.  It is carved
     into refs, one after the other, from the bottom up.
   * `alloc (SIZE)' maps SIZE bytes (the kernel rounds it up to whole
     pages, and gives the pages zero-filled), takes the next free slot
     of the bookkeeping, and fills it with the address and the size.
   * `dump (REF)' unmaps the block and marks the ref as freed.
   * `bookkeeping_dump ()' unmaps the bookkeeping itself, at the end
     of the program.

   The bookkeeping has a FIXED size and its slots are never reused, so
   it holds at most N / sizeof(ref) allocations over its whole life.
   When it is full, `alloc' fails.

   ERROR HANDLING

   Every failure is reported through `runtime_error (LIB)', which comes
   from `errors.h'.  The functions that can fail ALSO signal it in
   their return value (NULL or -1), so the caller can tell without
   inspecting the message.  A function that fails changes nothing, so
   it is safe to go on after an error.

   PORTABILITY

   Two operating systems are supported: Linux (x86_64 and i386, raw
   system calls, no libc) and Windows (`VirtualAlloc').  Each one
   provides the same two internal functions, `mmap' and `munmap', and
   the public API below is written only in terms of them.  */

#include "rmem.h"
#include <stddef.h>

#define LIB rmem
#include "errors.h"

/* ---------------------------------------------------------------------
   Platform detection.  See `nashj.c'.  Exactly one of the two
   RAWMEM_OS_* macros ends up defined; any other system is refused at
   compile time instead of failing at run time.  */

#if defined(_WIN32)
#define RAWMEM_OS_WINDOWS 1
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#elif defined(__linux__)
#define RAWMEM_OS_LINUX 1
#include "syscall.h"
#else
#error "raw: os not supported"
#endif

/* =====================================================================
   LINUX BACKEND

   Raw system calls, no libc.  Values from the Linux kernel ABI.  The
   constants below are written out by hand because <sys/mman.h> is a
   libc header, and this lib must not depend on libc.  */

#if defined(RAWMEM_OS_LINUX)

#define PROT_READ     0x1       /* The pages can be read.                */
#define PROT_WRITE    0x2       /* The pages can be written.             */
#define MAP_PRIVATE   0x02      /* Changes are not shared with others.   */
#define MAP_ANONYMOUS 0x20      /* Not backed by a file; zero-filled.    */

#if defined(__x86_64__)

/* Perform a raw system call with up to six arguments.  The kernel
   returns -errno (-4095..-1) on failure and touches no `errno'.

   The x86_64 convention puts the number in RAX and the arguments in
   RDI, RSI, RDX, R10, R8 and R9, in this order.  (Note R10, and not
   RCX as in the ordinary function-call convention: the `syscall'
   instruction itself overwrites RCX.)  The constraints "D", "S", "d"
   and "a" pin RDI, RSI, RDX and RAX; R10, R8 and R9 have no letter,
   so they are bound to their registers with `register ... asm'.

   `syscall' destroys RCX (it saves the return address there) and R11
   (it saves the flags there), so both are listed as clobbered.  The
   "memory" clobber stops the compiler from moving memory accesses
   across the call, since the kernel may read or write memory.  */
static inline long
mangled(mclibcsys6) (
    long number,
    long a1, long a2, long a3,
    long a4, long a5, long a6
) {
    register long r10 __asm__("r10") = a4;
    register long r8  __asm__("r8")  = a5;
    register long r9  __asm__("r9")  = a6;
    long result;
    __asm__ volatile (
        "syscall"
        : "=a"(result)
        : "a"(number), "D"(a1), "S"(a2), "d"(a3),
          "r"(r10), "r"(r8), "r"(r9)
        : "rcx", "r11", "memory"
    );
    return result;
}

/* System call 9, `mmap': anonymous, zero-filled, read/write.  The
   address hint is 0 (let the kernel choose), the file descriptor is
   -1 and the offset is 0, as an anonymous mapping requires.  */
static inline long
mangled(sys_mmap) (size_t length)
{   return mangled(mclibcsys6)(
        9, 0, (long)length,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1, 0
    );
}

/* System call 11, `munmap'.  The kernel unmaps every page that
   overlaps [PTR, PTR + LENGTH).  Returns 0 on success.  */
static inline long
mangled(sys_munmap) (mangled(addr) ptr, size_t length)
{   return mangled(mclibcsys6)(11, (long)ptr, (long)length, 0, 0, 0, 0);
}

#elif defined(__i386__)

/* Perform a raw system call with up to three arguments, through the
   `int $0x80' gate.  The number goes in EAX and the arguments in EBX,
   ECX and EDX.

   EBX is saved and restored by hand because, in position-independent
   code, the compiler reserves it as the GOT pointer, so it can't be
   named as an asm input.  The first argument is loaded into it only
   inside the asm block, between the push and the pop.  */
static inline long
mangled(mclibcsys3) (long number, long a1, long a2, long a3)
{   long result;
    __asm__ volatile (
        "pushl %%ebx\n\t"
        "movl  %2, %%ebx\n\t"
        "int   $0x80\n\t"
        "popl  %%ebx"
        : "=a"(result)
        : "0"(number), "ri"(a1), "c"(a2), "d"(a3)
        : "memory", "cc"
    );
    return result;
}

/* System call 90, `old_mmap'.  The i386 `mmap' takes more arguments
   than the registers can carry, so it receives ONE pointer to a block
   of six, laid out as in the struct below.  `-1' as the file
   descriptor and offset 0 make the mapping anonymous.  */
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

/* System call 91, `munmap'.  Returns 0 on success.  */
static inline long
mangled(sys_munmap) (mangled(addr) ptr, size_t length)
{   return mangled(mclibcsys3)(91, (long)ptr, (long)length, 0);
}

#else
#error "raw: Arch Linux not supported"
#endif

/* Map LENGTH bytes of fresh memory.  Returns the address, or NULL on
   failure.  Errors are the top 4095 values of the address space.

   The raw system call does not return NULL on failure: it returns a
   small negative number (-errno).  No valid mapping can live in the
   last 4095 values of the address space, so any result in that range
   is an error, and everything else is a good address.  */
static inline mangled(addr)
mangled(mmap) (size_t length)
{   long r = mangled(sys_mmap)(length);
    if( ((unsigned long)r) > (unsigned long)-4096L ) return NULL;
    return (mangled(addr)) r;
}

/* Unmap LENGTH bytes at PTR.  Returns 0 on success.  */
static inline int
mangled(munmap) (mangled(addr) ptr, size_t length)
{   return (int) mangled(sys_munmap)(ptr, length);
}

/* =====================================================================
   WINDOWS BACKEND

   Same two functions as above, on top of the Win32 virtual memory
   API.  */

#elif defined(RAWMEM_OS_WINDOWS)

/* Map LENGTH bytes of fresh, zero-filled memory.  Returns the address,
   or NULL on failure.  Reserving and committing in one call is what
   makes the pages usable right away.  */
static inline mangled(addr)
mangled(mmap) (size_t length)
{   return VirtualAlloc(NULL, length, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
}

/* Release the region that starts at PTR.  LENGTH is unused: with
   MEM_RELEASE, Windows frees the whole region that was reserved, and
   the size passed to `VirtualFree' must be 0.  Returns 0 on success,
   -1 on failure, to match the Linux backend.  */
static inline int
mangled(munmap) (mangled(addr) ptr, size_t length)
{   (void)length;
    return VirtualFree(ptr, 0, MEM_RELEASE) ? 0 : -1;
}

#endif

/* =====================================================================
   THE GLOBAL BOOKKEEPING

   It is the ONLY global state of the lib.  A zero-filled one (the
   state before `bookkeeping_set') has no memory, so every `alloc'
   fails.

   The region is used as a simple array of refs that only grows:

       mem                                          mem + capacity
        |<------------- used ------------->|<-- free -->|
        [ref][ref][ref]...[ref]            |            |

   so `used' is always a multiple of sizeof(ref), and the invariant
   `used <= capacity' always holds.  */

typedef struct mangled(book) {
    mangled(byte) *mem;         /* base of the bookkeeping, or NULL.     */
    size_t capacity;            /* size of the bookkeeping, in bytes.    */
    size_t used;                /* bytes already handed out as refs.     */
} mangled(book);

static mangled(book) mangled(global);

/* =====================================================================
   PUBLIC API  */

/* Map the global bookkeeping with BYTES bytes.  It holds at most
   BYTES / sizeof(ref) allocations, because refs are never recycled.

   Returns 0 on success.  Returns -1, after emitting a runtime error,
   if BYTES cannot hold even one ref, if the bookkeeping was already
   set, or if the system refuses the memory.  Nothing is changed on
   failure.  */
inline int
mangled(bookkeeping_set) (size_t bytes)
{   mangled(book) *b = &mangled(global);
    if( b->mem ) {
        runtime_error(LIB)("The bookkeeping was already set.");
        return -1;
    }
    if( bytes < sizeof(mangled(ref)) ) {
        runtime_error(LIB)("The bookkeeping is too small to hold a single ref.");
        return -1;
    }
    b->mem = (mangled(byte)*) mangled(mmap) (bytes);
    if( !b->mem ) {
        runtime_error(LIB)("Could not obtain memory for the bookkeeping.");
        return -1;
    }
    b->capacity = bytes;
    b->used = 0;
    return 0;
}

/* Unmap the global bookkeeping.  Meant to be called ONCE, at the end
   of the program, after the last `dump'.

   It does not unmap the blocks that are still live: it only gives
   back the region where their refs are stored, so those blocks can no
   longer be freed through `dump'.  The state is not reset either, so
   the lib can't be used again after this call.  */
inline void
mangled(bookkeeping_dump) ()
{   mangled(book) *b = &mangled(global);
    mangled(munmap) (b->mem, b->capacity);
}


/* Allocate SIZE bytes, all of them set to zero, as a mapping of their
   own, and return the ref that describes it.  The plain address is in
   `ref->ptr'.

   Returns NULL, after emitting a runtime error, if SIZE is zero or
   absurdly large, if the bookkeeping was never set or is full, or if
   the system refuses the memory.  Nothing is modified on failure: the
   bookkeeping is checked BEFORE the block is mapped, so a failure
   never leaves a mapping behind.

   The block is rounded up to whole pages by the kernel, but the ref
   keeps the size that was asked for.  */
inline mangled(ref)*
mangled(alloc) (size_t size)
{   mangled(book) *b = &mangled(global);
    if( size == 0 ) {
        runtime_error(LIB)("Can't allocate zero bytes.");
        return NULL;
    }
    /* Half of the address space is the largest size that can be valid;
       rejecting more also keeps SIZE safe to cast to `long' for the
       system call.  */
    if( size > ((size_t)-1 >> 1) ) {
        runtime_error(LIB)("The requested size is too large.");
        return NULL;
    }
    if( !b->mem ) {
        runtime_error(LIB)("The bookkeeping was not set. Call `init` first.");
        return NULL;
    }
    if( b->capacity - b->used < sizeof(mangled(ref)) ) {
        runtime_error(LIB)("The bookkeeping is full: no room left for another ref.");
        return NULL;
    }

    mangled(addr) p = mangled(mmap) (size);
    if( !p ) {
        runtime_error(LIB)("Could not obtain memory from the operating system.");
        return NULL;
    }

    /* Only now, with the block safely mapped, is the slot taken.  */
    mangled(ref) *h = (mangled(ref)*) (b->mem + b->used);
    b->used += sizeof(mangled(ref));
    h->freed = 0;
    h->ptr = p;
    h->size = size;
    return h;
}

/* Free the allocation described by the handle H and mark H as freed.

   A NULL handle is ignored, like `free (NULL)'.  A handle that is
   already freed, or that does not point to a slot of the bookkeeping,
   emits a runtime error and changes nothing.  If the system refuses to
   unmap the block, a runtime error is emitted, but H is marked as
   freed anyway, so that it is never used again.  */
inline void
mangled(dump) (mangled(ref) *h)
{   mangled(book) *b = &mangled(global);
    if( !h ) return;

    /* H must be exactly one of the slots already handed out: inside
       [mem, mem + used) and aligned to a whole slot.  This rejects
       foreign pointers and pointers into the middle of a ref.  */
    mangled(byte) *p = (mangled(byte)*) h;
    if( !b->mem || p < b->mem || p >= b->mem + b->used
        || (size_t)(p - b->mem) % sizeof(mangled(ref)) != 0 ) {
        runtime_error(LIB)("This handle does not belong to the bookkeeping.");
        return;
    }
    if( h->freed ) {
        runtime_error(LIB)("Address already freed. You can't free it twice.");
        return;
    }

    h->freed = 1;
    if( mangled(munmap)(h->ptr, h->size) )
        runtime_error(LIB)("Could not return the memory to the operating system.");
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


/* Tell whether the allocation described by H was already freed:
   returns 1 if it was, 0 if it is still live.

   A NULL handle emits a runtime error.  Unlike the other functions,
   this one has no return statement for that case, so it relies on
   `runtime_error' not returning to the caller.  */
inline char
mangled(is_freed) (mangled(ref) *h)
{   if( !h )
        runtime_error(LIB)("Unknown cell. Can't to know if the cell already was freed.");

    return h->freed;
}

/* Return the size, in bytes, that was requested for the allocation
   described by H (not the page-rounded size actually mapped).

   A NULL handle emits a runtime error.  As in `is_freed', the read
   that follows relies on `runtime_error' not returning.  */
inline size_t
mangled(size) (mangled(ref) *h)
{   if( !h )
        runtime_error(LIB)("Unknown cell. Can't to know the size of this cell.");
    return h->size;
}

#undef mangled
