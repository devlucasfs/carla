.extern nashj_start
.extern nashj_alloc
.extern nashj_dump
.extern nashj_coalesce
.extern nashj_destroy
.extern nashj_makeraw

.globl __carla_abi_extension_Nashj_init
.type __carla_abi_extension_Nashj_init, @function
__carla_abi_extension_Nashj_init:
.LFP0:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME0, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	leaq -80(%rbp), %rdi
	movq -16(%rbp), %rsi
	call nashj_start
	movq -8(%rbp), %rdi
	leaq -80(%rbp), %rsi
	movq $64, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE0
	.set .LFRAME0, 80
.LFE0:
	leave
	ret
.size __carla_abi_extension_Nashj_init, . - __carla_abi_extension_Nashj_init

.globl __carla_abi_extension_Nashj_alloc
.type __carla_abi_extension_Nashj_alloc, @function
__carla_abi_extension_Nashj_alloc:
.LFP1:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME1, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call nashj_alloc
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	jmp .LFE1
	.set .LFRAME1, 32
.LFE1:
	leave
	ret
.size __carla_abi_extension_Nashj_alloc, . - __carla_abi_extension_Nashj_alloc

.globl __carla_abi_extension_Nashj_dump
.type __carla_abi_extension_Nashj_dump, @function
__carla_abi_extension_Nashj_dump:
.LFP2:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME2, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call nashj_dump
	.set .LFRAME2, 16
.LFE2:
	leave
	ret
.size __carla_abi_extension_Nashj_dump, . - __carla_abi_extension_Nashj_dump

.globl __carla_abi_extension_Nashj_coalesce
.type __carla_abi_extension_Nashj_coalesce, @function
__carla_abi_extension_Nashj_coalesce:
.LFP3:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME3, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call nashj_coalesce
	.set .LFRAME3, 16
.LFE3:
	leave
	ret
.size __carla_abi_extension_Nashj_coalesce, . - __carla_abi_extension_Nashj_coalesce

.globl __carla_abi_extension_Nashj_deinit
.type __carla_abi_extension_Nashj_deinit, @function
__carla_abi_extension_Nashj_deinit:
.LFP4:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME4, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call nashj_destroy
	.set .LFRAME4, 16
.LFE4:
	leave
	ret
.size __carla_abi_extension_Nashj_deinit, . - __carla_abi_extension_Nashj_deinit

.globl __carla_abi_extension_Nashj_flush
.type __carla_abi_extension_Nashj_flush, @function
__carla_abi_extension_Nashj_flush:
.LFP5:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME5, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call nashj_dump
	movq -8(%rbp), %rdi
	call nashj_coalesce
	.set .LFRAME5, 16
.LFE5:
	leave
	ret
.size __carla_abi_extension_Nashj_flush, . - __carla_abi_extension_Nashj_flush

.globl __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
.type __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw, @function
__carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw:
.LFP6:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME6, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rdi
	call nashj_makeraw
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	jmp .LFE6
	.set .LFRAME6, 32
.LFE6:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw, . - __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw

.globl __carla_abi_extension_string_len
.type __carla_abi_extension_string_len, @function
__carla_abi_extension_string_len:
.LFP7:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME7, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
.Lb_L4c8b8495e06d:
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
	jz .Lb_Le403e703f3c0
	jmp .Lb_L5c686cfa5a2f
.Lb_Le403e703f3c0:
	movq -16(%rbp), %rax
	jmp .LFE7
.Lb_L5c686cfa5a2f:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .Lb_L4c8b8495e06d
.Lb_L304a803bdf20:
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr0:
	.byte 83, 116, 114, 105, 110, 103, 32, 115, 105, 122, 101, 32, 110, 111, 116, 32
	.byte 102, 111, 117, 110, 100, 33, 10, 0
	.popsection
	leaq .Lstr0(%rip), %rax
	movq %rax, -56(%rbp)
	movq $1, %rdi
	movq -56(%rbp), %rsi
	movq $24, %rdx
	movq $1, %rax
	syscall
	movq %rax, -64(%rbp)
	movq $0, %rax
	jmp .LFE7
	.set .LFRAME7, 64
.LFE7:
	leave
	ret
.size __carla_abi_extension_string_len, . - __carla_abi_extension_string_len

.globl __carla_abi_extension_string_print
.type __carla_abi_extension_string_print, @function
__carla_abi_extension_string_print:
.LFP8:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME8, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -24(%rbp)
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -32(%rbp)
	leaq -24(%rbp), %rdi
	call __carla_abi_extension_string_len
	movq %rax, -40(%rbp)
	movq $1, %rdi
	movq -32(%rbp), %rsi
	movq -40(%rbp), %rdx
	movq $1, %rax
	syscall
	movq %rax, -48(%rbp)
	.set .LFRAME8, 48
.LFE8:
	leave
	ret
.size __carla_abi_extension_string_print, . - __carla_abi_extension_string_print
.extern development_fill

.globl _start
.type _start, @function
_start:
.LFP9:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME9, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME9, 16
.LFE9:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP10:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME10, %rsp
	leaq -64(%rbp), %rdi
	movq $5120, %rsi
	call __carla_abi_extension_Nashj_init
	movq $512, %rsi
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_Nashj_alloc
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq %rax, -80(%rbp)
	movq $446, %rdx
	movq $14, %rsi
	movq -80(%rbp), %rdi
	call development_fill
	leaq -80(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -88(%rbp)
	movq -88(%rbp), %rax
	movq %rax, -96(%rbp)
	leaq -96(%rbp), %rdi
	call __carla_abi_extension_string_print
	movq $0, %rax
	movq %rax, -104(%rbp)
.Lb_L383d4b53320f:
	movq -104(%rbp), %rax
	movabsq $3000000000, %rcx
	cmpq %rcx, %rax
	setl %al
	movb %al, -105(%rbp)
	movb -105(%rbp), %al
	test %al, %al
	jz .Lb_L7cd2515f51dd
.Lb_La2f93228ca38:
	movq -104(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq %rax, -104(%rbp)
	jmp .Lb_L383d4b53320f
.Lb_L7cd2515f51dd:
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_Nashj_deinit
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr1:
	.byte 102, 114, 101, 101, 100, 33, 10, 0
	.popsection
	leaq .Lstr1(%rip), %rax
	movq %rax, -128(%rbp)
	movq -128(%rbp), %rax
	movq %rax, -136(%rbp)
	leaq -136(%rbp), %rdi
	call __carla_abi_extension_string_print
.Lb_Lcd746be22fd2:
	jmp .Lb_Lcd746be22fd2
.Lb_L7ea94c055a25:
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_Nashj_deinit
	movq $0, %rax
	jmp .LFE10
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_Nashj_deinit
	.set .LFRAME10, 144
.LFE10:
	leave
	ret
.size main, . - main
