#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_major() -> libc::c_uint {
    1
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_minor() -> libc::c_uint {
    11
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_patch() -> libc::c_uint {
    1
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version() -> *const libc::c_char {
    b"1.11.1\0" as *const u8 as *const libc::c_char
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_api_major() -> libc::c_uint {
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_api_minor() -> libc::c_uint {
    26
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_version_api() -> *const libc::c_char {
    b"0.26\0" as *const u8 as *const libc::c_char
}
