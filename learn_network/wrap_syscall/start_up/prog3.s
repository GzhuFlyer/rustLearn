	.file	"prog3.c"
	.text
	.globl	preinit
	.type	preinit, @function
preinit:
.LFB2:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$32, %rsp
	movl	%edi, -4(%rbp)
	movq	%rsi, -16(%rbp)
	movq	%rdx, -24(%rbp)
	movl	$__FUNCTION__.2889, %edi
	call	puts
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE2:
	.size	preinit, .-preinit
	.globl	init
	.type	init, @function
init:
.LFB3:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$32, %rsp
	movl	%edi, -4(%rbp)
	movq	%rsi, -16(%rbp)
	movq	%rdx, -24(%rbp)
	movl	$__FUNCTION__.2895, %edi
	call	puts
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE3:
	.size	init, .-init
	.globl	fini
	.type	fini, @function
fini:
.LFB4:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$__FUNCTION__.2898, %edi
	call	puts
	nop
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE4:
	.size	fini, .-fini
	.globl	__init
	.section	.init_array,"aw"
	.align 8
	.type	__init, @object
	.size	__init, 8
__init:
	.quad	init
	.globl	__preinit
	.section	.preinit_array,"aw"
	.align 8
	.type	__preinit, @object
	.size	__preinit, 8
__preinit:
	.quad	preinit
	.globl	__fini
	.section	.fini_array,"aw"
	.align 8
	.type	__fini, @object
	.size	__fini, 8
__fini:
	.quad	fini
	.text
	.globl	constructor
	.type	constructor, @function
constructor:
.LFB5:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$__FUNCTION__.2904, %edi
	call	puts
	nop
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE5:
	.size	constructor, .-constructor
	.section	.init_array
	.align 8
	.quad	constructor
	.text
	.globl	destructor
	.type	destructor, @function
destructor:
.LFB6:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$__FUNCTION__.2907, %edi
	call	puts
	nop
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE6:
	.size	destructor, .-destructor
	.section	.fini_array
	.align 8
	.quad	destructor
	.text
	.globl	my_atexit
	.type	my_atexit, @function
my_atexit:
.LFB7:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$__FUNCTION__.2910, %edi
	call	puts
	nop
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE7:
	.size	my_atexit, .-my_atexit
	.globl	my_atexit2
	.type	my_atexit2, @function
my_atexit2:
.LFB8:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$__FUNCTION__.2913, %edi
	call	puts
	nop
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE8:
	.size	my_atexit2, .-my_atexit2
	.globl	main
	.type	main, @function
main:
.LFB9:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$my_atexit, %edi
	call	atexit
	movl	$my_atexit2, %edi
	call	atexit
	movl	$0, %eax
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE9:
	.size	main, .-main
	.section	.rodata
	.align 8
	.type	__FUNCTION__.2889, @object
	.size	__FUNCTION__.2889, 8
__FUNCTION__.2889:
	.string	"preinit"
	.type	__FUNCTION__.2895, @object
	.size	__FUNCTION__.2895, 5
__FUNCTION__.2895:
	.string	"init"
	.type	__FUNCTION__.2898, @object
	.size	__FUNCTION__.2898, 5
__FUNCTION__.2898:
	.string	"fini"
	.align 8
	.type	__FUNCTION__.2904, @object
	.size	__FUNCTION__.2904, 12
__FUNCTION__.2904:
	.string	"constructor"
	.align 8
	.type	__FUNCTION__.2907, @object
	.size	__FUNCTION__.2907, 11
__FUNCTION__.2907:
	.string	"destructor"
	.align 8
	.type	__FUNCTION__.2910, @object
	.size	__FUNCTION__.2910, 10
__FUNCTION__.2910:
	.string	"my_atexit"
	.align 8
	.type	__FUNCTION__.2913, @object
	.size	__FUNCTION__.2913, 11
__FUNCTION__.2913:
	.string	"my_atexit2"
	.ident	"GCC: (GNU) 9.3.1 20200408 (Red Hat 9.3.1-2)"
	.section	.note.GNU-stack,"",@progbits
