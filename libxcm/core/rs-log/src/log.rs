#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(c_variadic)]

use std::ffi::CStr;
use std::io::{stderr, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use libc::{__errno_location, strcpy, snprintf};

use xcm_rust_common::xcm_tp::xcm_socket;
use rs_util::*;

unsafe extern "C" {
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
}
pub type log_type = libc::c_uint;
pub const log_type_error: log_type = 1;
pub const log_type_debug: log_type = 0;
unsafe extern "C" fn format_msg(
    buf: *mut libc::c_char,
    capacity: libc::c_ulong,
    file: *const libc::c_char,
    line: libc::c_int,
    function: *const libc::c_char,
    s: *mut xcm_socket,
    format: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) { unsafe {
    let mut sref: [libc::c_char; 64] = [0; 64];
    if !s.is_null() {
        snprintf(
            sref.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as usize,
            b" <%ld>\0" as *const u8 as *const libc::c_char,
            (*s).sock_id,
        );
    } else {
        sref[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    let mut bname: [libc::c_char; 256] = [0; 256];
    strcpy(bname.as_mut_ptr(), file);
    snprintf(
        buf,
        capacity as usize,
        b"TID %d: %s [%s:%d]%s: \0" as *const u8 as *const libc::c_char,
        ut_gettid(),
        function,
        __xpg_basename(bname.as_mut_ptr()),
        line,
        sref.as_mut_ptr(),
    );
    ut_vaprintf(buf, capacity, format, ap.as_va_list());
    ut_aprintf(buf, capacity, b"\n\0" as *const u8 as *const libc::c_char);
}}
static console_enabled: AtomicBool = AtomicBool::new(false);
unsafe extern "C" fn log_console(
    file: *const libc::c_char,
    line: libc::c_int,
    function: *const libc::c_char,
    s: *mut xcm_socket,
    format: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) { unsafe {
    if console_enabled.load(Ordering::Relaxed) {
        let mut _oerrno: libc::c_int = *__errno_location();
        let mut buf: [libc::c_char; 8192] = [0; 8192];
        format_msg(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            file,
            line,
            function,
            s,
            format,
            ap.as_va_list(),
        );
        let c_str = CStr::from_ptr(buf.as_ptr());
        match c_str.to_str() {
            Ok(s) => {
                // Print to stderr
                let _ = writeln!(stderr(), "{}", s);
            }
            Err(e) => {
                let _ = writeln!(stderr(), "Invalid UTF-8: {}", e);
            }
        }
        // stderr().lock().write_all(buf);
        // fputs(buf.as_mut_ptr(), stderr);
        let _ = stderr().flush();
        // fflush(stderr);
        *__errno_location() = _oerrno;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_is_enabled(mut _type_0: log_type) -> bool {
    console_enabled.load(Ordering::Relaxed)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __log_event(
    mut _type_0: log_type,
    file: *const libc::c_char,
    line: libc::c_int,
    function: *const libc::c_char,
    s: *mut xcm_socket,
    format: *const libc::c_char,
    args: ...
) { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    log_console(file, line, function, s, format, ap.as_va_list());
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_console_conf(enabled: bool) {
    console_enabled.store(enabled, Ordering::Relaxed);
}
