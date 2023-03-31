/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const _SYS_INOTIFY_H: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const __USE_ANSI: u32 = 1;
pub const _BSD_SOURCE: u32 = 1;
pub const _SVID_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_BSD: u32 = 1;
pub const __USE_SVID: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201103;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 17;
pub const __GLIBC_HAVE_LONG_LONG: u32 = 1;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const _BITS_WCHAR_H: u32 = 1;
pub const __WCHAR_MIN: i32 = -2147483648;
pub const __WCHAR_MAX: u32 = 2147483647;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WCHAR_MIN: i32 = -2147483648;
pub const WCHAR_MAX: u32 = 2147483647;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const IN_ACCESS: u32 = 1;
pub const IN_MODIFY: u32 = 2;
pub const IN_ATTRIB: u32 = 4;
pub const IN_CLOSE_WRITE: u32 = 8;
pub const IN_CLOSE_NOWRITE: u32 = 16;
pub const IN_CLOSE: u32 = 24;
pub const IN_OPEN: u32 = 32;
pub const IN_MOVED_FROM: u32 = 64;
pub const IN_MOVED_TO: u32 = 128;
pub const IN_MOVE: u32 = 192;
pub const IN_CREATE: u32 = 256;
pub const IN_DELETE: u32 = 512;
pub const IN_DELETE_SELF: u32 = 1024;
pub const IN_MOVE_SELF: u32 = 2048;
pub const IN_UNMOUNT: u32 = 8192;
pub const IN_Q_OVERFLOW: u32 = 16384;
pub const IN_IGNORED: u32 = 32768;
pub const IN_ONLYDIR: u32 = 16777216;
pub const IN_DONT_FOLLOW: u32 = 33554432;
pub const IN_EXCL_UNLINK: u32 = 67108864;
pub const IN_MASK_ADD: u32 = 536870912;
pub const IN_ISDIR: u32 = 1073741824;
pub const IN_ONESHOT: u32 = 2147483648;
pub const IN_ALL_EVENTS: u32 = 4095;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub const IN_CLOEXEC: _bindgen_ty_1 = 524288;
pub const IN_NONBLOCK: _bindgen_ty_1 = 2048;
pub type _bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Debug)]
pub struct inotify_event {
    pub wd: ::std::os::raw::c_int,
    pub mask: u32,
    pub cookie: u32,
    pub len: u32,
    pub name: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_inotify_event() {
    assert_eq!(
        ::std::mem::size_of::<inotify_event>(),
        16usize,
        concat!("Size of: ", stringify!(inotify_event))
    );
    assert_eq!(
        ::std::mem::align_of::<inotify_event>(),
        4usize,
        concat!("Alignment of ", stringify!(inotify_event))
    );
}
extern "C" {
    pub fn inotify_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inotify_init1(__flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inotify_add_watch(
        __fd: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __mask: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inotify_rm_watch(
        __fd: ::std::os::raw::c_int,
        __wd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
