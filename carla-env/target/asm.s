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

.globl __carla_abi_extension_byte_P_strlen
.type __carla_abi_extension_byte_P_strlen, @function
__carla_abi_extension_byte_P_strlen:
.LFP7:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME7, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
.Lb_L3c30225f7a14:
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
	jz .Lb_L284100157c29
.Lb_L095f83f718ff:
	movq -16(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp .Lb_L3c30225f7a14
.Lb_L284100157c29:
	movq -16(%rbp), %rax
	jmp .LFE7
	.set .LFRAME7, 48
.LFE7:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen, . - __carla_abi_extension_byte_P_strlen

.globl __carla_abi_extension_byte_P_strlen_r
.type __carla_abi_extension_byte_P_strlen_r, @function
__carla_abi_extension_byte_P_strlen_r:
.LFP8:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME8, %rsp
	movq %rdi, -8(%rbp)
	movq $0, %rax
	movq %rax, -16(%rbp)
	movq $0, %rax
	movq %rax, -24(%rbp)
.Lb_L5a813af9135a:
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
	jz .Lb_L35f0a8d26318
	movq -16(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq %rax, -16(%rbp)
.Lb_L11c7421578f3:
	movq -24(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rax
	movq %rax, -24(%rbp)
	jmp .Lb_L5a813af9135a
.Lb_L35f0a8d26318:
	movq -16(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -72(%rbp)
	movq -72(%rbp), %rax
	movq %rax, -16(%rbp)
	.set .LFRAME8, 80
.LFE8:
	leave
	ret
.size __carla_abi_extension_byte_P_strlen_r, . - __carla_abi_extension_byte_P_strlen_r

.globl __carla_abi_extension_byte_P_print
.type __carla_abi_extension_byte_P_print, @function
__carla_abi_extension_byte_P_print:
.LFP9:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME9, %rsp
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
	.set .LFRAME9, 32
.LFE9:
	leave
	ret
.size __carla_abi_extension_byte_P_print, . - __carla_abi_extension_byte_P_print

.globl __carla_abi_extension_int_print
.type __carla_abi_extension_int_print, @function
__carla_abi_extension_int_print:
.LFP10:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME10, %rsp
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
	jz .Lb_L30126faaa908
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
.Lb_L30126faaa908:
	movq $1, %rax
	movq %rax, -64(%rbp)
.Lb_L65aba8c7a54c:
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
	jz .Lb_L67466fcbb75b
	movq -64(%rbp), %rax
	movq $10, %rcx
	imulq %rcx, %rax
	movq %rax, -88(%rbp)
	movq -88(%rbp), %rax
	movq %rax, -64(%rbp)
	jmp .Lb_L65aba8c7a54c
.Lb_L67466fcbb75b:
.Lb_L0672c78d1e29:
	movq -64(%rbp), %rax
	movq $0, %rcx
	cmpq %rcx, %rax
	setg %al
	movb %al, -89(%rbp)
	movb -89(%rbp), %al
	test %al, %al
	jz .Lb_L53a9ce70f65e
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
	jmp .Lb_L0672c78d1e29
.Lb_L53a9ce70f65e:
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr0:
	.byte 10, 0
	.popsection
	leaq .Lstr0(%rip), %rax
	movq %rax, -152(%rbp)
	movq $1, %rdi
	movq -152(%rbp), %rsi
	movq $1, %rdx
	movq $1, %rax
	syscall
	movq %rax, -160(%rbp)
	.set .LFRAME10, 160
.LFE10:
	leave
	ret
.size __carla_abi_extension_int_print, . - __carla_abi_extension_int_print

.globl __carla_abi_extension_error_from
.type __carla_abi_extension_error_from, @function
__carla_abi_extension_error_from:
.LFP11:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME11, %rsp
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
	jmp .LFE11
	.set .LFRAME11, 48
.LFE11:
	leave
	ret
.size __carla_abi_extension_error_from, . - __carla_abi_extension_error_from

.globl __carla_abi_extension_error_make
.type __carla_abi_extension_error_make, @function
__carla_abi_extension_error_make:
.LFP12:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME12, %rsp
	movq %rdi, -8(%rbp)
	movb $0, %al
	movb %al, -9(%rbp)
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr1:
	.byte 0
	.popsection
	leaq .Lstr1(%rip), %rax
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
	jmp .LFE12
	.set .LFRAME12, 48
.LFE12:
	leave
	ret
.size __carla_abi_extension_error_make, . - __carla_abi_extension_error_make

.globl __carla_abi_extension_error_msg
.type __carla_abi_extension_error_msg, @function
__carla_abi_extension_error_msg:
.LFP13:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME13, %rsp
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
	jz .Lb_L60aabbb5fd48
.Lb_L60aabbb5fd48:
	movq -64(%rbp), %rax
	jmp .LFE13
	.set .LFRAME13, 80
.LFE13:
	leave
	ret
.size __carla_abi_extension_error_msg, . - __carla_abi_extension_error_msg

.globl __carla_abi_extension_error_isnt_empty
.type __carla_abi_extension_error_isnt_empty, @function
__carla_abi_extension_error_isnt_empty:
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
	movb (%rax), %al
	movb %al, -41(%rbp)
	movb -41(%rbp), %al
	movb %al, -42(%rbp)
	movb $255, %al
	movb %al, -43(%rbp)
	movb -43(%rbp), %al
	test %al, %al
	jz .Lb_Lea71e7edf657
.Lb_Lea71e7edf657:
	movb -42(%rbp), %al
	jmp .LFE14
	.set .LFRAME14, 48
.LFE14:
	leave
	ret
.size __carla_abi_extension_error_isnt_empty, . - __carla_abi_extension_error_isnt_empty

.globl __carla_abi_extension_vector_init
.type __carla_abi_extension_vector_init, @function
__carla_abi_extension_vector_init:
.LFP15:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME15, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -16(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -32(%rbp)
	movq -32(%rbp), %rax
	movq %rax, -40(%rbp)
	movq -24(%rbp), %rax
	movq %rax, -48(%rbp)
	movq -40(%rbp), %rax
	movq $4, %rcx
	addq %rcx, %rax
	movq %rax, -56(%rbp)
	movq -56(%rbp), %rax
	movq -48(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -64(%rbp)
	movq -64(%rbp), %rax
	movq %rax, -72(%rbp)
	leaq -144(%rbp), %rdi
	movq -72(%rbp), %rsi
	call __carla_abi_extension_Nashj_init
	movq -40(%rbp), %rsi
	leaq -144(%rbp), %rdi
	call __carla_abi_extension_Nashj_alloc
	movq %rax, -152(%rbp)
	movq -152(%rbp), %rax
	movq %rax, -160(%rbp)
	movq $4, %rsi
	leaq -144(%rbp), %rdi
	call __carla_abi_extension_Nashj_alloc
	movq %rax, -168(%rbp)
	movq -168(%rbp), %rax
	movq %rax, -176(%rbp)
	movq -48(%rbp), %rax
	movq $16, %rcx
	subq %rcx, %rax
	movq %rax, -184(%rbp)
	movq -184(%rbp), %rsi
	leaq -144(%rbp), %rdi
	call __carla_abi_extension_Nashj_alloc
	movq %rax, -192(%rbp)
	movq -192(%rbp), %rax
	movq %rax, -200(%rbp)
	leaq -144(%rbp), %rsi
	leaq -304(%rbp), %rdi
	movq $64, %rcx
	rep movsb
	movq -72(%rbp), %rax
	movq %rax, -240(%rbp)
	movq -16(%rbp), %rax
	movq %rax, -232(%rbp)
	movq -176(%rbp), %rax
	movq %rax, -224(%rbp)
	movq -160(%rbp), %rax
	movq %rax, -216(%rbp)
	movq -200(%rbp), %rax
	movq %rax, -208(%rbp)
	movq -8(%rbp), %rdi
	leaq -304(%rbp), %rsi
	movq $104, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE15
	.set .LFRAME15, 304
.LFE15:
	leave
	ret
.size __carla_abi_extension_vector_init, . - __carla_abi_extension_vector_init

.globl __carla_abi_extension_vector_deinit
.type __carla_abi_extension_vector_deinit, @function
__carla_abi_extension_vector_deinit:
.LFP16:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -112(%rbp), %rdi
	movq $104, %rcx
	rep movsb
	leaq -112(%rbp), %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq %rax, %rsi
	leaq -192(%rbp), %rdi
	movq $64, %rcx
	rep movsb
	movb $255, %al
	movb %al, -193(%rbp)
	movb -193(%rbp), %al
	test %al, %al
	jz .Lb_L2b7c369313fa
.Lb_L2b7c369313fa:
	leaq -192(%rbp), %rdi
	call __carla_abi_extension_Nashj_deinit
	.set .LFRAME16, 208
.LFE16:
	leave
	ret
.size __carla_abi_extension_vector_deinit, . - __carla_abi_extension_vector_deinit

.globl __carla_abi_extension_vector_allocation_size
.type __carla_abi_extension_vector_allocation_size, @function
__carla_abi_extension_vector_allocation_size:
.LFP17:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME17, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -112(%rbp), %rdi
	movq $104, %rcx
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
	jz .Lb_Lbd8506f4a330
.Lb_Lbd8506f4a330:
	movq -144(%rbp), %rax
	jmp .LFE17
	.set .LFRAME17, 160
.LFE17:
	leave
	ret
.size __carla_abi_extension_vector_allocation_size, . - __carla_abi_extension_vector_allocation_size

.globl __carla_abi_extension_vector_push
.type __carla_abi_extension_vector_push, @function
__carla_abi_extension_vector_push:
.LFP18:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME18, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -128(%rbp), %rdi
	movq $104, %rcx
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
	movq -136(%rbp), %rax
	movq $96, %rcx
	addq %rcx, %rax
	movq %rax, -216(%rbp)
	movq -216(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, -232(%rbp)
	movb $255, %al
	movb %al, -233(%rbp)
	movb -233(%rbp), %al
	test %al, %al
	jz .Lb_Ldeebc398f962
.Lb_Ldeebc398f962:
	leaq -184(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -248(%rbp)
	movq -248(%rbp), %rax
	movq %rax, -256(%rbp)
	movq -256(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -260(%rbp)
	movl -260(%rbp), %eax
	movq $1, %rcx
	addq %rcx, %rax
	movl %eax, -264(%rbp)
	movq -256(%rbp), %rcx
	movl -264(%rbp), %eax
	movl %eax, (%rcx)
	leaq -208(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -272(%rbp)
	movq -272(%rbp), %rax
	movq %rax, -280(%rbp)
	leaq -232(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -288(%rbp)
	movq -288(%rbp), %rax
	movq %rax, -296(%rbp)
	movq $0, %rax
	movq %rax, -304(%rbp)
.Lb_Lc79792fd6bbf:
	movq -304(%rbp), %rax
	movq -24(%rbp), %rcx
	cmpq %rcx, %rax
	setb %al
	movb %al, -305(%rbp)
	movb -305(%rbp), %al
	test %al, %al
	jz .Lb_L38d2f3c1f968
	movq -296(%rbp), %rax
	movq -304(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -320(%rbp)
	movq -16(%rbp), %rax
	movq -304(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -328(%rbp)
	movq -328(%rbp), %rax
	movb (%rax), %al
	movb %al, -329(%rbp)
	movq -320(%rbp), %rcx
	movb -329(%rbp), %al
	movb %al, (%rcx)
.Lb_Le6cdf4853962:
	movq -304(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	movq %rax, -344(%rbp)
	movq -344(%rbp), %rax
	movq %rax, -304(%rbp)
	jmp .Lb_Lc79792fd6bbf
.Lb_L38d2f3c1f968:
	movq -256(%rbp), %rax
	movl (%rax), %eax
	movl %eax, -348(%rbp)
	movl -348(%rbp), %eax
	movq %rax, -360(%rbp)
	movq -360(%rbp), %rax
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, -368(%rbp)
	movq -368(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -376(%rbp)
	movq -280(%rbp), %rax
	movq -376(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -384(%rbp)
	movq -384(%rbp), %rcx
	movq -296(%rbp), %rax
	movq %rax, (%rcx)
	.set .LFRAME18, 384
.LFE18:
	leave
	ret
.size __carla_abi_extension_vector_push, . - __carla_abi_extension_vector_push

.globl __carla_abi_extension_vector_size
.type __carla_abi_extension_vector_size, @function
__carla_abi_extension_vector_size:
.LFP19:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME19, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rax, %rsi
	leaq -112(%rbp), %rdi
	movq $104, %rcx
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
	jz .Lb_L014a81c4435c
.Lb_L014a81c4435c:
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
	jmp .LFE19
	.set .LFRAME19, 192
.LFE19:
	leave
	ret
.size __carla_abi_extension_vector_size, . - __carla_abi_extension_vector_size

.globl __carla_abi_extension_vector_get
.type __carla_abi_extension_vector_get, @function
__carla_abi_extension_vector_get:
.LFP20:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME20, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq -16(%rbp), %rax
	movq %rax, %rsi
	leaq -128(%rbp), %rdi
	movq $104, %rcx
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
	jz .Lb_L1aeda06be5c3
.Lb_L1aeda06be5c3:
	leaq -160(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -200(%rbp)
	movq -200(%rbp), %rax
	movq %rax, -208(%rbp)
	leaq -184(%rbp), %rdi
	call __carla_abi_extension___carla_abi_extension_Nashj_Qitah_to_raw
	movq %rax, -216(%rbp)
	movq -216(%rbp), %rax
	movq %rax, -224(%rbp)
	movq -16(%rbp), %rdi
	call __carla_abi_extension_vector_size
	movq %rax, -232(%rbp)
	movq -24(%rbp), %rax
	movq -232(%rbp), %rcx
	cmpq %rcx, %rax
	setae %al
	movb %al, -233(%rbp)
	movb -233(%rbp), %al
	test %al, %al
	jz .Lb_L76a69ecf4be3
	leaq .Lstr1(%rip), %rax
	movq %rax, -248(%rbp)
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr2:
	.byte 105, 110, 100, 101, 120, 32, 99, 97, 110, 39, 116, 32, 98, 101, 32, 97
	.byte 98, 111, 118, 101, 32, 111, 114, 32, 101, 113, 117, 97, 108, 32, 116, 104
	.byte 97, 110, 32, 116, 104, 101, 32, 118, 101, 99, 116, 111, 114, 32, 115, 105
	.byte 122, 101, 46, 0
	.popsection
	leaq .Lstr2(%rip), %rax
	movq %rax, -256(%rbp)
	leaq -272(%rbp), %rdi
	movq -256(%rbp), %rsi
	call __carla_abi_extension_error_from
	movq -248(%rbp), %rax
	movq %rax, -304(%rbp)
	leaq -272(%rbp), %rsi
	leaq -296(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -304(%rbp), %rsi
	movq $24, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE20
.Lb_L76a69ecf4be3:
	movq -24(%rbp), %rax
	movq $8, %rcx
	imulq %rcx, %rax
	movq %rax, -312(%rbp)
	movq -224(%rbp), %rax
	movq -312(%rbp), %rcx
	addq %rcx, %rax
	movq %rax, -320(%rbp)
	movq -320(%rbp), %rax
	movq (%rax), %rax
	movq %rax, -328(%rbp)
	leaq -352(%rbp), %rdi
	call __carla_abi_extension_error_make
	movq -328(%rbp), %rax
	movq %rax, -384(%rbp)
	leaq -352(%rbp), %rsi
	leaq -376(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	movq -8(%rbp), %rdi
	leaq -384(%rbp), %rsi
	movq $24, %rcx
	rep movsb
	movq -8(%rbp), %rax
	jmp .LFE20
	.set .LFRAME20, 384
.LFE20:
	leave
	ret
.size __carla_abi_extension_vector_get, . - __carla_abi_extension_vector_get

.globl _start
.type _start, @function
_start:
.LFP21:
	xorl %ebp, %ebp
	andq $-16, %rsp
	movq %rsp, %rbp
	subq $.LFRAME21, %rsp
	call main
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $60, %rax
	syscall
	movq %rax, -16(%rbp)
	.set .LFRAME21, 16
.LFE21:
	ud2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP22:
	pushq %rbp
	movq %rsp, %rbp
	subq $.LFRAME22, %rsp
	leaq -112(%rbp), %rdi
	movq $4096, %rdx
	movq $128, %rsi
	call __carla_abi_extension_vector_init
	.pushsection .rodata.str1.1,"aMS",@progbits,1
	.Lstr3:
	.byte 73, 102, 32, 105, 32, 108, 111, 118, 101, 32, 83, 97, 109, 97, 114, 44
	.byte 32, 116, 104, 97, 116, 32, 119, 105, 108, 108, 32, 119, 111, 114, 107, 32
	.byte 105, 110, 32, 116, 104, 101, 32, 102, 105, 114, 115, 116, 32, 97, 116, 116
	.byte 101, 109, 112, 116, 10, 0
	.popsection
	leaq .Lstr3(%rip), %rax
	movq %rax, -120(%rbp)
	movq -120(%rbp), %rax
	movq %rax, -128(%rbp)
	leaq -128(%rbp), %rdi
	call __carla_abi_extension_byte_P_strlen
	movq %rax, -136(%rbp)
	movq -136(%rbp), %rdx
	movq -128(%rbp), %rsi
	leaq -112(%rbp), %rdi
	call __carla_abi_extension_vector_push
	leaq -160(%rbp), %rdi
	movq $0, %rdx
	leaq -112(%rbp), %rsi
	call __carla_abi_extension_vector_get
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
	movq %rax, %rsi
	leaq -208(%rbp), %rdi
	movq $16, %rcx
	rep movsb
	leaq -208(%rbp), %rdi
	call __carla_abi_extension_error_isnt_empty
	movb %al, -209(%rbp)
	movb -209(%rbp), %al
	test %al, %al
	jz .Lb_L324d660ccbf2
	leaq -208(%rbp), %rdi
	call __carla_abi_extension_error_msg
	movq %rax, -224(%rbp)
	movq -224(%rbp), %rax
	movq %rax, -232(%rbp)
	leaq -232(%rbp), %rdi
	call __carla_abi_extension_byte_P_print
.Lb_L324d660ccbf2:
	movq -184(%rbp), %rax
	movq %rax, -240(%rbp)
	leaq -240(%rbp), %rdi
	call __carla_abi_extension_byte_P_print
	leaq -112(%rbp), %rdi
	call __carla_abi_extension_vector_deinit
	movq $0, %rax
	jmp .LFE22
	leaq -112(%rbp), %rdi
	call __carla_abi_extension_vector_deinit
	.set .LFRAME22, 240
.LFE22:
	leave
	ret
.size main, . - main
