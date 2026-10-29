/* rmem.h -- Public interface of the raw allocator (one mapping per block).

   Author:  contato.lucasdwbfff@gmail.com

   This file is part of mclibc.

   mclibc is a MINIMAL package of functions that ships bundled with
   Carla, the programming language.  Everything in mclibc is written to
   be tiny, dependency free (no libc, no malloc) and easy to audit.

   `rmem' is the simplest allocator of the package.  It plays the role
   of a `malloc' / `free' pair, with two differences:

   * Every allocation is its OWN mapping, obtained straight from the
     operating system with `mmap' (or `VirtualAlloc') and given back
     with `munmap' (or `VirtualFree').  There is no free list, no
     splitting and no coalescing.  Because the kernel hands out whole
     pages, a block always costs at least one page, so this allocator
     suits a few large or long-lived blocks, not many tiny ones.
   * The handles (refs) returned by `alloc' do NOT live inside the
     mapped blocks.  They live in ONE global bookkeeping region, whose
     size the program chooses through `bookkeeping_set'.

   Typical use:

       bookkeeping_set (4096);          -- once, at program start
       ref *r = alloc (128);            -- 128 zeroed bytes
       addr p = makeraw (r);            -- the plain address
       dump (r);                        -- give the block back
       bookkeeping_dump ();             -- at program end

   The implementation lives in `rmem.c'.  This header only declares
   the types and the public functions.  For the code conventions, see
   `nashj.h' and `nashj.c', which are the reference example.  */

#include <stddef.h>

/* ---------------------------------------------------------------------
   Namespacing.  Every public name is prefixed with `__carla_abi_corelib_rmem_func__', so it
   cannot collide with the names of the program that links the lib.
   */

#define mangled(id) __carla_abi_corelib_rmem_func__##id

/* ---------------------------------------------------------------------
   Basic types.  */

/* A single byte of memory.  */
typedef unsigned char mangled(byte);

/* An opaque address handed to (or received from) the user.  */
typedef void* mangled(addr);

/* ---------------------------------------------------------------------
   Ref: the handle returned by `alloc'.

   Same idea as the ref of `nashj': the address of the allocation and
   a byte that tells whether it has already been freed.  There is one
   difference.  The nashj finds the size of an allocation through its
   node lists, but `rmem' has no lists, and `munmap' needs the length.
   So the ref also stores the size.

   A ref is NEVER recycled, so `freed' keeps answering correctly after
   `dump', and a stale handle can always be told apart from a live one.
   The price is that each `alloc' consumes one ref from the global
   bookkeeping for good, and the bookkeeping has a fixed size (see
   `bookkeeping_set').  */

typedef struct mangled(ref) {
    mangled(byte) freed;            /* 0 while live, 1 once freed.       */
    mangled(addr) ptr;              /* first byte of the allocation.     */
    size_t size;                    /* length that was mapped, in bytes. */
} mangled(ref);

/* ---------------------------------------------------------------------
   Public functions, listed as an X-macro.  See `nashj.h'.

   The list below is expanded into the `extern inline' declarations
   that follow it, so each function is declared in ONE place only.

     bookkeeping_set   Map the global bookkeeping region with BYTES
                       bytes.  Must be called once, before `alloc'.
                       Returns 0, or -1 on failure.
     bookkeeping_dump  Unmap the global bookkeeping region.  Every
                       ref becomes invalid, so call it only at the
                       end of the program, after the last `dump'.
     alloc             Map SIZE zero-filled bytes and return a ref, or
                       NULL on failure.
     makeraw           Return the plain address stored in a ref, or
                       NULL if the ref is NULL or already freed.
     dump              Unmap the allocation of a ref and mark the ref
                       as freed.
     is_freed          Tell whether a ref has already been freed
                       (1) or is still live (0).
     size              Return the length, in bytes, that was requested
                       for a ref.  */

#define FUNCTIONS                                                           \
X(bookkeeping_set,  int,           (size_t bytes))                          \
X(bookkeeping_dump, void,          ())                                      \
X(alloc,            mangled(ref)*, (size_t size))                           \
X(makeraw,          mangled(addr), (mangled(ref) *handle))                  \
X(dump,             void,          (mangled(ref) *handle))                  \
X(is_freed,         char,          (mangled(ref) *handle))                  \
X(size,             size_t,        (mangled(ref) *handle))

#define X(id, ret, args)                                            \
extern inline ret mangled(id) args;
FUNCTIONS
#undef X
