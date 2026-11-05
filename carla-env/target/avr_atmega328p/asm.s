
.pushsection .vectors,"ax",@progbits
jmp _start
.popsection
.globl _start
.type _start, @function
_start:
.LFP0:
	ldi r28, lo8(2303)
	ldi r29, hi8(2303)
	out 62, r29
	out 61, r28
	ldi r17, hi8(__data_end)
	ldi r26, lo8(__data_start)
	ldi r27, hi8(__data_start)
	ldi r30, lo8(__data_load_start)
	ldi r31, hi8(__data_load_start)
	rjmp .Lt1
.Lt0:
	lpm r0, Z+
	st X+, r0
.Lt1:
	cpi r26, lo8(__data_end)
	cpc r27, r17
	brne .Lt0
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME0)
	sbci r25, hi8(.LFRAME0)
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
	.set .LFRAME0, 8
.LFE0:
	cli
	rjmp .-2
.size _start, . - _start

.globl main
.type main, @function
main:
.LFP1:
	push r28
	push r29
	in r28, 61
	in r29, 62
	in r24, 61
	in r25, 62
	subi r24, lo8(.LFRAME1)
	sbci r25, hi8(.LFRAME1)
	in r0, 63
	cli
	out 62, r25
	out 63, r0
	out 61, r24
	clr r18
	clr r19
	clr r20
	clr r21
	clr r22
	clr r23
	clr r24
	clr r25
	jmp .LFE1
	.set .LFRAME1, 0
.LFE1:
	in r0, 63
	cli
	out 62, r29
	out 63, r0
	out 61, r28
	pop r29
	pop r28
	ret
.size main, . - main
