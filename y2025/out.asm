.section __TEXT,__text,regular,pure_instructions
	.p2align	2
y2025::day7::count:
Lfunc_begin178:
	.cfi_startproc
	sub sp, sp, #80
	.cfi_def_cfa_offset 80
	stp x24, x23, [sp, #16]
	stp x22, x21, [sp, #32]
	stp x20, x19, [sp, #48]
	stp x29, x30, [sp, #64]
	add x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_remember_state
	mov x19, x3
	mov x20, x2
	mov x21, x1
	mov x22, x0
	stp x2, x3, [sp]
	ldr x8, [x1, #24]
	cbz x8, LBB178_6
	ldp x0, x1, [x21, #32]
	mov x2, sp
	bl core::hash::BuildHasher::hash_one
	mov x8, #0
	lsr x11, x0, #57
	ldp x10, x9, [x21]
	dup.8b v0, w11
	movi.2d v1, #0xffffffffffffffff
	mov w11, #24
	and x12, x0, x9
	ldr d2, [x10, x12]
	cmeq.8b v3, v2, v0
	fmov x13, d3
	ands x13, x13, #0x8080808080808080
	b.eq LBB178_4
LBB178_2:
	rbit x14, x13
	clz x14, x14
	add x14, x12, x14, lsr #3
	and x14, x14, x9
	mneg x14, x14, x11
	add x14, x10, x14
	ldp x15, x16, [x14, #-24]
	cmp x20, x15
	ccmp x19, x16, #0, eq
	b.eq LBB178_12
	sub x14, x13, #1
	ands x13, x14, x13
	b.ne LBB178_2
LBB178_4:
	cmeq.8b v2, v2, v1
	umaxv.8b b2, v2
	fmov w13, s2
	tbnz w13, #0, LBB178_6
	add x8, x8, #8
	add x0, x12, x8
	and x12, x0, x9
	ldr d2, [x10, x12]
	cmeq.8b v3, v2, v0
	fmov x13, d3
	ands x13, x13, #0x8080808080808080
	b.ne LBB178_2
	b LBB178_4
LBB178_6:
	str x19, [sp]
	ldr x8, [x22, #24]
	cbz x8, LBB178_18
	ldp x0, x1, [x22, #32]
	mov x2, sp
	bl core::hash::BuildHasher::hash_one
	mov x8, #0
	lsr x11, x0, #57
	ldp x9, x10, [x22]
	dup.8b v0, w11
	sub x11, x9, #32
	movi.2d v1, #0xffffffffffffffff
	and x12, x0, x10
	ldr d2, [x9, x12]
	cmeq.8b v3, v2, v0
	fmov x13, d3
	ands x13, x13, #0x8080808080808080
	b.eq LBB178_10
LBB178_8:
	rbit x14, x13
	clz x14, x14
	add x14, x12, x14, lsr #3
	and x14, x14, x10
	sub x15, x11, x14, lsl #5
	ldr x15, [x15]
	cmp x19, x15
	b.eq LBB178_13
	sub x14, x13, #1
	ands x13, x14, x13
	b.ne LBB178_8
LBB178_10:
	cmeq.8b v2, v2, v1
	umaxv.8b b2, v2
	fmov w13, s2
	tbnz w13, #0, LBB178_18
	add x8, x8, #8
	add x0, x12, x8
	and x12, x0, x10
	ldr d2, [x9, x12]
	cmeq.8b v3, v2, v0
	fmov x13, d3
	ands x13, x13, #0x8080808080808080
	b.ne LBB178_8
	b LBB178_10
LBB178_12:
	ldur x0, [x14, #-8]
	b LBB178_19
LBB178_13:
	neg x8, x14
	add x9, x9, x8, lsl #5
	ldp x8, x9, [x9, #-16]
	lsl x9, x9, #3
LBB178_14:
	cbz x9, LBB178_18
	ldr x24, [x8], #8
	sub x9, x9, #8
	cmp x24, x20
	b.ls LBB178_14
	cbz x19, LBB178_20
	sub x3, x19, #1
	mov x0, x22
	mov x1, x21
	mov x2, x24
	bl y2025::day7::count
	mov x23, x0
	b LBB178_21
LBB178_18:
	mov w0, #1
LBB178_19:
	.cfi_def_cfa wsp, 80
	ldp x29, x30, [sp, #64]
	ldp x20, x19, [sp, #48]
	ldp x22, x21, [sp, #32]
	ldp x24, x23, [sp, #16]
	add sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	ret
LBB178_20:
	.cfi_restore_state
	mov x23, #0
LBB178_21:
	add x3, x19, #1
	mov x0, x22
	mov x1, x21
	mov x2, x24
	bl y2025::day7::count
	mov x24, x0
	stp x20, x19, [sp]
	ldp x0, x1, [x21, #32]
	mov x2, sp
	bl core::hash::BuildHasher::hash_one
	mov x22, x0
	ldr x8, [x21, #16]
	cbz x8, LBB178_36
LBB178_22:
	mov x14, #0
	mov x12, #0
	add x0, x24, x23
	ldp x8, x10, [x21]
	lsr x9, x22, #57
	dup.8b v0, w9
	movi.2d v1, #0xffffffffffffffff
	mov w13, #24
	and x15, x22, x10
	ldr d2, [x8, x15]
	cmeq.8b v3, v2, v0
	fmov x16, d3
	ands x16, x16, #0x8080808080808080
	b.eq LBB178_25
LBB178_23:
	rbit x17, x16
	clz x17, x17
	add x17, x15, x17, lsr #3
	and x17, x17, x10
	mneg x1, x17, x13
	add x1, x8, x1
	ldp x2, x1, [x1, #-24]
	cmp x20, x2
	ccmp x19, x1, #0, eq
	b.eq LBB178_32
	sub x17, x16, #1
	ands x16, x17, x16
	b.ne LBB178_23
LBB178_25:
	cmp x14, #1
	b.eq LBB178_28
	cmlt.8b v3, v2, #0
	fmov x11, d3
	cbz x11, LBB178_30
	rbit x11, x11
	clz x11, x11
	add x11, x15, x11, lsr #3
	and x11, x11, x10
LBB178_28:
	cmeq.8b v2, v2, v1
	umaxv.8b b2, v2
	fmov w14, s2
	tbnz w14, #0, LBB178_33
	mov w14, #1
	b LBB178_31
LBB178_30:
	mov x14, #0
LBB178_31:
	add x12, x12, #8
	add x22, x12, x15
	and x15, x22, x10
	ldr d2, [x8, x15]
	cmeq.8b v3, v2, v0
	fmov x16, d3
	ands x16, x16, #0x8080808080808080
	b.ne LBB178_23
	b LBB178_25
LBB178_32:
	neg x9, x17
	b LBB178_35
LBB178_33:
	ldrsb w12, [x8, x11]
	tbz w12, #31, LBB178_37
LBB178_34:
	and x12, x12, #0x1
	sub x13, x11, #8
	and x10, x13, x10
	strb w9, [x8, x11]
	add x10, x8, x10
	strb w9, [x10, #8]
	ldr q0, [x21, #16]
	movi.2d v1, #0xffffffffffffffff
	mov.d v1[0], x12
	sub.2d v0, v0, v1
	str q0, [x21, #16]
	neg x9, x11
	mov w10, #24
	mneg x10, x11, x10
	add x10, x8, x10
	stp x20, x19, [x10, #-24]
LBB178_35:
	mov w10, #24
	madd x8, x9, x10, x8
	stur x0, [x8, #-8]
	b LBB178_19
LBB178_36:
	add x1, x21, #32
	mov x0, x21
	bl hashbrown::raw::RawTable<T,A>::reserve_rehash
	b LBB178_22
LBB178_37:
	ldr d0, [x8]
	cmlt.8b v0, v0, #0
	fmov x11, d0
	rbit x11, x11
	clz x11, x11
	lsr x11, x11, #3
	ldrb w12, [x8, x11]
	b LBB178_34
Lfunc_end178:
	.cfi_endproc

	.p2align	2
y2025::day8::compute_distances_simd:
Lfunc_begin179:
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	mov x26, x7
	mov x19, x6
	mov x25, x5
	mov x20, x4
	mov x24, x3
	mov x22, x2
	mov x23, x1
	mov x27, x0
	ldr x21, [x29, #16]
Lloh958:
	adrp x8, pulp::aarch64::Neon::__static_available::AVAILABLE@GOTPAGE
Lloh959:
	ldr x8, [x8, pulp::aarch64::Neon::__static_available::AVAILABLE@GOTPAGEOFF]
	ldrb w0, [x8]
	cmp w0, #255
	b.eq LBB179_5
	cbz w0, LBB179_6
LBB179_2:
	cbz x22, LBB179_23
	ldp s0, s1, [x27]
	ldr s2, [x27, #8]
	cmp x21, x19
	csel x8, x21, x19, lo
	cmp x8, x20
	csel x8, x8, x20, lo
	sub x9, x22, #1
	cmp x8, x9
	csel x8, x8, x9, lo
	add x8, x8, #1
	cmp x8, #5
	b.hs LBB179_9
	mov x8, #0
	b LBB179_11
LBB179_5:
	bl pulp::aarch64::Neon::__detect_is_available
	cbnz w0, LBB179_2
LBB179_6:
	cbz x22, LBB179_23
	ldp s0, s1, [x27]
	ldr s2, [x27, #8]
	cmp x21, x19
	csel x8, x21, x19, lo
	cmp x8, x20
	csel x8, x8, x20, lo
	sub x9, x22, #1
	cmp x8, x9
	csel x8, x8, x9, lo
	add x8, x8, #1
	cmp x8, #5
	b.hs LBB179_16
	mov x8, #0
	b LBB179_18
LBB179_9:
	ands x9, x8, #0x3
	mov w10, #4
	csel x9, x10, x9, eq
	sub x8, x8, x9
	dup.4s v3, v0[0]
	dup.4s v4, v1[0]
	mov x9, x23
	dup.4s v5, v2[0]
	mov x10, x24
	mov x11, x25
	mov x12, x26
	mov x13, x8
LBB179_10:
	ldr q6, [x9], #16
	fsub.4s v6, v6, v3
	fmul.4s v6, v6, v6
	ldr q7, [x10], #16
	fsub.4s v7, v7, v4
	fmul.4s v7, v7, v7
	fadd.4s v6, v6, v7
	ldr q7, [x11], #16
	fsub.4s v7, v7, v5
	fmul.4s v7, v7, v7
	fadd.4s v6, v6, v7
	fsqrt.4s v6, v6
	str q6, [x12], #16
	subs x13, x13, #4
	b.ne LBB179_10
LBB179_11:
	mov x9, #0
	lsl x16, x8, #2
	add x10, x26, x16
	add x11, x25, x16
	add x12, x24, x16
	sub x13, x21, x8
	sub x14, x19, x8
	sub x15, x20, x8
	add x16, x23, x16
	sub x8, x22, x8
LBB179_12:
	cmp x15, x9
	b.eq LBB179_24
	cmp x14, x9
	b.eq LBB179_25
	cmp x13, x9
	b.eq LBB179_26
	ldr s3, [x16, x9, lsl #2]
	fsub s3, s3, s0
	fmul s3, s3, s3
	ldr s4, [x12, x9, lsl #2]
	fsub s4, s4, s1
	fmul s4, s4, s4
	fadd s3, s3, s4
	ldr s4, [x11, x9, lsl #2]
	fsub s4, s4, s2
	fmul s4, s4, s4
	fadd s3, s3, s4
	fsqrt s3, s3
	str s3, [x10, x9, lsl #2]
	add x9, x9, #1
	cmp x8, x9
	b.ne LBB179_12
	b LBB179_23
LBB179_16:
	ands x9, x8, #0x3
	mov w10, #4
	csel x9, x10, x9, eq
	sub x8, x8, x9
	dup.4s v3, v0[0]
	dup.4s v4, v1[0]
	mov x9, x23
	dup.4s v5, v2[0]
	mov x10, x24
	mov x11, x25
	mov x12, x26
	mov x13, x8
LBB179_17:
	ldr q6, [x9], #16
	fsub.4s v6, v6, v3
	fmul.4s v6, v6, v6
	ldr q7, [x10], #16
	fsub.4s v7, v7, v4
	fmul.4s v7, v7, v7
	fadd.4s v6, v6, v7
	ldr q7, [x11], #16
	fsub.4s v7, v7, v5
	fmul.4s v7, v7, v7
	fadd.4s v6, v6, v7
	fsqrt.4s v6, v6
	str q6, [x12], #16
	subs x13, x13, #4
	b.ne LBB179_17
LBB179_18:
	mov x9, #0
	lsl x16, x8, #2
	add x10, x26, x16
	add x11, x25, x16
	add x12, x24, x16
	sub x13, x21, x8
	sub x14, x19, x8
	sub x15, x20, x8
	add x16, x23, x16
	sub x8, x22, x8
LBB179_19:
	cmp x15, x9
	b.eq LBB179_24
	cmp x14, x9
	b.eq LBB179_25
	cmp x13, x9
	b.eq LBB179_26
	ldr s3, [x16, x9, lsl #2]
	fsub s3, s3, s0
	fmul s3, s3, s3
	ldr s4, [x12, x9, lsl #2]
	fsub s4, s4, s1
	fmul s4, s4, s4
	fadd s3, s3, s4
	ldr s4, [x11, x9, lsl #2]
	fsub s4, s4, s2
	fmul s4, s4, s4
	fadd s3, s3, s4
	fsqrt s3, s3
	str s3, [x10, x9, lsl #2]
	add x9, x9, #1
	cmp x8, x9
	b.ne LBB179_19
LBB179_23:
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB179_24:
	.cfi_restore_state
Lloh960:
	adrp x2, l_anon.362c43e43d297b76bd8780629db1a2f6.192@PAGE
Lloh961:
	add x2, x2, l_anon.362c43e43d297b76bd8780629db1a2f6.192@PAGEOFF
	mov x0, x20
	mov x1, x20
	bl core::panicking::panic_bounds_check
LBB179_25:
Lloh962:
	adrp x2, l_anon.362c43e43d297b76bd8780629db1a2f6.193@PAGE
Lloh963:
	add x2, x2, l_anon.362c43e43d297b76bd8780629db1a2f6.193@PAGEOFF
	mov x0, x19
	mov x1, x19
	bl core::panicking::panic_bounds_check
LBB179_26:
Lloh964:
	adrp x2, l_anon.362c43e43d297b76bd8780629db1a2f6.194@PAGE
Lloh965:
	add x2, x2, l_anon.362c43e43d297b76bd8780629db1a2f6.194@PAGEOFF
	mov x0, x21
	mov x1, x21
	bl core::panicking::panic_bounds_check
	.loh AdrpLdrGot	Lloh958, Lloh959
	.loh AdrpAdd	Lloh960, Lloh961
	.loh AdrpAdd	Lloh962, Lloh963
	.loh AdrpAdd	Lloh964, Lloh965
