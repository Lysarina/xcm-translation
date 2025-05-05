#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_major() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_minor() -> libc::c_uint {
    return 11 as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_patch() -> libc::c_uint {
    return 1 as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version() -> *const libc::c_char {
    return b"1.11.1\0" as *const u8 as *const libc::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_api_major() -> libc::c_uint {
    return 0 as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_api_minor() -> libc::c_uint {
    return 26 as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_api() -> *const libc::c_char {
    return b"0.26\0" as *const u8 as *const libc::c_char;
}
