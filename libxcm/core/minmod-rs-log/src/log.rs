#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(c_variadic, extern_types)]

use std::sync::atomic::{AtomicBool, Ordering};
unsafe extern "C" {
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn ut_gettid() -> pid_t;
    fn ut_vaprintf(
        buf: *mut libc::c_char,
        capacity: size_t,
        format: *const libc::c_char,
        ap: ::core::ffi::VaList,
    );
    fn ut_aprintf(
        buf: *mut libc::c_char,
        capacity: size_t,
        format: *const libc::c_char,
        _: ...
    );
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_socket {
    pub proto: *const xcm_tp_proto,
    pub type_0: xcm_socket_type,
    pub sock_id: int64_t,
    pub auto_enable_ctl: bool,
    pub auto_update: bool,
    pub is_blocking: bool,
    pub xpoll: *mut xpoll,
    pub condition: libc::c_int,
    pub ctl: *mut ctl,
    pub skipped_ctl_calls: uint64_t,
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub type xcm_socket_type = libc::c_uint;
pub const xcm_socket_type_server: xcm_socket_type = 1;
pub const xcm_socket_type_conn: xcm_socket_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_tp_proto {
    pub name: [libc::c_char; 33],
    pub ops: *const xcm_tp_ops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_tp_ops {
    pub init: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *mut xcm_socket) -> libc::c_int,
    >,
    pub connect: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *const libc::c_char) -> libc::c_int,
    >,
    pub server: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *const libc::c_char) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
    pub cleanup: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
    pub accept: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *mut xcm_socket) -> libc::c_int,
    >,
    pub send: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub receive: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *mut libc::c_void, size_t) -> libc::c_int,
    >,
    pub update: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
    pub finish: Option::<unsafe extern "C" fn(*mut xcm_socket) -> libc::c_int>,
    pub get_transport: Option::<
        unsafe extern "C" fn(*mut xcm_socket) -> *const libc::c_char,
    >,
    pub get_remote_addr: Option::<
        unsafe extern "C" fn(*mut xcm_socket, bool) -> *const libc::c_char,
    >,
    pub get_local_addr: Option::<
        unsafe extern "C" fn(*mut xcm_socket, bool) -> *const libc::c_char,
    >,
    pub set_local_addr: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *const libc::c_char) -> libc::c_int,
    >,
    pub max_msg: Option::<unsafe extern "C" fn(*mut xcm_socket) -> size_t>,
    pub get_cnt: Option::<unsafe extern "C" fn(*mut xcm_socket, xcm_tp_cnt) -> int64_t>,
    pub enable_ctl: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
    pub attr_populate: Option::<
        unsafe extern "C" fn(*mut xcm_socket, *mut attr_tree) -> (),
    >,
    pub priv_size: Option::<unsafe extern "C" fn(xcm_socket_type) -> size_t>,
}
pub type size_t = libc::c_ulong;
pub type xcm_tp_cnt = libc::c_uint;
pub const xcm_tp_cnt_from_lower_msgs: xcm_tp_cnt = 7;
pub const xcm_tp_cnt_to_lower_msgs: xcm_tp_cnt = 6;
pub const xcm_tp_cnt_from_app_msgs: xcm_tp_cnt = 5;
pub const xcm_tp_cnt_to_app_msgs: xcm_tp_cnt = 4;
pub const xcm_tp_cnt_from_lower_bytes: xcm_tp_cnt = 3;
pub const xcm_tp_cnt_to_lower_bytes: xcm_tp_cnt = 2;
pub const xcm_tp_cnt_from_app_bytes: xcm_tp_cnt = 1;
pub const xcm_tp_cnt_to_app_bytes: xcm_tp_cnt = 0;
pub type log_type = libc::c_uint;
pub const log_type_error: log_type = 1;
pub const log_type_debug: log_type = 0;
pub type va_list = __builtin_va_list;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type pid_t = __pid_t;
pub type __pid_t = libc::c_int;
unsafe extern "C" fn format_msg(
    buf: *mut libc::c_char,
    capacity: size_t,
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
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
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
        capacity,
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
    // if ::core::intrinsics::atomic_load_relaxed(&raw mut console_enabled) {
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
        fputs(buf.as_mut_ptr(), stderr);
        fflush(stderr);
        *__errno_location() = _oerrno;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_is_enabled(mut _type_0: log_type) -> bool {
    if console_enabled.load(Ordering::Relaxed) {
    // if ::core::intrinsics::atomic_load_relaxed(&raw mut console_enabled) {
        return 1 as libc::c_int != 0;
    }
    0 as libc::c_int != 0
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
    // ::core::intrinsics::atomic_store_relaxed(&console_enabled as *mut bool, enabled);
}
