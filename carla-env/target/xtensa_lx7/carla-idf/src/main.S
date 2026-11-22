.text
.balign 4
.globl cycles_sleep
.type cycles_sleep, @function
cycles_sleep:
.LFP0:
	movi a8, .LFRAME0
	sub a1, a1, a8
	s32i a0, a1, 0
	s32i a15, a1, 4
	mov a15, a1
	s32i a2, a15, 8
	movi a2, 0
	s32i a2, a15, 12
.L0_L31538b46f2dc:
	l32i a2, a15, 12
	l32i a3, a15, 8
	movi a4, 1
	bge a3, a2, 1f
	movi a4, 0
1:
	mov a2, a4
	s8i a2, a15, 16
	l8ui a2, a15, 16
	beqz a2, .L0_L661993df8e5c
.L0_L3d84ac35b7c8:
	l32i a2, a15, 12
	movi a3, 1
	add a2, a2, a3
	s32i a2, a15, 20
	l32i a2, a15, 20
	s32i a2, a15, 12
	j .L0_L31538b46f2dc
.L0_L661993df8e5c:
.set .LFRAME0, 32
.LFE0:
	movi a8, .LFRAME0
	add a8, a15, a8
	l32i a0, a15, 0
	l32i a15, a15, 4
	mov a1, a8
	ret
.size cycles_sleep, .-cycles_sleep

.text
.balign 4
.globl app_main
.type app_main, @function
app_main:
.LFP1:
	movi a8, -16
	and a1, a1, a8
	movi a8, .LFRAME1
	sub a1, a1, a8
	mov a15, a1
	call0 main
	s32i a2, a15, 8
.set .LFRAME1, 16
.LFE1:
.LHALT1:
	j .LHALT1
.size app_main, .-app_main

.text
.balign 4
.globl main
.type main, @function
main:
.LFP2:
	movi a8, .LFRAME2
	sub a1, a1, a8
	s32i a0, a1, 0
	s32i a15, a1, 4
	mov a15, a1
	movi a2, 6144
	memw
	movi a4, 1610649612
	s32i a2, a4, 0
	memw
	movi a2, 256
	memw
	movi a4, 1610630492
	s32i a2, a4, 0
	memw
	movi a2, 4
	memw
	movi a4, 1610629156
	s32i a2, a4, 0
	memw
.L2_Lc9dd0fa36dd3:
	movi a2, 4
	memw
	movi a4, 1610629128
	s32i a2, a4, 0
	memw
	movi a2, 10000
	call0 cycles_sleep
	movi a2, 4
	memw
	movi a4, 1610629132
	s32i a2, a4, 0
	memw
	movi a2, 10000
	call0 cycles_sleep
	j .L2_Lc9dd0fa36dd3
.L2_L685f04d26f5f:
.set .LFRAME2, 16
.LFE2:
	movi a8, .LFRAME2
	add a8, a15, a8
	l32i a0, a15, 0
	l32i a15, a15, 4
	mov a1, a8
	ret
.size main, .-main

