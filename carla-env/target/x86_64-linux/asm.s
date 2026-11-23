.text
.globl cycles_sleep
.type cycles_sleep, @function
cycles_sleep:
.LFP0:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME0, %rsp
	movq %rdi, -8(%rbp)
	xorl %eax, %eax
	movq %rax, -16(%rbp)
.L0_L9efcd000882e:
	movq -16(%rbp), %rax
	movq -8(%rbp), %rcx
	cmpq %rcx, %rax
	setle %al
	movb %al, -17(%rbp)
	movzbq -17(%rbp), %rax
	testq %rax, %rax
	jz .L0_Lae567dba6b08
.L0_Lf0d06bb62e6e:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .L0_L9efcd000882e
.L0_Lae567dba6b08:
.set .LFRAME0, 32
.LFE0:
	leave
	ret
.size cycles_sleep, .-cycles_sleep

.text
.globl _start
.type _start, @function
_start:
.LFP1:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME1, %rsp
	xorl %eax, %eax
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
.set .LFRAME1, 16
.LFE1:
	movq %rax, %rdi
	movl $60, %eax
	syscall
.size _start, .-_start

.text
.globl main
.type main, @function
main:
.LFP2:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME2, %rsp
	movq $6144, %rax
	movabsq $1610649612, %rdx
	movl %eax, (%rdx)
	movq $256, %rax
	movabsq $1610630492, %rdx
	movl %eax, (%rdx)
	movq $4, %rax
	movabsq $1610629156, %rdx
	movl %eax, (%rdx)
.L2_L7016b192b4c1:
	movq $4, %rax
	movabsq $1610629128, %rdx
	movl %eax, (%rdx)
	movq $10000, %rdi
	xorl %eax, %eax
	call cycles_sleep
	movq $4, %rax
	movabsq $1610629132, %rdx
	movl %eax, (%rdx)
	movq $10000, %rdi
	xorl %eax, %eax
	call cycles_sleep
	jmp .L2_L7016b192b4c1
.L2_La187b7a3765a:
.set .LFRAME2, 0
.LFE2:
	leave
	ret
.size main, .-main

