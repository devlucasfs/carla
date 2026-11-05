.extern __carla_abi_corelib_rmem_func__bookkeeping_set
.extern __carla_abi_corelib_rmem_func__bookkeeping_dump
.extern __carla_abi_corelib_rmem_func__dump
.extern __carla_abi_corelib_rmem_func__alloc
.extern __carla_abi_corelib_rmem_func__munmap
.extern __carla_abi_corelib_rmem_func__makeraw
.extern __carla_abi_corelib_rmem_func__size
.extern __carla_abi_corelib_nashj_func__start
.extern __carla_abi_corelib_nashj_func__alloc
.extern __carla_abi_corelib_nashj_func__dump
.extern __carla_abi_corelib_nashj_func__coalesce
.extern __carla_abi_corelib_nashj_func__destroy
.extern __carla_abi_corelib_nashj_func__makeraw
.extern __carla_routines_ctx_switch

.globl _start
.type _start, @function
_start:
.LFP54:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME54, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME54, 16
.LFE54:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP55:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME55, %rsp
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr13:
	.byte 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 10, 0
	.popsection
	leaq .Lstr13(%rip), %rax
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_stdlib_io_print
	movq $0, %rax
	jmp .LFE55
	.set .LFRAME55, 16
.LFE55:
	leave
	ret
.size main, . - main

.globl __carla_abi_extension___carla_abi_extension_stdlib_io_print
.type __carla_abi_extension___carla_abi_extension_stdlib_io_print, @function
__carla_abi_extension___carla_abi_extension_stdlib_io_print:
.LFP44:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME44, %rsp
	movq %rdi, -8(%rbp)
	leaq -8(%rbp), %rdi
	call __carla_abi_extension_byte_P_strlen
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rdx
	movq -8(%rbp), %rsi
	movq $1, %rdi
	call __carla_abi_extension_stdlib_write
	.set .LFRAME44, 16
.LFE44:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_stdlib_io_print, . - __carla_abi_extension___carla_abi_extension_stdlib_io_print

.globl __carla_abi_extension_byte_P_strlen
.type __carla_abi_extension_byte_P_strlen, @function
__carla_abi_extension_byte_P_strlen:
.LFP21:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME21, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
.Lb_L483a342c8829:
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	movq -16(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movb (%rax), %al
	movb %al, -33(%rbp)
	movzbq -33(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	setne %al
	movb %al, -34(%rbp)
	movb -34(%rbp), %al
	test %al, %al
	jz .Lb_Lb19f8f354315
.Lb_Lbc47f29a4a74:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .Lb_L483a342c8829
.Lb_Lb19f8f354315:
	movq -16(%rbp), %rax
	jmp .LFE21
	.set .LFRAME21, 48
.LFE21:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen, . - __carla_abi_extension_byte_P_strlen

.globl __carla_abi_extension_stdlib_write
.type __carla_abi_extension_stdlib_write, @function
__carla_abi_extension_stdlib_write:
.LFP41:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME41, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -8(%rbp), %rdi
	movq -16(%rbp), %rsi
	movq -24(%rbp), %rdx
	movq $1, %rax
	syscall
	movq %rax, -32(%rbp)
	.set .LFRAME41, 32
.LFE41:
	leave
	ret
.size __carla_abi_extension_stdlib_write, . - __carla_abi_extension_stdlib_write
