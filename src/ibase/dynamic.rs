//!
//! Rust Firebird Client
//!
//! Dynamic loaded fbclient functions
//!

#![allow(non_upper_case_globals, dead_code, non_camel_case_types)]

use std::env;

use lazy_static::lazy_static;
use libloading::{Library, Symbol};

lazy_static! {
    static ref LIB: Library = Library::new("./fbclient.lib").unwrap();
    pub static ref isc_attach_database: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: ::std::os::raw::c_short,
            arg3: *const ISC_SCHAR,
            arg4: *mut isc_db_handle,
            arg5: ::std::os::raw::c_short,
            arg6: *const ISC_SCHAR,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_attach_database").unwrap() };
    pub static ref isc_detach_database: Symbol<
        'static,
        unsafe extern "C" fn(arg1: *mut ISC_STATUS, arg2: *mut isc_db_handle) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_detach_database").unwrap() };
    pub static ref isc_drop_database: Symbol<
        'static,
        unsafe extern "C" fn(arg1: *mut ISC_STATUS, arg2: *mut isc_db_handle) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_drop_database").unwrap() };
    pub static ref isc_dsql_allocate_statement: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_db_handle,
            arg3: *mut isc_stmt_handle,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_allocate_statement").unwrap() };
    pub static ref isc_dsql_prepare: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_tr_handle,
            arg3: *mut isc_stmt_handle,
            arg4: ::std::os::raw::c_ushort,
            arg5: *const ISC_SCHAR,
            arg6: ::std::os::raw::c_ushort,
            arg7: *mut XSQLDA,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_prepare").unwrap() };
    pub static ref isc_dsql_execute: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_tr_handle,
            arg3: *mut isc_stmt_handle,
            arg4: ::std::os::raw::c_ushort,
            arg5: *const XSQLDA,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_execute").unwrap() };
    pub static ref isc_dsql_execute_immediate: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_db_handle,
            arg3: *mut isc_tr_handle,
            arg4: ::std::os::raw::c_ushort,
            arg5: *const ISC_SCHAR,
            arg6: ::std::os::raw::c_ushort,
            arg7: *const XSQLDA,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_execute_immediate").unwrap() };
    pub static ref isc_dsql_fetch: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_stmt_handle,
            arg3: ::std::os::raw::c_ushort,
            arg4: *const XSQLDA,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_fetch").unwrap() };
    pub static ref isc_commit_retaining: Symbol<
        'static,
        unsafe extern "C" fn(arg1: *mut ISC_STATUS, arg2: *mut isc_tr_handle) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_commit_retaining").unwrap() };
    pub static ref isc_start_transaction: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_tr_handle,
            arg3: ::std::os::raw::c_short,
            ...
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_start_transaction").unwrap() };
    pub static ref isc_rollback_transaction: Symbol<
        'static,
        unsafe extern "C" fn(arg1: *mut ISC_STATUS, arg2: *mut isc_tr_handle) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_rollback_transaction").unwrap() };
    pub static ref isc_dsql_describe: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_stmt_handle,
            arg3: ::std::os::raw::c_ushort,
            arg4: *mut XSQLDA,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_describe").unwrap() };
    pub static ref isc_dsql_describe_bind: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_stmt_handle,
            arg3: ::std::os::raw::c_ushort,
            arg4: *mut XSQLDA,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_describe_bind").unwrap() };
    pub static ref isc_dsql_free_statement: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_STATUS,
            arg2: *mut isc_stmt_handle,
            arg3: ::std::os::raw::c_ushort,
        ) -> ISC_STATUS,
    > = unsafe { LIB.get(b"isc_dsql_free_statement").unwrap() };
    pub static ref isc_sqlcode: Symbol<'static, unsafe extern "C" fn(arg1: *const ISC_STATUS) -> ISC_LONG> =
        unsafe { LIB.get(b"isc_sqlcode").unwrap() };
    pub static ref fb_interpret: Symbol<
        'static,
        unsafe extern "C" fn(
            arg1: *mut ISC_SCHAR,
            arg2: ::std::os::raw::c_uint,
            arg3: *mut *const ISC_STATUS,
        ) -> ISC_LONG,
    > = unsafe { LIB.get(b"fb_interpret").unwrap() };
}

mod consts {
    #![allow(non_upper_case_globals, dead_code, non_camel_case_types)]

    pub const FB_API_VER: u32 = 30;
    pub const ISC_TRUE: u32 = 1;
    pub const ISC_FALSE: u32 = 0;
    pub const ISC__TRUE: u32 = 1;
    pub const ISC__FALSE: u32 = 0;
    pub const _INTTYPES_H: u32 = 1;
    pub const _FEATURES_H: u32 = 1;
    pub const _DEFAULT_SOURCE: u32 = 1;
    pub const __USE_ISOC11: u32 = 1;
    pub const __USE_ISOC99: u32 = 1;
    pub const __USE_ISOC95: u32 = 1;
    pub const __USE_POSIX_IMPLICITLY: u32 = 1;
    pub const _POSIX_SOURCE: u32 = 1;
    pub const _POSIX_C_SOURCE: u32 = 200809;
    pub const __USE_POSIX: u32 = 1;
    pub const __USE_POSIX2: u32 = 1;
    pub const __USE_POSIX199309: u32 = 1;
    pub const __USE_POSIX199506: u32 = 1;
    pub const __USE_XOPEN2K: u32 = 1;
    pub const __USE_XOPEN2K8: u32 = 1;
    pub const _ATFILE_SOURCE: u32 = 1;
    pub const __USE_MISC: u32 = 1;
    pub const __USE_ATFILE: u32 = 1;
    pub const __USE_FORTIFY_LEVEL: u32 = 0;
    pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
    pub const _STDC_PREDEF_H: u32 = 1;
    pub const __STDC_IEC_559__: u32 = 1;
    pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
    pub const __STDC_ISO_10646__: u32 = 201706;
    pub const __STDC_NO_THREADS__: u32 = 1;
    pub const __GNU_LIBRARY__: u32 = 6;
    pub const __GLIBC__: u32 = 2;
    pub const __GLIBC_MINOR__: u32 = 26;
    pub const _SYS_CDEFS_H: u32 = 1;
    pub const __glibc_c99_flexarr_available: u32 = 1;
    pub const __WORDSIZE: u32 = 64;
    pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
    pub const __SYSCALL_WORDSIZE: u32 = 64;
    pub const __HAVE_GENERIC_SELECTION: u32 = 1;
    pub const _STDINT_H: u32 = 1;
    pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
    pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
    pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
    pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
    pub const _BITS_TYPES_H: u32 = 1;
    pub const _BITS_TYPESIZES_H: u32 = 1;
    pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
    pub const __INO_T_MATCHES_INO64_T: u32 = 1;
    pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
    pub const __FD_SETSIZE: u32 = 1024;
    pub const _BITS_WCHAR_H: u32 = 1;
    pub const _BITS_STDINT_INTN_H: u32 = 1;
    pub const _BITS_STDINT_UINTN_H: u32 = 1;
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
    pub const WINT_MIN: u32 = 0;
    pub const WINT_MAX: u32 = 4294967295;
    pub const ____gwchar_t_defined: u32 = 1;
    pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
    pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
    pub const PRId8: &'static [u8; 2usize] = b"d\0";
    pub const PRId16: &'static [u8; 2usize] = b"d\0";
    pub const PRId32: &'static [u8; 2usize] = b"d\0";
    pub const PRId64: &'static [u8; 3usize] = b"ld\0";
    pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
    pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
    pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
    pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
    pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
    pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
    pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
    pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
    pub const PRIi8: &'static [u8; 2usize] = b"i\0";
    pub const PRIi16: &'static [u8; 2usize] = b"i\0";
    pub const PRIi32: &'static [u8; 2usize] = b"i\0";
    pub const PRIi64: &'static [u8; 3usize] = b"li\0";
    pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
    pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
    pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
    pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
    pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
    pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
    pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
    pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
    pub const PRIo8: &'static [u8; 2usize] = b"o\0";
    pub const PRIo16: &'static [u8; 2usize] = b"o\0";
    pub const PRIo32: &'static [u8; 2usize] = b"o\0";
    pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
    pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
    pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
    pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
    pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
    pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
    pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
    pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
    pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
    pub const PRIu8: &'static [u8; 2usize] = b"u\0";
    pub const PRIu16: &'static [u8; 2usize] = b"u\0";
    pub const PRIu32: &'static [u8; 2usize] = b"u\0";
    pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
    pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
    pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
    pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
    pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
    pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
    pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
    pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
    pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
    pub const PRIx8: &'static [u8; 2usize] = b"x\0";
    pub const PRIx16: &'static [u8; 2usize] = b"x\0";
    pub const PRIx32: &'static [u8; 2usize] = b"x\0";
    pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
    pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
    pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
    pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
    pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
    pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
    pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
    pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
    pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
    pub const PRIX8: &'static [u8; 2usize] = b"X\0";
    pub const PRIX16: &'static [u8; 2usize] = b"X\0";
    pub const PRIX32: &'static [u8; 2usize] = b"X\0";
    pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
    pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
    pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
    pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
    pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
    pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
    pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
    pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
    pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
    pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
    pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
    pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
    pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
    pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
    pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
    pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
    pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
    pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
    pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
    pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
    pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
    pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
    pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
    pub const SCNd32: &'static [u8; 2usize] = b"d\0";
    pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
    pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
    pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
    pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
    pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
    pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
    pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
    pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
    pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
    pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
    pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
    pub const SCNi32: &'static [u8; 2usize] = b"i\0";
    pub const SCNi64: &'static [u8; 3usize] = b"li\0";
    pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
    pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
    pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
    pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
    pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
    pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
    pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
    pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
    pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
    pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
    pub const SCNu32: &'static [u8; 2usize] = b"u\0";
    pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
    pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
    pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
    pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
    pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
    pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
    pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
    pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
    pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
    pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
    pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
    pub const SCNo32: &'static [u8; 2usize] = b"o\0";
    pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
    pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
    pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
    pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
    pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
    pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
    pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
    pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
    pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
    pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
    pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
    pub const SCNx32: &'static [u8; 2usize] = b"x\0";
    pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
    pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
    pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
    pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
    pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
    pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
    pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
    pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
    pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
    pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
    pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
    pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
    pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
    pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
    pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
    pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
    pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
    pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
    pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
    pub const ISC_STATUS_LENGTH: u32 = 20;
    pub const FB_SQLSTATE_LENGTH: u32 = 5;
    pub const FB_SQLSTATE_SIZE: u32 = 6;
    pub const FB_FALSE: u8 = 0u8;
    pub const FB_TRUE: u8 = 1u8;
    pub const DSC_null: u32 = 1;
    pub const DSC_no_subtype: u32 = 2;
    pub const DSC_nullable: u32 = 4;
    pub const dtype_unknown: u32 = 0;
    pub const dtype_text: u32 = 1;
    pub const dtype_cstring: u32 = 2;
    pub const dtype_varying: u32 = 3;
    pub const dtype_packed: u32 = 6;
    pub const dtype_byte: u32 = 7;
    pub const dtype_short: u32 = 8;
    pub const dtype_long: u32 = 9;
    pub const dtype_quad: u32 = 10;
    pub const dtype_real: u32 = 11;
    pub const dtype_double: u32 = 12;
    pub const dtype_d_float: u32 = 13;
    pub const dtype_sql_date: u32 = 14;
    pub const dtype_sql_time: u32 = 15;
    pub const dtype_timestamp: u32 = 16;
    pub const dtype_blob: u32 = 17;
    pub const dtype_array: u32 = 18;
    pub const dtype_int64: u32 = 19;
    pub const dtype_dbkey: u32 = 20;
    pub const dtype_boolean: u32 = 21;
    pub const DTYPE_TYPE_MAX: u32 = 22;
    pub const ISC_TIME_SECONDS_PRECISION: u32 = 10000;
    pub const ISC_TIME_SECONDS_PRECISION_SCALE: i32 = -4;
    pub const DSQL_close: u16 = 1;
    pub const DSQL_drop: u16 = 2;
    pub const DSQL_unprepare: u16 = 4;
    pub const SQLDA_VERSION1: u32 = 1;
    pub const SQL_TEXT: u32 = 452;
    pub const SQL_VARYING: u32 = 448;
    pub const SQL_SHORT: u32 = 500;
    pub const SQL_LONG: u32 = 496;
    pub const SQL_FLOAT: u32 = 482;
    pub const SQL_DOUBLE: u32 = 480;
    pub const SQL_D_FLOAT: u32 = 530;
    pub const SQL_TIMESTAMP: u32 = 510;
    pub const SQL_BLOB: u32 = 520;
    pub const SQL_ARRAY: u32 = 540;
    pub const SQL_QUAD: u32 = 550;
    pub const SQL_TYPE_TIME: u32 = 560;
    pub const SQL_TYPE_DATE: u32 = 570;
    pub const SQL_INT64: u32 = 580;
    pub const SQL_BOOLEAN: u32 = 32764;
    pub const SQL_NULL: u32 = 32766;
    pub const SQL_DATE: u32 = 510;
    pub const SQL_DIALECT_V5: u32 = 1;
    pub const SQL_DIALECT_V6_TRANSITION: u32 = 2;
    pub const SQL_DIALECT_V6: u32 = 3;
    pub const SQL_DIALECT_CURRENT: u32 = 3;
    pub const sec_uid_spec: u32 = 1;
    pub const sec_gid_spec: u32 = 2;
    pub const sec_server_spec: u32 = 4;
    pub const sec_password_spec: u32 = 8;
    pub const sec_group_name_spec: u32 = 16;
    pub const sec_first_name_spec: u32 = 32;
    pub const sec_middle_name_spec: u32 = 64;
    pub const sec_last_name_spec: u32 = 128;
    pub const sec_dba_user_name_spec: u32 = 256;
    pub const sec_dba_password_spec: u32 = 512;
    pub const sec_protocol_tcpip: u32 = 1;
    pub const sec_protocol_netbeui: u32 = 2;
    pub const sec_protocol_spx: u32 = 3;
    pub const sec_protocol_local: u32 = 4;
    pub const isc_blob_filter_open: u32 = 0;
    pub const isc_blob_filter_get_segment: u32 = 1;
    pub const isc_blob_filter_close: u32 = 2;
    pub const isc_blob_filter_create: u32 = 3;
    pub const isc_blob_filter_put_segment: u32 = 4;
    pub const isc_blob_filter_alloc: u32 = 5;
    pub const isc_blob_filter_free: u32 = 6;
    pub const isc_blob_filter_seek: u32 = 7;
    pub const isc_dpb_version1: u32 = 1;
    pub const isc_dpb_version2: u32 = 2;
    pub const isc_dpb_cdd_pathname: u32 = 1;
    pub const isc_dpb_allocation: u32 = 2;
    pub const isc_dpb_journal: u32 = 3;
    pub const isc_dpb_page_size: u32 = 4;
    pub const isc_dpb_num_buffers: u32 = 5;
    pub const isc_dpb_buffer_length: u32 = 6;
    pub const isc_dpb_debug: u32 = 7;
    pub const isc_dpb_garbage_collect: u32 = 8;
    pub const isc_dpb_verify: u32 = 9;
    pub const isc_dpb_sweep: u32 = 10;
    pub const isc_dpb_enable_journal: u32 = 11;
    pub const isc_dpb_disable_journal: u32 = 12;
    pub const isc_dpb_dbkey_scope: u32 = 13;
    pub const isc_dpb_number_of_users: u32 = 14;
    pub const isc_dpb_trace: u32 = 15;
    pub const isc_dpb_no_garbage_collect: u32 = 16;
    pub const isc_dpb_damaged: u32 = 17;
    pub const isc_dpb_license: u32 = 18;
    pub const isc_dpb_sys_user_name: u32 = 19;
    pub const isc_dpb_encrypt_key: u32 = 20;
    pub const isc_dpb_activate_shadow: u32 = 21;
    pub const isc_dpb_sweep_interval: u32 = 22;
    pub const isc_dpb_delete_shadow: u32 = 23;
    pub const isc_dpb_force_write: u32 = 24;
    pub const isc_dpb_begin_log: u32 = 25;
    pub const isc_dpb_quit_log: u32 = 26;
    pub const isc_dpb_no_reserve: u32 = 27;
    pub const isc_dpb_user_name: u16 = 28;
    pub const isc_dpb_password: u16 = 29;
    pub const isc_dpb_password_enc: u32 = 30;
    pub const isc_dpb_sys_user_name_enc: u32 = 31;
    pub const isc_dpb_interp: u32 = 32;
    pub const isc_dpb_online_dump: u32 = 33;
    pub const isc_dpb_old_file_size: u32 = 34;
    pub const isc_dpb_old_num_files: u32 = 35;
    pub const isc_dpb_old_file: u32 = 36;
    pub const isc_dpb_old_start_page: u32 = 37;
    pub const isc_dpb_old_start_seqno: u32 = 38;
    pub const isc_dpb_old_start_file: u32 = 39;
    pub const isc_dpb_drop_walfile: u32 = 40;
    pub const isc_dpb_old_dump_id: u32 = 41;
    pub const isc_dpb_wal_backup_dir: u32 = 42;
    pub const isc_dpb_wal_chkptlen: u32 = 43;
    pub const isc_dpb_wal_numbufs: u32 = 44;
    pub const isc_dpb_wal_bufsize: u32 = 45;
    pub const isc_dpb_wal_grp_cmt_wait: u32 = 46;
    pub const isc_dpb_lc_messages: u32 = 47;
    pub const isc_dpb_lc_ctype: u32 = 48;
    pub const isc_dpb_cache_manager: u32 = 49;
    pub const isc_dpb_shutdown: u32 = 50;
    pub const isc_dpb_online: u32 = 51;
    pub const isc_dpb_shutdown_delay: u32 = 52;
    pub const isc_dpb_reserved: u32 = 53;
    pub const isc_dpb_overwrite: u32 = 54;
    pub const isc_dpb_sec_attach: u32 = 55;
    pub const isc_dpb_disable_wal: u32 = 56;
    pub const isc_dpb_connect_timeout: u32 = 57;
    pub const isc_dpb_dummy_packet_interval: u32 = 58;
    pub const isc_dpb_gbak_attach: u32 = 59;
    pub const isc_dpb_sql_role_name: u32 = 60;
    pub const isc_dpb_set_page_buffers: u32 = 61;
    pub const isc_dpb_working_directory: u32 = 62;
    pub const isc_dpb_sql_dialect: u32 = 63;
    pub const isc_dpb_set_db_readonly: u32 = 64;
    pub const isc_dpb_set_db_sql_dialect: u32 = 65;
    pub const isc_dpb_gfix_attach: u32 = 66;
    pub const isc_dpb_gstat_attach: u32 = 67;
    pub const isc_dpb_set_db_charset: u32 = 68;
    pub const isc_dpb_gsec_attach: u32 = 69;
    pub const isc_dpb_address_path: u32 = 70;
    pub const isc_dpb_process_id: u32 = 71;
    pub const isc_dpb_no_db_triggers: u32 = 72;
    pub const isc_dpb_trusted_auth: u32 = 73;
    pub const isc_dpb_process_name: u32 = 74;
    pub const isc_dpb_trusted_role: u32 = 75;
    pub const isc_dpb_org_filename: u32 = 76;
    pub const isc_dpb_utf8_filename: u32 = 77;
    pub const isc_dpb_ext_call_depth: u32 = 78;
    pub const isc_dpb_auth_block: u32 = 79;
    pub const isc_dpb_client_version: u32 = 80;
    pub const isc_dpb_remote_protocol: u32 = 81;
    pub const isc_dpb_host_name: u32 = 82;
    pub const isc_dpb_os_user: u32 = 83;
    pub const isc_dpb_specific_auth_data: u32 = 84;
    pub const isc_dpb_auth_plugin_list: u32 = 85;
    pub const isc_dpb_auth_plugin_name: u32 = 86;
    pub const isc_dpb_config: u32 = 87;
    pub const isc_dpb_nolinger: u32 = 88;
    pub const isc_dpb_reset_icu: u32 = 89;
    pub const isc_dpb_map_attach: u32 = 90;
    pub const isc_dpb_address: u32 = 1;
    pub const isc_dpb_addr_protocol: u32 = 1;
    pub const isc_dpb_addr_endpoint: u32 = 2;
    pub const isc_dpb_pages: u32 = 1;
    pub const isc_dpb_records: u32 = 2;
    pub const isc_dpb_indices: u32 = 4;
    pub const isc_dpb_transactions: u32 = 8;
    pub const isc_dpb_no_update: u32 = 16;
    pub const isc_dpb_repair: u32 = 32;
    pub const isc_dpb_ignore: u32 = 64;
    pub const isc_dpb_shut_cache: u32 = 1;
    pub const isc_dpb_shut_attachment: u32 = 2;
    pub const isc_dpb_shut_transaction: u32 = 4;
    pub const isc_dpb_shut_force: u32 = 8;
    pub const isc_dpb_shut_mode_mask: u32 = 112;
    pub const isc_dpb_shut_default: u32 = 0;
    pub const isc_dpb_shut_normal: u32 = 16;
    pub const isc_dpb_shut_multi: u32 = 32;
    pub const isc_dpb_shut_single: u32 = 48;
    pub const isc_dpb_shut_full: u32 = 64;
    pub const RDB_system: u32 = 1;
    pub const RDB_id_assigned: u32 = 2;
    pub const isc_tpb_version1: i8 = 1;
    pub const isc_tpb_version3: i8 = 3;
    pub const isc_tpb_consistency: i8 = 1;
    pub const isc_tpb_concurrency: i8 = 2;
    pub const isc_tpb_shared: i8 = 3;
    pub const isc_tpb_protected: i8 = 4;
    pub const isc_tpb_exclusive: i8 = 5;
    pub const isc_tpb_wait: i8 = 6;
    pub const isc_tpb_nowait: i8 = 7;
    pub const isc_tpb_read: i8 = 8;
    pub const isc_tpb_write: i8 = 9;
    pub const isc_tpb_lock_read: i8 = 10;
    pub const isc_tpb_lock_write: i8 = 11;
    pub const isc_tpb_verb_time: i8 = 12;
    pub const isc_tpb_commit_time: i8 = 13;
    pub const isc_tpb_ignore_limbo: i8 = 14;
    pub const isc_tpb_read_committed: i8 = 15;
    pub const isc_tpb_autocommit: i8 = 16;
    pub const isc_tpb_rec_version: i8 = 17;
    pub const isc_tpb_no_rec_version: i8 = 18;
    pub const isc_tpb_restart_requests: i8 = 19;
    pub const isc_tpb_no_auto_undo: i8 = 20;
    pub const isc_tpb_lock_timeout: i8 = 21;
    pub const isc_bpb_version1: u32 = 1;
    pub const isc_bpb_source_type: u32 = 1;
    pub const isc_bpb_target_type: u32 = 2;
    pub const isc_bpb_type: u32 = 3;
    pub const isc_bpb_source_interp: u32 = 4;
    pub const isc_bpb_target_interp: u32 = 5;
    pub const isc_bpb_filter_parameter: u32 = 6;
    pub const isc_bpb_storage: u32 = 7;
    pub const isc_bpb_type_segmented: u32 = 0;
    pub const isc_bpb_type_stream: u32 = 1;
    pub const isc_bpb_storage_main: u32 = 0;
    pub const isc_bpb_storage_temp: u32 = 2;
    pub const isc_spb_version1: u32 = 1;
    pub const isc_spb_current_version: u32 = 2;
    pub const isc_spb_version: u32 = 2;
    pub const isc_spb_version3: u32 = 3;
    pub const isc_spb_user_name: u32 = 28;
    pub const isc_spb_sys_user_name: u32 = 19;
    pub const isc_spb_sys_user_name_enc: u32 = 31;
    pub const isc_spb_password: u32 = 29;
    pub const isc_spb_password_enc: u32 = 30;
    pub const isc_spb_command_line: u32 = 105;
    pub const isc_spb_dbname: u32 = 106;
    pub const isc_spb_verbose: u32 = 107;
    pub const isc_spb_options: u32 = 108;
    pub const isc_spb_address_path: u32 = 109;
    pub const isc_spb_process_id: u32 = 110;
    pub const isc_spb_trusted_auth: u32 = 111;
    pub const isc_spb_process_name: u32 = 112;
    pub const isc_spb_trusted_role: u32 = 113;
    pub const isc_spb_verbint: u32 = 114;
    pub const isc_spb_auth_block: u32 = 115;
    pub const isc_spb_auth_plugin_name: u32 = 116;
    pub const isc_spb_auth_plugin_list: u32 = 117;
    pub const isc_spb_utf8_filename: u32 = 118;
    pub const isc_spb_client_version: u32 = 119;
    pub const isc_spb_remote_protocol: u32 = 120;
    pub const isc_spb_host_name: u32 = 121;
    pub const isc_spb_os_user: u32 = 122;
    pub const isc_spb_config: u32 = 123;
    pub const isc_spb_expected_db: u32 = 124;
    pub const isc_spb_connect_timeout: u32 = 57;
    pub const isc_spb_dummy_packet_interval: u32 = 58;
    pub const isc_spb_sql_role_name: u32 = 60;
    pub const isc_spb_specific_auth_data: u32 = 111;
    pub const isc_action_svc_backup: u32 = 1;
    pub const isc_action_svc_restore: u32 = 2;
    pub const isc_action_svc_repair: u32 = 3;
    pub const isc_action_svc_add_user: u32 = 4;
    pub const isc_action_svc_delete_user: u32 = 5;
    pub const isc_action_svc_modify_user: u32 = 6;
    pub const isc_action_svc_display_user: u32 = 7;
    pub const isc_action_svc_properties: u32 = 8;
    pub const isc_action_svc_add_license: u32 = 9;
    pub const isc_action_svc_remove_license: u32 = 10;
    pub const isc_action_svc_db_stats: u32 = 11;
    pub const isc_action_svc_get_ib_log: u32 = 12;
    pub const isc_action_svc_get_fb_log: u32 = 12;
    pub const isc_action_svc_nbak: u32 = 20;
    pub const isc_action_svc_nrest: u32 = 21;
    pub const isc_action_svc_trace_start: u32 = 22;
    pub const isc_action_svc_trace_stop: u32 = 23;
    pub const isc_action_svc_trace_suspend: u32 = 24;
    pub const isc_action_svc_trace_resume: u32 = 25;
    pub const isc_action_svc_trace_list: u32 = 26;
    pub const isc_action_svc_set_mapping: u32 = 27;
    pub const isc_action_svc_drop_mapping: u32 = 28;
    pub const isc_action_svc_display_user_adm: u32 = 29;
    pub const isc_action_svc_validate: u32 = 30;
    pub const isc_action_svc_last: u32 = 31;
    pub const isc_info_svc_svr_db_info: u32 = 50;
    pub const isc_info_svc_get_license: u32 = 51;
    pub const isc_info_svc_get_license_mask: u32 = 52;
    pub const isc_info_svc_get_config: u32 = 53;
    pub const isc_info_svc_version: u32 = 54;
    pub const isc_info_svc_server_version: u32 = 55;
    pub const isc_info_svc_implementation: u32 = 56;
    pub const isc_info_svc_capabilities: u32 = 57;
    pub const isc_info_svc_user_dbpath: u32 = 58;
    pub const isc_info_svc_get_env: u32 = 59;
    pub const isc_info_svc_get_env_lock: u32 = 60;
    pub const isc_info_svc_get_env_msg: u32 = 61;
    pub const isc_info_svc_line: u32 = 62;
    pub const isc_info_svc_to_eof: u32 = 63;
    pub const isc_info_svc_timeout: u32 = 64;
    pub const isc_info_svc_get_licensed_users: u32 = 65;
    pub const isc_info_svc_limbo_trans: u32 = 66;
    pub const isc_info_svc_running: u32 = 67;
    pub const isc_info_svc_get_users: u32 = 68;
    pub const isc_info_svc_auth_block: u32 = 69;
    pub const isc_info_svc_stdin: u32 = 78;
    pub const isc_spb_sec_userid: u32 = 5;
    pub const isc_spb_sec_groupid: u32 = 6;
    pub const isc_spb_sec_username: u32 = 7;
    pub const isc_spb_sec_password: u32 = 8;
    pub const isc_spb_sec_groupname: u32 = 9;
    pub const isc_spb_sec_firstname: u32 = 10;
    pub const isc_spb_sec_middlename: u32 = 11;
    pub const isc_spb_sec_lastname: u32 = 12;
    pub const isc_spb_sec_admin: u32 = 13;
    pub const isc_spb_lic_key: u32 = 5;
    pub const isc_spb_lic_id: u32 = 6;
    pub const isc_spb_lic_desc: u32 = 7;
    pub const isc_spb_bkp_file: u32 = 5;
    pub const isc_spb_bkp_factor: u32 = 6;
    pub const isc_spb_bkp_length: u32 = 7;
    pub const isc_spb_bkp_skip_data: u32 = 8;
    pub const isc_spb_bkp_stat: u32 = 15;
    pub const isc_spb_bkp_ignore_checksums: u32 = 1;
    pub const isc_spb_bkp_ignore_limbo: u32 = 2;
    pub const isc_spb_bkp_metadata_only: u32 = 4;
    pub const isc_spb_bkp_no_garbage_collect: u32 = 8;
    pub const isc_spb_bkp_old_descriptions: u32 = 16;
    pub const isc_spb_bkp_non_transportable: u32 = 32;
    pub const isc_spb_bkp_convert: u32 = 64;
    pub const isc_spb_bkp_expand: u32 = 128;
    pub const isc_spb_bkp_no_triggers: u32 = 32768;
    pub const isc_spb_prp_page_buffers: u32 = 5;
    pub const isc_spb_prp_sweep_interval: u32 = 6;
    pub const isc_spb_prp_shutdown_db: u32 = 7;
    pub const isc_spb_prp_deny_new_attachments: u32 = 9;
    pub const isc_spb_prp_deny_new_transactions: u32 = 10;
    pub const isc_spb_prp_reserve_space: u32 = 11;
    pub const isc_spb_prp_write_mode: u32 = 12;
    pub const isc_spb_prp_access_mode: u32 = 13;
    pub const isc_spb_prp_set_sql_dialect: u32 = 14;
    pub const isc_spb_prp_activate: u32 = 256;
    pub const isc_spb_prp_db_online: u32 = 512;
    pub const isc_spb_prp_nolinger: u32 = 1024;
    pub const isc_spb_prp_force_shutdown: u32 = 41;
    pub const isc_spb_prp_attachments_shutdown: u32 = 42;
    pub const isc_spb_prp_transactions_shutdown: u32 = 43;
    pub const isc_spb_prp_shutdown_mode: u32 = 44;
    pub const isc_spb_prp_online_mode: u32 = 45;
    pub const isc_spb_prp_sm_normal: u32 = 0;
    pub const isc_spb_prp_sm_multi: u32 = 1;
    pub const isc_spb_prp_sm_single: u32 = 2;
    pub const isc_spb_prp_sm_full: u32 = 3;
    pub const isc_spb_prp_res_use_full: u32 = 35;
    pub const isc_spb_prp_res: u32 = 36;
    pub const isc_spb_prp_wm_async: u32 = 37;
    pub const isc_spb_prp_wm_sync: u32 = 38;
    pub const isc_spb_prp_am_readonly: u32 = 39;
    pub const isc_spb_prp_am_readwrite: u32 = 40;
    pub const isc_spb_rpr_commit_trans: u32 = 15;
    pub const isc_spb_rpr_rollback_trans: u32 = 34;
    pub const isc_spb_rpr_recover_two_phase: u32 = 17;
    pub const isc_spb_tra_id: u32 = 18;
    pub const isc_spb_single_tra_id: u32 = 19;
    pub const isc_spb_multi_tra_id: u32 = 20;
    pub const isc_spb_tra_state: u32 = 21;
    pub const isc_spb_tra_state_limbo: u32 = 22;
    pub const isc_spb_tra_state_commit: u32 = 23;
    pub const isc_spb_tra_state_rollback: u32 = 24;
    pub const isc_spb_tra_state_unknown: u32 = 25;
    pub const isc_spb_tra_host_site: u32 = 26;
    pub const isc_spb_tra_remote_site: u32 = 27;
    pub const isc_spb_tra_db_path: u32 = 28;
    pub const isc_spb_tra_advise: u32 = 29;
    pub const isc_spb_tra_advise_commit: u32 = 30;
    pub const isc_spb_tra_advise_rollback: u32 = 31;
    pub const isc_spb_tra_advise_unknown: u32 = 33;
    pub const isc_spb_tra_id_64: u32 = 46;
    pub const isc_spb_single_tra_id_64: u32 = 47;
    pub const isc_spb_multi_tra_id_64: u32 = 48;
    pub const isc_spb_rpr_commit_trans_64: u32 = 49;
    pub const isc_spb_rpr_rollback_trans_64: u32 = 50;
    pub const isc_spb_rpr_recover_two_phase_64: u32 = 51;
    pub const isc_spb_rpr_validate_db: u32 = 1;
    pub const isc_spb_rpr_sweep_db: u32 = 2;
    pub const isc_spb_rpr_mend_db: u32 = 4;
    pub const isc_spb_rpr_list_limbo_trans: u32 = 8;
    pub const isc_spb_rpr_check_db: u32 = 16;
    pub const isc_spb_rpr_ignore_checksum: u32 = 32;
    pub const isc_spb_rpr_kill_shadows: u32 = 64;
    pub const isc_spb_rpr_full: u32 = 128;
    pub const isc_spb_rpr_icu: u32 = 2048;
    pub const isc_spb_res_skip_data: u32 = 8;
    pub const isc_spb_res_buffers: u32 = 9;
    pub const isc_spb_res_page_size: u32 = 10;
    pub const isc_spb_res_length: u32 = 11;
    pub const isc_spb_res_access_mode: u32 = 12;
    pub const isc_spb_res_fix_fss_data: u32 = 13;
    pub const isc_spb_res_fix_fss_metadata: u32 = 14;
    pub const isc_spb_res_stat: u32 = 15;
    pub const isc_spb_res_metadata_only: u32 = 4;
    pub const isc_spb_res_deactivate_idx: u32 = 256;
    pub const isc_spb_res_no_shadow: u32 = 512;
    pub const isc_spb_res_no_validity: u32 = 1024;
    pub const isc_spb_res_one_at_a_time: u32 = 2048;
    pub const isc_spb_res_replace: u32 = 4096;
    pub const isc_spb_res_create: u32 = 8192;
    pub const isc_spb_res_use_all_space: u32 = 16384;
    pub const isc_spb_val_tab_incl: u32 = 1;
    pub const isc_spb_val_tab_excl: u32 = 2;
    pub const isc_spb_val_idx_incl: u32 = 3;
    pub const isc_spb_val_idx_excl: u32 = 4;
    pub const isc_spb_val_lock_timeout: u32 = 5;
    pub const isc_spb_res_am_readonly: u32 = 39;
    pub const isc_spb_res_am_readwrite: u32 = 40;
    pub const isc_spb_num_att: u32 = 5;
    pub const isc_spb_num_db: u32 = 6;
    pub const isc_spb_sts_table: u32 = 64;
    pub const isc_spb_sts_data_pages: u32 = 1;
    pub const isc_spb_sts_db_log: u32 = 2;
    pub const isc_spb_sts_hdr_pages: u32 = 4;
    pub const isc_spb_sts_idx_pages: u32 = 8;
    pub const isc_spb_sts_sys_relations: u32 = 16;
    pub const isc_spb_sts_record_versions: u32 = 32;
    pub const isc_spb_sts_nocreation: u32 = 128;
    pub const isc_spb_sts_encryption: u32 = 256;
    pub const isc_spb_nbk_level: u32 = 5;
    pub const isc_spb_nbk_file: u32 = 6;
    pub const isc_spb_nbk_direct: u32 = 7;
    pub const isc_spb_nbk_no_triggers: u32 = 1;
    pub const isc_spb_trc_id: u32 = 1;
    pub const isc_spb_trc_name: u32 = 2;
    pub const isc_spb_trc_cfg: u32 = 3;
    pub const isc_sdl_version1: u32 = 1;
    pub const isc_sdl_eoc: u32 = 255;
    pub const isc_sdl_relation: u32 = 2;
    pub const isc_sdl_rid: u32 = 3;
    pub const isc_sdl_field: u32 = 4;
    pub const isc_sdl_fid: u32 = 5;
    pub const isc_sdl_struct: u32 = 6;
    pub const isc_sdl_variable: u32 = 7;
    pub const isc_sdl_scalar: u32 = 8;
    pub const isc_sdl_tiny_integer: u32 = 9;
    pub const isc_sdl_short_integer: u32 = 10;
    pub const isc_sdl_long_integer: u32 = 11;
    pub const isc_sdl_add: u32 = 13;
    pub const isc_sdl_subtract: u32 = 14;
    pub const isc_sdl_multiply: u32 = 15;
    pub const isc_sdl_divide: u32 = 16;
    pub const isc_sdl_negate: u32 = 17;
    pub const isc_sdl_begin: u32 = 31;
    pub const isc_sdl_end: u32 = 32;
    pub const isc_sdl_do3: u32 = 33;
    pub const isc_sdl_do2: u32 = 34;
    pub const isc_sdl_do1: u32 = 35;
    pub const isc_sdl_element: u32 = 36;
    pub const isc_blob_untyped: u32 = 0;
    pub const isc_blob_text: u32 = 1;
    pub const isc_blob_blr: u32 = 2;
    pub const isc_blob_acl: u32 = 3;
    pub const isc_blob_ranges: u32 = 4;
    pub const isc_blob_summary: u32 = 5;
    pub const isc_blob_format: u32 = 6;
    pub const isc_blob_tra: u32 = 7;
    pub const isc_blob_extfile: u32 = 8;
    pub const isc_blob_debug_info: u32 = 9;
    pub const isc_blob_max_predefined_subtype: u32 = 10;
    pub const fb_shut_confirmation: u32 = 1;
    pub const fb_shut_preproviders: u32 = 2;
    pub const fb_shut_postproviders: u32 = 4;
    pub const fb_shut_finish: u32 = 8;
    pub const fb_shut_exit: u32 = 16;
    pub const fb_shutrsn_svc_stopped: i32 = -1;
    pub const fb_shutrsn_no_connection: i32 = -2;
    pub const fb_shutrsn_app_stopped: i32 = -3;
    pub const fb_shutrsn_signal: i32 = -5;
    pub const fb_shutrsn_services: i32 = -6;
    pub const fb_shutrsn_exit_called: i32 = -7;
    pub const fb_cancel_disable: u32 = 1;
    pub const fb_cancel_enable: u32 = 2;
    pub const fb_cancel_raise: u32 = 3;
    pub const fb_cancel_abort: u32 = 4;
    pub const fb_dbg_version: u32 = 1;
    pub const fb_dbg_end: u32 = 255;
    pub const fb_dbg_map_src2blr: u32 = 2;
    pub const fb_dbg_map_varname: u32 = 3;
    pub const fb_dbg_map_argument: u32 = 4;
    pub const fb_dbg_subproc: u32 = 5;
    pub const fb_dbg_subfunc: u32 = 6;
    pub const fb_dbg_map_curname: u32 = 7;
    pub const fb_dbg_arg_input: u32 = 0;
    pub const fb_dbg_arg_output: u32 = 1;
    pub const isc_info_end: u32 = 1;
    pub const isc_info_truncated: u32 = 2;
    pub const isc_info_error: u32 = 3;
    pub const isc_info_data_not_ready: u32 = 4;
    pub const isc_info_length: u32 = 126;
    pub const isc_info_flag_end: u32 = 127;
    pub const isc_info_number_messages: u32 = 4;
    pub const isc_info_max_message: u32 = 5;
    pub const isc_info_max_send: u32 = 6;
    pub const isc_info_max_receive: u32 = 7;
    pub const isc_info_state: u32 = 8;
    pub const isc_info_message_number: u32 = 9;
    pub const isc_info_message_size: u32 = 10;
    pub const isc_info_request_cost: u32 = 11;
    pub const isc_info_access_path: u32 = 12;
    pub const isc_info_req_select_count: u32 = 13;
    pub const isc_info_req_insert_count: u32 = 14;
    pub const isc_info_req_update_count: u32 = 15;
    pub const isc_info_req_delete_count: u32 = 16;
    pub const isc_info_rsb_end: u32 = 0;
    pub const isc_info_rsb_begin: u32 = 1;
    pub const isc_info_rsb_type: u32 = 2;
    pub const isc_info_rsb_relation: u32 = 3;
    pub const isc_info_rsb_plan: u32 = 4;
    pub const isc_info_rsb_unknown: u32 = 1;
    pub const isc_info_rsb_indexed: u32 = 2;
    pub const isc_info_rsb_navigate: u32 = 3;
    pub const isc_info_rsb_sequential: u32 = 4;
    pub const isc_info_rsb_cross: u32 = 5;
    pub const isc_info_rsb_sort: u32 = 6;
    pub const isc_info_rsb_first: u32 = 7;
    pub const isc_info_rsb_boolean: u32 = 8;
    pub const isc_info_rsb_union: u32 = 9;
    pub const isc_info_rsb_aggregate: u32 = 10;
    pub const isc_info_rsb_merge: u32 = 11;
    pub const isc_info_rsb_ext_sequential: u32 = 12;
    pub const isc_info_rsb_ext_indexed: u32 = 13;
    pub const isc_info_rsb_ext_dbkey: u32 = 14;
    pub const isc_info_rsb_left_cross: u32 = 15;
    pub const isc_info_rsb_select: u32 = 16;
    pub const isc_info_rsb_sql_join: u32 = 17;
    pub const isc_info_rsb_simulate: u32 = 18;
    pub const isc_info_rsb_sim_cross: u32 = 19;
    pub const isc_info_rsb_once: u32 = 20;
    pub const isc_info_rsb_procedure: u32 = 21;
    pub const isc_info_rsb_skip: u32 = 22;
    pub const isc_info_rsb_virt_sequential: u32 = 23;
    pub const isc_info_rsb_recursive: u32 = 24;
    pub const isc_info_rsb_window: u32 = 25;
    pub const isc_info_rsb_singular: u32 = 26;
    pub const isc_info_rsb_writelock: u32 = 27;
    pub const isc_info_rsb_buffer: u32 = 28;
    pub const isc_info_rsb_hash: u32 = 29;
    pub const isc_info_rsb_and: u32 = 1;
    pub const isc_info_rsb_or: u32 = 2;
    pub const isc_info_rsb_dbkey: u32 = 3;
    pub const isc_info_rsb_index: u32 = 4;
    pub const isc_info_req_active: u32 = 2;
    pub const isc_info_req_inactive: u32 = 3;
    pub const isc_info_req_send: u32 = 4;
    pub const isc_info_req_receive: u32 = 5;
    pub const isc_info_req_select: u32 = 6;
    pub const isc_info_req_sql_stall: u32 = 7;
    pub const isc_info_blob_num_segments: u32 = 4;
    pub const isc_info_blob_max_segment: u32 = 5;
    pub const isc_info_blob_total_length: u32 = 6;
    pub const isc_info_blob_type: u32 = 7;
    pub const isc_info_tra_id: u32 = 4;
    pub const isc_info_tra_oldest_interesting: u32 = 5;
    pub const isc_info_tra_oldest_snapshot: u32 = 6;
    pub const isc_info_tra_oldest_active: u32 = 7;
    pub const isc_info_tra_isolation: u32 = 8;
    pub const isc_info_tra_access: u32 = 9;
    pub const isc_info_tra_lock_timeout: u32 = 10;
    pub const fb_info_tra_dbpath: u32 = 11;
    pub const isc_info_tra_consistency: u32 = 1;
    pub const isc_info_tra_concurrency: u32 = 2;
    pub const isc_info_tra_read_committed: u32 = 3;
    pub const isc_info_tra_no_rec_version: u32 = 0;
    pub const isc_info_tra_rec_version: u32 = 1;
    pub const isc_info_tra_readonly: u32 = 0;
    pub const isc_info_tra_readwrite: u32 = 1;
    pub const isc_info_sql_select: u32 = 4;
    pub const isc_info_sql_bind: u32 = 5;
    pub const isc_info_sql_num_variables: u32 = 6;
    pub const isc_info_sql_describe_vars: u32 = 7;
    pub const isc_info_sql_describe_end: u32 = 8;
    pub const isc_info_sql_sqlda_seq: u32 = 9;
    pub const isc_info_sql_message_seq: u32 = 10;
    pub const isc_info_sql_type: u32 = 11;
    pub const isc_info_sql_sub_type: u32 = 12;
    pub const isc_info_sql_scale: u32 = 13;
    pub const isc_info_sql_length: u32 = 14;
    pub const isc_info_sql_null_ind: u32 = 15;
    pub const isc_info_sql_field: u32 = 16;
    pub const isc_info_sql_relation: u32 = 17;
    pub const isc_info_sql_owner: u32 = 18;
    pub const isc_info_sql_alias: u32 = 19;
    pub const isc_info_sql_sqlda_start: u32 = 20;
    pub const isc_info_sql_stmt_type: u32 = 21;
    pub const isc_info_sql_get_plan: u32 = 22;
    pub const isc_info_sql_records: u32 = 23;
    pub const isc_info_sql_batch_fetch: u32 = 24;
    pub const isc_info_sql_relation_alias: u32 = 25;
    pub const isc_info_sql_explain_plan: u32 = 26;
    pub const isc_info_sql_stmt_flags: u32 = 27;
    pub const isc_info_sql_stmt_select: u32 = 1;
    pub const isc_info_sql_stmt_insert: u32 = 2;
    pub const isc_info_sql_stmt_update: u32 = 3;
    pub const isc_info_sql_stmt_delete: u32 = 4;
    pub const isc_info_sql_stmt_ddl: u32 = 5;
    pub const isc_info_sql_stmt_get_segment: u32 = 6;
    pub const isc_info_sql_stmt_put_segment: u32 = 7;
    pub const isc_info_sql_stmt_exec_procedure: u32 = 8;
    pub const isc_info_sql_stmt_start_trans: u32 = 9;
    pub const isc_info_sql_stmt_commit: u32 = 10;
    pub const isc_info_sql_stmt_rollback: u32 = 11;
    pub const isc_info_sql_stmt_select_for_upd: u32 = 12;
    pub const isc_info_sql_stmt_set_generator: u32 = 13;
    pub const isc_info_sql_stmt_savepoint: u32 = 14;
    pub const isc_facility: u32 = 20;
    pub const isc_base: u32 = 335544320;
    pub const isc_factor: u32 = 1;
    pub const isc_arg_end: u32 = 0;
    pub const isc_arg_gds: u32 = 1;
    pub const isc_arg_string: u32 = 2;
    pub const isc_arg_cstring: u32 = 3;
    pub const isc_arg_number: u32 = 4;
    pub const isc_arg_interpreted: u32 = 5;
    pub const isc_arg_vms: u32 = 6;
    pub const isc_arg_unix: u32 = 7;
    pub const isc_arg_domain: u32 = 8;
    pub const isc_arg_dos: u32 = 9;
    pub const isc_arg_mpexl: u32 = 10;
    pub const isc_arg_mpexl_ipc: u32 = 11;
    pub const isc_arg_next_mach: u32 = 15;
    pub const isc_arg_netware: u32 = 16;
    pub const isc_arg_win32: u32 = 17;
    pub const isc_arg_warning: u32 = 18;
    pub const isc_arg_sql_state: u32 = 19;
    pub const isc_arith_except: u32 = 335544321;
    pub const isc_bad_dbkey: u32 = 335544322;
    pub const isc_bad_db_format: u32 = 335544323;
    pub const isc_bad_db_handle: u32 = 335544324;
    pub const isc_bad_dpb_content: u32 = 335544325;
    pub const isc_bad_dpb_form: u32 = 335544326;
    pub const isc_bad_req_handle: u32 = 335544327;
    pub const isc_bad_segstr_handle: u32 = 335544328;
    pub const isc_bad_segstr_id: u32 = 335544329;
    pub const isc_bad_tpb_content: u32 = 335544330;
    pub const isc_bad_tpb_form: u32 = 335544331;
    pub const isc_bad_trans_handle: u32 = 335544332;
    pub const isc_bug_check: u32 = 335544333;
    pub const isc_convert_error: u32 = 335544334;
    pub const isc_db_corrupt: u32 = 335544335;
    pub const isc_deadlock: u32 = 335544336;
    pub const isc_excess_trans: u32 = 335544337;
    pub const isc_from_no_match: u32 = 335544338;
    pub const isc_infinap: u32 = 335544339;
    pub const isc_infona: u32 = 335544340;
    pub const isc_infunk: u32 = 335544341;
    pub const isc_integ_fail: u32 = 335544342;
    pub const isc_invalid_blr: u32 = 335544343;
    pub const isc_io_error: u32 = 335544344;
    pub const isc_lock_conflict: u32 = 335544345;
    pub const isc_metadata_corrupt: u32 = 335544346;
    pub const isc_not_valid: u32 = 335544347;
    pub const isc_no_cur_rec: u32 = 335544348;
    pub const isc_no_dup: u32 = 335544349;
    pub const isc_no_finish: u32 = 335544350;
    pub const isc_no_meta_update: u32 = 335544351;
    pub const isc_no_priv: u32 = 335544352;
    pub const isc_no_recon: u32 = 335544353;
    pub const isc_no_record: u32 = 335544354;
    pub const isc_no_segstr_close: u32 = 335544355;
    pub const isc_obsolete_metadata: u32 = 335544356;
    pub const isc_open_trans: u32 = 335544357;
    pub const isc_port_len: u32 = 335544358;
    pub const isc_read_only_field: u32 = 335544359;
    pub const isc_read_only_rel: u32 = 335544360;
    pub const isc_read_only_trans: u32 = 335544361;
    pub const isc_read_only_view: u32 = 335544362;
    pub const isc_req_no_trans: u32 = 335544363;
    pub const isc_req_sync: u32 = 335544364;
    pub const isc_req_wrong_db: u32 = 335544365;
    pub const isc_segment: u32 = 335544366;
    pub const isc_segstr_eof: u32 = 335544367;
    pub const isc_segstr_no_op: u32 = 335544368;
    pub const isc_segstr_no_read: u32 = 335544369;
    pub const isc_segstr_no_trans: u32 = 335544370;
    pub const isc_segstr_no_write: u32 = 335544371;
    pub const isc_segstr_wrong_db: u32 = 335544372;
    pub const isc_sys_request: u32 = 335544373;
    pub const isc_stream_eof: u32 = 335544374;
    pub const isc_unavailable: u32 = 335544375;
    pub const isc_unres_rel: u32 = 335544376;
    pub const isc_uns_ext: u32 = 335544377;
    pub const isc_wish_list: u32 = 335544378;
    pub const isc_wrong_ods: u32 = 335544379;
    pub const isc_wronumarg: u32 = 335544380;
    pub const isc_imp_exc: u32 = 335544381;
    pub const isc_random: u32 = 335544382;
    pub const isc_fatal_conflict: u32 = 335544383;
    pub const isc_badblk: u32 = 335544384;
    pub const isc_invpoolcl: u32 = 335544385;
    pub const isc_nopoolids: u32 = 335544386;
    pub const isc_relbadblk: u32 = 335544387;
    pub const isc_blktoobig: u32 = 335544388;
    pub const isc_bufexh: u32 = 335544389;
    pub const isc_syntaxerr: u32 = 335544390;
    pub const isc_bufinuse: u32 = 335544391;
    pub const isc_bdbincon: u32 = 335544392;
    pub const isc_reqinuse: u32 = 335544393;
    pub const isc_badodsver: u32 = 335544394;
    pub const isc_relnotdef: u32 = 335544395;
    pub const isc_fldnotdef: u32 = 335544396;
    pub const isc_dirtypage: u32 = 335544397;
    pub const isc_waifortra: u32 = 335544398;
    pub const isc_doubleloc: u32 = 335544399;
    pub const isc_nodnotfnd: u32 = 335544400;
    pub const isc_dupnodfnd: u32 = 335544401;
    pub const isc_locnotmar: u32 = 335544402;
    pub const isc_badpagtyp: u32 = 335544403;
    pub const isc_corrupt: u32 = 335544404;
    pub const isc_badpage: u32 = 335544405;
    pub const isc_badindex: u32 = 335544406;
    pub const isc_dbbnotzer: u32 = 335544407;
    pub const isc_tranotzer: u32 = 335544408;
    pub const isc_trareqmis: u32 = 335544409;
    pub const isc_badhndcnt: u32 = 335544410;
    pub const isc_wrotpbver: u32 = 335544411;
    pub const isc_wroblrver: u32 = 335544412;
    pub const isc_wrodpbver: u32 = 335544413;
    pub const isc_blobnotsup: u32 = 335544414;
    pub const isc_badrelation: u32 = 335544415;
    pub const isc_nodetach: u32 = 335544416;
    pub const isc_notremote: u32 = 335544417;
    pub const isc_trainlim: u32 = 335544418;
    pub const isc_notinlim: u32 = 335544419;
    pub const isc_traoutsta: u32 = 335544420;
    pub const isc_connect_reject: u32 = 335544421;
    pub const isc_dbfile: u32 = 335544422;
    pub const isc_orphan: u32 = 335544423;
    pub const isc_no_lock_mgr: u32 = 335544424;
    pub const isc_ctxinuse: u32 = 335544425;
    pub const isc_ctxnotdef: u32 = 335544426;
    pub const isc_datnotsup: u32 = 335544427;
    pub const isc_badmsgnum: u32 = 335544428;
    pub const isc_badparnum: u32 = 335544429;
    pub const isc_virmemexh: u32 = 335544430;
    pub const isc_blocking_signal: u32 = 335544431;
    pub const isc_lockmanerr: u32 = 335544432;
    pub const isc_journerr: u32 = 335544433;
    pub const isc_keytoobig: u32 = 335544434;
    pub const isc_nullsegkey: u32 = 335544435;
    pub const isc_sqlerr: u32 = 335544436;
    pub const isc_wrodynver: u32 = 335544437;
    pub const isc_funnotdef: u32 = 335544438;
    pub const isc_funmismat: u32 = 335544439;
    pub const isc_bad_msg_vec: u32 = 335544440;
    pub const isc_bad_detach: u32 = 335544441;
    pub const isc_noargacc_read: u32 = 335544442;
    pub const isc_noargacc_write: u32 = 335544443;
    pub const isc_read_only: u32 = 335544444;
    pub const isc_ext_err: u32 = 335544445;
    pub const isc_non_updatable: u32 = 335544446;
    pub const isc_no_rollback: u32 = 335544447;
    pub const isc_bad_sec_info: u32 = 335544448;
    pub const isc_invalid_sec_info: u32 = 335544449;
    pub const isc_misc_interpreted: u32 = 335544450;
    pub const isc_update_conflict: u32 = 335544451;
    pub const isc_unlicensed: u32 = 335544452;
    pub const isc_obj_in_use: u32 = 335544453;
    pub const isc_nofilter: u32 = 335544454;
    pub const isc_shadow_accessed: u32 = 335544455;
    pub const isc_invalid_sdl: u32 = 335544456;
    pub const isc_out_of_bounds: u32 = 335544457;
    pub const isc_invalid_dimension: u32 = 335544458;
    pub const isc_rec_in_limbo: u32 = 335544459;
    pub const isc_shadow_missing: u32 = 335544460;
    pub const isc_cant_validate: u32 = 335544461;
    pub const isc_cant_start_journal: u32 = 335544462;
    pub const isc_gennotdef: u32 = 335544463;
    pub const isc_cant_start_logging: u32 = 335544464;
    pub const isc_bad_segstr_type: u32 = 335544465;
    pub const isc_foreign_key: u32 = 335544466;
    pub const isc_high_minor: u32 = 335544467;
    pub const isc_tra_state: u32 = 335544468;
    pub const isc_trans_invalid: u32 = 335544469;
    pub const isc_buf_invalid: u32 = 335544470;
    pub const isc_indexnotdefined: u32 = 335544471;
    pub const isc_login: u32 = 335544472;
    pub const isc_invalid_bookmark: u32 = 335544473;
    pub const isc_bad_lock_level: u32 = 335544474;
    pub const isc_relation_lock: u32 = 335544475;
    pub const isc_record_lock: u32 = 335544476;
    pub const isc_max_idx: u32 = 335544477;
    pub const isc_jrn_enable: u32 = 335544478;
    pub const isc_old_failure: u32 = 335544479;
    pub const isc_old_in_progress: u32 = 335544480;
    pub const isc_old_no_space: u32 = 335544481;
    pub const isc_no_wal_no_jrn: u32 = 335544482;
    pub const isc_num_old_files: u32 = 335544483;
    pub const isc_wal_file_open: u32 = 335544484;
    pub const isc_bad_stmt_handle: u32 = 335544485;
    pub const isc_wal_failure: u32 = 335544486;
    pub const isc_walw_err: u32 = 335544487;
    pub const isc_logh_small: u32 = 335544488;
    pub const isc_logh_inv_version: u32 = 335544489;
    pub const isc_logh_open_flag: u32 = 335544490;
    pub const isc_logh_open_flag2: u32 = 335544491;
    pub const isc_logh_diff_dbname: u32 = 335544492;
    pub const isc_logf_unexpected_eof: u32 = 335544493;
    pub const isc_logr_incomplete: u32 = 335544494;
    pub const isc_logr_header_small: u32 = 335544495;
    pub const isc_logb_small: u32 = 335544496;
    pub const isc_wal_illegal_attach: u32 = 335544497;
    pub const isc_wal_invalid_wpb: u32 = 335544498;
    pub const isc_wal_err_rollover: u32 = 335544499;
    pub const isc_no_wal: u32 = 335544500;
    pub const isc_drop_wal: u32 = 335544501;
    pub const isc_stream_not_defined: u32 = 335544502;
    pub const isc_wal_subsys_error: u32 = 335544503;
    pub const isc_wal_subsys_corrupt: u32 = 335544504;
    pub const isc_no_archive: u32 = 335544505;
    pub const isc_shutinprog: u32 = 335544506;
    pub const isc_range_in_use: u32 = 335544507;
    pub const isc_range_not_found: u32 = 335544508;
    pub const isc_charset_not_found: u32 = 335544509;
    pub const isc_lock_timeout: u32 = 335544510;
    pub const isc_prcnotdef: u32 = 335544511;
    pub const isc_prcmismat: u32 = 335544512;
    pub const isc_wal_bugcheck: u32 = 335544513;
    pub const isc_wal_cant_expand: u32 = 335544514;
    pub const isc_codnotdef: u32 = 335544515;
    pub const isc_xcpnotdef: u32 = 335544516;
    pub const isc_except: u32 = 335544517;
    pub const isc_cache_restart: u32 = 335544518;
    pub const isc_bad_lock_handle: u32 = 335544519;
    pub const isc_jrn_present: u32 = 335544520;
    pub const isc_wal_err_rollover2: u32 = 335544521;
    pub const isc_wal_err_logwrite: u32 = 335544522;
    pub const isc_wal_err_jrn_comm: u32 = 335544523;
    pub const isc_wal_err_expansion: u32 = 335544524;
    pub const isc_wal_err_setup: u32 = 335544525;
    pub const isc_wal_err_ww_sync: u32 = 335544526;
    pub const isc_wal_err_ww_start: u32 = 335544527;
    pub const isc_shutdown: u32 = 335544528;
    pub const isc_existing_priv_mod: u32 = 335544529;
    pub const isc_primary_key_ref: u32 = 335544530;
    pub const isc_primary_key_notnull: u32 = 335544531;
    pub const isc_ref_cnstrnt_notfound: u32 = 335544532;
    pub const isc_foreign_key_notfound: u32 = 335544533;
    pub const isc_ref_cnstrnt_update: u32 = 335544534;
    pub const isc_check_cnstrnt_update: u32 = 335544535;
    pub const isc_check_cnstrnt_del: u32 = 335544536;
    pub const isc_integ_index_seg_del: u32 = 335544537;
    pub const isc_integ_index_seg_mod: u32 = 335544538;
    pub const isc_integ_index_del: u32 = 335544539;
    pub const isc_integ_index_mod: u32 = 335544540;
    pub const isc_check_trig_del: u32 = 335544541;
    pub const isc_check_trig_update: u32 = 335544542;
    pub const isc_cnstrnt_fld_del: u32 = 335544543;
    pub const isc_cnstrnt_fld_rename: u32 = 335544544;
    pub const isc_rel_cnstrnt_update: u32 = 335544545;
    pub const isc_constaint_on_view: u32 = 335544546;
    pub const isc_invld_cnstrnt_type: u32 = 335544547;
    pub const isc_primary_key_exists: u32 = 335544548;
    pub const isc_systrig_update: u32 = 335544549;
    pub const isc_not_rel_owner: u32 = 335544550;
    pub const isc_grant_obj_notfound: u32 = 335544551;
    pub const isc_grant_fld_notfound: u32 = 335544552;
    pub const isc_grant_nopriv: u32 = 335544553;
    pub const isc_nonsql_security_rel: u32 = 335544554;
    pub const isc_nonsql_security_fld: u32 = 335544555;
    pub const isc_wal_cache_err: u32 = 335544556;
    pub const isc_shutfail: u32 = 335544557;
    pub const isc_check_constraint: u32 = 335544558;
    pub const isc_bad_svc_handle: u32 = 335544559;
    pub const isc_shutwarn: u32 = 335544560;
    pub const isc_wrospbver: u32 = 335544561;
    pub const isc_bad_spb_form: u32 = 335544562;
    pub const isc_svcnotdef: u32 = 335544563;
    pub const isc_no_jrn: u32 = 335544564;
    pub const isc_transliteration_failed: u32 = 335544565;
    pub const isc_start_cm_for_wal: u32 = 335544566;
    pub const isc_wal_ovflow_log_required: u32 = 335544567;
    pub const isc_text_subtype: u32 = 335544568;
    pub const isc_dsql_error: u32 = 335544569;
    pub const isc_dsql_command_err: u32 = 335544570;
    pub const isc_dsql_constant_err: u32 = 335544571;
    pub const isc_dsql_cursor_err: u32 = 335544572;
    pub const isc_dsql_datatype_err: u32 = 335544573;
    pub const isc_dsql_decl_err: u32 = 335544574;
    pub const isc_dsql_cursor_update_err: u32 = 335544575;
    pub const isc_dsql_cursor_open_err: u32 = 335544576;
    pub const isc_dsql_cursor_close_err: u32 = 335544577;
    pub const isc_dsql_field_err: u32 = 335544578;
    pub const isc_dsql_internal_err: u32 = 335544579;
    pub const isc_dsql_relation_err: u32 = 335544580;
    pub const isc_dsql_procedure_err: u32 = 335544581;
    pub const isc_dsql_request_err: u32 = 335544582;
    pub const isc_dsql_sqlda_err: u32 = 335544583;
    pub const isc_dsql_var_count_err: u32 = 335544584;
    pub const isc_dsql_stmt_handle: u32 = 335544585;
    pub const isc_dsql_function_err: u32 = 335544586;
    pub const isc_dsql_blob_err: u32 = 335544587;
    pub const isc_collation_not_found: u32 = 335544588;
    pub const isc_collation_not_for_charset: u32 = 335544589;
    pub const isc_dsql_dup_option: u32 = 335544590;
    pub const isc_dsql_tran_err: u32 = 335544591;
    pub const isc_dsql_invalid_array: u32 = 335544592;
    pub const isc_dsql_max_arr_dim_exceeded: u32 = 335544593;
    pub const isc_dsql_arr_range_error: u32 = 335544594;
    pub const isc_dsql_trigger_err: u32 = 335544595;
    pub const isc_dsql_subselect_err: u32 = 335544596;
    pub const isc_dsql_crdb_prepare_err: u32 = 335544597;
    pub const isc_specify_field_err: u32 = 335544598;
    pub const isc_num_field_err: u32 = 335544599;
    pub const isc_col_name_err: u32 = 335544600;
    pub const isc_where_err: u32 = 335544601;
    pub const isc_table_view_err: u32 = 335544602;
    pub const isc_distinct_err: u32 = 335544603;
    pub const isc_key_field_count_err: u32 = 335544604;
    pub const isc_subquery_err: u32 = 335544605;
    pub const isc_expression_eval_err: u32 = 335544606;
    pub const isc_node_err: u32 = 335544607;
    pub const isc_command_end_err: u32 = 335544608;
    pub const isc_index_name: u32 = 335544609;
    pub const isc_exception_name: u32 = 335544610;
    pub const isc_field_name: u32 = 335544611;
    pub const isc_token_err: u32 = 335544612;
    pub const isc_union_err: u32 = 335544613;
    pub const isc_dsql_construct_err: u32 = 335544614;
    pub const isc_field_aggregate_err: u32 = 335544615;
    pub const isc_field_ref_err: u32 = 335544616;
    pub const isc_order_by_err: u32 = 335544617;
    pub const isc_return_mode_err: u32 = 335544618;
    pub const isc_extern_func_err: u32 = 335544619;
    pub const isc_alias_conflict_err: u32 = 335544620;
    pub const isc_procedure_conflict_error: u32 = 335544621;
    pub const isc_relation_conflict_err: u32 = 335544622;
    pub const isc_dsql_domain_err: u32 = 335544623;
    pub const isc_idx_seg_err: u32 = 335544624;
    pub const isc_node_name_err: u32 = 335544625;
    pub const isc_table_name: u32 = 335544626;
    pub const isc_proc_name: u32 = 335544627;
    pub const isc_idx_create_err: u32 = 335544628;
    pub const isc_wal_shadow_err: u32 = 335544629;
    pub const isc_dependency: u32 = 335544630;
    pub const isc_idx_key_err: u32 = 335544631;
    pub const isc_dsql_file_length_err: u32 = 335544632;
    pub const isc_dsql_shadow_number_err: u32 = 335544633;
    pub const isc_dsql_token_unk_err: u32 = 335544634;
    pub const isc_dsql_no_relation_alias: u32 = 335544635;
    pub const isc_indexname: u32 = 335544636;
    pub const isc_no_stream_plan: u32 = 335544637;
    pub const isc_stream_twice: u32 = 335544638;
    pub const isc_stream_not_found: u32 = 335544639;
    pub const isc_collation_requires_text: u32 = 335544640;
    pub const isc_dsql_domain_not_found: u32 = 335544641;
    pub const isc_index_unused: u32 = 335544642;
    pub const isc_dsql_self_join: u32 = 335544643;
    pub const isc_stream_bof: u32 = 335544644;
    pub const isc_stream_crack: u32 = 335544645;
    pub const isc_db_or_file_exists: u32 = 335544646;
    pub const isc_invalid_operator: u32 = 335544647;
    pub const isc_conn_lost: u32 = 335544648;
    pub const isc_bad_checksum: u32 = 335544649;
    pub const isc_page_type_err: u32 = 335544650;
    pub const isc_ext_readonly_err: u32 = 335544651;
    pub const isc_sing_select_err: u32 = 335544652;
    pub const isc_psw_attach: u32 = 335544653;
    pub const isc_psw_start_trans: u32 = 335544654;
    pub const isc_invalid_direction: u32 = 335544655;
    pub const isc_dsql_var_conflict: u32 = 335544656;
    pub const isc_dsql_no_blob_array: u32 = 335544657;
    pub const isc_dsql_base_table: u32 = 335544658;
    pub const isc_duplicate_base_table: u32 = 335544659;
    pub const isc_view_alias: u32 = 335544660;
    pub const isc_index_root_page_full: u32 = 335544661;
    pub const isc_dsql_blob_type_unknown: u32 = 335544662;
    pub const isc_req_max_clones_exceeded: u32 = 335544663;
    pub const isc_dsql_duplicate_spec: u32 = 335544664;
    pub const isc_unique_key_violation: u32 = 335544665;
    pub const isc_srvr_version_too_old: u32 = 335544666;
    pub const isc_drdb_completed_with_errs: u32 = 335544667;
    pub const isc_dsql_procedure_use_err: u32 = 335544668;
    pub const isc_dsql_count_mismatch: u32 = 335544669;
    pub const isc_blob_idx_err: u32 = 335544670;
    pub const isc_array_idx_err: u32 = 335544671;
    pub const isc_key_field_err: u32 = 335544672;
    pub const isc_no_delete: u32 = 335544673;
    pub const isc_del_last_field: u32 = 335544674;
    pub const isc_sort_err: u32 = 335544675;
    pub const isc_sort_mem_err: u32 = 335544676;
    pub const isc_version_err: u32 = 335544677;
    pub const isc_inval_key_posn: u32 = 335544678;
    pub const isc_no_segments_err: u32 = 335544679;
    pub const isc_crrp_data_err: u32 = 335544680;
    pub const isc_rec_size_err: u32 = 335544681;
    pub const isc_dsql_field_ref: u32 = 335544682;
    pub const isc_req_depth_exceeded: u32 = 335544683;
    pub const isc_no_field_access: u32 = 335544684;
    pub const isc_no_dbkey: u32 = 335544685;
    pub const isc_jrn_format_err: u32 = 335544686;
    pub const isc_jrn_file_full: u32 = 335544687;
    pub const isc_dsql_open_cursor_request: u32 = 335544688;
    pub const isc_ib_error: u32 = 335544689;
    pub const isc_cache_redef: u32 = 335544690;
    pub const isc_cache_too_small: u32 = 335544691;
    pub const isc_log_redef: u32 = 335544692;
    pub const isc_log_too_small: u32 = 335544693;
    pub const isc_partition_too_small: u32 = 335544694;
    pub const isc_partition_not_supp: u32 = 335544695;
    pub const isc_log_length_spec: u32 = 335544696;
    pub const isc_precision_err: u32 = 335544697;
    pub const isc_scale_nogt: u32 = 335544698;
    pub const isc_expec_short: u32 = 335544699;
    pub const isc_expec_long: u32 = 335544700;
    pub const isc_expec_ushort: u32 = 335544701;
    pub const isc_escape_invalid: u32 = 335544702;
    pub const isc_svcnoexe: u32 = 335544703;
    pub const isc_net_lookup_err: u32 = 335544704;
    pub const isc_service_unknown: u32 = 335544705;
    pub const isc_host_unknown: u32 = 335544706;
    pub const isc_grant_nopriv_on_base: u32 = 335544707;
    pub const isc_dyn_fld_ambiguous: u32 = 335544708;
    pub const isc_dsql_agg_ref_err: u32 = 335544709;
    pub const isc_complex_view: u32 = 335544710;
    pub const isc_unprepared_stmt: u32 = 335544711;
    pub const isc_expec_positive: u32 = 335544712;
    pub const isc_dsql_sqlda_value_err: u32 = 335544713;
    pub const isc_invalid_array_id: u32 = 335544714;
    pub const isc_extfile_uns_op: u32 = 335544715;
    pub const isc_svc_in_use: u32 = 335544716;
    pub const isc_err_stack_limit: u32 = 335544717;
    pub const isc_invalid_key: u32 = 335544718;
    pub const isc_net_init_error: u32 = 335544719;
    pub const isc_loadlib_failure: u32 = 335544720;
    pub const isc_network_error: u32 = 335544721;
    pub const isc_net_connect_err: u32 = 335544722;
    pub const isc_net_connect_listen_err: u32 = 335544723;
    pub const isc_net_event_connect_err: u32 = 335544724;
    pub const isc_net_event_listen_err: u32 = 335544725;
    pub const isc_net_read_err: u32 = 335544726;
    pub const isc_net_write_err: u32 = 335544727;
    pub const isc_integ_index_deactivate: u32 = 335544728;
    pub const isc_integ_deactivate_primary: u32 = 335544729;
    pub const isc_cse_not_supported: u32 = 335544730;
    pub const isc_tra_must_sweep: u32 = 335544731;
    pub const isc_unsupported_network_drive: u32 = 335544732;
    pub const isc_io_create_err: u32 = 335544733;
    pub const isc_io_open_err: u32 = 335544734;
    pub const isc_io_close_err: u32 = 335544735;
    pub const isc_io_read_err: u32 = 335544736;
    pub const isc_io_write_err: u32 = 335544737;
    pub const isc_io_delete_err: u32 = 335544738;
    pub const isc_io_access_err: u32 = 335544739;
    pub const isc_udf_exception: u32 = 335544740;
    pub const isc_lost_db_connection: u32 = 335544741;
    pub const isc_no_write_user_priv: u32 = 335544742;
    pub const isc_token_too_long: u32 = 335544743;
    pub const isc_max_att_exceeded: u32 = 335544744;
    pub const isc_login_same_as_role_name: u32 = 335544745;
    pub const isc_reftable_requires_pk: u32 = 335544746;
    pub const isc_usrname_too_long: u32 = 335544747;
    pub const isc_password_too_long: u32 = 335544748;
    pub const isc_usrname_required: u32 = 335544749;
    pub const isc_password_required: u32 = 335544750;
    pub const isc_bad_protocol: u32 = 335544751;
    pub const isc_dup_usrname_found: u32 = 335544752;
    pub const isc_usrname_not_found: u32 = 335544753;
    pub const isc_error_adding_sec_record: u32 = 335544754;
    pub const isc_error_modifying_sec_record: u32 = 335544755;
    pub const isc_error_deleting_sec_record: u32 = 335544756;
    pub const isc_error_updating_sec_db: u32 = 335544757;
    pub const isc_sort_rec_size_err: u32 = 335544758;
    pub const isc_bad_default_value: u32 = 335544759;
    pub const isc_invalid_clause: u32 = 335544760;
    pub const isc_too_many_handles: u32 = 335544761;
    pub const isc_optimizer_blk_exc: u32 = 335544762;
    pub const isc_invalid_string_constant: u32 = 335544763;
    pub const isc_transitional_date: u32 = 335544764;
    pub const isc_read_only_database: u32 = 335544765;
    pub const isc_must_be_dialect_2_and_up: u32 = 335544766;
    pub const isc_blob_filter_exception: u32 = 335544767;
    pub const isc_exception_access_violation: u32 = 335544768;
    pub const isc_exception_datatype_missalignment: u32 = 335544769;
    pub const isc_exception_array_bounds_exceeded: u32 = 335544770;
    pub const isc_exception_float_denormal_operand: u32 = 335544771;
    pub const isc_exception_float_divide_by_zero: u32 = 335544772;
    pub const isc_exception_float_inexact_result: u32 = 335544773;
    pub const isc_exception_float_invalid_operand: u32 = 335544774;
    pub const isc_exception_float_overflow: u32 = 335544775;
    pub const isc_exception_float_stack_check: u32 = 335544776;
    pub const isc_exception_float_underflow: u32 = 335544777;
    pub const isc_exception_integer_divide_by_zero: u32 = 335544778;
    pub const isc_exception_integer_overflow: u32 = 335544779;
    pub const isc_exception_unknown: u32 = 335544780;
    pub const isc_exception_stack_overflow: u32 = 335544781;
    pub const isc_exception_sigsegv: u32 = 335544782;
    pub const isc_exception_sigill: u32 = 335544783;
    pub const isc_exception_sigbus: u32 = 335544784;
    pub const isc_exception_sigfpe: u32 = 335544785;
    pub const isc_ext_file_delete: u32 = 335544786;
    pub const isc_ext_file_modify: u32 = 335544787;
    pub const isc_adm_task_denied: u32 = 335544788;
    pub const isc_extract_input_mismatch: u32 = 335544789;
    pub const isc_insufficient_svc_privileges: u32 = 335544790;
    pub const isc_file_in_use: u32 = 335544791;
    pub const isc_service_att_err: u32 = 335544792;
    pub const isc_ddl_not_allowed_by_db_sql_dial: u32 = 335544793;
    pub const isc_cancelled: u32 = 335544794;
    pub const isc_unexp_spb_form: u32 = 335544795;
    pub const isc_sql_dialect_datatype_unsupport: u32 = 335544796;
    pub const isc_svcnouser: u32 = 335544797;
    pub const isc_depend_on_uncommitted_rel: u32 = 335544798;
    pub const isc_svc_name_missing: u32 = 335544799;
    pub const isc_too_many_contexts: u32 = 335544800;
    pub const isc_datype_notsup: u32 = 335544801;
    pub const isc_dialect_reset_warning: u32 = 335544802;
    pub const isc_dialect_not_changed: u32 = 335544803;
    pub const isc_database_create_failed: u32 = 335544804;
    pub const isc_inv_dialect_specified: u32 = 335544805;
    pub const isc_valid_db_dialects: u32 = 335544806;
    pub const isc_sqlwarn: u32 = 335544807;
    pub const isc_dtype_renamed: u32 = 335544808;
    pub const isc_extern_func_dir_error: u32 = 335544809;
    pub const isc_date_range_exceeded: u32 = 335544810;
    pub const isc_inv_client_dialect_specified: u32 = 335544811;
    pub const isc_valid_client_dialects: u32 = 335544812;
    pub const isc_optimizer_between_err: u32 = 335544813;
    pub const isc_service_not_supported: u32 = 335544814;
    pub const isc_generator_name: u32 = 335544815;
    pub const isc_udf_name: u32 = 335544816;
    pub const isc_bad_limit_param: u32 = 335544817;
    pub const isc_bad_skip_param: u32 = 335544818;
    pub const isc_io_32bit_exceeded_err: u32 = 335544819;
    pub const isc_invalid_savepoint: u32 = 335544820;
    pub const isc_dsql_column_pos_err: u32 = 335544821;
    pub const isc_dsql_agg_where_err: u32 = 335544822;
    pub const isc_dsql_agg_group_err: u32 = 335544823;
    pub const isc_dsql_agg_column_err: u32 = 335544824;
    pub const isc_dsql_agg_having_err: u32 = 335544825;
    pub const isc_dsql_agg_nested_err: u32 = 335544826;
    pub const isc_exec_sql_invalid_arg: u32 = 335544827;
    pub const isc_exec_sql_invalid_req: u32 = 335544828;
    pub const isc_exec_sql_invalid_var: u32 = 335544829;
    pub const isc_exec_sql_max_call_exceeded: u32 = 335544830;
    pub const isc_conf_access_denied: u32 = 335544831;
    pub const isc_wrong_backup_state: u32 = 335544832;
    pub const isc_wal_backup_err: u32 = 335544833;
    pub const isc_cursor_not_open: u32 = 335544834;
    pub const isc_bad_shutdown_mode: u32 = 335544835;
    pub const isc_concat_overflow: u32 = 335544836;
    pub const isc_bad_substring_offset: u32 = 335544837;
    pub const isc_foreign_key_target_doesnt_exist: u32 = 335544838;
    pub const isc_foreign_key_references_present: u32 = 335544839;
    pub const isc_no_update: u32 = 335544840;
    pub const isc_cursor_already_open: u32 = 335544841;
    pub const isc_stack_trace: u32 = 335544842;
    pub const isc_ctx_var_not_found: u32 = 335544843;
    pub const isc_ctx_namespace_invalid: u32 = 335544844;
    pub const isc_ctx_too_big: u32 = 335544845;
    pub const isc_ctx_bad_argument: u32 = 335544846;
    pub const isc_identifier_too_long: u32 = 335544847;
    pub const isc_except2: u32 = 335544848;
    pub const isc_malformed_string: u32 = 335544849;
    pub const isc_prc_out_param_mismatch: u32 = 335544850;
    pub const isc_command_end_err2: u32 = 335544851;
    pub const isc_partner_idx_incompat_type: u32 = 335544852;
    pub const isc_bad_substring_length: u32 = 335544853;
    pub const isc_charset_not_installed: u32 = 335544854;
    pub const isc_collation_not_installed: u32 = 335544855;
    pub const isc_att_shutdown: u32 = 335544856;
    pub const isc_blobtoobig: u32 = 335544857;
    pub const isc_must_have_phys_field: u32 = 335544858;
    pub const isc_invalid_time_precision: u32 = 335544859;
    pub const isc_blob_convert_error: u32 = 335544860;
    pub const isc_array_convert_error: u32 = 335544861;
    pub const isc_record_lock_not_supp: u32 = 335544862;
    pub const isc_partner_idx_not_found: u32 = 335544863;
    pub const isc_tra_num_exc: u32 = 335544864;
    pub const isc_field_disappeared: u32 = 335544865;
    pub const isc_met_wrong_gtt_scope: u32 = 335544866;
    pub const isc_subtype_for_internal_use: u32 = 335544867;
    pub const isc_illegal_prc_type: u32 = 335544868;
    pub const isc_invalid_sort_datatype: u32 = 335544869;
    pub const isc_collation_name: u32 = 335544870;
    pub const isc_domain_name: u32 = 335544871;
    pub const isc_domnotdef: u32 = 335544872;
    pub const isc_array_max_dimensions: u32 = 335544873;
    pub const isc_max_db_per_trans_allowed: u32 = 335544874;
    pub const isc_bad_debug_format: u32 = 335544875;
    pub const isc_bad_proc_BLR: u32 = 335544876;
    pub const isc_key_too_big: u32 = 335544877;
    pub const isc_concurrent_transaction: u32 = 335544878;
    pub const isc_not_valid_for_var: u32 = 335544879;
    pub const isc_not_valid_for: u32 = 335544880;
    pub const isc_need_difference: u32 = 335544881;
    pub const isc_long_login: u32 = 335544882;
    pub const isc_fldnotdef2: u32 = 335544883;
    pub const isc_invalid_similar_pattern: u32 = 335544884;
    pub const isc_bad_teb_form: u32 = 335544885;
    pub const isc_tpb_multiple_txn_isolation: u32 = 335544886;
    pub const isc_tpb_reserv_before_table: u32 = 335544887;
    pub const isc_tpb_multiple_spec: u32 = 335544888;
    pub const isc_tpb_option_without_rc: u32 = 335544889;
    pub const isc_tpb_conflicting_options: u32 = 335544890;
    pub const isc_tpb_reserv_missing_tlen: u32 = 335544891;
    pub const isc_tpb_reserv_long_tlen: u32 = 335544892;
    pub const isc_tpb_reserv_missing_tname: u32 = 335544893;
    pub const isc_tpb_reserv_corrup_tlen: u32 = 335544894;
    pub const isc_tpb_reserv_null_tlen: u32 = 335544895;
    pub const isc_tpb_reserv_relnotfound: u32 = 335544896;
    pub const isc_tpb_reserv_baserelnotfound: u32 = 335544897;
    pub const isc_tpb_missing_len: u32 = 335544898;
    pub const isc_tpb_missing_value: u32 = 335544899;
    pub const isc_tpb_corrupt_len: u32 = 335544900;
    pub const isc_tpb_null_len: u32 = 335544901;
    pub const isc_tpb_overflow_len: u32 = 335544902;
    pub const isc_tpb_invalid_value: u32 = 335544903;
    pub const isc_tpb_reserv_stronger_wng: u32 = 335544904;
    pub const isc_tpb_reserv_stronger: u32 = 335544905;
    pub const isc_tpb_reserv_max_recursion: u32 = 335544906;
    pub const isc_tpb_reserv_virtualtbl: u32 = 335544907;
    pub const isc_tpb_reserv_systbl: u32 = 335544908;
    pub const isc_tpb_reserv_temptbl: u32 = 335544909;
    pub const isc_tpb_readtxn_after_writelock: u32 = 335544910;
    pub const isc_tpb_writelock_after_readtxn: u32 = 335544911;
    pub const isc_time_range_exceeded: u32 = 335544912;
    pub const isc_datetime_range_exceeded: u32 = 335544913;
    pub const isc_string_truncation: u32 = 335544914;
    pub const isc_blob_truncation: u32 = 335544915;
    pub const isc_numeric_out_of_range: u32 = 335544916;
    pub const isc_shutdown_timeout: u32 = 335544917;
    pub const isc_att_handle_busy: u32 = 335544918;
    pub const isc_bad_udf_freeit: u32 = 335544919;
    pub const isc_eds_provider_not_found: u32 = 335544920;
    pub const isc_eds_connection: u32 = 335544921;
    pub const isc_eds_preprocess: u32 = 335544922;
    pub const isc_eds_stmt_expected: u32 = 335544923;
    pub const isc_eds_prm_name_expected: u32 = 335544924;
    pub const isc_eds_unclosed_comment: u32 = 335544925;
    pub const isc_eds_statement: u32 = 335544926;
    pub const isc_eds_input_prm_mismatch: u32 = 335544927;
    pub const isc_eds_output_prm_mismatch: u32 = 335544928;
    pub const isc_eds_input_prm_not_set: u32 = 335544929;
    pub const isc_too_big_blr: u32 = 335544930;
    pub const isc_montabexh: u32 = 335544931;
    pub const isc_modnotfound: u32 = 335544932;
    pub const isc_nothing_to_cancel: u32 = 335544933;
    pub const isc_ibutil_not_loaded: u32 = 335544934;
    pub const isc_circular_computed: u32 = 335544935;
    pub const isc_psw_db_error: u32 = 335544936;
    pub const isc_invalid_type_datetime_op: u32 = 335544937;
    pub const isc_onlycan_add_timetodate: u32 = 335544938;
    pub const isc_onlycan_add_datetotime: u32 = 335544939;
    pub const isc_onlycansub_tstampfromtstamp: u32 = 335544940;
    pub const isc_onlyoneop_mustbe_tstamp: u32 = 335544941;
    pub const isc_invalid_extractpart_time: u32 = 335544942;
    pub const isc_invalid_extractpart_date: u32 = 335544943;
    pub const isc_invalidarg_extract: u32 = 335544944;
    pub const isc_sysf_argmustbe_exact: u32 = 335544945;
    pub const isc_sysf_argmustbe_exact_or_fp: u32 = 335544946;
    pub const isc_sysf_argviolates_uuidtype: u32 = 335544947;
    pub const isc_sysf_argviolates_uuidlen: u32 = 335544948;
    pub const isc_sysf_argviolates_uuidfmt: u32 = 335544949;
    pub const isc_sysf_argviolates_guidigits: u32 = 335544950;
    pub const isc_sysf_invalid_addpart_time: u32 = 335544951;
    pub const isc_sysf_invalid_add_datetime: u32 = 335544952;
    pub const isc_sysf_invalid_addpart_dtime: u32 = 335544953;
    pub const isc_sysf_invalid_add_dtime_rc: u32 = 335544954;
    pub const isc_sysf_invalid_diff_dtime: u32 = 335544955;
    pub const isc_sysf_invalid_timediff: u32 = 335544956;
    pub const isc_sysf_invalid_tstamptimediff: u32 = 335544957;
    pub const isc_sysf_invalid_datetimediff: u32 = 335544958;
    pub const isc_sysf_invalid_diffpart: u32 = 335544959;
    pub const isc_sysf_argmustbe_positive: u32 = 335544960;
    pub const isc_sysf_basemustbe_positive: u32 = 335544961;
    pub const isc_sysf_argnmustbe_nonneg: u32 = 335544962;
    pub const isc_sysf_argnmustbe_positive: u32 = 335544963;
    pub const isc_sysf_invalid_zeropowneg: u32 = 335544964;
    pub const isc_sysf_invalid_negpowfp: u32 = 335544965;
    pub const isc_sysf_invalid_scale: u32 = 335544966;
    pub const isc_sysf_argmustbe_nonneg: u32 = 335544967;
    pub const isc_sysf_binuuid_mustbe_str: u32 = 335544968;
    pub const isc_sysf_binuuid_wrongsize: u32 = 335544969;
    pub const isc_missing_required_spb: u32 = 335544970;
    pub const isc_net_server_shutdown: u32 = 335544971;
    pub const isc_bad_conn_str: u32 = 335544972;
    pub const isc_bad_epb_form: u32 = 335544973;
    pub const isc_no_threads: u32 = 335544974;
    pub const isc_net_event_connect_timeout: u32 = 335544975;
    pub const isc_sysf_argmustbe_nonzero: u32 = 335544976;
    pub const isc_sysf_argmustbe_range_inc1_1: u32 = 335544977;
    pub const isc_sysf_argmustbe_gteq_one: u32 = 335544978;
    pub const isc_sysf_argmustbe_range_exc1_1: u32 = 335544979;
    pub const isc_internal_rejected_params: u32 = 335544980;
    pub const isc_sysf_fp_overflow: u32 = 335544981;
    pub const isc_udf_fp_overflow: u32 = 335544982;
    pub const isc_udf_fp_nan: u32 = 335544983;
    pub const isc_instance_conflict: u32 = 335544984;
    pub const isc_out_of_temp_space: u32 = 335544985;
    pub const isc_eds_expl_tran_ctrl: u32 = 335544986;
    pub const isc_no_trusted_spb: u32 = 335544987;
    pub const isc_package_name: u32 = 335544988;
    pub const isc_cannot_make_not_null: u32 = 335544989;
    pub const isc_feature_removed: u32 = 335544990;
    pub const isc_view_name: u32 = 335544991;
    pub const isc_lock_dir_access: u32 = 335544992;
    pub const isc_invalid_fetch_option: u32 = 335544993;
    pub const isc_bad_fun_BLR: u32 = 335544994;
    pub const isc_func_pack_not_implemented: u32 = 335544995;
    pub const isc_proc_pack_not_implemented: u32 = 335544996;
    pub const isc_eem_func_not_returned: u32 = 335544997;
    pub const isc_eem_proc_not_returned: u32 = 335544998;
    pub const isc_eem_trig_not_returned: u32 = 335544999;
    pub const isc_eem_bad_plugin_ver: u32 = 335545000;
    pub const isc_eem_engine_notfound: u32 = 335545001;
    pub const isc_attachment_in_use: u32 = 335545002;
    pub const isc_transaction_in_use: u32 = 335545003;
    pub const isc_pman_cannot_load_plugin: u32 = 335545004;
    pub const isc_pman_module_notfound: u32 = 335545005;
    pub const isc_pman_entrypoint_notfound: u32 = 335545006;
    pub const isc_pman_module_bad: u32 = 335545007;
    pub const isc_pman_plugin_notfound: u32 = 335545008;
    pub const isc_sysf_invalid_trig_namespace: u32 = 335545009;
    pub const isc_unexpected_null: u32 = 335545010;
    pub const isc_type_notcompat_blob: u32 = 335545011;
    pub const isc_invalid_date_val: u32 = 335545012;
    pub const isc_invalid_time_val: u32 = 335545013;
    pub const isc_invalid_timestamp_val: u32 = 335545014;
    pub const isc_invalid_index_val: u32 = 335545015;
    pub const isc_formatted_exception: u32 = 335545016;
    pub const isc_async_active: u32 = 335545017;
    pub const isc_private_function: u32 = 335545018;
    pub const isc_private_procedure: u32 = 335545019;
    pub const isc_request_outdated: u32 = 335545020;
    pub const isc_bad_events_handle: u32 = 335545021;
    pub const isc_cannot_copy_stmt: u32 = 335545022;
    pub const isc_invalid_boolean_usage: u32 = 335545023;
    pub const isc_sysf_argscant_both_be_zero: u32 = 335545024;
    pub const isc_spb_no_id: u32 = 335545025;
    pub const isc_ee_blr_mismatch_null: u32 = 335545026;
    pub const isc_ee_blr_mismatch_length: u32 = 335545027;
    pub const isc_ss_out_of_bounds: u32 = 335545028;
    pub const isc_missing_data_structures: u32 = 335545029;
    pub const isc_protect_sys_tab: u32 = 335545030;
    pub const isc_libtommath_generic: u32 = 335545031;
    pub const isc_wroblrver2: u32 = 335545032;
    pub const isc_trunc_limits: u32 = 335545033;
    pub const isc_info_access: u32 = 335545034;
    pub const isc_svc_no_stdin: u32 = 335545035;
    pub const isc_svc_start_failed: u32 = 335545036;
    pub const isc_svc_no_switches: u32 = 335545037;
    pub const isc_svc_bad_size: u32 = 335545038;
    pub const isc_no_crypt_plugin: u32 = 335545039;
    pub const isc_cp_name_too_long: u32 = 335545040;
    pub const isc_cp_process_active: u32 = 335545041;
    pub const isc_cp_already_crypted: u32 = 335545042;
    pub const isc_decrypt_error: u32 = 335545043;
    pub const isc_no_providers: u32 = 335545044;
    pub const isc_null_spb: u32 = 335545045;
    pub const isc_max_args_exceeded: u32 = 335545046;
    pub const isc_ee_blr_mismatch_names_count: u32 = 335545047;
    pub const isc_ee_blr_mismatch_name_not_found: u32 = 335545048;
    pub const isc_bad_result_set: u32 = 335545049;
    pub const isc_wrong_message_length: u32 = 335545050;
    pub const isc_no_output_format: u32 = 335545051;
    pub const isc_item_finish: u32 = 335545052;
    pub const isc_miss_config: u32 = 335545053;
    pub const isc_conf_line: u32 = 335545054;
    pub const isc_conf_include: u32 = 335545055;
    pub const isc_include_depth: u32 = 335545056;
    pub const isc_include_miss: u32 = 335545057;
    pub const isc_protect_ownership: u32 = 335545058;
    pub const isc_badvarnum: u32 = 335545059;
    pub const isc_sec_context: u32 = 335545060;
    pub const isc_multi_segment: u32 = 335545061;
    pub const isc_login_changed: u32 = 335545062;
    pub const isc_auth_handshake_limit: u32 = 335545063;
    pub const isc_wirecrypt_incompatible: u32 = 335545064;
    pub const isc_miss_wirecrypt: u32 = 335545065;
    pub const isc_wirecrypt_key: u32 = 335545066;
    pub const isc_wirecrypt_plugin: u32 = 335545067;
    pub const isc_secdb_name: u32 = 335545068;
    pub const isc_auth_data: u32 = 335545069;
    pub const isc_auth_datalength: u32 = 335545070;
    pub const isc_info_unprepared_stmt: u32 = 335545071;
    pub const isc_idx_key_value: u32 = 335545072;
    pub const isc_forupdate_virtualtbl: u32 = 335545073;
    pub const isc_forupdate_systbl: u32 = 335545074;
    pub const isc_forupdate_temptbl: u32 = 335545075;
    pub const isc_cant_modify_sysobj: u32 = 335545076;
    pub const isc_server_misconfigured: u32 = 335545077;
    pub const isc_alter_role: u32 = 335545078;
    pub const isc_map_already_exists: u32 = 335545079;
    pub const isc_map_not_exists: u32 = 335545080;
    pub const isc_map_load: u32 = 335545081;
    pub const isc_map_aster: u32 = 335545082;
    pub const isc_map_multi: u32 = 335545083;
    pub const isc_map_undefined: u32 = 335545084;
    pub const isc_baddpb_damaged_mode: u32 = 335545085;
    pub const isc_baddpb_buffers_range: u32 = 335545086;
    pub const isc_baddpb_temp_buffers: u32 = 335545087;
    pub const isc_map_nodb: u32 = 335545088;
    pub const isc_map_notable: u32 = 335545089;
    pub const isc_miss_trusted_role: u32 = 335545090;
    pub const isc_set_invalid_role: u32 = 335545091;
    pub const isc_cursor_not_positioned: u32 = 335545092;
    pub const isc_dup_attribute: u32 = 335545093;
    pub const isc_dyn_no_priv: u32 = 335545094;
    pub const isc_dsql_cant_grant_option: u32 = 335545095;
    pub const isc_read_conflict: u32 = 335545096;
    pub const isc_crdb_load: u32 = 335545097;
    pub const isc_crdb_nodb: u32 = 335545098;
    pub const isc_crdb_notable: u32 = 335545099;
    pub const isc_interface_version_too_old: u32 = 335545100;
    pub const isc_fun_param_mismatch: u32 = 335545101;
    pub const isc_savepoint_backout_err: u32 = 335545102;
    pub const isc_domain_primary_key_notnull: u32 = 335545103;
    pub const isc_invalid_attachment_charset: u32 = 335545104;
    pub const isc_map_down: u32 = 335545105;
    pub const isc_login_error: u32 = 335545106;
    pub const isc_already_opened: u32 = 335545107;
    pub const isc_bad_crypt_key: u32 = 335545108;
    pub const isc_encrypt_error: u32 = 335545109;
    pub const isc_gfix_db_name: u32 = 335740929;
    pub const isc_gfix_invalid_sw: u32 = 335740930;
    pub const isc_gfix_incmp_sw: u32 = 335740932;
    pub const isc_gfix_replay_req: u32 = 335740933;
    pub const isc_gfix_pgbuf_req: u32 = 335740934;
    pub const isc_gfix_val_req: u32 = 335740935;
    pub const isc_gfix_pval_req: u32 = 335740936;
    pub const isc_gfix_trn_req: u32 = 335740937;
    pub const isc_gfix_full_req: u32 = 335740940;
    pub const isc_gfix_usrname_req: u32 = 335740941;
    pub const isc_gfix_pass_req: u32 = 335740942;
    pub const isc_gfix_subs_name: u32 = 335740943;
    pub const isc_gfix_wal_req: u32 = 335740944;
    pub const isc_gfix_sec_req: u32 = 335740945;
    pub const isc_gfix_nval_req: u32 = 335740946;
    pub const isc_gfix_type_shut: u32 = 335740947;
    pub const isc_gfix_retry: u32 = 335740948;
    pub const isc_gfix_retry_db: u32 = 335740951;
    pub const isc_gfix_exceed_max: u32 = 335740991;
    pub const isc_gfix_corrupt_pool: u32 = 335740992;
    pub const isc_gfix_mem_exhausted: u32 = 335740993;
    pub const isc_gfix_bad_pool: u32 = 335740994;
    pub const isc_gfix_trn_not_valid: u32 = 335740995;
    pub const isc_gfix_unexp_eoi: u32 = 335741012;
    pub const isc_gfix_recon_fail: u32 = 335741018;
    pub const isc_gfix_trn_unknown: u32 = 335741036;
    pub const isc_gfix_mode_req: u32 = 335741038;
    pub const isc_gfix_pzval_req: u32 = 335741042;
    pub const isc_dsql_dbkey_from_non_table: u32 = 336003074;
    pub const isc_dsql_transitional_numeric: u32 = 336003075;
    pub const isc_dsql_dialect_warning_expr: u32 = 336003076;
    pub const isc_sql_db_dialect_dtype_unsupport: u32 = 336003077;
    pub const isc_sql_dialect_conflict_num: u32 = 336003079;
    pub const isc_dsql_warning_number_ambiguous: u32 = 336003080;
    pub const isc_dsql_warning_number_ambiguous1: u32 = 336003081;
    pub const isc_dsql_warn_precision_ambiguous: u32 = 336003082;
    pub const isc_dsql_warn_precision_ambiguous1: u32 = 336003083;
    pub const isc_dsql_warn_precision_ambiguous2: u32 = 336003084;
    pub const isc_dsql_ambiguous_field_name: u32 = 336003085;
    pub const isc_dsql_udf_return_pos_err: u32 = 336003086;
    pub const isc_dsql_invalid_label: u32 = 336003087;
    pub const isc_dsql_datatypes_not_comparable: u32 = 336003088;
    pub const isc_dsql_cursor_invalid: u32 = 336003089;
    pub const isc_dsql_cursor_redefined: u32 = 336003090;
    pub const isc_dsql_cursor_not_found: u32 = 336003091;
    pub const isc_dsql_cursor_exists: u32 = 336003092;
    pub const isc_dsql_cursor_rel_ambiguous: u32 = 336003093;
    pub const isc_dsql_cursor_rel_not_found: u32 = 336003094;
    pub const isc_dsql_cursor_not_open: u32 = 336003095;
    pub const isc_dsql_type_not_supp_ext_tab: u32 = 336003096;
    pub const isc_dsql_feature_not_supported_ods: u32 = 336003097;
    pub const isc_primary_key_required: u32 = 336003098;
    pub const isc_upd_ins_doesnt_match_pk: u32 = 336003099;
    pub const isc_upd_ins_doesnt_match_matching: u32 = 336003100;
    pub const isc_upd_ins_with_complex_view: u32 = 336003101;
    pub const isc_dsql_incompatible_trigger_type: u32 = 336003102;
    pub const isc_dsql_db_trigger_type_cant_change: u32 = 336003103;
    pub const isc_dsql_record_version_table: u32 = 336003104;
    pub const isc_dsql_invalid_sqlda_version: u32 = 336003105;
    pub const isc_dsql_sqlvar_index: u32 = 336003106;
    pub const isc_dsql_no_sqlind: u32 = 336003107;
    pub const isc_dsql_no_sqldata: u32 = 336003108;
    pub const isc_dsql_no_input_sqlda: u32 = 336003109;
    pub const isc_dsql_no_output_sqlda: u32 = 336003110;
    pub const isc_dsql_wrong_param_num: u32 = 336003111;
    pub const isc_dyn_filter_not_found: u32 = 336068645;
    pub const isc_dyn_func_not_found: u32 = 336068649;
    pub const isc_dyn_index_not_found: u32 = 336068656;
    pub const isc_dyn_view_not_found: u32 = 336068662;
    pub const isc_dyn_domain_not_found: u32 = 336068697;
    pub const isc_dyn_cant_modify_auto_trig: u32 = 336068717;
    pub const isc_dyn_dup_table: u32 = 336068740;
    pub const isc_dyn_proc_not_found: u32 = 336068748;
    pub const isc_dyn_exception_not_found: u32 = 336068752;
    pub const isc_dyn_proc_param_not_found: u32 = 336068754;
    pub const isc_dyn_trig_not_found: u32 = 336068755;
    pub const isc_dyn_charset_not_found: u32 = 336068759;
    pub const isc_dyn_collation_not_found: u32 = 336068760;
    pub const isc_dyn_role_not_found: u32 = 336068763;
    pub const isc_dyn_name_longer: u32 = 336068767;
    pub const isc_dyn_column_does_not_exist: u32 = 336068784;
    pub const isc_dyn_role_does_not_exist: u32 = 336068796;
    pub const isc_dyn_no_grant_admin_opt: u32 = 336068797;
    pub const isc_dyn_user_not_role_member: u32 = 336068798;
    pub const isc_dyn_delete_role_failed: u32 = 336068799;
    pub const isc_dyn_grant_role_to_user: u32 = 336068800;
    pub const isc_dyn_inv_sql_role_name: u32 = 336068801;
    pub const isc_dyn_dup_sql_role: u32 = 336068802;
    pub const isc_dyn_kywd_spec_for_role: u32 = 336068803;
    pub const isc_dyn_roles_not_supported: u32 = 336068804;
    pub const isc_dyn_domain_name_exists: u32 = 336068812;
    pub const isc_dyn_field_name_exists: u32 = 336068813;
    pub const isc_dyn_dependency_exists: u32 = 336068814;
    pub const isc_dyn_dtype_invalid: u32 = 336068815;
    pub const isc_dyn_char_fld_too_small: u32 = 336068816;
    pub const isc_dyn_invalid_dtype_conversion: u32 = 336068817;
    pub const isc_dyn_dtype_conv_invalid: u32 = 336068818;
    pub const isc_dyn_zero_len_id: u32 = 336068820;
    pub const isc_dyn_gen_not_found: u32 = 336068822;
    pub const isc_max_coll_per_charset: u32 = 336068829;
    pub const isc_invalid_coll_attr: u32 = 336068830;
    pub const isc_dyn_wrong_gtt_scope: u32 = 336068840;
    pub const isc_dyn_coll_used_table: u32 = 336068843;
    pub const isc_dyn_coll_used_domain: u32 = 336068844;
    pub const isc_dyn_cannot_del_syscoll: u32 = 336068845;
    pub const isc_dyn_cannot_del_def_coll: u32 = 336068846;
    pub const isc_dyn_table_not_found: u32 = 336068849;
    pub const isc_dyn_coll_used_procedure: u32 = 336068851;
    pub const isc_dyn_scale_too_big: u32 = 336068852;
    pub const isc_dyn_precision_too_small: u32 = 336068853;
    pub const isc_dyn_miss_priv_warning: u32 = 336068855;
    pub const isc_dyn_ods_not_supp_feature: u32 = 336068856;
    pub const isc_dyn_cannot_addrem_computed: u32 = 336068857;
    pub const isc_dyn_no_empty_pw: u32 = 336068858;
    pub const isc_dyn_dup_index: u32 = 336068859;
    pub const isc_dyn_package_not_found: u32 = 336068864;
    pub const isc_dyn_schema_not_found: u32 = 336068865;
    pub const isc_dyn_cannot_mod_sysproc: u32 = 336068866;
    pub const isc_dyn_cannot_mod_systrig: u32 = 336068867;
    pub const isc_dyn_cannot_mod_sysfunc: u32 = 336068868;
    pub const isc_dyn_invalid_ddl_proc: u32 = 336068869;
    pub const isc_dyn_invalid_ddl_trig: u32 = 336068870;
    pub const isc_dyn_funcnotdef_package: u32 = 336068871;
    pub const isc_dyn_procnotdef_package: u32 = 336068872;
    pub const isc_dyn_funcsignat_package: u32 = 336068873;
    pub const isc_dyn_procsignat_package: u32 = 336068874;
    pub const isc_dyn_defvaldecl_package_proc: u32 = 336068875;
    pub const isc_dyn_package_body_exists: u32 = 336068877;
    pub const isc_dyn_invalid_ddl_func: u32 = 336068878;
    pub const isc_dyn_newfc_oldsyntax: u32 = 336068879;
    pub const isc_dyn_func_param_not_found: u32 = 336068886;
    pub const isc_dyn_routine_param_not_found: u32 = 336068887;
    pub const isc_dyn_routine_param_ambiguous: u32 = 336068888;
    pub const isc_dyn_coll_used_function: u32 = 336068889;
    pub const isc_dyn_domain_used_function: u32 = 336068890;
    pub const isc_dyn_alter_user_no_clause: u32 = 336068891;
    pub const isc_dyn_duplicate_package_item: u32 = 336068894;
    pub const isc_dyn_cant_modify_sysobj: u32 = 336068895;
    pub const isc_dyn_cant_use_zero_increment: u32 = 336068896;
    pub const isc_dyn_cant_use_in_foreignkey: u32 = 336068897;
    pub const isc_dyn_defvaldecl_package_func: u32 = 336068898;
    pub const isc_gbak_unknown_switch: u32 = 336330753;
    pub const isc_gbak_page_size_missing: u32 = 336330754;
    pub const isc_gbak_page_size_toobig: u32 = 336330755;
    pub const isc_gbak_redir_ouput_missing: u32 = 336330756;
    pub const isc_gbak_switches_conflict: u32 = 336330757;
    pub const isc_gbak_unknown_device: u32 = 336330758;
    pub const isc_gbak_no_protection: u32 = 336330759;
    pub const isc_gbak_page_size_not_allowed: u32 = 336330760;
    pub const isc_gbak_multi_source_dest: u32 = 336330761;
    pub const isc_gbak_filename_missing: u32 = 336330762;
    pub const isc_gbak_dup_inout_names: u32 = 336330763;
    pub const isc_gbak_inv_page_size: u32 = 336330764;
    pub const isc_gbak_db_specified: u32 = 336330765;
    pub const isc_gbak_db_exists: u32 = 336330766;
    pub const isc_gbak_unk_device: u32 = 336330767;
    pub const isc_gbak_blob_info_failed: u32 = 336330772;
    pub const isc_gbak_unk_blob_item: u32 = 336330773;
    pub const isc_gbak_get_seg_failed: u32 = 336330774;
    pub const isc_gbak_close_blob_failed: u32 = 336330775;
    pub const isc_gbak_open_blob_failed: u32 = 336330776;
    pub const isc_gbak_put_blr_gen_id_failed: u32 = 336330777;
    pub const isc_gbak_unk_type: u32 = 336330778;
    pub const isc_gbak_comp_req_failed: u32 = 336330779;
    pub const isc_gbak_start_req_failed: u32 = 336330780;
    pub const isc_gbak_rec_failed: u32 = 336330781;
    pub const isc_gbak_rel_req_failed: u32 = 336330782;
    pub const isc_gbak_db_info_failed: u32 = 336330783;
    pub const isc_gbak_no_db_desc: u32 = 336330784;
    pub const isc_gbak_db_create_failed: u32 = 336330785;
    pub const isc_gbak_decomp_len_error: u32 = 336330786;
    pub const isc_gbak_tbl_missing: u32 = 336330787;
    pub const isc_gbak_blob_col_missing: u32 = 336330788;
    pub const isc_gbak_create_blob_failed: u32 = 336330789;
    pub const isc_gbak_put_seg_failed: u32 = 336330790;
    pub const isc_gbak_rec_len_exp: u32 = 336330791;
    pub const isc_gbak_inv_rec_len: u32 = 336330792;
    pub const isc_gbak_exp_data_type: u32 = 336330793;
    pub const isc_gbak_gen_id_failed: u32 = 336330794;
    pub const isc_gbak_unk_rec_type: u32 = 336330795;
    pub const isc_gbak_inv_bkup_ver: u32 = 336330796;
    pub const isc_gbak_missing_bkup_desc: u32 = 336330797;
    pub const isc_gbak_string_trunc: u32 = 336330798;
    pub const isc_gbak_cant_rest_record: u32 = 336330799;
    pub const isc_gbak_send_failed: u32 = 336330800;
    pub const isc_gbak_no_tbl_name: u32 = 336330801;
    pub const isc_gbak_unexp_eof: u32 = 336330802;
    pub const isc_gbak_db_format_too_old: u32 = 336330803;
    pub const isc_gbak_inv_array_dim: u32 = 336330804;
    pub const isc_gbak_xdr_len_expected: u32 = 336330807;
    pub const isc_gbak_open_bkup_error: u32 = 336330817;
    pub const isc_gbak_open_error: u32 = 336330818;
    pub const isc_gbak_missing_block_fac: u32 = 336330934;
    pub const isc_gbak_inv_block_fac: u32 = 336330935;
    pub const isc_gbak_block_fac_specified: u32 = 336330936;
    pub const isc_gbak_missing_username: u32 = 336330940;
    pub const isc_gbak_missing_password: u32 = 336330941;
    pub const isc_gbak_missing_skipped_bytes: u32 = 336330952;
    pub const isc_gbak_inv_skipped_bytes: u32 = 336330953;
    pub const isc_gbak_err_restore_charset: u32 = 336330965;
    pub const isc_gbak_err_restore_collation: u32 = 336330967;
    pub const isc_gbak_read_error: u32 = 336330972;
    pub const isc_gbak_write_error: u32 = 336330973;
    pub const isc_gbak_db_in_use: u32 = 336330985;
    pub const isc_gbak_sysmemex: u32 = 336330990;
    pub const isc_gbak_restore_role_failed: u32 = 336331002;
    pub const isc_gbak_role_op_missing: u32 = 336331005;
    pub const isc_gbak_page_buffers_missing: u32 = 336331010;
    pub const isc_gbak_page_buffers_wrong_param: u32 = 336331011;
    pub const isc_gbak_page_buffers_restore: u32 = 336331012;
    pub const isc_gbak_inv_size: u32 = 336331014;
    pub const isc_gbak_file_outof_sequence: u32 = 336331015;
    pub const isc_gbak_join_file_missing: u32 = 336331016;
    pub const isc_gbak_stdin_not_supptd: u32 = 336331017;
    pub const isc_gbak_stdout_not_supptd: u32 = 336331018;
    pub const isc_gbak_bkup_corrupt: u32 = 336331019;
    pub const isc_gbak_unk_db_file_spec: u32 = 336331020;
    pub const isc_gbak_hdr_write_failed: u32 = 336331021;
    pub const isc_gbak_disk_space_ex: u32 = 336331022;
    pub const isc_gbak_size_lt_min: u32 = 336331023;
    pub const isc_gbak_svc_name_missing: u32 = 336331025;
    pub const isc_gbak_not_ownr: u32 = 336331026;
    pub const isc_gbak_mode_req: u32 = 336331031;
    pub const isc_gbak_just_data: u32 = 336331033;
    pub const isc_gbak_data_only: u32 = 336331034;
    pub const isc_gbak_missing_interval: u32 = 336331078;
    pub const isc_gbak_wrong_interval: u32 = 336331079;
    pub const isc_gbak_verify_verbint: u32 = 336331081;
    pub const isc_gbak_option_only_restore: u32 = 336331082;
    pub const isc_gbak_option_only_backup: u32 = 336331083;
    pub const isc_gbak_option_conflict: u32 = 336331084;
    pub const isc_gbak_param_conflict: u32 = 336331085;
    pub const isc_gbak_option_repeated: u32 = 336331086;
    pub const isc_gbak_max_dbkey_recursion: u32 = 336331091;
    pub const isc_gbak_max_dbkey_length: u32 = 336331092;
    pub const isc_gbak_invalid_metadata: u32 = 336331093;
    pub const isc_gbak_invalid_data: u32 = 336331094;
    pub const isc_gbak_inv_bkup_ver2: u32 = 336331096;
    pub const isc_gbak_db_format_too_old2: u32 = 336331100;
    pub const isc_dsql_too_old_ods: u32 = 336397205;
    pub const isc_dsql_table_not_found: u32 = 336397206;
    pub const isc_dsql_view_not_found: u32 = 336397207;
    pub const isc_dsql_line_col_error: u32 = 336397208;
    pub const isc_dsql_unknown_pos: u32 = 336397209;
    pub const isc_dsql_no_dup_name: u32 = 336397210;
    pub const isc_dsql_too_many_values: u32 = 336397211;
    pub const isc_dsql_no_array_computed: u32 = 336397212;
    pub const isc_dsql_implicit_domain_name: u32 = 336397213;
    pub const isc_dsql_only_can_subscript_array: u32 = 336397214;
    pub const isc_dsql_max_sort_items: u32 = 336397215;
    pub const isc_dsql_max_group_items: u32 = 336397216;
    pub const isc_dsql_conflicting_sort_field: u32 = 336397217;
    pub const isc_dsql_derived_table_more_columns: u32 = 336397218;
    pub const isc_dsql_derived_table_less_columns: u32 = 336397219;
    pub const isc_dsql_derived_field_unnamed: u32 = 336397220;
    pub const isc_dsql_derived_field_dup_name: u32 = 336397221;
    pub const isc_dsql_derived_alias_select: u32 = 336397222;
    pub const isc_dsql_derived_alias_field: u32 = 336397223;
    pub const isc_dsql_auto_field_bad_pos: u32 = 336397224;
    pub const isc_dsql_cte_wrong_reference: u32 = 336397225;
    pub const isc_dsql_cte_cycle: u32 = 336397226;
    pub const isc_dsql_cte_outer_join: u32 = 336397227;
    pub const isc_dsql_cte_mult_references: u32 = 336397228;
    pub const isc_dsql_cte_not_a_union: u32 = 336397229;
    pub const isc_dsql_cte_nonrecurs_after_recurs: u32 = 336397230;
    pub const isc_dsql_cte_wrong_clause: u32 = 336397231;
    pub const isc_dsql_cte_union_all: u32 = 336397232;
    pub const isc_dsql_cte_miss_nonrecursive: u32 = 336397233;
    pub const isc_dsql_cte_nested_with: u32 = 336397234;
    pub const isc_dsql_col_more_than_once_using: u32 = 336397235;
    pub const isc_dsql_unsupp_feature_dialect: u32 = 336397236;
    pub const isc_dsql_cte_not_used: u32 = 336397237;
    pub const isc_dsql_col_more_than_once_view: u32 = 336397238;
    pub const isc_dsql_unsupported_in_auto_trans: u32 = 336397239;
    pub const isc_dsql_eval_unknode: u32 = 336397240;
    pub const isc_dsql_agg_wrongarg: u32 = 336397241;
    pub const isc_dsql_agg2_wrongarg: u32 = 336397242;
    pub const isc_dsql_nodateortime_pm_string: u32 = 336397243;
    pub const isc_dsql_invalid_datetime_subtract: u32 = 336397244;
    pub const isc_dsql_invalid_dateortime_add: u32 = 336397245;
    pub const isc_dsql_invalid_type_minus_date: u32 = 336397246;
    pub const isc_dsql_nostring_addsub_dial3: u32 = 336397247;
    pub const isc_dsql_invalid_type_addsub_dial3: u32 = 336397248;
    pub const isc_dsql_invalid_type_multip_dial1: u32 = 336397249;
    pub const isc_dsql_nostring_multip_dial3: u32 = 336397250;
    pub const isc_dsql_invalid_type_multip_dial3: u32 = 336397251;
    pub const isc_dsql_mustuse_numeric_div_dial1: u32 = 336397252;
    pub const isc_dsql_nostring_div_dial3: u32 = 336397253;
    pub const isc_dsql_invalid_type_div_dial3: u32 = 336397254;
    pub const isc_dsql_nostring_neg_dial3: u32 = 336397255;
    pub const isc_dsql_invalid_type_neg: u32 = 336397256;
    pub const isc_dsql_max_distinct_items: u32 = 336397257;
    pub const isc_dsql_alter_charset_failed: u32 = 336397258;
    pub const isc_dsql_comment_on_failed: u32 = 336397259;
    pub const isc_dsql_create_func_failed: u32 = 336397260;
    pub const isc_dsql_alter_func_failed: u32 = 336397261;
    pub const isc_dsql_create_alter_func_failed: u32 = 336397262;
    pub const isc_dsql_drop_func_failed: u32 = 336397263;
    pub const isc_dsql_recreate_func_failed: u32 = 336397264;
    pub const isc_dsql_create_proc_failed: u32 = 336397265;
    pub const isc_dsql_alter_proc_failed: u32 = 336397266;
    pub const isc_dsql_create_alter_proc_failed: u32 = 336397267;
    pub const isc_dsql_drop_proc_failed: u32 = 336397268;
    pub const isc_dsql_recreate_proc_failed: u32 = 336397269;
    pub const isc_dsql_create_trigger_failed: u32 = 336397270;
    pub const isc_dsql_alter_trigger_failed: u32 = 336397271;
    pub const isc_dsql_create_alter_trigger_failed: u32 = 336397272;
    pub const isc_dsql_drop_trigger_failed: u32 = 336397273;
    pub const isc_dsql_recreate_trigger_failed: u32 = 336397274;
    pub const isc_dsql_create_collation_failed: u32 = 336397275;
    pub const isc_dsql_drop_collation_failed: u32 = 336397276;
    pub const isc_dsql_create_domain_failed: u32 = 336397277;
    pub const isc_dsql_alter_domain_failed: u32 = 336397278;
    pub const isc_dsql_drop_domain_failed: u32 = 336397279;
    pub const isc_dsql_create_except_failed: u32 = 336397280;
    pub const isc_dsql_alter_except_failed: u32 = 336397281;
    pub const isc_dsql_create_alter_except_failed: u32 = 336397282;
    pub const isc_dsql_recreate_except_failed: u32 = 336397283;
    pub const isc_dsql_drop_except_failed: u32 = 336397284;
    pub const isc_dsql_create_sequence_failed: u32 = 336397285;
    pub const isc_dsql_create_table_failed: u32 = 336397286;
    pub const isc_dsql_alter_table_failed: u32 = 336397287;
    pub const isc_dsql_drop_table_failed: u32 = 336397288;
    pub const isc_dsql_recreate_table_failed: u32 = 336397289;
    pub const isc_dsql_create_pack_failed: u32 = 336397290;
    pub const isc_dsql_alter_pack_failed: u32 = 336397291;
    pub const isc_dsql_create_alter_pack_failed: u32 = 336397292;
    pub const isc_dsql_drop_pack_failed: u32 = 336397293;
    pub const isc_dsql_recreate_pack_failed: u32 = 336397294;
    pub const isc_dsql_create_pack_body_failed: u32 = 336397295;
    pub const isc_dsql_drop_pack_body_failed: u32 = 336397296;
    pub const isc_dsql_recreate_pack_body_failed: u32 = 336397297;
    pub const isc_dsql_create_view_failed: u32 = 336397298;
    pub const isc_dsql_alter_view_failed: u32 = 336397299;
    pub const isc_dsql_create_alter_view_failed: u32 = 336397300;
    pub const isc_dsql_recreate_view_failed: u32 = 336397301;
    pub const isc_dsql_drop_view_failed: u32 = 336397302;
    pub const isc_dsql_drop_sequence_failed: u32 = 336397303;
    pub const isc_dsql_recreate_sequence_failed: u32 = 336397304;
    pub const isc_dsql_drop_index_failed: u32 = 336397305;
    pub const isc_dsql_drop_filter_failed: u32 = 336397306;
    pub const isc_dsql_drop_shadow_failed: u32 = 336397307;
    pub const isc_dsql_drop_role_failed: u32 = 336397308;
    pub const isc_dsql_drop_user_failed: u32 = 336397309;
    pub const isc_dsql_create_role_failed: u32 = 336397310;
    pub const isc_dsql_alter_role_failed: u32 = 336397311;
    pub const isc_dsql_alter_index_failed: u32 = 336397312;
    pub const isc_dsql_alter_database_failed: u32 = 336397313;
    pub const isc_dsql_create_shadow_failed: u32 = 336397314;
    pub const isc_dsql_create_filter_failed: u32 = 336397315;
    pub const isc_dsql_create_index_failed: u32 = 336397316;
    pub const isc_dsql_create_user_failed: u32 = 336397317;
    pub const isc_dsql_alter_user_failed: u32 = 336397318;
    pub const isc_dsql_grant_failed: u32 = 336397319;
    pub const isc_dsql_revoke_failed: u32 = 336397320;
    pub const isc_dsql_cte_recursive_aggregate: u32 = 336397321;
    pub const isc_dsql_mapping_failed: u32 = 336397322;
    pub const isc_dsql_alter_sequence_failed: u32 = 336397323;
    pub const isc_dsql_create_generator_failed: u32 = 336397324;
    pub const isc_dsql_set_generator_failed: u32 = 336397325;
    pub const isc_dsql_wlock_simple: u32 = 336397326;
    pub const isc_dsql_firstskip_rows: u32 = 336397327;
    pub const isc_dsql_wlock_aggregates: u32 = 336397328;
    pub const isc_dsql_wlock_conflict: u32 = 336397329;
    pub const isc_dsql_max_exception_arguments: u32 = 336397330;
    pub const isc_dsql_string_byte_length: u32 = 336397331;
    pub const isc_dsql_string_char_length: u32 = 336397332;
    pub const isc_dsql_max_nesting: u32 = 336397333;
    pub const isc_gsec_cant_open_db: u32 = 336723983;
    pub const isc_gsec_switches_error: u32 = 336723984;
    pub const isc_gsec_no_op_spec: u32 = 336723985;
    pub const isc_gsec_no_usr_name: u32 = 336723986;
    pub const isc_gsec_err_add: u32 = 336723987;
    pub const isc_gsec_err_modify: u32 = 336723988;
    pub const isc_gsec_err_find_mod: u32 = 336723989;
    pub const isc_gsec_err_rec_not_found: u32 = 336723990;
    pub const isc_gsec_err_delete: u32 = 336723991;
    pub const isc_gsec_err_find_del: u32 = 336723992;
    pub const isc_gsec_err_find_disp: u32 = 336723996;
    pub const isc_gsec_inv_param: u32 = 336723997;
    pub const isc_gsec_op_specified: u32 = 336723998;
    pub const isc_gsec_pw_specified: u32 = 336723999;
    pub const isc_gsec_uid_specified: u32 = 336724000;
    pub const isc_gsec_gid_specified: u32 = 336724001;
    pub const isc_gsec_proj_specified: u32 = 336724002;
    pub const isc_gsec_org_specified: u32 = 336724003;
    pub const isc_gsec_fname_specified: u32 = 336724004;
    pub const isc_gsec_mname_specified: u32 = 336724005;
    pub const isc_gsec_lname_specified: u32 = 336724006;
    pub const isc_gsec_inv_switch: u32 = 336724008;
    pub const isc_gsec_amb_switch: u32 = 336724009;
    pub const isc_gsec_no_op_specified: u32 = 336724010;
    pub const isc_gsec_params_not_allowed: u32 = 336724011;
    pub const isc_gsec_incompat_switch: u32 = 336724012;
    pub const isc_gsec_inv_username: u32 = 336724044;
    pub const isc_gsec_inv_pw_length: u32 = 336724045;
    pub const isc_gsec_db_specified: u32 = 336724046;
    pub const isc_gsec_db_admin_specified: u32 = 336724047;
    pub const isc_gsec_db_admin_pw_specified: u32 = 336724048;
    pub const isc_gsec_sql_role_specified: u32 = 336724049;
    pub const isc_gstat_unknown_switch: u32 = 336920577;
    pub const isc_gstat_retry: u32 = 336920578;
    pub const isc_gstat_wrong_ods: u32 = 336920579;
    pub const isc_gstat_unexpected_eof: u32 = 336920580;
    pub const isc_gstat_open_err: u32 = 336920605;
    pub const isc_gstat_read_err: u32 = 336920606;
    pub const isc_gstat_sysmemex: u32 = 336920607;
    pub const isc_fbsvcmgr_bad_am: u32 = 336986113;
    pub const isc_fbsvcmgr_bad_wm: u32 = 336986114;
    pub const isc_fbsvcmgr_bad_rs: u32 = 336986115;
    pub const isc_fbsvcmgr_info_err: u32 = 336986116;
    pub const isc_fbsvcmgr_query_err: u32 = 336986117;
    pub const isc_fbsvcmgr_switch_unknown: u32 = 336986118;
    pub const isc_fbsvcmgr_bad_sm: u32 = 336986159;
    pub const isc_fbsvcmgr_fp_open: u32 = 336986160;
    pub const isc_fbsvcmgr_fp_read: u32 = 336986161;
    pub const isc_fbsvcmgr_fp_empty: u32 = 336986162;
    pub const isc_fbsvcmgr_bad_arg: u32 = 336986164;
    pub const isc_utl_trusted_switch: u32 = 337051649;
    pub const isc_nbackup_missing_param: u32 = 337117213;
    pub const isc_nbackup_allowed_switches: u32 = 337117214;
    pub const isc_nbackup_unknown_param: u32 = 337117215;
    pub const isc_nbackup_unknown_switch: u32 = 337117216;
    pub const isc_nbackup_nofetchpw_svc: u32 = 337117217;
    pub const isc_nbackup_pwfile_error: u32 = 337117218;
    pub const isc_nbackup_size_with_lock: u32 = 337117219;
    pub const isc_nbackup_no_switch: u32 = 337117220;
    pub const isc_nbackup_err_read: u32 = 337117223;
    pub const isc_nbackup_err_write: u32 = 337117224;
    pub const isc_nbackup_err_seek: u32 = 337117225;
    pub const isc_nbackup_err_opendb: u32 = 337117226;
    pub const isc_nbackup_err_fadvice: u32 = 337117227;
    pub const isc_nbackup_err_createdb: u32 = 337117228;
    pub const isc_nbackup_err_openbk: u32 = 337117229;
    pub const isc_nbackup_err_createbk: u32 = 337117230;
    pub const isc_nbackup_err_eofdb: u32 = 337117231;
    pub const isc_nbackup_fixup_wrongstate: u32 = 337117232;
    pub const isc_nbackup_err_db: u32 = 337117233;
    pub const isc_nbackup_userpw_toolong: u32 = 337117234;
    pub const isc_nbackup_lostrec_db: u32 = 337117235;
    pub const isc_nbackup_lostguid_db: u32 = 337117236;
    pub const isc_nbackup_err_eofhdrdb: u32 = 337117237;
    pub const isc_nbackup_db_notlock: u32 = 337117238;
    pub const isc_nbackup_lostguid_bk: u32 = 337117239;
    pub const isc_nbackup_page_changed: u32 = 337117240;
    pub const isc_nbackup_dbsize_inconsistent: u32 = 337117241;
    pub const isc_nbackup_failed_lzbk: u32 = 337117242;
    pub const isc_nbackup_err_eofhdrbk: u32 = 337117243;
    pub const isc_nbackup_invalid_incbk: u32 = 337117244;
    pub const isc_nbackup_unsupvers_incbk: u32 = 337117245;
    pub const isc_nbackup_invlevel_incbk: u32 = 337117246;
    pub const isc_nbackup_wrong_orderbk: u32 = 337117247;
    pub const isc_nbackup_err_eofbk: u32 = 337117248;
    pub const isc_nbackup_err_copy: u32 = 337117249;
    pub const isc_nbackup_err_eofhdr_restdb: u32 = 337117250;
    pub const isc_nbackup_lostguid_l0bk: u32 = 337117251;
    pub const isc_nbackup_switchd_parameter: u32 = 337117255;
    pub const isc_nbackup_user_stop: u32 = 337117257;
    pub const isc_nbackup_deco_parse: u32 = 337117259;
    pub const isc_trace_conflict_acts: u32 = 337182750;
    pub const isc_trace_act_notfound: u32 = 337182751;
    pub const isc_trace_switch_once: u32 = 337182752;
    pub const isc_trace_param_val_miss: u32 = 337182753;
    pub const isc_trace_param_invalid: u32 = 337182754;
    pub const isc_trace_switch_unknown: u32 = 337182755;
    pub const isc_trace_switch_svc_only: u32 = 337182756;
    pub const isc_trace_switch_user_only: u32 = 337182757;
    pub const isc_trace_switch_param_miss: u32 = 337182758;
    pub const isc_trace_param_act_notcompat: u32 = 337182759;
    pub const isc_trace_mandatory_switch_miss: u32 = 337182760;
    pub const isc_err_max: u32 = 1259;
}
pub use consts::*;

mod typedefs {
    #![allow(non_upper_case_globals, dead_code, non_camel_case_types)]

    pub type size_t = ::std::os::raw::c_ulong;
    pub type wchar_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    pub struct max_align_t {
        pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
        pub __bindgen_padding_0: u64,
        pub __clang_max_align_nonce2: u128,
    }
    pub type __u_char = ::std::os::raw::c_uchar;
    pub type __u_short = ::std::os::raw::c_ushort;
    pub type __u_int = ::std::os::raw::c_uint;
    pub type __u_long = ::std::os::raw::c_ulong;
    pub type __int8_t = ::std::os::raw::c_schar;
    pub type __uint8_t = ::std::os::raw::c_uchar;
    pub type __int16_t = ::std::os::raw::c_short;
    pub type __uint16_t = ::std::os::raw::c_ushort;
    pub type __int32_t = ::std::os::raw::c_int;
    pub type __uint32_t = ::std::os::raw::c_uint;
    pub type __int64_t = ::std::os::raw::c_long;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __quad_t = ::std::os::raw::c_long;
    pub type __u_quad_t = ::std::os::raw::c_ulong;
    pub type __intmax_t = ::std::os::raw::c_long;
    pub type __uintmax_t = ::std::os::raw::c_ulong;
    pub type __dev_t = ::std::os::raw::c_ulong;
    pub type __uid_t = ::std::os::raw::c_uint;
    pub type __gid_t = ::std::os::raw::c_uint;
    pub type __ino_t = ::std::os::raw::c_ulong;
    pub type __ino64_t = ::std::os::raw::c_ulong;
    pub type __mode_t = ::std::os::raw::c_uint;
    pub type __nlink_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __pid_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __fsid_t {
        pub __val: [::std::os::raw::c_int; 2usize],
    }
    pub type __clock_t = ::std::os::raw::c_long;
    pub type __rlim_t = ::std::os::raw::c_ulong;
    pub type __rlim64_t = ::std::os::raw::c_ulong;
    pub type __id_t = ::std::os::raw::c_uint;
    pub type __time_t = ::std::os::raw::c_long;
    pub type __useconds_t = ::std::os::raw::c_uint;
    pub type __suseconds_t = ::std::os::raw::c_long;
    pub type __daddr_t = ::std::os::raw::c_int;
    pub type __key_t = ::std::os::raw::c_int;
    pub type __clockid_t = ::std::os::raw::c_int;
    pub type __timer_t = *mut ::std::os::raw::c_void;
    pub type __blksize_t = ::std::os::raw::c_long;
    pub type __blkcnt_t = ::std::os::raw::c_long;
    pub type __blkcnt64_t = ::std::os::raw::c_long;
    pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
    pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
    pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
    pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
    pub type __fsword_t = ::std::os::raw::c_long;
    pub type __ssize_t = ::std::os::raw::c_long;
    pub type __syscall_slong_t = ::std::os::raw::c_long;
    pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
    pub type __loff_t = __off64_t;
    pub type __qaddr_t = *mut __quad_t;
    pub type __caddr_t = *mut ::std::os::raw::c_char;
    pub type __intptr_t = ::std::os::raw::c_long;
    pub type __socklen_t = ::std::os::raw::c_uint;
    pub type __sig_atomic_t = ::std::os::raw::c_int;
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
    pub type intmax_t = __intmax_t;
    pub type uintmax_t = __uintmax_t;
    pub type __gwchar_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct imaxdiv_t {
        pub quot: ::std::os::raw::c_long,
        pub rem: ::std::os::raw::c_long,
    }
    extern "C" {
        pub fn imaxabs(__n: intmax_t) -> intmax_t;
    }
    extern "C" {
        pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
    }
    extern "C" {
        pub fn strtoimax(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> intmax_t;
    }
    extern "C" {
        pub fn strtoumax(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> uintmax_t;
    }
    extern "C" {
        pub fn wcstoimax(
            __nptr: *const __gwchar_t,
            __endptr: *mut *mut __gwchar_t,
            __base: ::std::os::raw::c_int,
        ) -> intmax_t;
    }
    extern "C" {
        pub fn wcstoumax(
            __nptr: *const __gwchar_t,
            __endptr: *mut *mut __gwchar_t,
            __base: ::std::os::raw::c_int,
        ) -> uintmax_t;
    }
    pub type FB_API_HANDLE = ::std::os::raw::c_uint;
    pub type FB_SIZE_T = ::std::os::raw::c_uint;
    pub type FB_SSIZE_T = ::std::os::raw::c_int;
    pub type ISC_STATUS = isize;
    pub type ISC_STATUS_ARRAY = [ISC_STATUS; 20usize];
    pub type FB_SQLSTATE_STRING = [::std::os::raw::c_char; 6usize];
    pub type ISC_LONG = ::std::os::raw::c_int;
    pub type ISC_ULONG = ::std::os::raw::c_uint;
    pub type ISC_SHORT = ::std::os::raw::c_short;
    pub type ISC_USHORT = ::std::os::raw::c_ushort;
    pub type ISC_UCHAR = ::std::os::raw::c_uchar;
    pub type ISC_SCHAR = ::std::os::raw::c_char;
    pub type FB_BOOLEAN = ISC_UCHAR;
    pub type ISC_INT64 = ::std::os::raw::c_longlong;
    pub type ISC_UINT64 = ::std::os::raw::c_ulonglong;
    pub type ISC_DATE = ::std::os::raw::c_int;
    pub type ISC_TIME = ::std::os::raw::c_uint;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ISC_TIMESTAMP {
        pub timestamp_date: ISC_DATE,
        pub timestamp_time: ISC_TIME,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct GDS_QUAD_t {
        pub gds_quad_high: ISC_LONG,
        pub gds_quad_low: ISC_ULONG,
    }
    pub type GDS_QUAD = GDS_QUAD_t;
    pub type ISC_QUAD = GDS_QUAD_t;
    pub type FB_SHUTDOWN_CALLBACK = ::std::option::Option<
        unsafe extern "C" fn(
            reason: ::std::os::raw::c_int,
            mask: ::std::os::raw::c_int,
            arg: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >;
    pub type isc_att_handle = FB_API_HANDLE;
    pub type isc_blob_handle = FB_API_HANDLE;
    pub type isc_db_handle = FB_API_HANDLE;
    pub type isc_req_handle = FB_API_HANDLE;
    pub type isc_stmt_handle = FB_API_HANDLE;
    pub type isc_svc_handle = FB_API_HANDLE;
    pub type isc_tr_handle = FB_API_HANDLE;
    pub type isc_callback = ::std::option::Option<unsafe extern "C" fn()>;
    pub type isc_resv_handle = ISC_LONG;
    pub type ISC_PRINT_CALLBACK = ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ISC_SHORT,
            arg3: *const ::std::os::raw::c_char,
        ),
    >;
    pub type ISC_VERSION_CALLBACK = ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
        ),
    >;
    pub type ISC_EVENT_CALLBACK = ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ISC_USHORT,
            arg3: *const ISC_UCHAR,
        ),
    >;
    pub type GDS__QUAD = GDS_QUAD;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ISC_ARRAY_BOUND {
        pub array_bound_lower: ::std::os::raw::c_short,
        pub array_bound_upper: ::std::os::raw::c_short,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ISC_ARRAY_DESC {
        pub array_desc_dtype: ISC_UCHAR,
        pub array_desc_scale: ISC_SCHAR,
        pub array_desc_length: ::std::os::raw::c_ushort,
        pub array_desc_field_name: [ISC_SCHAR; 32usize],
        pub array_desc_relation_name: [ISC_SCHAR; 32usize],
        pub array_desc_dimensions: ::std::os::raw::c_short,
        pub array_desc_flags: ::std::os::raw::c_short,
        pub array_desc_bounds: [ISC_ARRAY_BOUND; 16usize],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ISC_BLOB_DESC {
        pub blob_desc_subtype: ::std::os::raw::c_short,
        pub blob_desc_charset: ::std::os::raw::c_short,
        pub blob_desc_segment_size: ::std::os::raw::c_short,
        pub blob_desc_field_name: [ISC_UCHAR; 32usize],
        pub blob_desc_relation_name: [ISC_UCHAR; 32usize],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct isc_blob_ctl {
        pub ctl_source: ::std::option::Option<unsafe extern "C" fn() -> ISC_STATUS>,
        pub ctl_source_handle: *mut isc_blob_ctl,
        pub ctl_to_sub_type: ::std::os::raw::c_short,
        pub ctl_from_sub_type: ::std::os::raw::c_short,
        pub ctl_buffer_length: ::std::os::raw::c_ushort,
        pub ctl_segment_length: ::std::os::raw::c_ushort,
        pub ctl_bpb_length: ::std::os::raw::c_ushort,
        pub ctl_bpb: *mut ISC_SCHAR,
        pub ctl_buffer: *mut ISC_UCHAR,
        pub ctl_max_segment: ISC_LONG,
        pub ctl_number_segments: ISC_LONG,
        pub ctl_total_length: ISC_LONG,
        pub ctl_status: *mut ISC_STATUS,
        pub ctl_data: [::std::os::raw::c_long; 8usize],
    }
    pub type ISC_BLOB_CTL = *mut isc_blob_ctl;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct bstream {
        pub bstr_blob: isc_blob_handle,
        pub bstr_buffer: *mut ISC_SCHAR,
        pub bstr_ptr: *mut ISC_SCHAR,
        pub bstr_length: ::std::os::raw::c_short,
        pub bstr_cnt: ::std::os::raw::c_short,
        pub bstr_mode: ::std::os::raw::c_char,
    }
    pub type BSTREAM = bstream;
    pub type FB_BLOB_STREAM = *mut bstream;
    pub const blob_lseek_mode_blb_seek_relative: blob_lseek_mode = 1;
    pub const blob_lseek_mode_blb_seek_from_tail: blob_lseek_mode = 2;
    pub type blob_lseek_mode = u32;
    pub const blob_get_result_blb_got_fragment: blob_get_result = -1;
    pub const blob_get_result_blb_got_eof: blob_get_result = 0;
    pub const blob_get_result_blb_got_full_segment: blob_get_result = 1;
    pub type blob_get_result = i32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct blobcallback {
        pub blob_get_segment: ::std::option::Option<
            unsafe extern "C" fn(
                hnd: *mut ::std::os::raw::c_void,
                buffer: *mut ISC_UCHAR,
                buf_size: ISC_USHORT,
                result_len: *mut ISC_USHORT,
            ) -> ::std::os::raw::c_short,
        >,
        pub blob_handle: *mut ::std::os::raw::c_void,
        pub blob_number_segments: ISC_LONG,
        pub blob_max_segment: ISC_LONG,
        pub blob_total_length: ISC_LONG,
        pub blob_put_segment: ::std::option::Option<
            unsafe extern "C" fn(
                hnd: *mut ::std::os::raw::c_void,
                buffer: *const ISC_UCHAR,
                buf_size: ISC_USHORT,
            ),
        >,
        pub blob_lseek: ::std::option::Option<
            unsafe extern "C" fn(
                hnd: *mut ::std::os::raw::c_void,
                mode: ISC_USHORT,
                offset: ISC_LONG,
            ) -> ISC_LONG,
        >,
    }
    pub type BLOBCALLBACK = *mut blobcallback;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct paramdsc {
        pub dsc_dtype: ISC_UCHAR,
        pub dsc_scale: ::std::os::raw::c_schar,
        pub dsc_length: ISC_USHORT,
        pub dsc_sub_type: ::std::os::raw::c_short,
        pub dsc_flags: ISC_USHORT,
        pub dsc_address: *mut ISC_UCHAR,
    }
    pub type PARAMDSC = paramdsc;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct paramvary {
        pub vary_length: ISC_USHORT,
        pub vary_string: [ISC_UCHAR; 1usize],
    }
    pub type PARAMVARY = paramvary;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct XSQLVAR {
        pub sqltype: ISC_SHORT,
        pub sqlscale: ISC_SHORT,
        pub sqlsubtype: ISC_SHORT,
        pub sqllen: ISC_SHORT,
        pub sqldata: *mut ISC_SCHAR,
        pub sqlind: *mut ISC_SHORT,
        pub sqlname_length: ISC_SHORT,
        pub sqlname: [ISC_SCHAR; 32usize],
        pub relname_length: ISC_SHORT,
        pub relname: [ISC_SCHAR; 32usize],
        pub ownname_length: ISC_SHORT,
        pub ownname: [ISC_SCHAR; 32usize],
        pub aliasname_length: ISC_SHORT,
        pub aliasname: [ISC_SCHAR; 32usize],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct XSQLDA {
        pub version: ISC_SHORT,
        pub sqldaid: [ISC_SCHAR; 8usize],
        pub sqldabc: ISC_LONG,
        pub sqln: ISC_SHORT,
        pub sqld: ISC_SHORT,
        pub sqlvar: [XSQLVAR; 1usize],
    }
}
pub use typedefs::*;

#[cfg(test)]
mod test {
    use super::LIB;
    use crate::ibase;

    #[test]
    fn test() {
        // let a = LIB.get(symbol)
    }
}
