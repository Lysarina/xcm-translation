#![allow(non_camel_case_types, non_upper_case_globals)]
#![feature(extern_types)]

pub mod c_functions {
    unsafe extern "C" {

        pub unsafe fn abort() -> !;
        pub unsafe fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        pub unsafe fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        pub unsafe fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        pub unsafe fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        pub unsafe fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
        pub unsafe fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
        pub unsafe fn strerror(_: libc::c_int) -> *mut libc::c_char;

        pub unsafe fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;
    }
}

use crate::xcm_tp::xcm_socket;
unsafe extern "C" {
    pub type ctl;
    pub fn __errno_location() -> *mut libc::c_int;
    pub fn xcm_set_blocking(socket: *mut xcm_socket, should_block: bool) -> libc::c_int;
    pub fn xcm_remote_addr(conn_socket: *mut xcm_socket) -> *const libc::c_char;
    pub fn xcm_local_addr(socket: *mut xcm_socket) -> *const libc::c_char;

    
    pub fn xcm_addr_parse_proto(
        addr_s: *const libc::c_char,
        proto: *mut libc::c_char,
        capacity: libc::c_ulong,
    ) -> libc::c_int;
    pub fn ctl_create(socket: *mut xcm_socket) -> *mut ctl;
    pub fn ctl_destroy(ctl: *mut ctl, owner: bool);
    pub fn ctl_process(ctl: *mut ctl);
}

pub mod attr_node_mod {
    use super::*;
    use super::xcm_attr::*;
    pub type attr_node_type = libc::c_uint;
    pub const attr_node_type_list: attr_node_type = 2;
    pub const attr_node_type_dict: attr_node_type = 1;
    pub const attr_node_type_value: attr_node_type = 0;

    
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_node {
        pub type_0: attr_node_type,
        pub c2rust_unnamed: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed {
        pub value: attr_node_value,
        pub dict: attr_node_dict,
        pub list: attr_node_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_node_list {
        pub tqh_first: *mut attr_node_list_elem,
        pub tqh_last: *mut *mut attr_node_list_elem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_node_list_elem {
        pub node: *mut attr_node,
        pub entry: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_0 {
        pub tqe_next: *mut attr_node_list_elem,
        pub tqe_prev: *mut *mut attr_node_list_elem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_node_dict {
        pub tqh_first: *mut attr_node_dict_elem,
        pub tqh_last: *mut *mut attr_node_dict_elem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_node_dict_elem {
        pub key: *mut libc::c_char,
        pub node: *mut attr_node,
        pub entry: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_1 {
        pub tqe_next: *mut attr_node_dict_elem,
        pub tqe_prev: *mut *mut attr_node_dict_elem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_node_value {
        pub type_0: xcm_attr_type,
        pub s: *mut xcm_socket,
        pub context: *mut libc::c_void,
        pub set: attr_set,
        pub get: attr_get,
    }
    pub type attr_get = Option::<
        unsafe extern "C" fn(
            *mut xcm_socket,
            *mut libc::c_void,
            *mut libc::c_void,
            libc::c_ulong,
        ) -> libc::c_int,
    >;
    pub type attr_set = Option::<
        unsafe extern "C" fn(
            *mut xcm_socket,
            *mut libc::c_void,
            *const libc::c_void,
            libc::c_ulong,
        ) -> libc::c_int,
    >;
    pub type attr_dict_foreach_cb = Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut attr_node, *mut libc::c_void) -> (),
    >;
    pub type attr_list_foreach_cb = Option::<
        unsafe extern "C" fn(libc::c_ulong, *mut attr_node, *mut libc::c_void) -> (),
    >;
}

pub mod attr_tree_mod {
    use super::*;
    use super::xcm_attr::*;
    use super::attr_node_mod::*;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct attr_tree {
        pub root: *mut attr_node,
    }

    pub type xcm_attr_cb = Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            xcm_attr_type,
            *mut libc::c_void,
            libc::c_ulong,
            *mut libc::c_void,
        ) -> (),
    >;
}

pub mod xpoll_mod {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xpoll {
        pub epoll_fd: libc::c_int,
        pub fd_regs: *mut xpoll_fd_reg,
        pub fd_regs_capacity: libc::c_int,
        pub num_fd_regs: libc::c_int,
        pub active_fd: libc::c_int,
        pub active_fd_reg_id: libc::c_int,
        pub bell_regs: *mut xpoll_bell_reg,
        pub bell_regs_capacity: libc::c_int,
        pub num_bell_regs: libc::c_int,
        pub log_ref: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xpoll_bell_reg {
        pub free: bool,
        pub ringing: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xpoll_fd_reg {
        pub fd: libc::c_int,
        pub event: libc::c_int,
    }
}

pub mod xcm_attr {

    pub type xcm_attr_type = libc::c_uint;
    pub const xcm_attr_type_double: xcm_attr_type = 5;
    pub const xcm_attr_type_bin: xcm_attr_type = 4;
    pub const xcm_attr_type_str: xcm_attr_type = 3;
    pub const xcm_attr_type_int64: xcm_attr_type = 2;
    pub const xcm_attr_type_bool: xcm_attr_type = 1;
}

pub mod xcm_tp {

    use super::*;
    use super::attr_tree_mod::*;
    use super::xpoll_mod::xpoll;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xcm_socket {
        pub proto: *const xcm_tp_proto,
        pub type_0: xcm_socket_type,
        pub sock_id: libc::c_long,
        pub auto_enable_ctl: bool,
        pub auto_update: bool,
        pub is_blocking: bool,
        pub xpoll: *mut xpoll,
        pub condition: libc::c_int,
        pub ctl: *mut ctl,
        pub skipped_ctl_calls: libc::c_ulong,
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
            unsafe extern "C" fn(*mut xcm_socket, *const libc::c_void, libc::c_ulong) -> libc::c_int,
        >,
        pub receive: Option::<
            unsafe extern "C" fn(*mut xcm_socket, *mut libc::c_void, libc::c_ulong) -> libc::c_int,
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
        pub max_msg: Option::<unsafe extern "C" fn(*mut xcm_socket) -> libc::c_ulong>,
        pub get_cnt: Option::<unsafe extern "C" fn(*mut xcm_socket, xcm_tp_cnt) -> libc::c_long>,
        pub enable_ctl: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
        pub attr_populate: Option::<
            unsafe extern "C" fn(*mut xcm_socket, *mut attr_tree) -> (),
        >,
        pub priv_size: Option::<unsafe extern "C" fn(xcm_socket_type) -> libc::c_ulong>,
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

}

