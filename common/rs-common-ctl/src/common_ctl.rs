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

use rs_log;

unsafe extern "C" {
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
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
    fn log_console_conf(enabled: bool);
    fn log_is_enabled(type_0: log_type) -> bool;
    fn __log_event(
        type_0: log_type,
        file: *const libc::c_char,
        line: libc::c_int,
        function: *const libc::c_char,
        s: *mut xcm_socket,
        format: *const libc::c_char,
        _: ...
    );
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_get_dir(mut buf: *mut libc::c_char, mut capacity: size_t) {
    let mut env: *const libc::c_char = getenv(
        b"XCM_CTL\0" as *const u8 as *const libc::c_char,
    );
    if !env.is_null() && strlen(env) < capacity {
        strcpy(buf, env);
    } else {
        strcpy(buf, b"/run/xcm/ctl\0" as *const u8 as *const libc::c_char);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_derive_path(
    mut ctl_dir: *const libc::c_char,
    mut creator_pid: pid_t,
    mut sock_id: int64_t,
    mut buf: *mut libc::c_char,
    mut capacity: size_t,
) {
    let mut rc: libc::c_int = snprintf(
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
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_parse_info(
    mut filename: *const libc::c_char,
    mut creator_pid: *mut pid_t,
    mut sock_ref: *mut int64_t,
) -> bool {
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
    let mut pid_start: *const libc::c_char = filename
        .offset(strlen(b"ctl-\0" as *const u8 as *const libc::c_char) as isize);
    let mut end_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cpid: pid_t = strtol(pid_start, &mut end_ptr, 10 as libc::c_int) as pid_t;
    if end_ptr == pid_start as *mut libc::c_char {
        return 0 as libc::c_int != 0;
    }
    if *end_ptr.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        return 0 as libc::c_int != 0;
    }
    let mut sref_start: *const libc::c_char = end_ptr.offset(1 as libc::c_int as isize);
    let mut sref: int64_t = strtoll(sref_start, &mut end_ptr, 10 as libc::c_int)
        as int64_t;
    if end_ptr == sref_start as *mut libc::c_char {
        return 0 as libc::c_int != 0;
    }
    if *end_ptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    *creator_pid = cpid;
    *sock_ref = sref;
    return 1 as libc::c_int != 0;
}
