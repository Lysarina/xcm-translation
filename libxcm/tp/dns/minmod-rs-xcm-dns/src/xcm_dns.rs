#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
#[macro_use]
extern crate c2rust_bitfields;
unsafe extern "C" {
    pub type re_dfa_t;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ut_mem_exhausted() -> !;
    fn ut_fatal() -> !;
}
pub type size_t = libc::c_ulong;
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut libc::c_char,
    pub __translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type reg_syntax_t = libc::c_ulong;
pub type __re_long_size_t = libc::c_ulong;
pub const _REG_NOMATCH: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type regoff_t = libc::c_int;
pub type C2RustUnnamed = libc::c_int;
pub const _REG_ERPAREN: C2RustUnnamed = 16;
pub const _REG_ESIZE: C2RustUnnamed = 15;
pub const _REG_EEND: C2RustUnnamed = 14;
pub const _REG_BADRPT: C2RustUnnamed = 13;
pub const _REG_ESPACE: C2RustUnnamed = 12;
pub const _REG_ERANGE: C2RustUnnamed = 11;
pub const _REG_BADBR: C2RustUnnamed = 10;
pub const _REG_EBRACE: C2RustUnnamed = 9;
pub const _REG_EPAREN: C2RustUnnamed = 8;
pub const _REG_EBRACK: C2RustUnnamed = 7;
pub const _REG_ESUBREG: C2RustUnnamed = 6;
pub const _REG_EESCAPE: C2RustUnnamed = 5;
pub const _REG_ECTYPE: C2RustUnnamed = 4;
pub const _REG_ECOLLATE: C2RustUnnamed = 3;
pub const _REG_BADPAT: C2RustUnnamed = 2;
pub const _REG_NOERROR: C2RustUnnamed = 0;
pub const _REG_ENOSYS: C2RustUnnamed = -1;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_dns_is_valid_name(mut name: *const libc::c_char) -> bool {
    if strlen(name) > 253 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    let mut re: regex_t = regex_t {
        __buffer: 0 as *mut re_dfa_t,
        __allocated: 0,
        __used: 0,
        __syntax: 0,
        __fastmap: 0 as *mut libc::c_char,
        __translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut rc: libc::c_int = regcomp(
        &mut re,
        b"^[a-z0-9\\-]+(\\.[a-z0-9\\-]+\\.?)*$\0" as *const u8 as *const libc::c_char,
        (1 as libc::c_int) << 1 as libc::c_int | 1 as libc::c_int,
    );
    if rc != 0 as libc::c_int {
        ut_mem_exhausted();
    }
    let mut result: bool = false;
    let mut m: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    rc = regexec(&mut re, name, 1 as libc::c_int as size_t, &mut m, 0 as libc::c_int);
    if rc == 0 as libc::c_int {
        result = 1 as libc::c_int != 0;
    } else if rc == _REG_NOMATCH as libc::c_int {
        result = 0 as libc::c_int != 0;
    } else {
        ut_fatal();
    }
    regfree(&mut re);
    return result;
}
