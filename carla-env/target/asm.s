
.globl __carla_abi_extension_stream_from
.type __carla_abi_extension_stream_from, @function
__carla_abi_extension_stream_from:
.LFP0:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME0, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -48(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -8(%rbp), %rdi
	leaq -48(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE0
	.set .LFRAME0, 48
.LFE0:
	leave
	ret
.size __carla_abi_extension_stream_from, . - __carla_abi_extension_stream_from

.globl __carla_abi_extension_stream_make
.type __carla_abi_extension_stream_make, @function
__carla_abi_extension_stream_make:
.LFP1:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME1, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr0:
	.byte 0
	.popsection
	leaq .Lstr0(%rip), %rax
	movq %rax, -24(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -48(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -8(%rbp), %rdi
	leaq -48(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE1
	.set .LFRAME1, 48
.LFE1:
	leave
	ret
.size __carla_abi_extension_stream_make, . - __carla_abi_extension_stream_make

.globl __carla_abi_extension_stream_from_str
.type __carla_abi_extension_stream_from_str, @function
__carla_abi_extension_stream_from_str:
.LFP2:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME2, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq $0, %rax
	movq %rax, -24(%rbp)
.Lb_L0293f01d4667:
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
	jz .Lb_Lfe5ba5607f13
.Lb_L42c343f7765d:
	movq -24(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -24(%rbp)
	jmp .Lb_L0293f01d4667
.Lb_Lfe5ba5607f13:
	movq -24(%rbp), %rax
	movq %rax, -64(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -56(%rbp)
	movq -8(%rbp), %rdi
	leaq -64(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE2
	.set .LFRAME2, 64
.LFE2:
	leave
	ret
.size __carla_abi_extension_stream_from_str, . - __carla_abi_extension_stream_from_str

.globl __carla_abi_extension_byte_P_strlen
.type __carla_abi_extension_byte_P_strlen, @function
__carla_abi_extension_byte_P_strlen:
.LFP3:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME3, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
.Lb_Lf9af4f4fcece:
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
	jz .Lb_L4952c7188b98
.Lb_Lab3838cfce9e:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .Lb_Lf9af4f4fcece
.Lb_L4952c7188b98:
	movq -16(%rbp), %rax
	jmp .LFE3
	.set .LFRAME3, 48
.LFE3:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen, . - __carla_abi_extension_byte_P_strlen

.globl __carla_abi_extension_byte_P_strlen_r
.type __carla_abi_extension_byte_P_strlen_r, @function
__carla_abi_extension_byte_P_strlen_r:
.LFP4:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME4, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
	movq $0, %rax
	movq %rax, -24(%rbp)
.Lb_L012fe45d3576:
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq -24(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movb (%rax), %al
	movb %al, -41(%rbp)
	movzbq -41(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	setne %al
	movb %al, -42(%rbp)
	movb -42(%rbp), %al
	test %al, %al
	jz .Lb_L5368943c4314
	movq -16(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -16(%rbp)
.Lb_Leb2e009b5e78:
	movq -24(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rax
	movq %rax, -24(%rbp)
	jmp .Lb_L012fe45d3576
.Lb_L5368943c4314:
	movq -16(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq %rax, -16(%rbp)
	.set .LFRAME4, 80
.LFE4:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen_r, . - __carla_abi_extension_byte_P_strlen_r

.globl __carla_abi_extension_byte_P_print
.type __carla_abi_extension_byte_P_print, @function
__carla_abi_extension_byte_P_print:
.LFP5:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME5, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -16(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_extension_byte_P_strlen
	movq %rax, -24(%rbp)
	movq $1, %rdi
	movq -16(%rbp), %rsi
	movq -24(%rbp), %rdx
	movq $1, %rax
	syscall
	movq %rax, -32(%rbp)
	.set .LFRAME5, 32
.LFE5:
	leave
	ret
.size __carla_abi_extension_byte_P_print, . - __carla_abi_extension_byte_P_print

.globl __carla_abi_extension_byte_P_eprint
.type __carla_abi_extension_byte_P_eprint, @function
__carla_abi_extension_byte_P_eprint:
.LFP6:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME6, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -16(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_extension_byte_P_strlen
	movq %rax, -24(%rbp)
	movq $1, %rdi
	movq -16(%rbp), %rsi
	movq -24(%rbp), %rdx
	movq $1, %rax
	syscall
	movq %rax, -32(%rbp)
	movq $1, %rdi
	movq $60, %rax
	syscall
	movq %rax, -40(%rbp)
	.set .LFRAME6, 48
.LFE6:
	leave
	ret
.size __carla_abi_extension_byte_P_eprint, . - __carla_abi_extension_byte_P_eprint

.globl __carla_abi_extension_byte_P_to_stream
.type __carla_abi_extension_byte_P_to_stream, @function
__carla_abi_extension_byte_P_to_stream:
.LFP7:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME7, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rdi
	call __carla_abi_extension_byte_P_strlen
	movq %rax, -24(%rbp)
	movq -16(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -32(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -48(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -8(%rbp), %rdi
	leaq -48(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE7
	.set .LFRAME7, 48
.LFE7:
	leave
	ret
.size __carla_abi_extension_byte_P_to_stream, . - __carla_abi_extension_byte_P_to_stream

.globl __carla_abi_extension_generic_ptr_to_stream
.type __carla_abi_extension_generic_ptr_to_stream, @function
__carla_abi_extension_generic_ptr_to_stream:
.LFP8:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME8, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -32(%rbp)
	leaq -32(%rbp), %rdi
	call __carla_abi_extension_byte_P_strlen
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movq %rax, -64(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -56(%rbp)
	movq -8(%rbp), %rdi
	leaq -64(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE8
	.set .LFRAME8, 64
.LFE8:
	leave
	ret
.size __carla_abi_extension_generic_ptr_to_stream, . - __carla_abi_extension_generic_ptr_to_stream

.globl __carla_abi_extension_int_print
.type __carla_abi_extension_int_print, @function
__carla_abi_extension_int_print:
.LFP9:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME9, %rsp
	movq %rdi, -8(%rbp)
	movb $0, %al
	movb %al, -9(%rbp)
	movq -8(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	setl %al
	movb %al, -33(%rbp)
	movb -33(%rbp), %al
	test %al, %al
	jz .Lb_L3ff17a6a7cb1
	movb $45, %al
	movb %al, -9(%rbp)
	movq $1, %rdi
	leaq -9(%rbp), %rsi
	movq $1, %rdx
	movq $1, %rax
	syscall
	movq %rax, -48(%rbp)
	movq $0, %rax
	movq -32(%rbp), %rcx
	subq %rcx, %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -32(%rbp)
.Lb_L3ff17a6a7cb1:
	movq $1, %rax
	movq %rax, -64(%rbp)
.Lb_L2a5eeca88ab7:
	movq -32(%rbp), %rax
	movq -64(%rbp), %rcx
	cqto
	idivq %rcx
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq $10, %rcx
	cmpq %rcx, %rax
	setge %al
	movb %al, -73(%rbp)
	movb -73(%rbp), %al
	test %al, %al
	jz .Lb_L0b9685862121
	movq -64(%rbp), %rax
	movq $10, %rcx
	imulq %rcx, %rax
	movq %rax, -88(%rbp)
	movq -88(%rbp), %rax
	movq %rax, -64(%rbp)
	jmp .Lb_L2a5eeca88ab7
.Lb_L0b9685862121:
.Lb_La11e0d7f56e3:
	movq -64(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	setg %al
	movb %al, -89(%rbp)
	movb -89(%rbp), %al
	test %al, %al
	jz .Lb_Lbe44f546d629
	movq -32(%rbp), %rax
	movq -64(%rbp), %rcx
	cqto
	idivq %rcx
	movq %rax, -104(%rbp)
	movq -104(%rbp), %rax
	movq $10, %rcx
	cqto
	idivq %rcx
	movq %rdx, %rax
	movq %rax, -112(%rbp)
	movq -112(%rbp), %rax
	movq $48, %rcx
	addq %rcx, %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movb %al, -121(%rbp)
	movb -121(%rbp), %al
	movb %al, -9(%rbp)
	movq $1, %rdi
	leaq -9(%rbp), %rsi
	movq $1, %rdx
	movq $1, %rax
	syscall
	movq %rax, -136(%rbp)
	movq -64(%rbp), %rax
	movq $10, %rcx
	cqto
	idivq %rcx
	movq %rax, -144(%rbp)
	movq -144(%rbp), %rax
	movq %rax, -64(%rbp)
	jmp .Lb_La11e0d7f56e3
.Lb_Lbe44f546d629:
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr1:
	.byte 10, 0
	.popsection
	leaq .Lstr1(%rip), %rax
	movq %rax, -152(%rbp)
	movq $1, %rdi
	movq -152(%rbp), %rsi
	movq $1, %rdx
	movq $1, %rax
	syscall
	movq %rax, -160(%rbp)
	.set .LFRAME9, 160
.LFE9:
	leave
	ret
.size __carla_abi_extension_int_print, . - __carla_abi_extension_int_print
.extern __carla_abi_corelib_nashj_func__start
.extern __carla_abi_corelib_nashj_func__alloc
.extern __carla_abi_corelib_nashj_func__dump
.extern __carla_abi_corelib_nashj_func__coalesce
.extern __carla_abi_corelib_nashj_func__destroy
.extern __carla_abi_corelib_nashj_func__makeraw

.globl __carla_abi_extension_Nashj_init
.type __carla_abi_extension_Nashj_init, @function
__carla_abi_extension_Nashj_init:
.LFP10:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME10, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	leaq -80(%rbp), %rdi
	movq -16(%rbp), %rsi
	call __carla_abi_corelib_nashj_func__start
	movq -8(%rbp), %rdi
	leaq -80(%rbp), %rsi
	movq $64, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE10
	.set .LFRAME10, 80
.LFE10:
	leave
	ret
.size __carla_abi_extension_Nashj_init, . - __carla_abi_extension_Nashj_init

.globl __carla_abi_extension_Nashj_alloc
.type __carla_abi_extension_Nashj_alloc, @function
__carla_abi_extension_Nashj_alloc:
.LFP11:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME11, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__alloc
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	jmp .LFE11
	.set .LFRAME11, 32
.LFE11:
	leave
	ret
.size __carla_abi_extension_Nashj_alloc, . - __carla_abi_extension_Nashj_alloc

.globl __carla_abi_extension_Nashj_dump
.type __carla_abi_extension_Nashj_dump, @function
__carla_abi_extension_Nashj_dump:
.LFP12:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME12, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__dump
	.set .LFRAME12, 16
.LFE12:
	leave
	ret
.size __carla_abi_extension_Nashj_dump, . - __carla_abi_extension_Nashj_dump

.globl __carla_abi_extension_Nashj_coalesce
.type __carla_abi_extension_Nashj_coalesce, @function
__carla_abi_extension_Nashj_coalesce:
.LFP13:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME13, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__coalesce
	.set .LFRAME13, 16
.LFE13:
	leave
	ret
.size __carla_abi_extension_Nashj_coalesce, . - __carla_abi_extension_Nashj_coalesce

.globl __carla_abi_extension_Nashj_deinit
.type __carla_abi_extension_Nashj_deinit, @function
__carla_abi_extension_Nashj_deinit:
.LFP14:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME14, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__destroy
	.set .LFRAME14, 16
.LFE14:
	leave
	ret
.size __carla_abi_extension_Nashj_deinit, . - __carla_abi_extension_Nashj_deinit

.globl __carla_abi_extension_Nashj_flush
.type __carla_abi_extension_Nashj_flush, @function
__carla_abi_extension_Nashj_flush:
.LFP15:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME15, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__dump
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__coalesce
	.set .LFRAME15, 16
.LFE15:
	leave
	ret
.size __carla_abi_extension_Nashj_flush, . - __carla_abi_extension_Nashj_flush

.globl __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
.type __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw, @function
__carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw:
.LFP16:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__makeraw
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	jmp .LFE16
	.set .LFRAME16, 32
.LFE16:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw, . - __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw

.globl __carla_abi_extension_error_from
.type __carla_abi_extension_error_from, @function
__carla_abi_extension_error_from:
.LFP17:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME17, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movb $255, %al
	movb %al, -17(%rbp)
	movb -17(%rbp), %al
	movb %al, -48(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -8(%rbp), %rdi
	leaq -48(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE17
	.set .LFRAME17, 48
.LFE17:
	leave
	ret
.size __carla_abi_extension_error_from, . - __carla_abi_extension_error_from

.globl __carla_abi_extension_error_make
.type __carla_abi_extension_error_make, @function
__carla_abi_extension_error_make:
.LFP18:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME18, %rsp
	movq %rdi, -8(%rbp)
	movb $0, %al
	movb %al, -9(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -24(%rbp)
	movb -9(%rbp), %al
	movb %al, -48(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -8(%rbp), %rdi
	leaq -48(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE18
	.set .LFRAME18, 48
.LFE18:
	leave
	ret
.size __carla_abi_extension_error_make, . - __carla_abi_extension_error_make

.globl __carla_abi_extension_error_panic
.type __carla_abi_extension_error_panic, @function
__carla_abi_extension_error_panic:
.LFP19:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME19, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -32(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -64(%rbp)
	movb $255, %al
	movb %al, -65(%rbp)
	movb -65(%rbp), %al
	test %al, %al
	jz .Lb_L636fbc86943a
.Lb_L636fbc86943a:
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_byte_P_eprint
	.set .LFRAME19, 80
.LFE19:
	leave
	ret
.size __carla_abi_extension_error_panic, . - __carla_abi_extension_error_panic

.globl __carla_abi_extension_error_msg
.type __carla_abi_extension_error_msg, @function
__carla_abi_extension_error_msg:
.LFP20:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME20, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -32(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -64(%rbp)
	movb $255, %al
	movb %al, -65(%rbp)
	movb -65(%rbp), %al
	test %al, %al
	jz .Lb_L6edde0bc67de
.Lb_L6edde0bc67de:
	movq -64(%rbp), %rax
	jmp .LFE20
	.set .LFRAME20, 80
.LFE20:
	leave
	ret
.size __carla_abi_extension_error_msg, . - __carla_abi_extension_error_msg

.globl __carla_abi_extension_error_isnt_empty
.type __carla_abi_extension_error_isnt_empty, @function
__carla_abi_extension_error_isnt_empty:
.LFP21:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME21, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -32(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movb (%rax), %al
	movb %al, -41(%rbp)
	movb -41(%rbp), %al
	movb %al, -42(%rbp)
	movb $255, %al
	movb %al, -43(%rbp)
	movb -43(%rbp), %al
	test %al, %al
	jz .Lb_L5ef96120a5bc
.Lb_L5ef96120a5bc:
	movb -42(%rbp), %al
	jmp .LFE21
	.set .LFRAME21, 48
.LFE21:
	leave
	ret
.size __carla_abi_extension_error_isnt_empty, . - __carla_abi_extension_error_isnt_empty
.extern __carla_abi_corelib_rmem_func__bookkeeping_set
.extern __carla_abi_corelib_rmem_func__bookkeeping_dump
.extern __carla_abi_corelib_rmem_func__dump
.extern __carla_abi_corelib_rmem_func__alloc

.globl __carla_abi_extension_SafeHeap_init
.type __carla_abi_extension_SafeHeap_init, @function
__carla_abi_extension_SafeHeap_init:
.LFP22:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME22, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__bookkeeping_set
	.set .LFRAME22, 16
.LFE22:
	leave
	ret
.size __carla_abi_extension_SafeHeap_init, . - __carla_abi_extension_SafeHeap_init

.globl __carla_abi_extension_SafeHeap_deinit
.type __carla_abi_extension_SafeHeap_deinit, @function
__carla_abi_extension_SafeHeap_deinit:
.LFP23:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME23, %rsp
	call __carla_abi_corelib_rmem_func__bookkeeping_dump
	.set .LFRAME23, 0
.LFE23:
	leave
	ret
.size __carla_abi_extension_SafeHeap_deinit, . - __carla_abi_extension_SafeHeap_deinit

.globl __carla_abi_extension_SafeHeap_dump
.type __carla_abi_extension_SafeHeap_dump, @function
__carla_abi_extension_SafeHeap_dump:
.LFP24:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME24, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__dump
	.set .LFRAME24, 16
.LFE24:
	leave
	ret
.size __carla_abi_extension_SafeHeap_dump, . - __carla_abi_extension_SafeHeap_dump

.globl __carla_abi_extension_SafeHeap_alloc
.type __carla_abi_extension_SafeHeap_alloc, @function
__carla_abi_extension_SafeHeap_alloc:
.LFP25:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME25, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__alloc
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	jmp .LFE25
	.set .LFRAME25, 16
.LFE25:
	leave
	ret
.size __carla_abi_extension_SafeHeap_alloc, . - __carla_abi_extension_SafeHeap_alloc
.extern __carla_abi_corelib_rmem_func__makeraw
.extern __carla_abi_corelib_rmem_func__size

.globl __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
.type __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw, @function
__carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw:
.LFP26:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME26, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__makeraw
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	jmp .LFE26
	.set .LFRAME26, 32
.LFE26:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw, . - __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw

.globl __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size
.type __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size, @function
__carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size:
.LFP27:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME27, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__size
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	jmp .LFE27
	.set .LFRAME27, 32
.LFE27:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size, . - __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size

.globl __carla_abi_extension_vector_init
.type __carla_abi_extension_vector_init, @function
__carla_abi_extension_vector_init:
.LFP28:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME28, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movq $4, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq $32, %rcx
	addq %rcx, %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rax
	movq %rax, -72(%rbp)
	leaq -144(%rbp), %rdi
	movq -72(%rbp), %rsi
	call __carla_abi_extension_Nashj_init
	movq $4, %rsi
	leaq -144(%rbp), %rdi
	call __carla_abi_extension_Nashj_alloc
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq %rax, -160(%rbp)
	movq -32(%rbp), %rsi
	leaq -144(%rbp), %rdi
	call __carla_abi_extension_Nashj_alloc
	movq %rax, -168(%rbp)
	movq -168(%rbp), %rax
	movq %rax, -176(%rbp)
	movq -64(%rbp), %rax
	movq %rax, -184(%rbp)
	leaq -144(%rbp), %rsi
	leaq -288(%rbp), %rdi
	movq $64, %rcx
	rep movsb
	movq -184(%rbp), %rax
	movq %rax, -224(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -208(%rbp)
	movq -176(%rbp), %rax
	movq %rax, -200(%rbp)
	movq -8(%rbp), %rdi
	leaq -288(%rbp), %rsi
	movq $96, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE28
	.set .LFRAME28, 288
.LFE28:
	leave
	ret
.size __carla_abi_extension_vector_init, . - __carla_abi_extension_vector_init

.globl __carla_abi_extension_vector_size
.type __carla_abi_extension_vector_size, @function
__carla_abi_extension_vector_size:
.LFP29:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME29, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -112(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	leaq -112(%rbp), %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq $80, %rcx
	addq %rcx, %rax
	movq %rax, -128(%rbp)
	movq -128(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -136(%rbp)
	movq -136(%rbp), %rax
	movq %rax, -144(%rbp)
	movb $255, %al
	movb %al, -145(%rbp)
	movb -145(%rbp), %al
	test %al, %al
	jz .Lb_L18f15fb8c81e
.Lb_L18f15fb8c81e:
	leaq -144(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -160(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movq -168(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -172(%rbp)
	movl -172(%rbp), %eax
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	jmp .LFE29
	.set .LFRAME29, 192
.LFE29:
	leave
	ret
.size __carla_abi_extension_vector_size, . - __carla_abi_extension_vector_size

.globl __carla_abi_extension_vector_deinit
.type __carla_abi_extension_vector_deinit, @function
__carla_abi_extension_vector_deinit:
.LFP30:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME30, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -112(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	leaq -112(%rbp), %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq %rax, %rsi
	leaq -192(%rbp), %rdi
	movq $64, %rcx
	rep movsb
	movq -120(%rbp), %rax
	movq $88, %rcx
	addq %rcx, %rax
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -208(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -216(%rbp)
	movb $255, %al
	movb %al, -217(%rbp)
	movb -217(%rbp), %al
	test %al, %al
	jz .Lb_Lfd002c3f93a8
.Lb_Lfd002c3f93a8:
	leaq -216(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -232(%rbp)
	movq -232(%rbp), %rax
	movq %rax, -240(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_extension_vector_size
	movq %rax, -248(%rbp)
	movq -248(%rbp), %rax
	movq %rax, -256(%rbp)
	movq $0, %rax
	movq %rax, -264(%rbp)
.Lb_Ld7be4261794d:
	movq -264(%rbp), %rax
	movq -256(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -265(%rbp)
	movb -265(%rbp), %al
	test %al, %al
	jz .Lb_Lcfb35fc4f0f9
	movq -264(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -280(%rbp)
	movq -240(%rbp), %rax
	movq -280(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -288(%rbp)
	movq -288(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -296(%rbp)
	movq -296(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_dump
.Lb_L3654ac148657:
	movq -264(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -304(%rbp)
	movq -304(%rbp), %rax
	movq %rax, -264(%rbp)
	jmp .Lb_Ld7be4261794d
.Lb_Lcfb35fc4f0f9:
	leaq -192(%rbp), %rdi
	call __carla_abi_extension_Nashj_deinit
	.set .LFRAME30, 304
.LFE30:
	leave
	ret
.size __carla_abi_extension_vector_deinit, . - __carla_abi_extension_vector_deinit

.globl __carla_abi_extension_vector_allocation_size
.type __carla_abi_extension_vector_allocation_size, @function
__carla_abi_extension_vector_allocation_size:
.LFP31:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME31, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -112(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	leaq -112(%rbp), %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq $64, %rcx
	addq %rcx, %rax
	movq %rax, -128(%rbp)
	movq -128(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -136(%rbp)
	movq -136(%rbp), %rax
	movq %rax, -144(%rbp)
	movb $255, %al
	movb %al, -145(%rbp)
	movb -145(%rbp), %al
	test %al, %al
	jz .Lb_L662b975c306b
.Lb_L662b975c306b:
	movq -144(%rbp), %rax
	jmp .LFE31
	.set .LFRAME31, 160
.LFE31:
	leave
	ret
.size __carla_abi_extension_vector_allocation_size, . - __carla_abi_extension_vector_allocation_size

.globl __carla_abi_extension_vector_push
.type __carla_abi_extension_vector_push, @function
__carla_abi_extension_vector_push:
.LFP32:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME32, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, %r11
	movq 0(%r11), %rax
	movq %rax, -32(%rbp)
	movq 8(%r11), %rax
	movq %rax, -24(%rbp)
	movq -16(%rbp), %rax
	movq %rax, %rsi
	leaq -128(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	leaq -128(%rbp), %rax
	movq %rax, -136(%rbp)
	movq -136(%rbp), %rax
	movq $72, %rcx
	addq %rcx, %rax
	movq %rax, -144(%rbp)
	movq -144(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq %rax, -160(%rbp)
	movq -136(%rbp), %rax
	movq $80, %rcx
	addq %rcx, %rax
	movq %rax, -168(%rbp)
	movq -168(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -176(%rbp)
	movq -176(%rbp), %rax
	movq %rax, -184(%rbp)
	movq -136(%rbp), %rax
	movq $88, %rcx
	addq %rcx, %rax
	movq %rax, -192(%rbp)
	movq -192(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq %rax, -208(%rbp)
	movb $255, %al
	movb %al, -209(%rbp)
	movb -209(%rbp), %al
	test %al, %al
	jz .Lb_La70d1610e968
.Lb_La70d1610e968:
	leaq -184(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, -232(%rbp)
	movq -232(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -236(%rbp)
	movl -236(%rbp), %eax
	movq %rax, -248(%rbp)
	movq -248(%rbp), %rax
	movq %rax, -256(%rbp)
	movq -160(%rbp), %rax
	movq -256(%rbp), %rcx
	cmpq %rcx, %rax
	setbe %al
	movb %al, -257(%rbp)
	movb -257(%rbp), %al
	test %al, %al
	jz .Lb_L226be051c047
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr2:
	.byte 86, 101, 99, 116, 111, 114, 32, 105, 115, 32, 97, 108, 114, 101, 97, 100
	.byte 121, 32, 102, 117, 108, 108, 0
	.popsection
	leaq .Lstr2(%rip), %rax
	movq %rax, -272(%rbp)
	leaq -288(%rbp), %rdi
	movq -272(%rbp), %rsi
	call __carla_abi_extension_error_from
	movq -8(%rbp), %rdi
	leaq -288(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE32
.Lb_L226be051c047:
	movq -232(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -292(%rbp)
	movl -292(%rbp), %eax
	movq $1, %rcx
	addq %rcx, %rax
	movl %eax, -296(%rbp)
	movq -232(%rbp), %rcx
	movl -296(%rbp), %eax
	movl %eax, (%rcx)
	leaq -32(%rbp), %rax
	movq %rax, -304(%rbp)
	movq -304(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -312(%rbp)
	movq -312(%rbp), %rax
	movq %rax, -320(%rbp)
	movq -304(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -336(%rbp)
	movq -336(%rbp), %rax
	movq %rax, -344(%rbp)
	movq -320(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	sete %al
	movb %al, -345(%rbp)
	movb -345(%rbp), %al
	test %al, %al
	jz .Lb_L4ab082054ca4
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr3:
	.byte 73, 110, 118, 97, 108, 105, 100, 32, 115, 116, 114, 101, 97, 109, 32, 114
	.byte 101, 99, 101, 105, 118, 101, 100, 58, 32, 83, 105, 122, 101, 32, 101, 113
	.byte 117, 97, 108, 115, 32, 116, 111, 32, 48, 10, 0
	.popsection
	leaq .Lstr3(%rip), %rax
	movq %rax, -360(%rbp)
	leaq -384(%rbp), %rdi
	movq -360(%rbp), %rsi
	call __carla_abi_extension_error_from
	movq -8(%rbp), %rdi
	leaq -384(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE32
.Lb_L4ab082054ca4:
	movq -320(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_alloc
	movq %rax, -392(%rbp)
	movq -392(%rbp), %rax
	movq %rax, -400(%rbp)
	leaq -400(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -408(%rbp)
	movq -408(%rbp), %rax
	movq %rax, -416(%rbp)
	movq -320(%rbp), %rax
	movq %rax, -424(%rbp)
	movq $0, %rax
	movq %rax, -432(%rbp)
.Lb_Ld8908c5dffff:
	movq -432(%rbp), %rax
	movq -424(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -433(%rbp)
	movb -433(%rbp), %al
	test %al, %al
	jz .Lb_L9d01f432e81e
	movq -416(%rbp), %rax
	movq -432(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -448(%rbp)
	movq -344(%rbp), %rax
	movq -432(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -456(%rbp)
	movq -456(%rbp), %rax
	movb (%rax), %al
	movb %al, -457(%rbp)
	movq -448(%rbp), %rcx
	movb -457(%rbp), %al
	movb %al, (%rcx)
.Lb_Lea4452500464:
	movq -432(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -472(%rbp)
	movq -472(%rbp), %rax
	movq %rax, -432(%rbp)
	jmp .Lb_Ld8908c5dffff
.Lb_L9d01f432e81e:
	leaq -208(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -480(%rbp)
	movq -480(%rbp), %rax
	movq %rax, -488(%rbp)
	movq -256(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -496(%rbp)
	movq -488(%rbp), %rax
	movq -496(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -504(%rbp)
	movq -504(%rbp), %rcx
	movq -400(%rbp), %rax
	movq %rax, (%rcx)
	leaq -528(%rbp), %rdi
	call __carla_abi_extension_error_make
	movq -8(%rbp), %rdi
	leaq -528(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE32
	.set .LFRAME32, 528
.LFE32:
	leave
	ret
.size __carla_abi_extension_vector_push, . - __carla_abi_extension_vector_push

.globl __carla_abi_extension_vector_get
.type __carla_abi_extension_vector_get, @function
__carla_abi_extension_vector_get:
.LFP33:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME33, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -16(%rbp), %rax
	movq %rax, %rsi
	leaq -128(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	leaq -128(%rbp), %rax
	movq %rax, -136(%rbp)
	movq -136(%rbp), %rax
	movq $80, %rcx
	addq %rcx, %rax
	movq %rax, -144(%rbp)
	movq -144(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq %rax, -160(%rbp)
	movq -136(%rbp), %rax
	movq $88, %rcx
	addq %rcx, %rax
	movq %rax, -168(%rbp)
	movq -168(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -176(%rbp)
	movq -176(%rbp), %rax
	movq %rax, -184(%rbp)
	movb $255, %al
	movb %al, -185(%rbp)
	movb -185(%rbp), %al
	test %al, %al
	jz .Lb_L47c24c0f596c
.Lb_L47c24c0f596c:
	movq -16(%rbp), %rdi
	call __carla_abi_extension_vector_size
	movq %rax, -200(%rbp)
	movq -24(%rbp), %rax
	movq -200(%rbp), %rcx
	cmpq %rcx, %rax
	setae %al
	movb %al, -201(%rbp)
	movb -201(%rbp), %al
	test %al, %al
	jz .Lb_L77f18d76cd06
	leaq -224(%rbp), %rdi
	call __carla_abi_extension_stream_make
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr4:
	.byte 105, 110, 100, 101, 120, 32, 99, 97, 110, 39, 116, 32, 98, 101, 32, 103
	.byte 114, 101, 97, 116, 101, 114, 32, 111, 114, 32, 101, 113, 117, 97, 108, 32
	.byte 116, 111, 32, 116, 104, 101, 32, 118, 101, 99, 116, 111, 114, 32, 115, 105
	.byte 122, 101, 46, 10, 0
	.popsection
	leaq .Lstr4(%rip), %rax
	movq %rax, -232(%rbp)
	leaq -256(%rbp), %rdi
	movq -232(%rbp), %rsi
	call __carla_abi_extension_error_from
	leaq -224(%rbp), %rsi
	leaq -288(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -256(%rbp), %rsi
	leaq -272(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -288(%rbp), %rsi
	movq $32, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE33
.Lb_L77f18d76cd06:
	leaq -184(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -296(%rbp)
	movq -296(%rbp), %rax
	movq %rax, -304(%rbp)
	movq -24(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -312(%rbp)
	movq -304(%rbp), %rax
	movq -312(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -320(%rbp)
	movq -320(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq %rax, -336(%rbp)
	leaq -336(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size
	movq %rax, -344(%rbp)
	leaq -336(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -352(%rbp)
	leaq -368(%rbp), %rdi
	movq -352(%rbp), %rdx
	movq -344(%rbp), %rsi
	call __carla_abi_extension_stream_from
	leaq -384(%rbp), %rdi
	call __carla_abi_extension_error_make
	leaq -368(%rbp), %rsi
	leaq -416(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -384(%rbp), %rsi
	leaq -400(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -416(%rbp), %rsi
	movq $32, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE33
	.set .LFRAME33, 416
.LFE33:
	leave
	ret
.size __carla_abi_extension_vector_get, . - __carla_abi_extension_vector_get

.globl _start
.type _start, @function
_start:
.LFP34:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME34, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME34, 16
.LFE34:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP35:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME35, %rsp
	movq $4096, %rdi
	call __carla_abi_extension_SafeHeap_init
	leaq -96(%rbp), %rdi
	movq $1024, %rsi
	call __carla_abi_extension_vector_init
	movq $0, %rax
	movq %rax, -104(%rbp)
.Lb_Le620df3a2c40:
	movq -104(%rbp), %rax
	movq $12, %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -105(%rbp)
	movb -105(%rbp), %al
	test %al, %al
	jz .Lb_Lf4a6975d747b
	movq -104(%rbp), %rax
	movq $2, %rcx
	xorl %edx, %edx
	divq %rcx
	movq %rdx, %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	sete %al
	movb %al, -121(%rbp)
	movb -121(%rbp), %al
	test %al, %al
	jz .Lb_L8a930a75cb28
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr5:
	.byte 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 0
	.popsection
	leaq .Lstr5(%rip), %rax
	movq %rax, -136(%rbp)
	leaq -160(%rbp), %rdi
	movq -136(%rbp), %rsi
	call __carla_abi_extension_stream_from_str
	leaq -176(%rbp), %rdi
	leaq -160(%rbp), %rdx
	leaq -96(%rbp), %rsi
	call __carla_abi_extension_vector_push
	jmp .Lb_L81bcb776ee34
.Lb_L8a930a75cb28:
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr6:
	.byte 73, 32, 108, 111, 118, 101, 32, 83, 97, 109, 97, 114, 33, 10, 0
	.popsection
	leaq .Lstr6(%rip), %rax
	movq %rax, -184(%rbp)
	leaq -208(%rbp), %rdi
	movq -184(%rbp), %rsi
	call __carla_abi_extension_stream_from_str
	leaq -224(%rbp), %rdi
	leaq -208(%rbp), %rdx
	leaq -96(%rbp), %rsi
	call __carla_abi_extension_vector_push
.Lb_L81bcb776ee34:
	movq -104(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -232(%rbp)
	movq -232(%rbp), %rax
	movq %rax, -104(%rbp)
	jmp .Lb_Le620df3a2c40
.Lb_Lf4a6975d747b:
	leaq -96(%rbp), %rdi
	call __carla_abi_extension_vector_size
	movq %rax, -240(%rbp)
	movq -240(%rbp), %rax
	movq %rax, -248(%rbp)
	movq $0, %rax
	movq %rax, -256(%rbp)
.Lb_Lcc4ac5d60b02:
	movq -256(%rbp), %rax
	movq -248(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -257(%rbp)
	movb -257(%rbp), %al
	test %al, %al
	jz .Lb_L3a0871f7c88d
	leaq -304(%rbp), %rdi
	movq -256(%rbp), %rdx
	leaq -96(%rbp), %rsi
	call __carla_abi_extension_vector_get
	leaq -304(%rbp), %rax
	movq %rax, -312(%rbp)
	movq -312(%rbp), %rax
	movq %rax, %rsi
	leaq -336(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -312(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -344(%rbp)
	movq -344(%rbp), %rax
	movq %rax, %rsi
	leaq -368(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -368(%rbp), %rdi
	call __carla_abi_extension_error_isnt_empty
	movb %al, -369(%rbp)
	movb -369(%rbp), %al
	test %al, %al
	jz .Lb_L4ea8ce5f871f
	leaq -368(%rbp), %rdi
	call __carla_abi_extension_error_panic
.Lb_L4ea8ce5f871f:
	leaq -336(%rbp), %rax
	movq %rax, -384(%rbp)
	movq -384(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -392(%rbp)
	movq -392(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -400(%rbp)
	movq -400(%rbp), %rax
	movq %rax, -408(%rbp)
	movb $255, %al
	movb %al, -409(%rbp)
	movb -409(%rbp), %al
	test %al, %al
	jz .Lb_L7e260ed01d9f
.Lb_L7e260ed01d9f:
	leaq -408(%rbp), %rdi
	call __carla_abi_extension_byte_P_print
.Lb_L5ad4d149583b:
	movq -256(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -424(%rbp)
	movq -424(%rbp), %rax
	movq %rax, -256(%rbp)
	jmp .Lb_Lcc4ac5d60b02
.Lb_L3a0871f7c88d:
	leaq -96(%rbp), %rdi
	call __carla_abi_extension_vector_deinit
	call __carla_abi_extension_SafeHeap_deinit
	movq $0, %rax
	jmp .LFE35
	leaq -96(%rbp), %rdi
	call __carla_abi_extension_vector_deinit
	call __carla_abi_extension_SafeHeap_deinit
	.set .LFRAME35, 432
.LFE35:
	leave
	ret
.size main, . - main
