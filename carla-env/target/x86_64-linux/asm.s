
.globl _start
.type _start, @function
_start:
.LFP7:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME7, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME7, 16
.LFE7:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP8:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME8, %rsp
	call __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu
	movq $5, %rdx
	movzbl (%rdx), %eax
	movb %al, -1(%rbp)
	movzbq -1(%rbp), %rax
	movq $32, %rcx
	orq %rcx, %rax
	movb %al, -2(%rbp)
	movzbq -2(%rbp), %rax
	movq $5, %rdx
	movb %al, (%rdx)
.Lb_L10d32f4bae25:
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr1:
	.byte 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 13, 0
	.popsection
	leaq .Lstr1(%rip), %rax
	movq %rax, -16(%rbp)
	movq $15, %rdx
	movq -16(%rbp), %rsi
	movq $1, %rdi
	call __carla_abi_extension_stdlib_write
	movq $5, %rdx
	movzbl (%rdx), %eax
	movb %al, -17(%rbp)
	movzbq -17(%rbp), %rax
	movq $32, %rcx
	orq %rcx, %rax
	movb %al, -18(%rbp)
	movzbq -18(%rbp), %rax
	movq $5, %rdx
	movb %al, (%rdx)
	movq $70000, %rdi
	call sleep
	movq $5, %rdx
	movzbl (%rdx), %eax
	movb %al, -19(%rbp)
	movzbq -19(%rbp), %rax
	movq $223, %rcx
	andq %rcx, %rax
	movb %al, -20(%rbp)
	movzbq -20(%rbp), %rax
	movq $5, %rdx
	movb %al, (%rdx)
	movq $70000, %rdi
	call sleep
	jmp .Lb_L10d32f4bae25
.Lb_Le2e1c1c7a278:
	movq $0, %rax
	jmp .LFE8
	.set .LFRAME8, 32
.LFE8:
	leave
	ret
.size main, . - main

.globl __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu
.type __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu, @function
__carla_abi_extension___carla_abi_extension_stdlib_serial_mcu:
.LFP2:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME2, %rsp
	.set .LFRAME2, 0
.LFE2:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu, . - __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu

.globl __carla_abi_extension_stdlib_write
.type __carla_abi_extension_stdlib_write, @function
__carla_abi_extension_stdlib_write:
.LFP0:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME0, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -8(%rbp), %rdi
	movq -16(%rbp), %rsi
	movq -24(%rbp), %rdx
	movq $1, %rax
	syscall
	movq %rax, -32(%rbp)
	.set .LFRAME0, 32
.LFE0:
	leave
	ret
.size __carla_abi_extension_stdlib_write, . - __carla_abi_extension_stdlib_write

.globl sleep
.type sleep, @function
sleep:
.LFP6:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME6, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
.Lb_Lb5ca1b7b8926:
	movq -16(%rbp), %rax
	movq -8(%rbp), %rcx
	cmpq %rcx, %rax
	setle %al
	movb %al, -17(%rbp)
	movb -17(%rbp), %al
	test %al, %al
	jz .Lb_L1d20aa893da0
.Lb_L0ecb1bef852c:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .Lb_Lb5ca1b7b8926
.Lb_L1d20aa893da0:
	.set .LFRAME6, 32
.LFE6:
	leave
	ret
.size sleep, . - sleep
