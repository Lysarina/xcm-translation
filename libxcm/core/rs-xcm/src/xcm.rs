#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]

use std::sync::atomic::{AtomicBool, Ordering};

unsafe extern "C" {
    pub type xcm_attr_map;
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    fn __errno_location() -> *mut libc::c_int;
    fn xcm_attr_map_create() -> *mut xcm_attr_map;
    fn xcm_attr_map_add_bool(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: bool,
    );
    fn xcm_attr_map_exists(
        attr_map: *const xcm_attr_map,
        attr_name: *const libc::c_char,
    ) -> bool;
    fn xcm_attr_map_foreach(
        attr_map: *const xcm_attr_map,
        cb: xcm_attr_map_foreach_cb,
        user: *mut libc::c_void,
    );
    fn xcm_attr_map_destroy(attr_map: *mut xcm_attr_map);
    fn xcm_tp_socket_create(
        proto: *const xcm_tp_proto,
        type_0: xcm_socket_type,
        xpoll: *mut xpoll,
        enable_ctl: bool,
        auto_update: bool,
        is_blocking: bool,
    ) -> *mut xcm_socket;
    fn xcm_tp_socket_destroy(s: *mut xcm_socket);
    fn xcm_tp_socket_init(s: *mut xcm_socket, parent: *mut xcm_socket) -> libc::c_int;
    fn xcm_tp_socket_connect(
        s: *mut xcm_socket,
        remote_addr: *const libc::c_char,
    ) -> libc::c_int;
    fn xcm_tp_socket_server(
        s: *mut xcm_socket,
        local_addr: *const libc::c_char,
    ) -> libc::c_int;
    fn xcm_tp_socket_close(s: *mut xcm_socket);
    fn xcm_tp_socket_cleanup(s: *mut xcm_socket);
    fn xcm_tp_socket_accept(
        conn_s: *mut xcm_socket,
        server_s: *mut xcm_socket,
    ) -> libc::c_int;
    fn xcm_tp_socket_send(
        s: *mut xcm_socket,
        buf: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn xcm_tp_socket_receive(
        s: *mut xcm_socket,
        buf: *mut libc::c_void,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_tp_socket_update(s: *mut xcm_socket);
    fn xcm_tp_socket_finish(s: *mut xcm_socket) -> libc::c_int;
    fn xcm_tp_socket_is_bytestream(conn_s: *mut xcm_socket) -> bool;
    fn xcm_tp_socket_get_remote_addr(
        conn_s: *mut xcm_socket,
        suppress_tracing: bool,
    ) -> *const libc::c_char;
    fn xcm_tp_socket_get_local_addr(
        s: *mut xcm_socket,
        suppress_tracing: bool,
    ) -> *const libc::c_char;
    fn xcm_tp_socket_attr_populate(s: *mut xcm_socket, attr_tree: *mut attr_tree);
    fn xcm_tp_common_attr_populate(s: *mut xcm_socket, attr_tree: *mut attr_tree);
    fn xcm_tp_proto_by_addr(addr: *const libc::c_char) -> *mut xcm_tp_proto;
    fn attr_tree_create() -> *mut attr_tree;
    fn attr_tree_destroy(tree: *mut attr_tree);
    fn attr_tree_set_value(
        tree: *mut attr_tree,
        path: *const libc::c_char,
        type_0: xcm_attr_type,
        value: *const libc::c_void,
        len: size_t,
        log_ref: *mut libc::c_void,
    ) -> libc::c_int;
    fn attr_tree_get_value(
        tree: *mut attr_tree,
        path: *const libc::c_char,
        type_0: *mut xcm_attr_type,
        value: *mut libc::c_void,
        capacity: size_t,
        log_ref: *mut libc::c_void,
    ) -> libc::c_int;
    fn attr_tree_get_list_len(
        tree: *mut attr_tree,
        path: *const libc::c_char,
        log_ref: *mut libc::c_void,
    ) -> libc::c_int;
    fn attr_tree_get_all(
        tree: *mut attr_tree,
        cb: xcm_attr_cb,
        cb_data: *mut libc::c_void,
    );
    fn xpoll_create(log_ref: *mut libc::c_void) -> *mut xpoll;
    fn xpoll_destroy(xpoll: *mut xpoll);
    fn xpoll_get_fd(xpoll: *mut xpoll) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ut_free(ptr: *mut libc::c_void);
    fn ut_vasprintf(
        fmt: *const libc::c_char,
        ap: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
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
    fn tp_so_condition_name(condition: libc::c_int) -> *const libc::c_char;
    fn xcm_version() -> *const libc::c_char;
    fn xcm_version_api() -> *const libc::c_char;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type xcm_attr_type = libc::c_uint;
pub const xcm_attr_type_double: xcm_attr_type = 5;
pub const xcm_attr_type_bin: xcm_attr_type = 4;
pub const xcm_attr_type_str: xcm_attr_type = 3;
pub const xcm_attr_type_int64: xcm_attr_type = 2;
pub const xcm_attr_type_bool: xcm_attr_type = 1;
pub type uint64_t = __uint64_t;
pub type xcm_attr_map_foreach_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_attr_state {
    pub s: *mut xcm_socket,
    pub rc: libc::c_int,
}
pub type va_list = __builtin_va_list;
pub type xcm_attr_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
// static mut version_logged: bool = 0 as libc::c_int != 0;
static version_logged: AtomicBool = AtomicBool::new(false);
unsafe extern "C" fn assure_library_version_logged() {
    if !version_logged.load(Ordering::Relaxed) {
    // if !::core::intrinsics::atomic_load_relaxed(&mut version_logged as *mut bool) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                45 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"assure_library_version_logged\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"XCM library version %s (API %s).\0" as *const u8
                    as *const libc::c_char,
                xcm_version(),
                xcm_version_api(),
            );
        }
        version_logged.store(true, Ordering::Relaxed);
        // ::core::intrinsics::atomic_store_relaxed(
        //     &mut version_logged,
        //     1 as libc::c_int != 0,
        // );
    }
}
unsafe extern "C" fn await_0(mut s: *mut xcm_socket, mut condition: libc::c_int) {
    (*s).condition = condition;
    xcm_tp_socket_update(s);
}
unsafe extern "C" fn socket_wait(
    mut conn_s: *mut xcm_socket,
    mut condition: libc::c_int,
) -> libc::c_int {
    await_0(conn_s, condition);
    let mut pfd: pollfd = {
        let mut init = pollfd {
            fd: xpoll_get_fd((*conn_s).xpoll),
            events: 0x1 as libc::c_int as libc::c_short,
            revents: 0,
        };
        init
    };
    let mut rc: libc::c_int = poll(
        &mut pfd,
        1 as libc::c_int as nfds_t,
        -(1 as libc::c_int),
    );
    return if rc > 0 as libc::c_int { 0 as libc::c_int } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn socket_finish(mut s: *mut xcm_socket) -> libc::c_int {
    let mut f_rc: libc::c_int = 0;
    loop {
        f_rc = xcm_tp_socket_finish(s);
        if !(f_rc < 0 as libc::c_int
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 115 as libc::c_int))
        {
            break;
        }
        if socket_wait(s, 0 as libc::c_int) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return f_rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_connect(
    mut remote_addr: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut xcm_socket {
    let mut attrs: *mut xcm_attr_map = 0 as *mut xcm_attr_map;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        attrs = xcm_attr_map_create();
        xcm_attr_map_add_bool(
            attrs,
            b"xcm.blocking\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
    }
    let mut conn: *mut xcm_socket = xcm_connect_a(remote_addr, attrs);
    xcm_attr_map_destroy(attrs);
    return conn;
}
unsafe extern "C" fn set_default_attrs(
    mut s: *mut xcm_socket,
    mut parent_s: *mut xcm_socket,
    mut attrs: *const xcm_attr_map,
) -> libc::c_int {
    if attrs.is_null()
        || !xcm_attr_map_exists(
            attrs,
            b"xcm.service\0" as *const u8 as *const libc::c_char,
        )
    {
        let mut bytestream: bool = 0 as libc::c_int != 0;
        if !parent_s.is_null() {
            bytestream = xcm_tp_socket_is_bytestream(parent_s);
        }
        if xcm_attr_set_str(
            s,
            b"xcm.service\0" as *const u8 as *const libc::c_char,
            (if bytestream as libc::c_int != 0 {
                b"bytestream\0" as *const u8 as *const libc::c_char
            } else {
                b"messaging\0" as *const u8 as *const libc::c_char
            }),
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_attr_cb(
    mut attr_name: *const libc::c_char,
    mut attr_type: xcm_attr_type,
    mut attr_value: *const libc::c_void,
    mut attr_value_len: size_t,
    mut user: *mut libc::c_void,
) {
    let mut state: *mut set_attr_state = user as *mut set_attr_state;
    if (*state).rc != 0 as libc::c_int {
        return;
    }
    (*state)
        .rc = xcm_attr_set((*state).s, attr_name, attr_type, attr_value, attr_value_len);
}
unsafe extern "C" fn set_user_attrs(
    mut s: *mut xcm_socket,
    mut attrs: *const xcm_attr_map,
) -> libc::c_int {
    if attrs.is_null() {
        return 0 as libc::c_int;
    }
    let mut state: set_attr_state = {
        let mut init = set_attr_state { s: s, rc: 0 };
        init
    };
    xcm_attr_map_foreach(
        attrs,
        Some(
            set_attr_cb
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    xcm_attr_type,
                    *const libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        &mut state as *mut set_attr_state as *mut libc::c_void,
    );
    return state.rc;
}
unsafe extern "C" fn set_attrs(
    mut s: *mut xcm_socket,
    mut parent_s: *mut xcm_socket,
    mut attrs: *const xcm_attr_map,
) -> libc::c_int {
    if set_default_attrs(s, parent_s, attrs) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if set_user_attrs(s, attrs) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn socket_create(
    mut proto: *const xcm_tp_proto,
    mut type_0: xcm_socket_type,
    mut is_blocking: bool,
) -> *mut xcm_socket {
    let mut s: *mut xcm_socket = xcm_tp_socket_create(
        proto,
        type_0,
        0 as *mut xpoll,
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
        is_blocking,
    );
    let mut xpoll: *mut xpoll = xpoll_create(s as *mut libc::c_void);
    if xpoll.is_null() {
        xcm_tp_socket_destroy(s);
        return 0 as *mut xcm_socket;
    }
    (*s).xpoll = xpoll;
    return s;
}
unsafe extern "C" fn socket_destroy(mut s: *mut xcm_socket) {
    if !s.is_null() {
        let mut xpoll: *mut xpoll = (*s).xpoll;
        xcm_tp_socket_destroy(s);
        xpoll_destroy(xpoll);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_connect_a(
    mut remote_addr: *const libc::c_char,
    mut attrs: *const xcm_attr_map,
) -> *mut xcm_socket {
    let mut current_block: u64;
    assure_library_version_logged();
    let mut proto: *const xcm_tp_proto = xcm_tp_proto_by_addr(remote_addr);
    if proto.is_null() {
        return 0 as *mut xcm_socket;
    }
    let mut s: *mut xcm_socket = socket_create(
        proto,
        xcm_socket_type_conn,
        1 as libc::c_int != 0,
    );
    if !s.is_null() {
        if !(xcm_tp_socket_init(s, 0 as *mut xcm_socket) < 0 as libc::c_int) {
            if set_attrs(s, 0 as *mut xcm_socket, attrs) < 0 as libc::c_int {
                current_block = 12691915414181357882;
            } else if xcm_tp_socket_connect(s, remote_addr) < 0 as libc::c_int {
                current_block = 7471077250034626850;
            } else {
                if (*s).is_blocking as libc::c_int != 0
                    && socket_finish(s) < 0 as libc::c_int
                {
                    if log_is_enabled(log_type_debug) {
                        __log_event(
                            log_type_debug,
                            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                                as *const u8 as *const libc::c_char,
                            213 as libc::c_int,
                            (*::core::mem::transmute::<
                                &[u8; 14],
                                &[libc::c_char; 14],
                            >(b"xcm_connect_a\0"))
                                .as_ptr(),
                            s,
                            b"Failed to establish connection; errno %d (%s).\0"
                                as *const u8 as *const libc::c_char,
                            *__errno_location(),
                            strerror(*__errno_location()),
                        );
                    }
                } else {
                    return s
                }
                current_block = 12691915414181357882;
            }
            match current_block {
                7471077250034626850 => {}
                _ => {
                    xcm_tp_socket_close(s);
                }
            }
        }
        socket_destroy(s);
    }
    return 0 as *mut xcm_socket;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_server(
    mut local_addr: *const libc::c_char,
) -> *mut xcm_socket {
    return xcm_server_a(local_addr, 0 as *const xcm_attr_map);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_server_a(
    mut local_addr: *const libc::c_char,
    mut attrs: *const xcm_attr_map,
) -> *mut xcm_socket {
    let mut s: *mut xcm_socket = 0 as *mut xcm_socket;
    assure_library_version_logged();
    let mut proto: *const xcm_tp_proto = xcm_tp_proto_by_addr(local_addr);
    if !proto.is_null() {
        s = socket_create(proto, xcm_socket_type_server, 1 as libc::c_int != 0);
        if !s.is_null() {
            if !(xcm_tp_socket_init(s, 0 as *mut xcm_socket) < 0 as libc::c_int) {
                if set_attrs(s, 0 as *mut xcm_socket, attrs) < 0 as libc::c_int {
                    xcm_tp_socket_close(s);
                } else if !(xcm_tp_socket_server(s, local_addr) < 0 as libc::c_int) {
                    return s
                }
            }
            socket_destroy(s);
        }
    }
    return 0 as *mut xcm_socket;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_close(mut s: *mut xcm_socket) -> libc::c_int {
    if !s.is_null() {
        xcm_tp_socket_close(s);
        socket_destroy(s);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_cleanup(mut s: *mut xcm_socket) {
    if !s.is_null() {
        xcm_tp_socket_cleanup(s);
        socket_destroy(s);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_accept(mut server_s: *mut xcm_socket) -> *mut xcm_socket {
    return xcm_accept_a(server_s, 0 as *const xcm_attr_map);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_accept_a(
    mut server_s: *mut xcm_socket,
    mut attrs: *const xcm_attr_map,
) -> *mut xcm_socket {
    let mut current_block: u64;
    if (*server_s).type_0 as libc::c_uint
        != xcm_socket_type_server as libc::c_int as libc::c_uint
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                290 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"xcm_accept_a\0"))
                    .as_ptr(),
                server_s,
                b"Operation failed; socket is of the wrong type.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut xcm_socket;
    }
    let mut is_blocking: bool = (*server_s).is_blocking;
    let mut conn_s: *mut xcm_socket = 0 as *mut xcm_socket;
    loop {
        conn_s = socket_create(
            (*server_s).proto,
            xcm_socket_type_conn,
            (*server_s).is_blocking,
        );
        if conn_s.is_null() {
            current_block = 14252043271018405008;
            break;
        }
        if is_blocking as libc::c_int != 0
            && socket_wait(server_s, (1 as libc::c_int) << 2 as libc::c_int)
                < 0 as libc::c_int
        {
            current_block = 14418216915952698982;
            break;
        }
        if xcm_tp_socket_init(conn_s, server_s) < 0 as libc::c_int {
            current_block = 14418216915952698982;
            break;
        }
        if set_attrs(conn_s, server_s, attrs) < 0 as libc::c_int {
            current_block = 11449320154384187214;
            break;
        }
        if xcm_tp_socket_accept(conn_s, server_s) < 0 as libc::c_int {
            if !(is_blocking as libc::c_int != 0
                && *__errno_location() == 11 as libc::c_int)
            {
                current_block = 14418216915952698982;
                break;
            }
            socket_destroy(conn_s);
        } else if is_blocking as libc::c_int != 0
            && socket_finish(conn_s) < 0 as libc::c_int
        {
            current_block = 11449320154384187214;
            break;
        } else {
            current_block = 13242334135786603907;
            break;
        }
    }
    match current_block {
        11449320154384187214 => {
            xcm_tp_socket_close(conn_s);
            current_block = 14418216915952698982;
        }
        13242334135786603907 => return conn_s,
        _ => {}
    }
    match current_block {
        14418216915952698982 => {
            socket_destroy(conn_s);
        }
        _ => {}
    }
    return 0 as *mut xcm_socket;
}
unsafe extern "C" fn bytestream_bsend(
    mut conn_s: *mut xcm_socket,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut sent: libc::c_int = 0 as libc::c_int;
    loop {
        let mut left: libc::c_int = len.wrapping_sub(sent as libc::c_ulong)
            as libc::c_int;
        let mut rc: libc::c_int = xcm_tp_socket_send(
            conn_s,
            buf.offset(sent as isize),
            left as size_t,
        );
        if rc < 0 as libc::c_int {
            if *__errno_location() != 11 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if socket_wait(conn_s, (1 as libc::c_int) << 1 as libc::c_int)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        } else {
            sent += rc;
        }
        if !((sent as libc::c_ulong) < len) {
            break;
        }
    }
    return sent;
}
unsafe extern "C" fn msg_bsend(
    mut conn_s: *mut xcm_socket,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    loop {
        let mut s_rc: libc::c_int = xcm_tp_socket_send(conn_s, buf, len);
        if s_rc < 0 as libc::c_int {
            if *__errno_location() != 11 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if socket_wait(conn_s, (1 as libc::c_int) << 1 as libc::c_int)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        } else {
            return 0 as libc::c_int
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_send(
    mut conn_s: *mut xcm_socket,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    if (*conn_s).type_0 as libc::c_uint
        != xcm_socket_type_conn as libc::c_int as libc::c_uint
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                369 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"xcm_send\0"))
                    .as_ptr(),
                conn_s,
                b"Operation failed; socket is of the wrong type.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*conn_s).is_blocking {
        let mut rc: libc::c_int = 0;
        if xcm_tp_socket_is_bytestream(conn_s) {
            rc = bytestream_bsend(conn_s, buf, len);
        } else {
            rc = msg_bsend(conn_s, buf, len);
        }
        if rc >= 0 as libc::c_int && socket_finish(conn_s) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        return rc;
    } else {
        return xcm_tp_socket_send(conn_s, buf, len)
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_receive(
    mut conn_s: *mut xcm_socket,
    mut buf: *mut libc::c_void,
    mut capacity: size_t,
) -> libc::c_int {
    if (*conn_s).type_0 as libc::c_uint
        != xcm_socket_type_conn as libc::c_int as libc::c_uint
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                389 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 12],
                    &[libc::c_char; 12],
                >(b"xcm_receive\0"))
                    .as_ptr(),
                conn_s,
                b"Operation failed; socket is of the wrong type.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*conn_s).is_blocking {
        loop {
            if socket_wait(conn_s, (1 as libc::c_int) << 0 as libc::c_int)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            let mut s_rc: libc::c_int = xcm_tp_socket_receive(conn_s, buf, capacity);
            if s_rc >= 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                return s_rc;
            }
        }
    } else {
        return xcm_tp_socket_receive(conn_s, buf, capacity)
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_await(
    mut s: *mut xcm_socket,
    mut condition: libc::c_int,
) -> libc::c_int {
    if (*s).is_blocking {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"xcm_await\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Operation failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                22 as libc::c_int,
                strerror(22 as libc::c_int),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if if (*s).type_0 as libc::c_uint
        == xcm_socket_type_server as libc::c_int as libc::c_uint
    {
        if condition & !((1 as libc::c_int) << 2 as libc::c_int) != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else if condition
        & !((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    } == 0
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                407 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"xcm_await\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Operation failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                22 as libc::c_int,
                strerror(22 as libc::c_int),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*s).condition != condition {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                409 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"xcm_await\0"))
                    .as_ptr(),
                s,
                b"Await called; condition changed from %s to %s.\0" as *const u8
                    as *const libc::c_char,
                tp_so_condition_name((*s).condition),
                tp_so_condition_name(condition),
            );
        }
    } else if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                as *const u8 as *const libc::c_char,
            409 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"xcm_await\0"))
                .as_ptr(),
            s,
            b"Await called but condition remains %s.\0" as *const u8
                as *const libc::c_char,
            tp_so_condition_name((*s).condition),
        );
    }
    await_0(s, condition);
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_fd(mut s: *mut xcm_socket) -> libc::c_int {
    if (*s).is_blocking {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                418 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"xcm_fd\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Operation failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                22 as libc::c_int,
                strerror(22 as libc::c_int),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return xpoll_get_fd((*s).xpoll);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_finish(mut s: *mut xcm_socket) -> libc::c_int {
    if (*s).is_blocking {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                425 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"xcm_finish\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Operation failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                22 as libc::c_int,
                strerror(22 as libc::c_int),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = xcm_tp_socket_finish(s);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_set_blocking(
    mut s: *mut xcm_socket,
    mut should_block: bool,
) -> libc::c_int {
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                as *const u8 as *const libc::c_char,
            434 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"xcm_set_blocking\0"))
                .as_ptr(),
            s,
            b"Received request to put socket into %s mode.\0" as *const u8
                as *const libc::c_char,
            if should_block as libc::c_int != 0 {
                b"blocking\0" as *const u8 as *const libc::c_char
            } else {
                b"non-blocking\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if (*s).is_blocking as libc::c_int == should_block as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                437 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"xcm_set_blocking\0"))
                    .as_ptr(),
                s,
                b"Mode unchanged.\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if !(*s).is_blocking {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                444 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"xcm_set_blocking\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"When switching from non-blocking mode to blocking; making sure to finish any outstanding work.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if socket_finish(s) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                as *const u8 as *const libc::c_char,
            449 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"xcm_set_blocking\0"))
                .as_ptr(),
            s,
            b"Mode changed.\0" as *const u8 as *const libc::c_char,
        );
    }
    (*s).is_blocking = should_block;
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_is_blocking(mut s: *mut xcm_socket) -> bool {
    return (*s).is_blocking;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_remote_addr(
    mut conn_s: *mut xcm_socket,
) -> *const libc::c_char {
    if (*conn_s).type_0 as libc::c_uint
        != xcm_socket_type_conn as libc::c_int as libc::c_uint
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                463 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"xcm_remote_addr\0"))
                    .as_ptr(),
                conn_s,
                b"Operation failed; socket is of the wrong type.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return 0 as *const libc::c_char;
    }
    return xcm_tp_socket_get_remote_addr(conn_s, 0 as libc::c_int != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_local_addr(mut s: *mut xcm_socket) -> *const libc::c_char {
    return xcm_tp_socket_get_local_addr(s, 0 as libc::c_int != 0);
}
unsafe extern "C" fn build_attr_tree(mut s: *mut xcm_socket) -> *mut attr_tree {
    let mut attr_tree: *mut attr_tree = attr_tree_create();
    xcm_tp_common_attr_populate(s, attr_tree);
    xcm_tp_socket_attr_populate(s, attr_tree);
    return attr_tree;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut type_0: xcm_attr_type,
    mut value: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut attr_tree: *mut attr_tree = build_attr_tree(s);
    let mut rc: libc::c_int = attr_tree_set_value(
        attr_tree,
        name,
        type_0,
        value,
        len,
        s as *mut libc::c_void,
    );
    attr_tree_destroy(attr_tree);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_bool(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: bool,
) -> libc::c_int {
    return xcm_attr_set(
        s,
        name,
        xcm_attr_type_bool,
        &mut value as *mut bool as *const libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_int64(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: int64_t,
) -> libc::c_int {
    return xcm_attr_set(
        s,
        name,
        xcm_attr_type_int64,
        &mut value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_double(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: libc::c_double,
) -> libc::c_int {
    return xcm_attr_set(
        s,
        name,
        xcm_attr_type_double,
        &mut value as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_str(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    return xcm_attr_set(
        s,
        name,
        xcm_attr_type_str,
        value as *const libc::c_void,
        (strlen(value)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut type_0: *mut xcm_attr_type,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
) -> libc::c_int {
    let mut attr_tree: *mut attr_tree = build_attr_tree(s);
    let mut rc: libc::c_int = attr_tree_get_value(
        attr_tree,
        name,
        type_0,
        value,
        capacity,
        s as *mut libc::c_void,
    );
    attr_tree_destroy(attr_tree);
    return rc;
}
unsafe extern "C" fn attr_get_with_type(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut required_type: xcm_attr_type,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
) -> libc::c_int {
    let mut actual_type: xcm_attr_type = 0 as xcm_attr_type;
    let mut rc: libc::c_int = xcm_attr_get(s, name, &mut actual_type, value, capacity);
    if rc < 0 as libc::c_int {
        if *__errno_location() == 75 as libc::c_int {
            *__errno_location() = 2 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if actual_type as libc::c_uint != required_type as libc::c_uint {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_bool(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: *mut bool,
) -> libc::c_int {
    return attr_get_with_type(
        s,
        name,
        xcm_attr_type_bool,
        value as *mut libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_int64(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: *mut int64_t,
) -> libc::c_int {
    return attr_get_with_type(
        s,
        name,
        xcm_attr_type_int64,
        value as *mut libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_double(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_double,
) -> libc::c_int {
    return attr_get_with_type(
        s,
        name,
        xcm_attr_type_double,
        value as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_str(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    let mut type_0: xcm_attr_type = 0 as xcm_attr_type;
    let mut rc: libc::c_int = xcm_attr_get(
        s,
        name,
        &mut type_0,
        value as *mut libc::c_void,
        capacity,
    );
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if type_0 as libc::c_uint != xcm_attr_type_str as libc::c_int as libc::c_uint {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_bin(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
) -> libc::c_int {
    let mut type_0: xcm_attr_type = 0 as xcm_attr_type;
    let mut rc: libc::c_int = xcm_attr_get(s, name, &mut type_0, value, capacity);
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if type_0 as libc::c_uint != xcm_attr_type_bin as libc::c_int as libc::c_uint {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_list_len(
    mut s: *mut xcm_socket,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut attr_tree: *mut attr_tree = build_attr_tree(s);
    let mut rc: libc::c_int = attr_tree_get_list_len(
        attr_tree,
        name,
        s as *mut libc::c_void,
    );
    attr_tree_destroy(attr_tree);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf(
    mut s: *mut xcm_socket,
    mut type_0: *mut xcm_attr_type,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
    mut name_fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut name: *mut libc::c_char = ut_vasprintf(name_fmt, ap.as_va_list());
    let mut rc: libc::c_int = xcm_attr_get(s, name, type_0, value, capacity);
    ut_free(name as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn attr_vgetf_with_type(
    mut s: *mut xcm_socket,
    mut required_type: xcm_attr_type,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
    mut name_fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut name: *mut libc::c_char = ut_vasprintf(name_fmt, ap.as_va_list());
    let mut rc: libc::c_int = attr_get_with_type(
        s,
        name,
        required_type,
        value,
        capacity,
    );
    ut_free(name as *mut libc::c_void);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_bool(
    mut s: *mut xcm_socket,
    mut value: *mut bool,
    mut name_fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_bool,
        value as *mut libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
        name_fmt,
        ap.as_va_list(),
    );
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_int64(
    mut s: *mut xcm_socket,
    mut value: *mut int64_t,
    mut name_fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_int64,
        value as *mut libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
        name_fmt,
        ap.as_va_list(),
    );
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_double(
    mut s: *mut xcm_socket,
    mut value: *mut libc::c_double,
    mut name_fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_double,
        value as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        name_fmt,
        ap.as_va_list(),
    );
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_str(
    mut s: *mut xcm_socket,
    mut value: *mut libc::c_char,
    mut capacity: size_t,
    mut name_fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_str,
        value as *mut libc::c_void,
        capacity,
        name_fmt,
        ap.as_va_list(),
    );
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_bin(
    mut s: *mut xcm_socket,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
    mut name_fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_bin,
        value,
        capacity,
        name_fmt,
        ap.as_va_list(),
    );
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_all(
    mut s: *mut xcm_socket,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) {
    let mut attr_tree: *mut attr_tree = build_attr_tree(s);
    attr_tree_get_all(attr_tree, cb, cb_data);
    attr_tree_destroy(attr_tree);
}
