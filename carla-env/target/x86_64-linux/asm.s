
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
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME0, 16
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
	movq $0, %rax
	jmp .LFE1
	.set .LFRAME1, 0
.LFE1:
	leave
	ret
.size main, . - main
