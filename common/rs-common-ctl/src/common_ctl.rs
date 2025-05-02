#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

#![feature(extern_types)]

use rs_log::{__log_event, log_type_error, log_is_enabled, log_console_conf};
use xcm_rust_common::xcm_tp::xcm_socket;

unsafe extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn abort() -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_get_dir(buf: *mut libc::c_char, capacity: libc::c_ulong) { unsafe {
    let env: *const libc::c_char = getenv(
        b"XCM_CTL\0" as *const u8 as *const libc::c_char,
    );
    if !env.is_null() && strlen(env) < capacity {
        strcpy(buf, env);
    } else {
        strcpy(buf, b"/run/xcm/ctl\0" as *const u8 as *const libc::c_char);
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_derive_path(
    ctl_dir: *const libc::c_char,
    creator_pid: libc::c_int,
    sock_id: libc::c_long,
    buf: *mut libc::c_char,
    capacity: libc::c_ulong,
) { unsafe {
    let rc: libc::c_int = snprintf(
        buf,
        capacity,
        b"%s/%s%d-%ld\0" as *const u8 as *const libc::c_char,
        ctl_dir,
        b"ctl-\0" as *const u8 as *const libc::c_char,
        creator_pid,
        sock_id,
    );
    if !(rc as libc::c_ulong <= capacity) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/common/common_ctl.c\0"
                    as *const u8 as *const libc::c_char,
                33 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"ctl_derive_path\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc <= capacity\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_parse_info(
    filename: *const libc::c_char,
    creator_pid: *mut libc::c_int,
    sock_ref: *mut libc::c_long,
) -> bool { unsafe {
    if strlen(filename) <= strlen(b"ctl-\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if strncmp(
        filename,
        b"ctl-\0" as *const u8 as *const libc::c_char,
        strlen(b"ctl-\0" as *const u8 as *const libc::c_char),
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    let pid_start: *const libc::c_char = filename
        .offset(strlen(b"ctl-\0" as *const u8 as *const libc::c_char) as isize);
    let mut end_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let cpid: libc::c_int = strtol(pid_start, &mut end_ptr, 10 as libc::c_int) as libc::c_int;
    if end_ptr == pid_start as *mut libc::c_char {
        return 0 as libc::c_int != 0;
    }
    if *end_ptr.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        return 0 as libc::c_int != 0;
    }
    let sref_start: *const libc::c_char = end_ptr.offset(1 as libc::c_int as isize);
    let sref: libc::c_long = strtoll(sref_start, &mut end_ptr, 10 as libc::c_int)
        as libc::c_long;
    if end_ptr == sref_start as *mut libc::c_char {
        return 0 as libc::c_int != 0;
    }
    if *end_ptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    *creator_pid = cpid;
    *sock_ref = sref;
    return 1 as libc::c_int != 0;
}}
