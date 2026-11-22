.text
.globl cycles_sleep
.type cycles_sleep, @function
cycles_sleep:
.LFP0:
	push r28
	push r29
	in r28, 0x3d
	in r29, 0x3e
	subi r28, lo8(.LFRAME0)
	sbci r29, hi8(.LFRAME0)
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	std Y+1, r24
	std Y+2, r25
	ldi r18, 0
	ldi r19, 0
	std Y+3, r18
	std Y+4, r19
.L0_Ld97f6b741e15:
	ldd r18, Y+3
	ldd r19, Y+4
	ldd r0, Y+1
	cp r18, r0
	ldd r0, Y+2
	cpc r19, r0
	ldi r18, 1
	breq .LX0_0
	brlt .LX0_0
	ldi r18, 0
.LX0_0:
	std Y+5, r18
	ldd r18, Y+5
	tst r18
	brne .LX0_1
	jmp .L0_L41cfd0161e4d
.LX0_1:
.L0_L721544720de7:
	ldd r18, Y+3
	ldd r19, Y+4
	ldi r26, 1
	add r18, r26
	ldi r26, 0
	adc r19, r26
	std Y+6, r18
	std Y+7, r19
	ldd r18, Y+6
	ldd r19, Y+7
	std Y+3, r18
	std Y+4, r19
	jmp .L0_Ld97f6b741e15
.L0_L41cfd0161e4d:
.set .LFRAME0, 7
.LFE0:
	subi r28, 249
	sbci r29, 255
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	pop r29
	pop r28
	ret
.size cycles_sleep, .-cycles_sleep

.section .vectors,"ax",@progbits
.global __vectors
__vectors:
jmp _start
.rept 25
jmp __bad_interrupt
.endr
.weak __bad_interrupt

.text
.globl _start
.type _start, @function
_start:
.LFP1:
	clr r1
	out 0x3f, r1
	ldi r28, 0xFF
	ldi r29, 0x08
	out 0x3e, r29
	out 0x3d, r28
	ldi r26, lo8(__data_start)
	ldi r27, hi8(__data_start)
	ldi r30, lo8(__data_load_start)
	ldi r31, hi8(__data_load_start)
	ldi r16, hi8(__data_end)
	rjmp .LX1_3
.LX1_2:
	lpm r0, Z+
	st X+, r0
.LX1_3:
	cpi r26, lo8(__data_end)
	cpc r27, r16
	brne .LX1_2
	in r28, 0x3d
	in r29, 0x3e
	subi r28, lo8(.LFRAME1)
	sbci r29, hi8(.LFRAME1)
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	call main
	std Y+1, r24
	std Y+2, r25
.set .LFRAME1, 2
.LFE1:
	cli
.LHALT1:
	rjmp .LHALT1
.size _start, .-_start

.text
.globl main
.type main, @function
main:
.LFP2:
	push r28
	push r29
	in r28, 0x3d
	in r29, 0x3e
	subi r28, lo8(.LFRAME2)
	sbci r29, hi8(.LFRAME2)
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	ldi r18, 0
	ldi r19, 24
	ldi r20, 0
	ldi r21, 0
	sts 1610649612, r18
	sts 1610649613, r19
	sts 1610649614, r20
	sts 1610649615, r21
	ldi r18, 0
	ldi r19, 1
	ldi r20, 0
	ldi r21, 0
	sts 1610630492, r18
	sts 1610630493, r19
	sts 1610630494, r20
	sts 1610630495, r21
	ldi r18, 4
	ldi r19, 0
	ldi r20, 0
	ldi r21, 0
	sts 1610629156, r18
	sts 1610629157, r19
	sts 1610629158, r20
	sts 1610629159, r21
.L2_L2ee7f1c07bbc:
	ldi r18, 4
	ldi r19, 0
	ldi r20, 0
	ldi r21, 0
	sts 1610629128, r18
	sts 1610629129, r19
	sts 1610629130, r20
	sts 1610629131, r21
	ldi r24, 16
	ldi r25, 39
	call cycles_sleep
	ldi r18, 4
	ldi r19, 0
	ldi r20, 0
	ldi r21, 0
	sts 1610629132, r18
	sts 1610629133, r19
	sts 1610629134, r20
	sts 1610629135, r21
	ldi r24, 16
	ldi r25, 39
	call cycles_sleep
	jmp .L2_L2ee7f1c07bbc
.L2_L9126723bada4:
.set .LFRAME2, 0
.LFE2:
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	pop r29
	pop r28
	ret
.size main, .-main

