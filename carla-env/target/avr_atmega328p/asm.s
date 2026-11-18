.text
.globl sleep
.type sleep, @function
sleep:
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
	std Y+1, r18
	std Y+2, r19
	std Y+3, r20
	std Y+4, r21
	std Y+5, r22
	std Y+6, r23
	std Y+7, r24
	std Y+8, r25
	ldi r18, 0
	ldi r19, 0
	ldi r20, 0
	ldi r21, 0
	ldi r22, 0
	ldi r23, 0
	ldi r24, 0
	ldi r25, 0
	std Y+9, r18
	std Y+10, r19
	std Y+11, r20
	std Y+12, r21
	std Y+13, r22
	std Y+14, r23
	std Y+15, r24
	std Y+16, r25
.L0_L9e3dc1450444:
	ldd r18, Y+9
	ldd r19, Y+10
	ldd r20, Y+11
	ldd r21, Y+12
	ldd r22, Y+13
	ldd r23, Y+14
	ldd r24, Y+15
	ldd r25, Y+16
	ldd r0, Y+1
	cp r18, r0
	ldd r0, Y+2
	cpc r19, r0
	ldd r0, Y+3
	cpc r20, r0
	ldd r0, Y+4
	cpc r21, r0
	ldd r0, Y+5
	cpc r22, r0
	ldd r0, Y+6
	cpc r23, r0
	ldd r0, Y+7
	cpc r24, r0
	ldd r0, Y+8
	cpc r25, r0
	ldi r18, 1
	breq .LX0_0
	brlt .LX0_0
	ldi r18, 0
.LX0_0:
	std Y+17, r18
	ldd r18, Y+17
	tst r18
	brne .LX0_1
	jmp .L0_L7d2cf52c29c7
.LX0_1:
.L0_Ld50fb7dd8c65:
	ldd r18, Y+9
	ldd r19, Y+10
	ldd r20, Y+11
	ldd r21, Y+12
	ldd r22, Y+13
	ldd r23, Y+14
	ldd r24, Y+15
	ldd r25, Y+16
	ldi r26, 1
	add r18, r26
	ldi r26, 0
	adc r19, r26
	ldi r26, 0
	adc r20, r26
	ldi r26, 0
	adc r21, r26
	ldi r26, 0
	adc r22, r26
	ldi r26, 0
	adc r23, r26
	ldi r26, 0
	adc r24, r26
	ldi r26, 0
	adc r25, r26
	std Y+18, r18
	std Y+19, r19
	std Y+20, r20
	std Y+21, r21
	std Y+22, r22
	std Y+23, r23
	std Y+24, r24
	std Y+25, r25
	ldd r18, Y+18
	ldd r19, Y+19
	ldd r20, Y+20
	ldd r21, Y+21
	ldd r22, Y+22
	ldd r23, Y+23
	ldd r24, Y+24
	ldd r25, Y+25
	std Y+9, r18
	std Y+10, r19
	std Y+11, r20
	std Y+12, r21
	std Y+13, r22
	std Y+14, r23
	std Y+15, r24
	std Y+16, r25
	jmp .L0_L9e3dc1450444
.L0_L7d2cf52c29c7:
.set .LFRAME0, 25
.LFE0:
	subi r28, 231
	sbci r29, 255
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	pop r29
	pop r28
	ret
.size sleep, .-sleep

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
	std Y+1, r18
	std Y+2, r19
	std Y+3, r20
	std Y+4, r21
	std Y+5, r22
	std Y+6, r23
	std Y+7, r24
	std Y+8, r25
.set .LFRAME1, 8
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
	in r18, 4
	std Y+1, r18
	ldd r18, Y+1
	ldi r26, 32
	or r18, r26
	std Y+2, r18
	ldd r18, Y+2
	out 4, r18
.L2_Lf0d10b22324a:
	in r18, 5
	std Y+3, r18
	ldd r18, Y+3
	ldi r26, 32
	or r18, r26
	std Y+4, r18
	ldd r18, Y+4
	out 5, r18
	ldi r18, 80
	ldi r19, 195
	ldi r20, 0
	ldi r21, 0
	ldi r22, 0
	ldi r23, 0
	ldi r24, 0
	ldi r25, 0
	call sleep
	in r18, 5
	std Y+5, r18
	ldd r18, Y+5
	ldi r26, 223
	and r18, r26
	std Y+6, r18
	ldd r18, Y+6
	out 5, r18
	ldi r18, 80
	ldi r19, 195
	ldi r20, 0
	ldi r21, 0
	ldi r22, 0
	ldi r23, 0
	ldi r24, 0
	ldi r25, 0
	call sleep
	jmp .L2_Lf0d10b22324a
.L2_L832f34557e40:
.set .LFRAME2, 6
.LFE2:
	subi r28, 250
	sbci r29, 255
	in r0, 0x3f
	cli
	out 0x3e, r29
	out 0x3f, r0
	out 0x3d, r28
	pop r29
	pop r28
	ret
.size main, .-main

