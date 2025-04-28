#![allow(non_camel_case_types, non_upper_case_globals)]
#![feature(extern_types)]

pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;

pub type log_type = libc::c_uint;
pub const log_type_error: log_type = 1;
pub const log_type_debug: log_type = 0;

unsafe extern "C" {
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    pub fn __errno_location() -> *mut libc::c_int;
    pub fn xcm_set_blocking(socket: *mut xcm_socket, should_block: bool) -> libc::c_int;
    pub fn xcm_remote_addr(conn_socket: *mut xcm_socket) -> *const libc::c_char;
    pub fn xcm_local_addr(socket: *mut xcm_socket) -> *const libc::c_char;
    pub fn abort() -> !;
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    pub fn ut_calloc(size: size_t) -> *mut libc::c_void;
    pub fn ut_free(ptr: *mut libc::c_void);
    pub fn log_console_conf(enabled: bool);
    pub fn log_is_enabled(type_0: log_type) -> bool;
    pub fn __log_event(
        type_0: log_type,
        file: *const libc::c_char,
        line: libc::c_int,
        function: *const libc::c_char,
        s: *mut xcm_socket,
        format: *const libc::c_char,
        _: ...
    );
    pub fn xcm_addr_parse_proto(
        addr_s: *const libc::c_char,
        proto: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    pub fn ctl_create(socket: *mut xcm_socket) -> *mut ctl;
    pub fn ctl_destroy(ctl: *mut ctl, owner: bool);
    pub fn ctl_process(ctl: *mut ctl);
}

pub type xcm_attr_type = libc::c_uint;
pub const xcm_attr_type_double: xcm_attr_type = 5;
pub const xcm_attr_type_bin: xcm_attr_type = 4;
pub const xcm_attr_type_str: xcm_attr_type = 3;
pub const xcm_attr_type_int64: xcm_attr_type = 2;
pub const xcm_attr_type_bool: xcm_attr_type = 1;
pub type uint64_t = __uint64_t;
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


pub type attr_set = Option::<
    unsafe extern "C" fn(
        *mut xcm_socket,
        *mut libc::c_void,
        *const libc::c_void,
        size_t,
    ) -> libc::c_int,
>;
pub type attr_get = Option::<
    unsafe extern "C" fn(
        *mut xcm_socket,
        *mut libc::c_void,
        *mut libc::c_void,
        size_t,
    ) -> libc::c_int,
>;

pub type xcm_tp_cnt = libc::c_uint;
pub const xcm_tp_cnt_from_lower_msgs: xcm_tp_cnt = 7;
pub const xcm_tp_cnt_to_lower_msgs: xcm_tp_cnt = 6;
pub const xcm_tp_cnt_from_app_msgs: xcm_tp_cnt = 5;
pub const xcm_tp_cnt_to_app_msgs: xcm_tp_cnt = 4;
pub const xcm_tp_cnt_from_lower_bytes: xcm_tp_cnt = 3;
pub const xcm_tp_cnt_to_lower_bytes: xcm_tp_cnt = 2;
pub const xcm_tp_cnt_from_app_bytes: xcm_tp_cnt = 1;
pub const xcm_tp_cnt_to_app_bytes: xcm_tp_cnt = 0;