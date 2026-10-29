/* nashj.h -- Public interface of the Nashj (نشج) allocator (no per-block headers).

   Author:  contato.lucasdwbfff@gmail.com

   This file is part of mclibc.

   mclibc is a MINIMAL package of functions that ships bundled with
   Carla, the programming language.  Everything in mclibc is written to
   be tiny, dependency free (no libc, no malloc) and easy to audit.

   Nashj (نشج) is the Arabic word for "weaving".

   The implementation lives in `nashj.c'.  This header only declares
   the types, the constants and the public functions.

   ---------------------------------------------------------------------
   CODE CONVENTIONS (READ THIS BEFORE WRITING ANY "stdlibcore" LIB)
   ---------------------------------------------------------------------

   The code in this file and in `nashj.c' is the REFERENCE EXAMPLE for
   every "lib" that belongs to Carla's "stdlibcore".  Its style is
   intentionally uniform, so that all libs look and behave the same
   way.  The rules that this header demonstrates are:

   1. Namespacing.  Every identifier that the lib defines is spelled
      through the `mangled(id)' macro.  For this lib,
      `mangled(alloc)' expands to `nashj_alloc'.  Changing the prefix
      means changing ONE line.  Only the lib's own constants and
      macros (such as NASHJ_ALIGN) carry the prefix in plain text, in
      upper case.

   2. One header per lib, one source file per lib.  The header holds
      types, constants and the public function table; the source file
      holds every definition.

   3. Public API through an X-macro.  The functions are listed ONCE, in
      a `FUNCTIONS' table, and that single list is expanded to emit
      the declarations.  Adding a public function means adding one line
      to the table, so the declarations can never drift apart.

   4. Data types are plain structs, passed to functions by pointer,
      with no hidden global state.  The caller owns all the memory.

   5. Comments follow the GNU style: full sentences, two spaces after a
      period, and the closing marker of a multi-line comment on the
      last line of text.

   For the function layout and the platform rules, see `nashj.c'.  */

#include <stddef.h>

/* ---------------------------------------------------------------------
   Namespacing.

   `mangled(id)' pastes the prefix `__carla_abi_corelib_func__' in front of ID, so that
   the lib's names cannot clash with the ones of any other lib or of
   the user's program.  It is used for types and for functions alike.  */

#define mangled(id) __carla_abi_corelib_nashj_func__##id

/* ---------------------------------------------------------------------
   Basic types.  */

/* A single byte of memory.  Unsigned, so that pointer arithmetic on
   `byte *' moves one byte at a time and never sign-extends.  */
typedef unsigned char mangled(byte);

/* An opaque address handed to (or received from) the user.  It is a
   `void *' so that it converts to any pointer type without a cast.  */
typedef void* mangled(addr);

/* ---------------------------------------------------------------------
   Ref: the handle returned by `alloc'.

   It holds the address of the allocation and one byte that tells
   whether that address has already been freed.  The handle is stored
   inside the nashj and, unlike a node, it is NEVER recycled while the
   nashj lives (only `reset' and `destroy' invalidate it).  This is
   what lets `freed' keep answering correctly after `dump'.

   Use `makeraw' to obtain the plain address from a handle.  */

typedef struct mangled(ref) {
    mangled(byte) freed;            /* 0 while live, 1 once freed.       */
    mangled(addr) ptr;              /* first byte of the allocation.     */
} mangled(ref);

/* ---------------------------------------------------------------------
   Node: describes a range of memory as "ADDR until ADDR + SIZE".

   There is no concept of chunk or block with a header.  A node is just
   an address and a length, and the same structure describes both live
   allocations and freed ranges.  Nodes are stored in the bookkeeping
   chunks (see `chunk' below), so no external allocator is needed to
   manage them and they never take room from the user's data.

   The same node lives in exactly one of three lists (see `t' below).  */

typedef struct mangled(node) {
    mangled(byte) *addr;            /* first byte of the range.          */
    size_t size;                    /* length of the range, in bytes.    */
    struct mangled(node) *next;     /* next node in the same list, or
                                       NULL at the end of the list.      */
    mangled(ref) *ref;              /* handle of a live allocation; not
                                       used by free ranges.              */
} mangled(node);

/* ---------------------------------------------------------------------
   Chunk: one region of bookkeeping.

   The nodes and the refs live in chunks, mapped on demand and chained
   newest first.  A chunk is mapped separately from the data region, so
   the bookkeeping NEVER takes room from the data capacity.  The chunk
   header is followed by the entries, handed out from `tab_used'.  */

typedef struct mangled(chunk) {
    struct mangled(chunk) *prev;    /* older chunk, or NULL.             */
    size_t size;                    /* size of this chunk, header too.   */
} mangled(chunk);

/* ---------------------------------------------------------------------
   Nashj: the state of one allocator.

   The nashj owns two kinds of memory.  The DATA REGION is ONE
   contiguous region obtained from the operating system, with exactly
   the size given to `start'.  It holds user data and nothing else:

       low addresses                                    high addresses
       +--------------------------+---------------------------------+
       | user data  ->            |           (free room)           |
       +--------------------------+---------------------------------+
       ^ mem                      ^ mem+offset                      ^ mem+capacity

   The BOOKKEEPING CHUNKS, which hold nodes and refs, are separate.

   The struct is small and is returned BY VALUE from `start'; every
   other function receives a pointer to it.  A zero-filled struct is a
   valid, empty nashj that has no memory (every `alloc' on it fails).  */

typedef struct mangled(t) {
    mangled(byte) *mem;         /* base of the data region.              */
    size_t capacity;            /* size of the data region, in bytes:
                                   the most user data it can hold.       */

    size_t offset;              /* top of the data area.  It grows
                                   upwards; everything below it has been
                                   handed out at least once.             */
    mangled(chunk) *tab;        /* newest bookkeeping chunk, or NULL.    */
    size_t tab_used;            /* bytes used in that chunk, header
                                   included.                             */

    mangled(node) *free_list;   /* ranges that were freed, sorted by
                                   address and merged with their
                                   neighbours when they touch.           */
    mangled(node) *used_list;   /* live allocations.  Looked up by
                                   address when the user frees one.      */
    mangled(node) *spare;       /* dead nodes kept for reuse, so that the
                                   number of nodes is bounded by the PEAK
                                   number of simultaneous allocations.   */
} mangled(t);

/* ---------------------------------------------------------------------
   Alignment.

   NASHJ_ALIGN is the alignment, in bytes, of every pointer returned by
   `alloc'.  It MUST be a power of two: the rounding below relies on it.
   16 bytes satisfies the strictest scalar type on x86_64 (and SSE
   vectors), and the base of the region is page aligned by the OS.  */

#define NASHJ_ALIGN 16

/* Size of the bookkeeping chunks that the nashj maps on demand.  A
   bigger chunk is mapped when one entry does not fit in this size.  */

#define NASHJ_CHUNK 4096

/* Round the request N up to the size that is really reserved: the
   next multiple of NASHJ_ALIGN, with the usual trick
   `(x + A - 1) & ~(A - 1)', valid because A is a power of two.

   There is no minimum block size, because the nodes are not stored
   inside the freed blocks.  The macro evaluates N more than once, so N
   must not have side effects.  */

#define NASHJ_NORM(n)                                               \
    ( ((n) + NASHJ_ALIGN - 1) & ~(size_t)(NASHJ_ALIGN - 1) )

/* ---------------------------------------------------------------------
   Public functions, listed as an X-macro.

   Each line is `X (name, return type, (parameters))'.  Expanding
   `FUNCTIONS' with different definitions of `X' yields different
   artifacts from the same single list.  Here it emits the declarations;
   the contract of each function is documented in `nashj.c', next to its
   definition.

     start    Create a nashj with BYTES bytes of data from the OS.  The
              bookkeeping is extra and never counts against BYTES.
     alloc    Allocate SIZE zero-filled bytes; returns a handle, or
              NULL if there is no room.
     makeraw  Return the plain address stored in a handle (NULL if the
              handle is NULL or already freed).
     dump     Free the allocation described by HANDLE (the size is
              found by the nashj itself, from the address) and mark
              the handle as freed.
     coalesce Slide the live allocations down to close the gaps left
              by `dump'.  Handles stay valid; raw addresses do not.
              Returns 0, or -1 if the temporary copy cannot be made.
     reset    Forget every allocation at once, keeping the memory.
     destroy  Return the whole nashj to the OS.  */

#define FUNCTIONS                                                           \
X(start,    mangled(t),     (size_t bytes))                                 \
X(alloc,    mangled(ref)*,  (mangled(t) *nashj, size_t size))               \
X(makeraw,  mangled(addr),  (mangled(ref) *handle))                         \
X(dump,     void,           (mangled(t) *nashj, mangled(ref) *handle))      \
X(coalesce, int,            (mangled(t) *nashj))                            \
X(reset,    void,           (mangled(t) *nashj))                            \
X(destroy,  void,           (mangled(t) *nashj))

/* Emit one `extern inline' declaration per entry of the table.

   In C99 and later, an `inline' definition in `nashj.c' together with
   an `extern inline' declaration visible in that same file makes the
   compiler emit ONE external definition of each function.  Other
   translation units can inline the calls or link against that
   definition, with no duplicated symbols.  */

#define X(id, ret, args)                                            \
extern inline ret mangled(id) args;
FUNCTIONS
#undef X
