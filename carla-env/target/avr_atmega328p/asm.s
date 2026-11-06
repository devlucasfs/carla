
.pushsection .vectors,"ax",@progbits
jmp _start
.popsection
.globl _start
.type _start, @function
_start:
.LFP7:
	ldi r28, lo8(2303)
	ldi r29, hi8(2303)
	out 62, r29
	out 61, r28
	ldi r17, hi8(__data_end)
	ldi r26, lo8(__data_start)
	ldi r27, hi8(__data_start)
	ldi r30, lo8(__data_load_start)
	ldi r31, hi8(__data_load_start)
	rjmp .Lt10
.Lt9:
	lpm r0, Z+
	st X+, r0
.Lt10:
	cpi r26, lo8(__data_end)
	cpc r27, r17
	brne .Lt9
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME7)
	sbci r25, hi8(.LFRAME7)
	in r0, 63
	cli
	out 62, r25
	out 63, r0
	out 61, r24
	call main
	movw r30, r28
	sbiw r30, 8
	std Z+0, r18
	std Z+1, r19
	std Z+2, r20
	std Z+3, r21
	std Z+4, r22
	std Z+5, r23
	std Z+6, r24
	std Z+7, r25
	.set .LFLOC7, 8
	.set .LFRAME7, 8
.LFE7:
	cli
	rjmp .-2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP8:
	push r28
	push r29
	in r28, 61
	in r29, 62
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME8)
	sbci r25, hi8(.LFRAME8)
	in r0, 63
	cli
	out 62, r25
	out 63, r0
	out 61, r24
	call __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu
	in r10, 5
	movw r30, r28
	sbiw r30, 1
	std Z+0, r10
	movw r30, r28
	sbiw r30, 1
	ldd r10, Z+0
	ldi r24, 32
	mov r2, r24
	or r10, r2
	movw r30, r28
	sbiw r30, 2
	std Z+0, r10
	movw r30, r28
	sbiw r30, 2
	ldd r10, Z+0
	out 5, r10
.Lb_L6f27232c3cc6:
	movw r26, r28
	subi r26, lo8(.LFLOC8+16)
	sbci r27, hi8(.LFLOC8+16)
	.pushsection .progmem.data,"a",@progbits
	.Lstr11:
	.byte 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 13, 0
	.balign 2
	.popsection
	ldi r30, lo8(.Lstr11)
	ldi r31, hi8(.Lstr11)
	ldi r22, 16
	ldi r23, 0
.Lt12:
	lpm r0, Z+
	st X+, r0
	subi r22, 1
	sbci r23, 0
	brne .Lt12
	movw r24, r28
	subi r24, lo8(.LFLOC8+16)
	sbci r25, hi8(.LFLOC8+16)
	movw r30, r28
	sbiw r30, 16
	std Z+0, r24
	std Z+1, r25
	clr r0
	std Z+2, r0
	std Z+3, r0
	std Z+4, r0
	std Z+5, r0
	std Z+6, r0
	std Z+7, r0
	ldi r24, 15
	mov r10, r24
	clr r11
	clr r12
	clr r13
	clr r14
	clr r15
	clr r16
	clr r17
	in r30, 61
	in r31, 62
	adiw r30, 17
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	sbiw r30, 16
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	in r30, 61
	in r31, 62
	adiw r30, 9
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	ldi r24, 1
	mov r10, r24
	clr r11
	clr r12
	clr r13
	clr r14
	clr r15
	clr r16
	clr r17
	in r30, 61
	in r31, 62
	adiw r30, 1
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	call __carla_abi_extension_stdlib_write
	in r10, 5
	movw r30, r28
	sbiw r30, 17
	std Z+0, r10
	movw r30, r28
	sbiw r30, 17
	ldd r10, Z+0
	ldi r24, 32
	mov r2, r24
	or r10, r2
	movw r30, r28
	sbiw r30, 18
	std Z+0, r10
	movw r30, r28
	sbiw r30, 18
	ldd r10, Z+0
	out 5, r10
	ldi r24, 112
	mov r10, r24
	ldi r24, 17
	mov r11, r24
	ldi r24, 1
	mov r12, r24
	clr r13
	clr r14
	clr r15
	clr r16
	clr r17
	in r30, 61
	in r31, 62
	adiw r30, 1
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	call sleep
	in r10, 5
	movw r30, r28
	sbiw r30, 19
	std Z+0, r10
	movw r30, r28
	sbiw r30, 19
	ldd r10, Z+0
	ldi r24, 223
	mov r2, r24
	and r10, r2
	movw r30, r28
	sbiw r30, 20
	std Z+0, r10
	movw r30, r28
	sbiw r30, 20
	ldd r10, Z+0
	out 5, r10
	ldi r24, 112
	mov r10, r24
	ldi r24, 17
	mov r11, r24
	ldi r24, 1
	mov r12, r24
	clr r13
	clr r14
	clr r15
	clr r16
	clr r17
	in r30, 61
	in r31, 62
	adiw r30, 1
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	call sleep
	jmp .Lb_L6f27232c3cc6
.Lb_Ld9fe449a789b:
	clr r18
	clr r19
	clr r20
	clr r21
	clr r22
	clr r23
	clr r24
	clr r25
	jmp .LFE8
	.set .LFLOC8, 20
	.set .LFRAME8, 61
.LFE8:
	in r0, 63
	cli
	out 62, r29
	out 63, r0
	out 61, r28
	pop r29
	pop r28
	ret
.size main, . - main

.globl __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu
.type __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu, @function
__carla_abi_extension___carla_abi_extension_stdlib_serial_mcu:
.LFP2:
	push r28
	push r29
	in r28, 61
	in r29, 62
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME2)
	sbci r25, hi8(.LFRAME2)
	in r0, 63
	cli
	out 62, r25
	out 63, r0
	out 61, r24
	clr r10
	sts 197, r10
	ldi r24, 103
	mov r10, r24
	sts 196, r10
	ldi r24, 4
	mov r10, r24
	movw r30, r28
	sbiw r30, 1
	std Z+0, r10
	movw r30, r28
	sbiw r30, 1
	ldd r10, Z+0
	ldi r24, 2
	mov r2, r24
	or r10, r2
	movw r30, r28
	sbiw r30, 2
	std Z+0, r10
	movw r30, r28
	sbiw r30, 2
	ldd r10, Z+0
	movw r30, r28
	sbiw r30, 1
	std Z+0, r10
	movw r30, r28
	sbiw r30, 1
	ldd r10, Z+0
	sts 194, r10
	ldi r24, 8
	mov r10, r24
	sts 193, r10
	.set .LFLOC2, 2
	.set .LFRAME2, 2
.LFE2:
	in r0, 63
	cli
	out 62, r29
	out 63, r0
	out 61, r28
	pop r29
	pop r28
	ret
.size __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu, . - __carla_abi_extension___carla_abi_extension_stdlib_serial_mcu

.globl __carla_abi_extension_stdlib_write
.type __carla_abi_extension_stdlib_write, @function
__carla_abi_extension_stdlib_write:
.LFP0:
	push r28
	push r29
	in r28, 61
	in r29, 62
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME0)
	sbci r25, hi8(.LFRAME0)
	in r0, 63
	cli
	out 62, r25
	out 63, r0
	out 61, r24
	movw r30, r28
	adiw r30, 5
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 8
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	adiw r30, 13
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 16
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	adiw r30, 21
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 24
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	sbiw r30, 24
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 32
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	clr r10
	clr r11
	clr r12
	clr r13
	clr r14
	clr r15
	clr r16
	clr r17
	movw r30, r28
	sbiw r30, 40
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
.Lb_La880ac10c7c7:
	movw r30, r28
	sbiw r30, 40
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 32
	ldd r2, Z+0
	ldd r3, Z+1
	ldd r4, Z+2
	ldd r5, Z+3
	ldd r6, Z+4
	ldd r7, Z+5
	ldd r8, Z+6
	ldd r9, Z+7
	cp r10, r2
	cpc r11, r3
	cpc r12, r4
	cpc r13, r5
	cpc r14, r6
	cpc r15, r7
	cpc r16, r8
	cpc r17, r9
	ldi r24, 1
	brlo .Lt0
	ldi r24, 0
.Lt0:
	movw r30, r28
	sbiw r30, 41
	std Z+0, r24
	movw r30, r28
	sbiw r30, 41
	ld r24, Z
	tst r24
	brne .Lt1
	jmp .Lb_Labc9518b0af9
.Lt1:
.Lb_L7c99f2aa5047:
	lds r10, 192
	movw r30, r28
	sbiw r30, 42
	std Z+0, r10
	movw r30, r28
	sbiw r30, 42
	ldd r10, Z+0
	ldi r24, 32
	mov r2, r24
	and r10, r2
	movw r30, r28
	sbiw r30, 43
	std Z+0, r10
	movw r30, r28
	sbiw r30, 43
	ldd r10, Z+0
	clr r2
	cp r10, r2
	ldi r24, 1
	breq .Lt2
	ldi r24, 0
.Lt2:
	movw r30, r28
	sbiw r30, 44
	std Z+0, r24
	movw r30, r28
	sbiw r30, 44
	ld r24, Z
	tst r24
	brne .Lt3
	jmp .Lb_L02ca8543e4cb
.Lt3:
	jmp .Lb_L7c99f2aa5047
.Lb_L02ca8543e4cb:
	movw r30, r28
	sbiw r30, 16
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 40
	ldd r2, Z+0
	ldd r3, Z+1
	ldd r4, Z+2
	ldd r5, Z+3
	ldd r6, Z+4
	ldd r7, Z+5
	ldd r8, Z+6
	ldd r9, Z+7
	add r10, r2
	adc r11, r3
	adc r12, r4
	adc r13, r5
	adc r14, r6
	adc r15, r7
	adc r16, r8
	adc r17, r9
	movw r30, r28
	sbiw r30, 56
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	sbiw r30, 56
	ld r24, Z
	ldd r25, Z+1
	movw r30, r24
	movw r26, r28
	sbiw r26, 57
	ld r0, Z+
	st X+, r0
	movw r30, r28
	sbiw r30, 57
	ldd r10, Z+0
	sts 198, r10
.Lb_L504fc37d64fb:
	movw r30, r28
	sbiw r30, 40
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	ldi r24, 1
	mov r2, r24
	clr r3
	clr r4
	clr r5
	clr r6
	clr r7
	clr r8
	clr r9
	add r10, r2
	adc r11, r3
	adc r12, r4
	adc r13, r5
	adc r14, r6
	adc r15, r7
	adc r16, r8
	adc r17, r9
	movw r30, r28
	subi r30, 72
	sbci r31, 0
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	subi r30, 72
	sbci r31, 0
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 40
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	jmp .Lb_La880ac10c7c7
.Lb_Labc9518b0af9:
	.set .LFLOC0, 72
	.set .LFRAME0, 72
.LFE0:
	in r0, 63
	cli
	out 62, r29
	out 63, r0
	out 61, r28
	pop r29
	pop r28
	ret
.size __carla_abi_extension_stdlib_write, . - __carla_abi_extension_stdlib_write

.globl sleep
.type sleep, @function
sleep:
.LFP6:
	push r28
	push r29
	in r28, 61
	in r29, 62
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME6)
	sbci r25, hi8(.LFRAME6)
	in r0, 63
	cli
	out 62, r25
	out 63, r0
	out 61, r24
	movw r30, r28
	adiw r30, 5
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 8
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	clr r10
	clr r11
	clr r12
	clr r13
	clr r14
	clr r15
	clr r16
	clr r17
	movw r30, r28
	sbiw r30, 16
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
.Lb_L84cc75485010:
	movw r30, r28
	sbiw r30, 16
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 8
	ldd r2, Z+0
	ldd r3, Z+1
	ldd r4, Z+2
	ldd r5, Z+3
	ldd r6, Z+4
	ldd r7, Z+5
	ldd r8, Z+6
	ldd r9, Z+7
	cp r2, r10
	cpc r3, r11
	cpc r4, r12
	cpc r5, r13
	cpc r6, r14
	cpc r7, r15
	cpc r8, r16
	cpc r9, r17
	ldi r24, 1
	brge .Lt7
	ldi r24, 0
.Lt7:
	movw r30, r28
	sbiw r30, 17
	std Z+0, r24
	movw r30, r28
	sbiw r30, 17
	ld r24, Z
	tst r24
	brne .Lt8
	jmp .Lb_Lba5980c1a8cd
.Lt8:
.Lb_Ld969bef0d0f4:
	movw r30, r28
	sbiw r30, 16
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	ldi r24, 1
	mov r2, r24
	clr r3
	clr r4
	clr r5
	clr r6
	clr r7
	clr r8
	clr r9
	add r10, r2
	adc r11, r3
	adc r12, r4
	adc r13, r5
	adc r14, r6
	adc r15, r7
	adc r16, r8
	adc r17, r9
	movw r30, r28
	sbiw r30, 32
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	movw r30, r28
	sbiw r30, 32
	ldd r10, Z+0
	ldd r11, Z+1
	ldd r12, Z+2
	ldd r13, Z+3
	ldd r14, Z+4
	ldd r15, Z+5
	ldd r16, Z+6
	ldd r17, Z+7
	movw r30, r28
	sbiw r30, 16
	std Z+0, r10
	std Z+1, r11
	std Z+2, r12
	std Z+3, r13
	std Z+4, r14
	std Z+5, r15
	std Z+6, r16
	std Z+7, r17
	jmp .Lb_L84cc75485010
.Lb_Lba5980c1a8cd:
	.set .LFLOC6, 32
	.set .LFRAME6, 32
.LFE6:
	in r0, 63
	cli
	out 62, r29
	out 63, r0
	out 61, r28
	pop r29
	pop r28
	ret
.size sleep, . - sleep
