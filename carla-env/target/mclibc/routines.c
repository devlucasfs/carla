#define _GNU_SOURCE
#include <stdint.h>

__asm__(".text\n"
        ".globl __carla_routines_ctx_switch\n"
        "__carla_routines_ctx_switch:\n"
        "    pushq %rbp\n"
        "    pushq %rbx\n"
        "    pushq %r12\n"
        "    pushq %r13\n"
        "    pushq %r14\n"
        "    pushq %r15\n"

        "    movq %rsp, (%rdi)\n"
        "    movq %rsi, %rsp\n"

        "    popq %r15\n"
        "    popq %r14\n"
        "    popq %r13\n"
        "    popq %r12\n"
        "    popq %rbx\n"
        "    popq %rbp\n"

        "    movq %rdx, %rdi\n"
        "    ret\n");
void __carla_routines_ctx_switch(void **save_sp, void *load_sp, void *manager);
