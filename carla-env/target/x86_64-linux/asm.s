.text
.globl _start
.type _start, @function
_start:
.LFP0:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME0, %rsp
	xorl %eax, %eax
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
.set .LFRAME0, 16
.LFE0:
	movq %rax, %rdi
	movl $60, %eax
	syscall
.size _start, .-_start

.text
.globl main
.type main, @function
main:
.LFP1:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME1, %rsp
	movups %xmm0, -64(%rbp)
	movq $12, %rax
	movl %eax, -64(%rbp)
	movups -64(%rbp), %xmm0
	movups %xmm0, -64(%rbp)
	movq $25, %rax
	movl %eax, -60(%rbp)
	movups -64(%rbp), %xmm0
	movups %xmm0, -64(%rbp)
	movq $37, %rax
	movl %eax, -56(%rbp)
	movups -64(%rbp), %xmm0
	movups %xmm0, -64(%rbp)
	movq $49, %rax
	movl %eax, -52(%rbp)
	movups -64(%rbp), %xmm0
	movups %xmm0, -64(%rbp)
	movl -64(%rbp), %eax
	movl %eax, -68(%rbp)
	movl -68(%rbp), %eax
	movq $3, %rcx
	imulq %rcx, %rax
	movl %eax, -72(%rbp)
	xorl %eax, %eax
	movl -72(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -76(%rbp)
	movups %xmm0, -64(%rbp)
	movl -60(%rbp), %eax
	movl %eax, -80(%rbp)
	movl -80(%rbp), %eax
	movq $5, %rcx
	subq %rcx, %rax
	movl %eax, -84(%rbp)
	movl -76(%rbp), %eax
	movl -84(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -88(%rbp)
	movups %xmm1, -64(%rbp)
	movl -88(%rbp), %eax
	movl %eax, -64(%rbp)
	movups -64(%rbp), %xmm1
	movups %xmm0, -64(%rbp)
	movl -60(%rbp), %eax
	movl %eax, -92(%rbp)
	movl -92(%rbp), %eax
	movq $2, %rcx
	imulq %rcx, %rax
	movl %eax, -96(%rbp)
	xorl %eax, %eax
	movl -96(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -100(%rbp)
	movups %xmm0, -64(%rbp)
	movl -56(%rbp), %eax
	movl %eax, -104(%rbp)
	movl -104(%rbp), %eax
	movq $2, %rcx
	movl %eax, %eax
	movl %ecx, %ecx
	xorl %edx, %edx
	divq %rcx
	movl %eax, -108(%rbp)
	movl -100(%rbp), %eax
	movl -108(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -112(%rbp)
	movups %xmm1, -64(%rbp)
	movl -112(%rbp), %eax
	movl %eax, -60(%rbp)
	movups -64(%rbp), %xmm1
	movups %xmm0, -64(%rbp)
	movl -56(%rbp), %eax
	movl %eax, -116(%rbp)
	movl -116(%rbp), %eax
	movq $4, %rcx
	imulq %rcx, %rax
	movl %eax, -120(%rbp)
	xorl %eax, %eax
	movl -120(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -124(%rbp)
	movups %xmm0, -64(%rbp)
	movl -52(%rbp), %eax
	movl %eax, -128(%rbp)
	movl -128(%rbp), %eax
	movq $2, %rcx
	imulq %rcx, %rax
	movl %eax, -132(%rbp)
	movl -124(%rbp), %eax
	movl -132(%rbp), %ecx
	subq %rcx, %rax
	movl %eax, -136(%rbp)
	movups %xmm1, -64(%rbp)
	movl -136(%rbp), %eax
	movl %eax, -56(%rbp)
	movups -64(%rbp), %xmm1
	movups %xmm0, -64(%rbp)
	movl -52(%rbp), %eax
	movl %eax, -140(%rbp)
	movups %xmm0, -64(%rbp)
	movl -64(%rbp), %eax
	movl %eax, -144(%rbp)
	movl -140(%rbp), %eax
	movl -144(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -148(%rbp)
	movl -148(%rbp), %eax
	movq $3, %rcx
	imulq %rcx, %rax
	movl %eax, -152(%rbp)
	xorl %eax, %eax
	movl -152(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -156(%rbp)
	movups %xmm1, -64(%rbp)
	movl -156(%rbp), %eax
	movl %eax, -52(%rbp)
	movups -64(%rbp), %xmm1
	movups %xmm1, -64(%rbp)
	movl -64(%rbp), %eax
	movl %eax, -160(%rbp)
	movl -160(%rbp), %eax
	movq %rax, -168(%rbp)
	movups %xmm1, -64(%rbp)
	movl -60(%rbp), %eax
	movl %eax, -172(%rbp)
	movl -172(%rbp), %eax
	movq %rax, -184(%rbp)
	movq -168(%rbp), %rax
	movq -184(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -192(%rbp)
	movups %xmm1, -64(%rbp)
	movl -56(%rbp), %eax
	movl %eax, -196(%rbp)
	movl -196(%rbp), %eax
	movq %rax, -208(%rbp)
	movq -192(%rbp), %rax
	movq -208(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -216(%rbp)
	movups %xmm1, -64(%rbp)
	movl -52(%rbp), %eax
	movl %eax, -220(%rbp)
	movl -220(%rbp), %eax
	movq %rax, -232(%rbp)
	movq -216(%rbp), %rax
	movq -232(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -240(%rbp)
	movq -240(%rbp), %rax
	movq %rax, -248(%rbp)
	movq $357, %rax
	movq %rax, -256(%rbp)
	movq -248(%rbp), %rax
	movq -256(%rbp), %rcx
	cmpq %rcx, %rax
	setne %al
	movb %al, -257(%rbp)
	movzbq -257(%rbp), %rax
	testq %rax, %rax
	jz .L1_L7d307d16a84a
	movq $1, %rax
	jmp .LFE1
.L1_L7d307d16a84a:
	movups %xmm1, -64(%rbp)
	movl -64(%rbp), %eax
	movl %eax, -264(%rbp)
	movl -264(%rbp), %eax
	movq $2, %rcx
	imulq %rcx, %rax
	movl %eax, -268(%rbp)
	xorl %eax, %eax
	movl -268(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -272(%rbp)
	movups %xmm1, -64(%rbp)
	movl -60(%rbp), %eax
	movl %eax, -276(%rbp)
	movl -276(%rbp), %eax
	movq $3, %rcx
	imulq %rcx, %rax
	movl %eax, -280(%rbp)
	movl -272(%rbp), %eax
	movl -280(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -284(%rbp)
	movups %xmm1, -64(%rbp)
	movl -56(%rbp), %eax
	movl %eax, -288(%rbp)
	movl -288(%rbp), %eax
	movq $4, %rcx
	imulq %rcx, %rax
	movl %eax, -292(%rbp)
	movl -284(%rbp), %eax
	movl -292(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -296(%rbp)
	movups %xmm1, -64(%rbp)
	movl -52(%rbp), %eax
	movl %eax, -300(%rbp)
	movl -300(%rbp), %eax
	movq $5, %rcx
	imulq %rcx, %rax
	movl %eax, -304(%rbp)
	movl -296(%rbp), %eax
	movl -304(%rbp), %ecx
	addq %rcx, %rax
	movl %eax, -308(%rbp)
	movl -308(%rbp), %eax
	movl %eax, -312(%rbp)
	movl -312(%rbp), %eax
	movq $1453, %rcx
	movl %eax, %eax
	movl %ecx, %ecx
	cmpq %rcx, %rax
	setne %al
	movb %al, -313(%rbp)
	movzbq -313(%rbp), %rax
	testq %rax, %rax
	jz .L1_L5412edfe98f0
	movq $2, %rax
	jmp .LFE1
.L1_L5412edfe98f0:
	xorl %eax, %eax
	jmp .LFE1
.set .LFRAME1, 320
.LFE1:
	leave
	ret
.size main, .-main

