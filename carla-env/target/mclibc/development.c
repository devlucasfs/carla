//h: errors nashj_ref

#include "development.h"
#include <stdint.h>

#define LIB development
#include "errors.h"
#include "nashj_ref.h"

void
mangled(fill) (void *re, int32_t start, int32_t size)
{   ref *r = (ref*)re;
    if( r->freed ) runtime_error(LIB)("Address already freed. You can't to fill it.");
    char *input = "Hello, world!\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
    int32_t i = 0;
    while( i < size && input[i] != '\0' ) ((char*)r->addr)[i] = input[i + start], i++;
}

void
mangled(exit)(int code)
{
    asm volatile(
        "syscall"
        :
        : "a"(60L), "D"((long) code)
        : "rcx", "r11", "memory"
    );
    __builtin_unreachable();
}

void
mangled(debug) (void *re)
{   ref *r = (ref*)re;
    if( r->freed ) runtime_error(LIB)("Address already freed. You can't to print it.");
    unsigned long len = 0;
    char *str = (char*)r->addr;
    while(str[len] != '\0') len++;

    asm volatile(
        "mov $1, %%rax\n"
        "mov $1, %%rdi\n"
        "mov %0, %%rsi\n"
        "mov %1, %%rdx\n"
        "syscall\n"
        :
        : "r"(str), "r"(len)
        : "rax", "rdi", "rsi", "rdx", "rcx", "r11", "memory"
    );
}

#undef mangled
