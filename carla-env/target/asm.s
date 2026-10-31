
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
.Lb_L30cd8ce82947:
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
	jz .Lb_L85190ce0213c
.Lb_L4d42e36c8f81:
	movq -24(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -24(%rbp)
	jmp .Lb_L30cd8ce82947
.Lb_L85190ce0213c:
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

.globl __carla_abi_extension_stream_from_ptr
.type __carla_abi_extension_stream_from_ptr, @function
__carla_abi_extension_stream_from_ptr:
.LFP3:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME3, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq $0, %rax
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -48(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -8(%rbp), %rdi
	leaq -48(%rbp), %rsi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE3
	.set .LFRAME3, 48
.LFE3:
	leave
	ret
.size __carla_abi_extension_stream_from_ptr, . - __carla_abi_extension_stream_from_ptr

.globl __carla_abi_extension_stream_to_ptr
.type __carla_abi_extension_stream_to_ptr, @function
__carla_abi_extension_stream_to_ptr:
.LFP4:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME4, %rsp
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
	movb $0, %al
	movb %al, -65(%rbp)
	movb -65(%rbp), %al
	test %al, %al
	jz .Lb_La6588aa0050a
.Lb_La6588aa0050a:
	movq -64(%rbp), %rax
	jmp .LFE4
	.set .LFRAME4, 80
.LFE4:
	leave
	ret
.size __carla_abi_extension_stream_to_ptr, . - __carla_abi_extension_stream_to_ptr

.globl __carla_abi_extension_byte_P_strlen
.type __carla_abi_extension_byte_P_strlen, @function
__carla_abi_extension_byte_P_strlen:
.LFP5:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME5, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
.Lb_L1e3435e79290:
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
	jz .Lb_L8a3cdc761a79
.Lb_Laeb1e55b76b0:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .Lb_L1e3435e79290
.Lb_L8a3cdc761a79:
	movq -16(%rbp), %rax
	jmp .LFE5
	.set .LFRAME5, 48
.LFE5:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen, . - __carla_abi_extension_byte_P_strlen

.globl __carla_abi_extension_byte_P_strlen_r
.type __carla_abi_extension_byte_P_strlen_r, @function
__carla_abi_extension_byte_P_strlen_r:
.LFP6:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME6, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
	movq $0, %rax
	movq %rax, -24(%rbp)
.Lb_L3ef8a3a5c7e1:
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
	jz .Lb_L0a826b158f47
	movq -16(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -16(%rbp)
.Lb_Lfa8f53e9e4eb:
	movq -24(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rax
	movq %rax, -24(%rbp)
	jmp .Lb_L3ef8a3a5c7e1
.Lb_L0a826b158f47:
	movq -16(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq %rax, -16(%rbp)
	.set .LFRAME6, 80
.LFE6:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen_r, . - __carla_abi_extension_byte_P_strlen_r

.globl __carla_abi_extension_byte_P_print
.type __carla_abi_extension_byte_P_print, @function
__carla_abi_extension_byte_P_print:
.LFP7:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME7, %rsp
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
	.set .LFRAME7, 32
.LFE7:
	leave
	ret
.size __carla_abi_extension_byte_P_print, . - __carla_abi_extension_byte_P_print

.globl __carla_abi_extension_byte_P_eprint
.type __carla_abi_extension_byte_P_eprint, @function
__carla_abi_extension_byte_P_eprint:
.LFP8:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME8, %rsp
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
	.set .LFRAME8, 48
.LFE8:
	leave
	ret
.size __carla_abi_extension_byte_P_eprint, . - __carla_abi_extension_byte_P_eprint

.globl __carla_abi_extension_byte_P_to_stream
.type __carla_abi_extension_byte_P_to_stream, @function
__carla_abi_extension_byte_P_to_stream:
.LFP9:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME9, %rsp
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
	jmp .LFE9
	.set .LFRAME9, 48
.LFE9:
	leave
	ret
.size __carla_abi_extension_byte_P_to_stream, . - __carla_abi_extension_byte_P_to_stream

.globl __carla_abi_extension_generic_ptr_to_stream
.type __carla_abi_extension_generic_ptr_to_stream, @function
__carla_abi_extension_generic_ptr_to_stream:
.LFP10:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME10, %rsp
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
	jmp .LFE10
	.set .LFRAME10, 64
.LFE10:
	leave
	ret
.size __carla_abi_extension_generic_ptr_to_stream, . - __carla_abi_extension_generic_ptr_to_stream

.globl __carla_abi_extension_int_print
.type __carla_abi_extension_int_print, @function
__carla_abi_extension_int_print:
.LFP11:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME11, %rsp
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
	jz .Lb_L364ce0cd1a7c
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
.Lb_L364ce0cd1a7c:
	movq $1, %rax
	movq %rax, -64(%rbp)
.Lb_L6aedd285421f:
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
	jz .Lb_L3da062ed5fa6
	movq -64(%rbp), %rax
	movq $10, %rcx
	imulq %rcx, %rax
	movq %rax, -88(%rbp)
	movq -88(%rbp), %rax
	movq %rax, -64(%rbp)
	jmp .Lb_L6aedd285421f
.Lb_L3da062ed5fa6:
.Lb_L5f1cd61b833e:
	movq -64(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	setg %al
	movb %al, -89(%rbp)
	movb -89(%rbp), %al
	test %al, %al
	jz .Lb_Lb5bfff85d0ac
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
	jmp .Lb_L5f1cd61b833e
.Lb_Lb5bfff85d0ac:
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
	.set .LFRAME11, 160
.LFE11:
	leave
	ret
.size __carla_abi_extension_int_print, . - __carla_abi_extension_int_print

.globl __carla_abi_extension_error_from
.type __carla_abi_extension_error_from, @function
__carla_abi_extension_error_from:
.LFP12:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME12, %rsp
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
	jmp .LFE12
	.set .LFRAME12, 48
.LFE12:
	leave
	ret
.size __carla_abi_extension_error_from, . - __carla_abi_extension_error_from

.globl __carla_abi_extension_error_make
.type __carla_abi_extension_error_make, @function
__carla_abi_extension_error_make:
.LFP13:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME13, %rsp
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
	jmp .LFE13
	.set .LFRAME13, 48
.LFE13:
	leave
	ret
.size __carla_abi_extension_error_make, . - __carla_abi_extension_error_make

.globl __carla_abi_extension_error_panic
.type __carla_abi_extension_error_panic, @function
__carla_abi_extension_error_panic:
.LFP14:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME14, %rsp
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
	jz .Lb_L90700c04ee81
.Lb_L90700c04ee81:
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_byte_P_eprint
	.set .LFRAME14, 80
.LFE14:
	leave
	ret
.size __carla_abi_extension_error_panic, . - __carla_abi_extension_error_panic

.globl __carla_abi_extension_error_msg
.type __carla_abi_extension_error_msg, @function
__carla_abi_extension_error_msg:
.LFP15:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME15, %rsp
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
	jz .Lb_L9b2d93086d63
.Lb_L9b2d93086d63:
	movq -64(%rbp), %rax
	jmp .LFE15
	.set .LFRAME15, 80
.LFE15:
	leave
	ret
.size __carla_abi_extension_error_msg, . - __carla_abi_extension_error_msg

.globl __carla_abi_extension_error_isnt_empty
.type __carla_abi_extension_error_isnt_empty, @function
__carla_abi_extension_error_isnt_empty:
.LFP16:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME16, %rsp
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
	jz .Lb_L06a37a3d2df5
.Lb_L06a37a3d2df5:
	movb -42(%rbp), %al
	jmp .LFE16
	.set .LFRAME16, 48
.LFE16:
	leave
	ret
.size __carla_abi_extension_error_isnt_empty, . - __carla_abi_extension_error_isnt_empty

.globl __carla_abi_extension_error_is_empty
.type __carla_abi_extension_error_is_empty, @function
__carla_abi_extension_error_is_empty:
.LFP17:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME17, %rsp
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
	jz .Lb_L81832be6efe2
.Lb_L81832be6efe2:
	movb -42(%rbp), %al
	test %al, %al
	jz .Lb_L3e3de6e4c3cf
	movq $0, %rax
	jmp .LFE17
.Lb_L3e3de6e4c3cf:
	movq $255, %rax
	jmp .LFE17
	.set .LFRAME17, 48
.LFE17:
	leave
	ret
.size __carla_abi_extension_error_is_empty, . - __carla_abi_extension_error_is_empty
.extern __carla_abi_corelib_nashj_func__start
.extern __carla_abi_corelib_nashj_func__alloc
.extern __carla_abi_corelib_nashj_func__dump
.extern __carla_abi_corelib_nashj_func__coalesce
.extern __carla_abi_corelib_nashj_func__destroy
.extern __carla_abi_corelib_nashj_func__makeraw

.globl __carla_abi_extension_Nashj_init
.type __carla_abi_extension_Nashj_init, @function
__carla_abi_extension_Nashj_init:
.LFP18:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME18, %rsp
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
	jmp .LFE18
	.set .LFRAME18, 80
.LFE18:
	leave
	ret
.size __carla_abi_extension_Nashj_init, . - __carla_abi_extension_Nashj_init

.globl __carla_abi_extension_Nashj_alloc
.type __carla_abi_extension_Nashj_alloc, @function
__carla_abi_extension_Nashj_alloc:
.LFP19:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME19, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__alloc
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	jmp .LFE19
	.set .LFRAME19, 32
.LFE19:
	leave
	ret
.size __carla_abi_extension_Nashj_alloc, . - __carla_abi_extension_Nashj_alloc

.globl __carla_abi_extension_Nashj_dump
.type __carla_abi_extension_Nashj_dump, @function
__carla_abi_extension_Nashj_dump:
.LFP20:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME20, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__dump
	.set .LFRAME20, 16
.LFE20:
	leave
	ret
.size __carla_abi_extension_Nashj_dump, . - __carla_abi_extension_Nashj_dump

.globl __carla_abi_extension_Nashj_coalesce
.type __carla_abi_extension_Nashj_coalesce, @function
__carla_abi_extension_Nashj_coalesce:
.LFP21:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME21, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__coalesce
	.set .LFRAME21, 16
.LFE21:
	leave
	ret
.size __carla_abi_extension_Nashj_coalesce, . - __carla_abi_extension_Nashj_coalesce

.globl __carla_abi_extension_Nashj_deinit
.type __carla_abi_extension_Nashj_deinit, @function
__carla_abi_extension_Nashj_deinit:
.LFP22:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME22, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__destroy
	.set .LFRAME22, 16
.LFE22:
	leave
	ret
.size __carla_abi_extension_Nashj_deinit, . - __carla_abi_extension_Nashj_deinit

.globl __carla_abi_extension_Nashj_flush
.type __carla_abi_extension_Nashj_flush, @function
__carla_abi_extension_Nashj_flush:
.LFP23:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME23, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__dump
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_nashj_func__coalesce
	.set .LFRAME23, 16
.LFE23:
	leave
	ret
.size __carla_abi_extension_Nashj_flush, . - __carla_abi_extension_Nashj_flush

.globl __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
.type __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw, @function
__carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw:
.LFP24:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME24, %rsp
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
	jmp .LFE24
	.set .LFRAME24, 32
.LFE24:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw, . - __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
.extern __carla_abi_corelib_rmem_func__bookkeeping_set
.extern __carla_abi_corelib_rmem_func__bookkeeping_dump
.extern __carla_abi_corelib_rmem_func__dump
.extern __carla_abi_corelib_rmem_func__alloc

.globl __carla_abi_extension_SafeHeap_init
.type __carla_abi_extension_SafeHeap_init, @function
__carla_abi_extension_SafeHeap_init:
.LFP25:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME25, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__bookkeeping_set
	.set .LFRAME25, 16
.LFE25:
	leave
	ret
.size __carla_abi_extension_SafeHeap_init, . - __carla_abi_extension_SafeHeap_init

.globl __carla_abi_extension_SafeHeap_deinit
.type __carla_abi_extension_SafeHeap_deinit, @function
__carla_abi_extension_SafeHeap_deinit:
.LFP26:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME26, %rsp
	call __carla_abi_corelib_rmem_func__bookkeeping_dump
	.set .LFRAME26, 0
.LFE26:
	leave
	ret
.size __carla_abi_extension_SafeHeap_deinit, . - __carla_abi_extension_SafeHeap_deinit

.globl __carla_abi_extension_SafeHeap_dump
.type __carla_abi_extension_SafeHeap_dump, @function
__carla_abi_extension_SafeHeap_dump:
.LFP27:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME27, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__dump
	.set .LFRAME27, 16
.LFE27:
	leave
	ret
.size __carla_abi_extension_SafeHeap_dump, . - __carla_abi_extension_SafeHeap_dump

.globl __carla_abi_extension_SafeHeap_alloc
.type __carla_abi_extension_SafeHeap_alloc, @function
__carla_abi_extension_SafeHeap_alloc:
.LFP28:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME28, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	call __carla_abi_corelib_rmem_func__alloc
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	jmp .LFE28
	.set .LFRAME28, 16
.LFE28:
	leave
	ret
.size __carla_abi_extension_SafeHeap_alloc, . - __carla_abi_extension_SafeHeap_alloc

.globl __carla_abi_extension_SafeHeap_memcpy
.type __carla_abi_extension_SafeHeap_memcpy, @function
__carla_abi_extension_SafeHeap_memcpy:
.LFP29:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME29, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -32(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -48(%rbp)
	movq $0, %rax
	movq %rax, -56(%rbp)
.Lb_L1fe64c970c48:
	movq -56(%rbp), %rax
	movq -48(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -57(%rbp)
	movb -57(%rbp), %al
	test %al, %al
	jz .Lb_L48c3b957cdc9
	movq -32(%rbp), %rax
	movq -56(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -72(%rbp)
	movq -40(%rbp), %rax
	movq -56(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -80(%rbp)
	movq -80(%rbp), %rax
	movb (%rax), %al
	movb %al, -81(%rbp)
	movq -72(%rbp), %rcx
	movb -81(%rbp), %al
	movb %al, (%rcx)
.Lb_L75e8c329ed2c:
	movq -56(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -96(%rbp)
	movq -96(%rbp), %rax
	movq %rax, -56(%rbp)
	jmp .Lb_L1fe64c970c48
.Lb_L48c3b957cdc9:
	.set .LFRAME29, 96
.LFE29:
	leave
	ret
.size __carla_abi_extension_SafeHeap_memcpy, . - __carla_abi_extension_SafeHeap_memcpy
.extern __carla_abi_corelib_rmem_func__makeraw
.extern __carla_abi_corelib_rmem_func__size

.globl __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
.type __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw, @function
__carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw:
.LFP30:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME30, %rsp
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
	jmp .LFE30
	.set .LFRAME30, 32
.LFE30:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw, . - __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw

.globl __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size
.type __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size, @function
__carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size:
.LFP31:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME31, %rsp
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
	jmp .LFE31
	.set .LFRAME31, 32
.LFE31:
	leave
	ret
.size __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size, . - __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size

.globl __carla_abi_extension_vector_init
.type __carla_abi_extension_vector_init, @function
__carla_abi_extension_vector_init:
.LFP32:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME32, %rsp
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
	jmp .LFE32
	.set .LFRAME32, 288
.LFE32:
	leave
	ret
.size __carla_abi_extension_vector_init, . - __carla_abi_extension_vector_init

.globl __carla_abi_extension_vector_size
.type __carla_abi_extension_vector_size, @function
__carla_abi_extension_vector_size:
.LFP33:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME33, %rsp
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
	jz .Lb_Lf266d0d8457a
.Lb_Lf266d0d8457a:
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
	jmp .LFE33
	.set .LFRAME33, 192
.LFE33:
	leave
	ret
.size __carla_abi_extension_vector_size, . - __carla_abi_extension_vector_size

.globl __carla_abi_extension_vector_deinit
.type __carla_abi_extension_vector_deinit, @function
__carla_abi_extension_vector_deinit:
.LFP34:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME34, %rsp
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
	jz .Lb_L58ff0a31c707
.Lb_L58ff0a31c707:
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
.Lb_Lbc40d99f0c69:
	movq -264(%rbp), %rax
	movq -256(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -265(%rbp)
	movb -265(%rbp), %al
	test %al, %al
	jz .Lb_Lacaf25765303
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
.Lb_Lc3e446f77986:
	movq -264(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -304(%rbp)
	movq -304(%rbp), %rax
	movq %rax, -264(%rbp)
	jmp .Lb_Lbc40d99f0c69
.Lb_Lacaf25765303:
	leaq -192(%rbp), %rdi
	call __carla_abi_extension_Nashj_deinit
	.set .LFRAME34, 304
.LFE34:
	leave
	ret
.size __carla_abi_extension_vector_deinit, . - __carla_abi_extension_vector_deinit

.globl __carla_abi_extension_vector_allocation_size
.type __carla_abi_extension_vector_allocation_size, @function
__carla_abi_extension_vector_allocation_size:
.LFP35:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME35, %rsp
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
	jz .Lb_L66b2d26a5093
.Lb_L66b2d26a5093:
	movq -144(%rbp), %rax
	jmp .LFE35
	.set .LFRAME35, 160
.LFE35:
	leave
	ret
.size __carla_abi_extension_vector_allocation_size, . - __carla_abi_extension_vector_allocation_size

.globl __carla_abi_extension_vector_push
.type __carla_abi_extension_vector_push, @function
__carla_abi_extension_vector_push:
.LFP36:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME36, %rsp
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
	jz .Lb_L377a0ef5d5f9
.Lb_L377a0ef5d5f9:
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
	jz .Lb_Lb788f2228187
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
	jmp .LFE36
.Lb_Lb788f2228187:
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
	jz .Lb_L5fddde4f76b7
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
	jmp .LFE36
.Lb_L5fddde4f76b7:
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
.Lb_L40872343e236:
	movq -432(%rbp), %rax
	movq -424(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -433(%rbp)
	movb -433(%rbp), %al
	test %al, %al
	jz .Lb_L39c1631b13a7
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
.Lb_L1701f679d8d9:
	movq -432(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -472(%rbp)
	movq -472(%rbp), %rax
	movq %rax, -432(%rbp)
	jmp .Lb_L40872343e236
.Lb_L39c1631b13a7:
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
	jmp .LFE36
	.set .LFRAME36, 528
.LFE36:
	leave
	ret
.size __carla_abi_extension_vector_push, . - __carla_abi_extension_vector_push

.globl __carla_abi_extension_vector_get
.type __carla_abi_extension_vector_get, @function
__carla_abi_extension_vector_get:
.LFP37:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME37, %rsp
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
	jz .Lb_L26c190936b0b
.Lb_L26c190936b0b:
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
	jz .Lb_L483f87bb6f82
	leaq -224(%rbp), %rdi
	call __carla_abi_extension_stream_make
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr4:
	.byte 79, 117, 116, 32, 111, 102, 32, 114, 97, 110, 103, 101, 58, 32, 73, 110
	.byte 100, 101, 120, 32, 99, 97, 110, 39, 116, 32, 98, 101, 32, 103, 114, 101
	.byte 97, 116, 101, 114, 32, 111, 114, 32, 101, 113, 117, 97, 108, 32, 116, 111
	.byte 32, 116, 104, 101, 32, 118, 101, 99, 116, 111, 114, 32, 115, 105, 122, 101
	.byte 46, 10, 0
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
	jmp .LFE37
.Lb_L483f87bb6f82:
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
	jmp .LFE37
	.set .LFRAME37, 416
.LFE37:
	leave
	ret
.size __carla_abi_extension_vector_get, . - __carla_abi_extension_vector_get

.globl __carla_abi_extension_vector_pop
.type __carla_abi_extension_vector_pop, @function
__carla_abi_extension_vector_pop:
.LFP38:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME38, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rax
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
	movq -120(%rbp), %rax
	movq $88, %rcx
	addq %rcx, %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -160(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movb $255, %al
	movb %al, -169(%rbp)
	movb -169(%rbp), %al
	test %al, %al
	jz .Lb_L68fdce3e478d
.Lb_L68fdce3e478d:
	leaq -144(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	movq %rax, -192(%rbp)
	movq -192(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -196(%rbp)
	movl -196(%rbp), %eax
	movq %rax, -208(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -216(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	sete %al
	movb %al, -217(%rbp)
	movb -217(%rbp), %al
	test %al, %al
	jz .Lb_Lb370ebef7545
	leaq -240(%rbp), %rdi
	call __carla_abi_extension_stream_make
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr5:
	.byte 81, 117, 101, 117, 101, 32, 105, 115, 32, 101, 109, 112, 116, 121, 0
	.popsection
	leaq .Lstr5(%rip), %rax
	movq %rax, -248(%rbp)
	leaq -272(%rbp), %rdi
	movq -248(%rbp), %rsi
	call __carla_abi_extension_error_from
	leaq -240(%rbp), %rsi
	leaq -304(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -272(%rbp), %rsi
	leaq -288(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -304(%rbp), %rsi
	movq $32, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE38
.Lb_Lb370ebef7545:
	leaq -168(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -312(%rbp)
	movq -312(%rbp), %rax
	movq %rax, -320(%rbp)
	movq -216(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq %rax, -336(%rbp)
	movq -336(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -344(%rbp)
	movq -320(%rbp), %rax
	movq -344(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -352(%rbp)
	movq -352(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -360(%rbp)
	movq -360(%rbp), %rax
	movq %rax, -368(%rbp)
	leaq -368(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size
	movq %rax, -376(%rbp)
	leaq -368(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -384(%rbp)
	leaq -400(%rbp), %rdi
	movq -384(%rbp), %rdx
	movq -376(%rbp), %rsi
	call __carla_abi_extension_stream_from
	movq -192(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -404(%rbp)
	movl -404(%rbp), %eax
	movq $1, %rcx
	subq %rcx, %rax
	movl %eax, -408(%rbp)
	movq -192(%rbp), %rcx
	movl -408(%rbp), %eax
	movl %eax, (%rcx)
	leaq -432(%rbp), %rdi
	call __carla_abi_extension_error_make
	leaq -400(%rbp), %rsi
	leaq -464(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -432(%rbp), %rsi
	leaq -448(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -464(%rbp), %rsi
	movq $32, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE38
	.set .LFRAME38, 464
.LFE38:
	leave
	ret
.size __carla_abi_extension_vector_pop, . - __carla_abi_extension_vector_pop

.globl __carla_abi_extension_vector_pop_first
.type __carla_abi_extension_vector_pop_first, @function
__carla_abi_extension_vector_pop_first:
.LFP39:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME39, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq -16(%rbp), %rax
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
	movq -120(%rbp), %rax
	movq $88, %rcx
	addq %rcx, %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -160(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movb $255, %al
	movb %al, -169(%rbp)
	movb -169(%rbp), %al
	test %al, %al
	jz .Lb_Lecf3328ab192
.Lb_Lecf3328ab192:
	leaq -144(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	movq %rax, -192(%rbp)
	movq -192(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -196(%rbp)
	movl -196(%rbp), %eax
	movq %rax, -208(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -216(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	sete %al
	movb %al, -217(%rbp)
	movb -217(%rbp), %al
	test %al, %al
	jz .Lb_L1fdebf024c39
	leaq -240(%rbp), %rdi
	call __carla_abi_extension_stream_make
	leaq .Lstr5(%rip), %rax
	movq %rax, -248(%rbp)
	leaq -272(%rbp), %rdi
	movq -248(%rbp), %rsi
	call __carla_abi_extension_error_from
	leaq -240(%rbp), %rsi
	leaq -304(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -272(%rbp), %rsi
	leaq -288(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -304(%rbp), %rsi
	movq $32, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE39
.Lb_L1fdebf024c39:
	leaq -168(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -312(%rbp)
	movq -312(%rbp), %rax
	movq %rax, -320(%rbp)
	movq -320(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -336(%rbp)
	movq -336(%rbp), %rax
	movq %rax, -344(%rbp)
	leaq -344(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_size
	movq %rax, -352(%rbp)
	leaq -344(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -360(%rbp)
	leaq -384(%rbp), %rdi
	movq -360(%rbp), %rdx
	movq -352(%rbp), %rsi
	call __carla_abi_extension_stream_from
	movq -216(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -392(%rbp)
	movq -392(%rbp), %rax
	movq %rax, -400(%rbp)
	movq $0, %rax
	movq %rax, -408(%rbp)
.Lb_L994aa442d5e1:
	movq -408(%rbp), %rax
	movq -400(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -409(%rbp)
	movb -409(%rbp), %al
	test %al, %al
	jz .Lb_L3eedd9668058
	movq -408(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -424(%rbp)
	movq -320(%rbp), %rax
	movq -424(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -432(%rbp)
	movq -408(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -440(%rbp)
	movq -440(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -448(%rbp)
	movq -320(%rbp), %rax
	movq -448(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -456(%rbp)
	movq -456(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -464(%rbp)
	movq -432(%rbp), %rcx
	movq -464(%rbp), %rax
	movq %rax, (%rcx)
.Lb_L66428e0d156f:
	movq -408(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -472(%rbp)
	movq -472(%rbp), %rax
	movq %rax, -408(%rbp)
	jmp .Lb_L994aa442d5e1
.Lb_L3eedd9668058:
	movq -192(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -476(%rbp)
	movl -476(%rbp), %eax
	movq $1, %rcx
	subq %rcx, %rax
	movl %eax, -480(%rbp)
	movq -192(%rbp), %rcx
	movl -480(%rbp), %eax
	movl %eax, (%rcx)
	leaq -496(%rbp), %rdi
	call __carla_abi_extension_error_make
	leaq -384(%rbp), %rsi
	leaq -528(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -496(%rbp), %rsi
	leaq -512(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -528(%rbp), %rsi
	movq $32, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE39
	.set .LFRAME39, 528
.LFE39:
	leave
	ret
.size __carla_abi_extension_vector_pop_first, . - __carla_abi_extension_vector_pop_first

.globl __carla_abi_extension_CRoutine_ready
.type __carla_abi_extension_CRoutine_ready, @function
__carla_abi_extension_CRoutine_ready:
.LFP40:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME40, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq $56, %rax
	movq %rax, -24(%rbp)
	movq -8(%rbp), %rax
	movq %rax, -32(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -40(%rbp)
	leaq -64(%rbp), %rdi
	movq -32(%rbp), %rdx
	movq -40(%rbp), %rsi
	call __carla_abi_extension_stream_from
	leaq -80(%rbp), %rdi
	leaq -64(%rbp), %rdx
	movq -16(%rbp), %rsi
	call __carla_abi_extension_vector_push
	leaq -80(%rbp), %rdi
	call __carla_abi_extension_error_isnt_empty
	movb %al, -81(%rbp)
	movb -81(%rbp), %al
	test %al, %al
	jz .Lb_Lcc0ea67fb4f7
	leaq -80(%rbp), %rdi
	call __carla_abi_extension_error_panic
.Lb_Lcc0ea67fb4f7:
	.set .LFRAME40, 96
.LFE40:
	leave
	ret
.size __carla_abi_extension_CRoutine_ready, . - __carla_abi_extension_CRoutine_ready
.extern __carla_routines_ctx_switch

.globl __carla_abi_extension_CRoutines_from
.type __carla_abi_extension_CRoutines_from, @function
__carla_abi_extension_CRoutines_from:
.LFP41:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME41, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq $16, %rdi
	call __carla_abi_extension_SafeHeap_alloc
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	leaq -40(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rcx
	movq $0, %rax
	movq %rax, (%rcx)
	movq -56(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rcx
	movq $1, %rax
	movq %rax, (%rcx)
	leaq -176(%rbp), %rdi
	movq -24(%rbp), %rsi
	call __carla_abi_extension_vector_init
	movq $56, %rdi
	call __carla_abi_extension_SafeHeap_alloc
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	movq %rax, -192(%rbp)
	movq $128, %rdi
	call __carla_abi_extension_SafeHeap_alloc
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq %rax, -208(%rbp)
	leaq -208(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -216(%rbp)
	movq -216(%rbp), %rax
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -232(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -240(%rbp)
	movq -232(%rbp), %rcx
	movq -240(%rbp), %rax
	movq %rax, (%rcx)
	movq -16(%rbp), %rax
	movq %rax, -368(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -360(%rbp)
	movq -192(%rbp), %rax
	movq %rax, -352(%rbp)
	leaq -176(%rbp), %rsi
	leaq -344(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	movq -40(%rbp), %rax
	movq %rax, -248(%rbp)
	movq -8(%rbp), %rdi
	leaq -368(%rbp), %rsi
	movq $128, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE41
	.set .LFRAME41, 368
.LFE41:
	leave
	ret
.size __carla_abi_extension_CRoutines_from, . - __carla_abi_extension_CRoutines_from

.globl __carla_abi_extension_CRoutines_yield
.type __carla_abi_extension_CRoutines_yield, @function
__carla_abi_extension_CRoutines_yield:
.LFP42:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME42, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -144(%rbp), %rdi
	movq $128, %rcx
	rep movsb
	leaq -144(%rbp), %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -160(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movq -152(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -176(%rbp)
	movq -176(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	movq %rax, -192(%rbp)
	movq -152(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -208(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -152(%rbp), %rax
	movq $24, %rcx
	addq %rcx, %rax
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, %rsi
	leaq -320(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	movq -152(%rbp), %rax
	movq $120, %rcx
	addq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -336(%rbp)
	movq -336(%rbp), %rax
	movq %rax, -344(%rbp)
	movb $0, %al
	movb %al, -345(%rbp)
	movb -345(%rbp), %al
	test %al, %al
	jz .Lb_Lffdfcb6451d2
.Lb_Lffdfcb6451d2:
	leaq -216(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -360(%rbp)
	movq -360(%rbp), %rax
	movq %rax, -368(%rbp)
	movq -368(%rbp), %rax
	movq %rax, %rsi
	leaq -432(%rbp), %rdi
	movq $56, %rcx
	rep movsb
	leaq -432(%rbp), %rax
	movq %rax, -440(%rbp)
	movq -440(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -448(%rbp)
	movq -448(%rbp), %rax
	movq %rax, -456(%rbp)
	movq -440(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -464(%rbp)
	movq -464(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -472(%rbp)
	movq -472(%rbp), %rax
	movq %rax, -480(%rbp)
	movq -440(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -488(%rbp)
	movq -488(%rbp), %rax
	movq %rax, %rsi
	leaq -512(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -440(%rbp), %rax
	movq $32, %rcx
	addq %rcx, %rax
	movq %rax, -520(%rbp)
	movq -520(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -528(%rbp)
	movq -528(%rbp), %rax
	movq %rax, -536(%rbp)
	movq -440(%rbp), %rax
	movq $40, %rcx
	addq %rcx, %rax
	movq %rax, -544(%rbp)
	movq -544(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -552(%rbp)
	movq -552(%rbp), %rax
	movq %rax, -560(%rbp)
	movq -440(%rbp), %rax
	movq $48, %rcx
	addq %rcx, %rax
	movq %rax, -568(%rbp)
	movq -568(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -576(%rbp)
	movq -576(%rbp), %rax
	movq %rax, -584(%rbp)
	movb $0, %al
	movb %al, -585(%rbp)
	movb -585(%rbp), %al
	test %al, %al
	jz .Lb_L9798dfc4e37e
.Lb_L9798dfc4e37e:
	leaq -536(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -600(%rbp)
	movq -600(%rbp), %rax
	movq %rax, -608(%rbp)
	movq -608(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -616(%rbp)
	movq -616(%rbp), %rcx
	movq $0, %rax
	movq %rax, (%rcx)
	leaq -192(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -624(%rbp)
	movq -624(%rbp), %rax
	movq %rax, -632(%rbp)
	leaq -216(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -640(%rbp)
	movq -632(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -648(%rbp)
	movq -648(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -656(%rbp)
	movq -8(%rbp), %rdx
	movq -656(%rbp), %rsi
	movq -640(%rbp), %rdi
	call __carla_routines_ctx_switch
	.set .LFRAME42, 656
.LFE42:
	leave
	ret
.size __carla_abi_extension_CRoutines_yield, . - __carla_abi_extension_CRoutines_yield

.globl __carla_croutines_entry
.type __carla_croutines_entry, @function
__carla_croutines_entry:
.LFP43:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME43, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -144(%rbp), %rdi
	movq $128, %rcx
	rep movsb
	leaq -144(%rbp), %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -160(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movq -152(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -176(%rbp)
	movq -176(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	movq %rax, -192(%rbp)
	movq -152(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -208(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -152(%rbp), %rax
	movq $24, %rcx
	addq %rcx, %rax
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, %rsi
	leaq -320(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	movq -152(%rbp), %rax
	movq $120, %rcx
	addq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -336(%rbp)
	movq -336(%rbp), %rax
	movq %rax, -344(%rbp)
	movb $0, %al
	movb %al, -345(%rbp)
	movb -345(%rbp), %al
	test %al, %al
	jz .Lb_Ldd41253c948d
.Lb_Ldd41253c948d:
	leaq -216(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -360(%rbp)
	movq -360(%rbp), %rax
	movq %rax, -368(%rbp)
	movq -368(%rbp), %rax
	movq %rax, %rsi
	leaq -432(%rbp), %rdi
	movq $56, %rcx
	rep movsb
	leaq -432(%rbp), %rax
	movq %rax, -440(%rbp)
	movq -440(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -448(%rbp)
	movq -448(%rbp), %rax
	movq %rax, -456(%rbp)
	movq -440(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -464(%rbp)
	movq -464(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -472(%rbp)
	movq -472(%rbp), %rax
	movq %rax, -480(%rbp)
	movq -440(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -488(%rbp)
	movq -488(%rbp), %rax
	movq %rax, %rsi
	leaq -512(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -440(%rbp), %rax
	movq $32, %rcx
	addq %rcx, %rax
	movq %rax, -520(%rbp)
	movq -520(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -528(%rbp)
	movq -528(%rbp), %rax
	movq %rax, -536(%rbp)
	movq -440(%rbp), %rax
	movq $40, %rcx
	addq %rcx, %rax
	movq %rax, -544(%rbp)
	movq -544(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -552(%rbp)
	movq -552(%rbp), %rax
	movq %rax, -560(%rbp)
	movq -440(%rbp), %rax
	movq $48, %rcx
	addq %rcx, %rax
	movq %rax, -568(%rbp)
	movq -568(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -576(%rbp)
	movq -576(%rbp), %rax
	movq %rax, -584(%rbp)
	movb $0, %al
	movb %al, -585(%rbp)
	movb -585(%rbp), %al
	test %al, %al
	jz .Lb_La7a7999577a2
.Lb_La7a7999577a2:
	leaq -512(%rbp), %rsi
	movq -8(%rbp), %rdi
	movq -480(%rbp), %r11
	call *%r11
	leaq -344(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -600(%rbp)
	movq -600(%rbp), %rax
	movq %rax, -608(%rbp)
	movq -608(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -616(%rbp)
	movq -608(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -624(%rbp)
	movq -624(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -632(%rbp)
	movq -632(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -640(%rbp)
	movq -616(%rbp), %rcx
	movq -640(%rbp), %rax
	movq %rax, (%rcx)
	leaq -536(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -648(%rbp)
	movq -648(%rbp), %rax
	movq %rax, -656(%rbp)
	movq -656(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -664(%rbp)
	movq -664(%rbp), %rcx
	movq $3, %rax
	movq %rax, (%rcx)
	leaq -192(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -672(%rbp)
	movq -672(%rbp), %rax
	movq %rax, -680(%rbp)
	movq -680(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -688(%rbp)
	movq -688(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -696(%rbp)
	movq -8(%rbp), %rdx
	movq -696(%rbp), %rsi
	movq -456(%rbp), %rdi
	call __carla_routines_ctx_switch
	.set .LFRAME43, 704
.LFE43:
	leave
	ret
.size __carla_croutines_entry, . - __carla_croutines_entry

.globl __carla_abi_extension_CRoutine_from
.type __carla_abi_extension_CRoutine_from, @function
__carla_abi_extension_CRoutine_from:
.LFP44:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME44, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq %rcx, %r11
	movq 0(%r11), %rax
	movq %rax, -48(%rbp)
	movq 8(%r11), %rax
	movq %rax, -40(%rbp)
	movq %r8, -56(%rbp)
	movq %r9, -64(%rbp)
	movq $16, %rdi
	call __carla_abi_extension_SafeHeap_alloc
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq %rax, -80(%rbp)
	leaq -80(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -88(%rbp)
	movq -88(%rbp), %rax
	movq %rax, -96(%rbp)
	movq -96(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -104(%rbp)
	movq -104(%rbp), %rcx
	movq $0, %rax
	movq %rax, (%rcx)
	movq -96(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -112(%rbp)
	movq -112(%rbp), %rcx
	movq -64(%rbp), %rax
	movq %rax, (%rcx)
	leaq .Lstr0(%rip), %rax
	movq %rax, -120(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -176(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -168(%rbp)
	leaq -48(%rbp), %rsi
	leaq -160(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -80(%rbp), %rax
	movq %rax, -144(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -136(%rbp)
	movq -120(%rbp), %rax
	movq %rax, -128(%rbp)
	movq -8(%rbp), %rdi
	leaq -176(%rbp), %rsi
	movq $56, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE44
	.set .LFRAME44, 176
.LFE44:
	leave
	ret
.size __carla_abi_extension_CRoutine_from, . - __carla_abi_extension_CRoutine_from

.globl __carla_abi_extension_CRoutine_from_stream
.type __carla_abi_extension_CRoutine_from_stream, @function
__carla_abi_extension_CRoutine_from_stream:
.LFP45:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME45, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %r11
	movq 0(%r11), %rax
	movq %rax, -32(%rbp)
	movq 8(%r11), %rax
	movq %rax, -24(%rbp)
	leaq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -40(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -56(%rbp)
	movq -40(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq %rax, -80(%rbp)
	movb $0, %al
	movb %al, -81(%rbp)
	movb -81(%rbp), %al
	test %al, %al
	jz .Lb_L4674fb688791
.Lb_L4674fb688791:
	movq -80(%rbp), %rax
	movq %rax, -96(%rbp)
	movq -96(%rbp), %rax
	movq %rax, %rsi
	leaq -160(%rbp), %rdi
	movq $56, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -160(%rbp), %rsi
	movq $56, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE45
	.set .LFRAME45, 160
.LFE45:
	leave
	ret
.size __carla_abi_extension_CRoutine_from_stream, . - __carla_abi_extension_CRoutine_from_stream

.globl __carla_abi_extension_CRoutine_spawn
.type __carla_abi_extension_CRoutine_spawn, @function
__carla_abi_extension_CRoutine_spawn:
.LFP46:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME46, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, %r11
	movq 0(%r11), %rax
	movq %rax, -32(%rbp)
	movq 8(%r11), %rax
	movq %rax, -24(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -160(%rbp), %rdi
	movq $128, %rcx
	rep movsb
	leaq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movq -168(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -176(%rbp)
	movq -176(%rbp), %rax
	movq %rax, -184(%rbp)
	movq -168(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -192(%rbp)
	movq -192(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq %rax, -208(%rbp)
	movq -168(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -216(%rbp)
	movq -216(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, -232(%rbp)
	movq -168(%rbp), %rax
	movq $24, %rcx
	addq %rcx, %rax
	movq %rax, -240(%rbp)
	movq -240(%rbp), %rax
	movq %rax, %rsi
	leaq -336(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	movq -168(%rbp), %rax
	movq $120, %rcx
	addq %rcx, %rax
	movq %rax, -344(%rbp)
	movq -344(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -352(%rbp)
	movq -352(%rbp), %rax
	movq %rax, -360(%rbp)
	movb $0, %al
	movb %al, -361(%rbp)
	movb -361(%rbp), %al
	test %al, %al
	jz .Lb_L4ca21ea22492
.Lb_L4ca21ea22492:
	leaq -360(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -376(%rbp)
	movq -376(%rbp), %rax
	movq %rax, -384(%rbp)
	movq -384(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -392(%rbp)
	movq -392(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -400(%rbp)
	movq -400(%rbp), %rax
	movq %rax, -408(%rbp)
	movq -384(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -416(%rbp)
	movq -416(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -424(%rbp)
	movq -424(%rbp), %rax
	movq %rax, -432(%rbp)
	movq -384(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -440(%rbp)
	movq -384(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -448(%rbp)
	movq -448(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -456(%rbp)
	movq -456(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -464(%rbp)
	movq -440(%rbp), %rcx
	movq -464(%rbp), %rax
	movq %rax, (%rcx)
	movq -184(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_alloc
	movq %rax, -472(%rbp)
	movq -472(%rbp), %rax
	movq %rax, -480(%rbp)
	leaq -480(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -488(%rbp)
	movq -488(%rbp), %rax
	movq -184(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -496(%rbp)
	movq -496(%rbp), %rax
	movq %rax, -504(%rbp)
	movq -504(%rbp), %rax
	movq %rax, -512(%rbp)
	movq -512(%rbp), %rax
	movq %rax, -520(%rbp)
	movq -520(%rbp), %rax
	movq $16, %rcx
	xorl %edx, %edx
	divq %rcx
	movq %rdx, %rax
	movq %rax, -528(%rbp)
	movq -520(%rbp), %rax
	movq -528(%rbp), %rcx
	subq %rcx, %rax
	movq %rax, -536(%rbp)
	movq -536(%rbp), %rax
	movq $64, %rcx
	subq %rcx, %rax
	movq %rax, -544(%rbp)
	movq -544(%rbp), %rax
	movq %rax, -552(%rbp)
	movq -552(%rbp), %rax
	movq %rax, -560(%rbp)
	movq -560(%rbp), %rax
	movq %rax, -568(%rbp)
	movq -568(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -576(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -584(%rbp)
	movq -576(%rbp), %rcx
	movq -584(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -592(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -600(%rbp)
	movq -592(%rbp), %rcx
	movq -600(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -608(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -616(%rbp)
	movq -608(%rbp), %rcx
	movq -616(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $24, %rcx
	addq %rcx, %rax
	movq %rax, -624(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -632(%rbp)
	movq -624(%rbp), %rcx
	movq -632(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $32, %rcx
	addq %rcx, %rax
	movq %rax, -640(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -648(%rbp)
	movq -640(%rbp), %rcx
	movq -648(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $40, %rcx
	addq %rcx, %rax
	movq %rax, -656(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -664(%rbp)
	movq -656(%rbp), %rcx
	movq -664(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $48, %rcx
	addq %rcx, %rax
	movq %rax, -672(%rbp)
	leaq __carla_croutines_entry(%rip), %rax
	movq %rax, -680(%rbp)
	movq -672(%rbp), %rcx
	movq -680(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq $56, %rcx
	addq %rcx, %rax
	movq %rax, -688(%rbp)
	leaq .Lstr0(%rip), %rax
	movq %rax, -696(%rbp)
	movq -688(%rbp), %rcx
	movq -696(%rbp), %rax
	movq %rax, (%rcx)
	movq -568(%rbp), %rax
	movq %rax, -704(%rbp)
	leaq -768(%rbp), %rdi
	movq -432(%rbp), %r9
	movq -480(%rbp), %r8
	leaq -32(%rbp), %rcx
	movq -16(%rbp), %rdx
	movq -704(%rbp), %rsi
	call __carla_abi_extension_CRoutine_from
	leaq -336(%rbp), %rsi
	leaq -768(%rbp), %rdi
	call __carla_abi_extension_CRoutine_ready
	.set .LFRAME46, 768
.LFE46:
	leave
	ret
.size __carla_abi_extension_CRoutine_spawn, . - __carla_abi_extension_CRoutine_spawn

.globl __carla_abi_extension_CRoutine_join
.type __carla_abi_extension_CRoutine_join, @function
__carla_abi_extension_CRoutine_join:
.LFP47:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME47, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -144(%rbp), %rdi
	movq $128, %rcx
	rep movsb
	leaq -144(%rbp), %rax
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -160(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -168(%rbp)
	movq -152(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -176(%rbp)
	movq -176(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rax
	movq %rax, -192(%rbp)
	movq -152(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -208(%rbp)
	movq -208(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -152(%rbp), %rax
	movq $24, %rcx
	addq %rcx, %rax
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, %rsi
	leaq -320(%rbp), %rdi
	movq $96, %rcx
	rep movsb
	movq -152(%rbp), %rax
	movq $120, %rcx
	addq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -336(%rbp)
	movq -336(%rbp), %rax
	movq %rax, -344(%rbp)
	movb $0, %al
	movb %al, -345(%rbp)
	movb -345(%rbp), %al
	test %al, %al
	jz .Lb_Lcbf562545d51
.Lb_Lcbf562545d51:
.Lb_Le50e64b7af36:
	leaq -384(%rbp), %rdi
	leaq -320(%rbp), %rsi
	call __carla_abi_extension_vector_pop_first
	leaq -384(%rbp), %rax
	movq %rax, -392(%rbp)
	movq -392(%rbp), %rax
	movq %rax, %rsi
	leaq -416(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -392(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -424(%rbp)
	movq -424(%rbp), %rax
	movq %rax, %rsi
	leaq -448(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -448(%rbp), %rdi
	call __carla_abi_extension_error_is_empty
	movb %al, -449(%rbp)
	movb -449(%rbp), %al
	test %al, %al
	jz .Lb_L6dd02bfaa7e9
	leaq -512(%rbp), %rdi
	leaq -416(%rbp), %rsi
	call __carla_abi_extension_CRoutine_from_stream
	leaq -216(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -520(%rbp)
	movq $56, %rdx
	leaq -512(%rbp), %rsi
	movq -520(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_memcpy
	leaq -512(%rbp), %rax
	movq %rax, -528(%rbp)
	movq -528(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -536(%rbp)
	movq -536(%rbp), %rax
	movq %rax, -544(%rbp)
	movq -528(%rbp), %rax
	movq $8, %rcx
	addq %rcx, %rax
	movq %rax, -552(%rbp)
	movq -552(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -560(%rbp)
	movq -560(%rbp), %rax
	movq %rax, -568(%rbp)
	movq -528(%rbp), %rax
	movq $16, %rcx
	addq %rcx, %rax
	movq %rax, -576(%rbp)
	movq -576(%rbp), %rax
	movq %rax, %rsi
	leaq -592(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -528(%rbp), %rax
	movq $32, %rcx
	addq %rcx, %rax
	movq %rax, -600(%rbp)
	movq -600(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -608(%rbp)
	movq -608(%rbp), %rax
	movq %rax, -616(%rbp)
	movq -528(%rbp), %rax
	movq $40, %rcx
	addq %rcx, %rax
	movq %rax, -624(%rbp)
	movq -624(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -632(%rbp)
	movq -632(%rbp), %rax
	movq %rax, -640(%rbp)
	movq -528(%rbp), %rax
	movq $48, %rcx
	addq %rcx, %rax
	movq %rax, -648(%rbp)
	movq -648(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -656(%rbp)
	movq -656(%rbp), %rax
	movq %rax, -664(%rbp)
	movb $0, %al
	movb %al, -665(%rbp)
	movb -665(%rbp), %al
	test %al, %al
	jz .Lb_L4d275086ffb6
.Lb_L4d275086ffb6:
	leaq -616(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -680(%rbp)
	movq -680(%rbp), %rax
	movq %rax, -688(%rbp)
	movq -688(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -696(%rbp)
	movq -696(%rbp), %rcx
	movq $1, %rax
	movq %rax, (%rcx)
	leaq -192(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -704(%rbp)
	movq -704(%rbp), %rax
	movq %rax, -712(%rbp)
	leaq -192(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -720(%rbp)
	movq -8(%rbp), %rdx
	movq -544(%rbp), %rsi
	movq -720(%rbp), %rdi
	call __carla_routines_ctx_switch
	movq -688(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -728(%rbp)
	movq -728(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -736(%rbp)
	movq -736(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	sete %al
	movb %al, -737(%rbp)
	movb -737(%rbp), %al
	test %al, %al
	jz .Lb_Le43cececec1d
	leaq -216(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_SafeHeap_Cell_to_raw
	movq %rax, -752(%rbp)
	movq -752(%rbp), %rax
	movq %rax, -760(%rbp)
	leaq -320(%rbp), %rsi
	movq -760(%rbp), %rdi
	call __carla_abi_extension_CRoutine_ready
.Lb_Le43cececec1d:
	movq -688(%rbp), %rax
	movq $0, %rcx
	addq %rcx, %rax
	movq %rax, -768(%rbp)
	movq -768(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -776(%rbp)
	movq -776(%rbp), %rax
	movq $3, %rcx
	cmpq %rcx, %rax
	sete %al
	movb %al, -777(%rbp)
	movb -777(%rbp), %al
	test %al, %al
	jz .Lb_L28cb23411c0e
	movq -640(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_dump
	movq -616(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_dump
.Lb_L28cb23411c0e:
	jmp .Lb_Le50e64b7af36
.Lb_L6dd02bfaa7e9:
	movq -216(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_dump
	movq -192(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_dump
	movq -344(%rbp), %rdi
	call __carla_abi_extension_SafeHeap_dump
	leaq -320(%rbp), %rdi
	call __carla_abi_extension_vector_deinit
	.set .LFRAME47, 784
.LFE47:
	leave
	ret
.size __carla_abi_extension_CRoutine_join, . - __carla_abi_extension_CRoutine_join

.globl test
.type test, @function
test:
.LFP48:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME48, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %r11
	movq 0(%r11), %rax
	movq %rax, -32(%rbp)
	movq 8(%r11), %rax
	movq %rax, -24(%rbp)
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
	movb $0, %al
	movb %al, -65(%rbp)
	movb -65(%rbp), %al
	test %al, %al
	jz .Lb_Lb3c37f9ca3c0
.Lb_Lb3c37f9ca3c0:
	movq $0, %rax
	movq %rax, -80(%rbp)
.Lb_Ld35acfa2d653:
	movq -80(%rbp), %rax
	movq $3, %rcx
	cmpq %rcx, %rax
	setl %al
	movb %al, -81(%rbp)
	movb -81(%rbp), %al
	test %al, %al
	jz .Lb_L6049f1cbcae7
	leaq -64(%rbp), %rdi
	call __carla_abi_extension_byte_P_print
	movq -8(%rbp), %rdi
	call __carla_abi_extension_CRoutines_yield
.Lb_Lab35303062a2:
	movq -80(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -96(%rbp)
	movq -96(%rbp), %rax
	movq %rax, -80(%rbp)
	jmp .Lb_Ld35acfa2d653
.Lb_L6049f1cbcae7:
	.set .LFRAME48, 96
.LFE48:
	leave
	ret
.size test, . - test

.globl _start
.type _start, @function
_start:
.LFP49:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME49, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME49, 16
.LFE49:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP50:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME50, %rsp
	movq $10240, %rdi
	call __carla_abi_extension_SafeHeap_init
	leaq -128(%rbp), %rdi
	movq $1024, %rdx
	movq $65536, %rsi
	call __carla_abi_extension_CRoutines_from
	leaq test(%rip), %rax
	movq %rax, -136(%rbp)
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr6:
	.byte 73, 32, 108, 111, 118, 101, 32, 0
	.popsection
	leaq .Lstr6(%rip), %rax
	movq %rax, -144(%rbp)
	leaq -160(%rbp), %rdi
	movq -144(%rbp), %rsi
	call __carla_abi_extension_stream_from_str
	leaq -160(%rbp), %rdx
	movq -136(%rbp), %rsi
	leaq -128(%rbp), %rdi
	call __carla_abi_extension_CRoutine_spawn
	leaq test(%rip), %rax
	movq %rax, -168(%rbp)
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr7:
	.byte 83, 97, 109, 97, 114, 0
	.popsection
	leaq .Lstr7(%rip), %rax
	movq %rax, -176(%rbp)
	leaq -192(%rbp), %rdi
	movq -176(%rbp), %rsi
	call __carla_abi_extension_stream_from_str
	leaq -192(%rbp), %rdx
	movq -168(%rbp), %rsi
	leaq -128(%rbp), %rdi
	call __carla_abi_extension_CRoutine_spawn
	leaq test(%rip), %rax
	movq %rax, -200(%rbp)
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr8:
	.byte 33, 10, 0
	.popsection
	leaq .Lstr8(%rip), %rax
	movq %rax, -208(%rbp)
	leaq -224(%rbp), %rdi
	movq -208(%rbp), %rsi
	call __carla_abi_extension_stream_from_str
	leaq -224(%rbp), %rdx
	movq -200(%rbp), %rsi
	leaq -128(%rbp), %rdi
	call __carla_abi_extension_CRoutine_spawn
	leaq -128(%rbp), %rdi
	call __carla_abi_extension_CRoutine_join
	call __carla_abi_extension_SafeHeap_deinit
	movq $0, %rax
	jmp .LFE50
	call __carla_abi_extension_SafeHeap_deinit
	.set .LFRAME50, 224
.LFE50:
	leave
	ret
.size main, . - main
