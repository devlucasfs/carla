
.globl _start
.type _start, @function
_start:
.LFP0:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME0, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -24(%rbp)
	.set .LFRAME0, 32
.LFE0:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP1:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME1, %rsp
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr0:
	.byte 72, 101, 108, 108, 111, 44, 32, 102, 117, 99, 107, 105, 110, 103, 32, 119
	.byte 111, 114, 108, 100, 33, 32, 73, 32, 108, 111, 118, 101, 32, 83, 97, 109
	.byte 97, 114, 32, 97, 110, 100, 32, 105, 32, 97, 109, 32, 100, 111, 105, 110
	.byte 103, 32, 116, 104, 105, 115, 32, 115, 104, 105, 116, 32, 99, 117, 122, 32
	.byte 105, 32, 100, 111, 110, 116, 32, 119, 97, 110, 110, 97, 32, 117, 115, 101
	.byte 32, 97, 32, 115, 116, 100, 108, 105, 98, 10, 0
	.popsection
	leaq .Lstr0(%rip), %rax
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -16(%rbp)
	movq $0, %rax
	movq %rax, -24(%rbp)
.Lb_L70dfe32fae6c:
	movq -16(%rbp), %rax
	movq -24(%rbp), %rcx
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
	jz .Lb_Lb984379b31e0
	movq -24(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -24(%rbp)
	jmp .Lb_L70dfe32fae6c
.Lb_Lb984379b31e0:
	movq $1, %rdi
	movq -16(%rbp), %rsi
	movq -24(%rbp), %rdx
	movq $1, %rax
	syscall
	movq %rax, -56(%rbp)
	movq $0, %rax
	jmp .LFE1
	.set .LFRAME1, 64
.LFE1:
	leave
	ret
.size main, . - main
