#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(c_variadic, extern_types)]

use std::sync::atomic::{AtomicBool, Ordering};
use libc::{__errno_location, strlen, strerror, poll, pollfd};

use xcm_rust_common::{xcm_attr::*, xpoll_mod::*, attr_tree_mod::*};
use xcm_rust_common::xcm_tp::{xcm_socket, xcm_socket_type, xcm_socket_type_conn,
    xcm_socket_type_server, xcm_tp_proto};
use rs_common_tp::*;
use rs_xpoll::*;
use rs_xcm_tp::*;
use rs_log::*;
use rs_util::*;
use rs_attr_tree::{attr_tree_destroy, attr_tree_get_all, attr_tree_get_list_len,
    attr_tree_get_value, attr_tree_set_value, attr_tree_create};
use rs_xcm_version::*;
use rs_xcm_attr_map::*;

unsafe extern "C" {
    
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_attr_state {
    pub s: *mut xcm_socket,
    pub rc: libc::c_int,
}
static version_logged: AtomicBool = AtomicBool::new(false);
unsafe extern "C" fn assure_library_version_logged() { unsafe {
    if !version_logged.load(Ordering::Relaxed) {
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
                std::ptr::null_mut::<xcm_socket>(),
                b"XCM library version %s (API %s).\0" as *const u8
                    as *const libc::c_char,
                xcm_version(),
                xcm_version_api(),
            );
        }
        version_logged.store(true, Ordering::Relaxed);
    }
}}
unsafe extern "C" fn await_0(s: *mut xcm_socket, condition: libc::c_int) { unsafe {
    (*s).condition = condition;
    xcm_tp_socket_update(s);
}}
unsafe extern "C" fn socket_wait(
    conn_s: *mut xcm_socket,
    condition: libc::c_int,
) -> libc::c_int { unsafe {
    await_0(conn_s, condition);
    let mut pfd: pollfd = {
        
        pollfd {
            fd: xpoll_get_fd((*conn_s).xpoll),
            events: 0x1 as libc::c_int as libc::c_short,
            revents: 0,
        }
    };
    let rc: libc::c_int = poll(
        &mut pfd,
        1 as libc::c_int as libc::c_ulong,
        -(1 as libc::c_int),
    );
    if rc > 0 as libc::c_int { 0 as libc::c_int } else { -(1 as libc::c_int) }
}}
unsafe extern "C" fn socket_finish(s: *mut xcm_socket) -> libc::c_int { unsafe {
    let mut f_rc: libc::c_int;
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
    f_rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_connect(
    remote_addr: *const libc::c_char,
    flags: libc::c_int,
) -> *mut xcm_socket { unsafe {
    let mut attrs: *mut xcm_attr_map = std::ptr::null_mut::<xcm_attr_map>();
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        attrs = xcm_attr_map_create();
        xcm_attr_map_add_bool(
            attrs,
            b"xcm.blocking\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
    }
    let conn: *mut xcm_socket = xcm_connect_a(remote_addr, attrs);
    xcm_attr_map_destroy(attrs);
    conn
}}
unsafe extern "C" fn set_default_attrs(
    s: *mut xcm_socket,
    parent_s: *mut xcm_socket,
    attrs: *const xcm_attr_map,
) -> libc::c_int { unsafe {
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
            if bytestream as libc::c_int != 0 {
                b"bytestream\0" as *const u8 as *const libc::c_char
            } else {
                b"messaging\0" as *const u8 as *const libc::c_char
            },
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    0 as libc::c_int
}}
unsafe extern "C" fn set_attr_cb(
    attr_name: *const libc::c_char,
    attr_type: xcm_attr_type,
    attr_value: *const libc::c_void,
    attr_value_len: libc::c_ulong,
    user: *mut libc::c_void,
) { unsafe {
    let state: *mut set_attr_state = user as *mut set_attr_state;
    if (*state).rc != 0 as libc::c_int {
        return;
    }
    (*state)
        .rc = xcm_attr_set((*state).s, attr_name, attr_type, attr_value, attr_value_len);
}}
unsafe extern "C" fn set_user_attrs(
    s: *mut xcm_socket,
    attrs: *const xcm_attr_map,
) -> libc::c_int { unsafe {
    if attrs.is_null() {
        return 0 as libc::c_int;
    }
    let mut state: set_attr_state = {
        
        set_attr_state { s, rc: 0 }
    };
    xcm_attr_map_foreach(
        attrs,
        Some(
            set_attr_cb
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    xcm_attr_type,
                    *const libc::c_void,
                    libc::c_ulong,
                    *mut libc::c_void,
                ) -> (),
        ),
        &mut state as *mut set_attr_state as *mut libc::c_void,
    );
    state.rc
}}
unsafe extern "C" fn set_attrs(
    s: *mut xcm_socket,
    parent_s: *mut xcm_socket,
    attrs: *const xcm_attr_map,
) -> libc::c_int { unsafe {
    if set_default_attrs(s, parent_s, attrs) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if set_user_attrs(s, attrs) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
unsafe extern "C" fn socket_create(
    proto: *const xcm_tp_proto,
    type_0: xcm_socket_type,
    is_blocking: bool,
) -> *mut xcm_socket { unsafe {
    let s: *mut xcm_socket = xcm_tp_socket_create(
        proto,
        type_0,
        std::ptr::null_mut::<xpoll>(),
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
        is_blocking,
    );
    let xpoll: *mut xpoll = xpoll_create(s as *mut libc::c_void);
    if xpoll.is_null() {
        xcm_tp_socket_destroy(s);
        return std::ptr::null_mut::<xcm_socket>();
    }
    (*s).xpoll = xpoll;
    s
}}
unsafe extern "C" fn socket_destroy(s: *mut xcm_socket) { unsafe {
    if !s.is_null() {
        let xpoll: *mut xpoll = (*s).xpoll;
        xcm_tp_socket_destroy(s);
        xpoll_destroy(xpoll);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_connect_a(
    remote_addr: *const libc::c_char,
    attrs: *const xcm_attr_map,
) -> *mut xcm_socket { unsafe {
    let current_block: u64;
    assure_library_version_logged();
    let proto: *const xcm_tp_proto = xcm_tp_proto_by_addr(remote_addr);
    if proto.is_null() {
        return std::ptr::null_mut::<xcm_socket>();
    }
    let s: *mut xcm_socket = socket_create(
        proto,
        xcm_socket_type_conn,
        1 as libc::c_int != 0,
    );
    if !s.is_null() {
        if xcm_tp_socket_init(s, std::ptr::null_mut::<xcm_socket>()) >= 0 as libc::c_int {
            if set_attrs(s, std::ptr::null_mut::<xcm_socket>(), attrs) < 0 as libc::c_int {
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
    std::ptr::null_mut::<xcm_socket>()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_server(
    local_addr: *const libc::c_char,
) -> *mut xcm_socket { unsafe {
    xcm_server_a(local_addr, std::ptr::null::<xcm_attr_map>())
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_server_a(
    local_addr: *const libc::c_char,
    attrs: *const xcm_attr_map,
) -> *mut xcm_socket { unsafe {
    assure_library_version_logged();
    let proto: *const xcm_tp_proto = xcm_tp_proto_by_addr(local_addr);
    if !proto.is_null() {
        let s = socket_create(proto, xcm_socket_type_server, 1 as libc::c_int != 0);
        if !s.is_null() {
            if xcm_tp_socket_init(s, std::ptr::null_mut::<xcm_socket>()) >= 0 as libc::c_int {
                if set_attrs(s, std::ptr::null_mut::<xcm_socket>(), attrs) < 0 as libc::c_int {
                    xcm_tp_socket_close(s);
                } else if xcm_tp_socket_server(s, local_addr) >= 0 as libc::c_int {
                    return s
                }
            }
            socket_destroy(s);
        }
    }
    std::ptr::null_mut::<xcm_socket>()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_close(s: *mut xcm_socket) -> libc::c_int { unsafe {
    if !s.is_null() {
        xcm_tp_socket_close(s);
        socket_destroy(s);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_cleanup(s: *mut xcm_socket) { unsafe {
    if !s.is_null() {
        xcm_tp_socket_cleanup(s);
        socket_destroy(s);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_accept(server_s: *mut xcm_socket) -> *mut xcm_socket { unsafe {
    xcm_accept_a(server_s, std::ptr::null::<xcm_attr_map>())
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_accept_a(
    server_s: *mut xcm_socket,
    attrs: *const xcm_attr_map,
) -> *mut xcm_socket { unsafe {
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
        return std::ptr::null_mut::<xcm_socket>();
    }
    let is_blocking: bool = (*server_s).is_blocking;
    let mut conn_s: *mut xcm_socket; // = std::ptr::null_mut::<xcm_socket>()
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
    if current_block == 14418216915952698982 {
        socket_destroy(conn_s);
    }
    std::ptr::null_mut::<xcm_socket>()
}}
unsafe extern "C" fn bytestream_bsend(
    conn_s: *mut xcm_socket,
    buf: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut sent: libc::c_int = 0 as libc::c_int;
    loop {
        let left: libc::c_int = len.wrapping_sub(sent as libc::c_ulong)
            as libc::c_int;
        let rc: libc::c_int = xcm_tp_socket_send(
            conn_s,
            buf.offset(sent as isize),
            left as libc::c_ulong,
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
        if (sent as libc::c_ulong) >= len {
            break;
        }
    }
    sent
}}
unsafe extern "C" fn msg_bsend(
    conn_s: *mut xcm_socket,
    buf: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
    loop {
        let s_rc: libc::c_int = xcm_tp_socket_send(conn_s, buf, len);
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_send(
    conn_s: *mut xcm_socket,
    buf: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
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
        let rc: libc::c_int = if xcm_tp_socket_is_bytestream(conn_s) {
            bytestream_bsend(conn_s, buf, len)
        } else {
            msg_bsend(conn_s, buf, len)
        };
        if rc >= 0 as libc::c_int && socket_finish(conn_s) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        rc
    } else {
        xcm_tp_socket_send(conn_s, buf, len)
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_receive(
    conn_s: *mut xcm_socket,
    buf: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
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
            let s_rc: libc::c_int = xcm_tp_socket_receive(conn_s, buf, capacity);
            if s_rc >= 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                return s_rc;
            }
        }
    } else {
        xcm_tp_socket_receive(conn_s, buf, capacity)
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_await(
    s: *mut xcm_socket,
    condition: libc::c_int,
) -> libc::c_int { unsafe {
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
                std::ptr::null_mut::<xcm_socket>(),
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
                std::ptr::null_mut::<xcm_socket>(),
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
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_fd(s: *mut xcm_socket) -> libc::c_int { unsafe {
    if (*s).is_blocking {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm.c\0"
                    as *const u8 as *const libc::c_char,
                418 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"xcm_fd\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Operation failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                22 as libc::c_int,
                strerror(22 as libc::c_int),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    xpoll_get_fd((*s).xpoll)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_finish(s: *mut xcm_socket) -> libc::c_int { unsafe {
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Operation failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                22 as libc::c_int,
                strerror(22 as libc::c_int),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let rc: libc::c_int = xcm_tp_socket_finish(s);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_set_blocking(
    s: *mut xcm_socket,
    should_block: bool,
) -> libc::c_int { unsafe {
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
                std::ptr::null_mut::<xcm_socket>(),
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
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_is_blocking(s: *mut xcm_socket) -> bool { unsafe {
    (*s).is_blocking
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_remote_addr(
    conn_s: *mut xcm_socket,
) -> *const libc::c_char { unsafe {
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
        return std::ptr::null::<libc::c_char>();
    }
    xcm_tp_socket_get_remote_addr(conn_s, 0 as libc::c_int != 0)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_local_addr(s: *mut xcm_socket) -> *const libc::c_char { unsafe {
    xcm_tp_socket_get_local_addr(s, 0 as libc::c_int != 0)
}}
unsafe extern "C" fn build_attr_tree(s: *mut xcm_socket) -> *mut attr_tree { unsafe {
    let attr_tree: *mut attr_tree = attr_tree_create();
    xcm_tp_common_attr_populate(s, attr_tree);
    xcm_tp_socket_attr_populate(s, attr_tree);
    attr_tree
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    type_0: xcm_attr_type,
    value: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
    let attr_tree: *mut attr_tree = build_attr_tree(s);
    let rc: libc::c_int = attr_tree_set_value(
        attr_tree,
        name,
        type_0,
        value,
        len,
        s as *mut libc::c_void,
    );
    attr_tree_destroy(attr_tree);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_bool(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    mut value: bool,
) -> libc::c_int { unsafe {
    xcm_attr_set(
        s,
        name,
        xcm_attr_type_bool,
        &mut value as *mut bool as *const libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_int64(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    mut value: libc::c_long,
) -> libc::c_int { unsafe {
    xcm_attr_set(
        s,
        name,
        xcm_attr_type_int64,
        &mut value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_double(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    mut value: libc::c_double,
) -> libc::c_int { unsafe {
    xcm_attr_set(
        s,
        name,
        xcm_attr_type_double,
        &mut value as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_set_str(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    value: *const libc::c_char,
) -> libc::c_int { unsafe {
    xcm_attr_set(
        s,
        name,
        xcm_attr_type_str,
        value as *const libc::c_void,
        (strlen(value)).wrapping_add(1) as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    type_0: *mut xcm_attr_type,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let attr_tree: *mut attr_tree = build_attr_tree(s);
    let rc: libc::c_int = attr_tree_get_value(
        attr_tree,
        name,
        type_0,
        value,
        capacity,
        s as *mut libc::c_void,
    );
    attr_tree_destroy(attr_tree);
    rc
}}
unsafe extern "C" fn attr_get_with_type(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    required_type: xcm_attr_type,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut actual_type: xcm_attr_type = 0 as xcm_attr_type;
    let rc: libc::c_int = xcm_attr_get(s, name, &mut actual_type, value, capacity);
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
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_bool(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    value: *mut bool,
) -> libc::c_int { unsafe {
    attr_get_with_type(
        s,
        name,
        xcm_attr_type_bool,
        value as *mut libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_int64(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    value: *mut libc::c_long,
) -> libc::c_int { unsafe {
    attr_get_with_type(
        s,
        name,
        xcm_attr_type_int64,
        value as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_double(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    value: *mut libc::c_double,
) -> libc::c_int { unsafe {
    attr_get_with_type(
        s,
        name,
        xcm_attr_type_double,
        value as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_str(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    value: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut type_0: xcm_attr_type = 0 as xcm_attr_type;
    let rc: libc::c_int = xcm_attr_get(
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
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_bin(
    s: *mut xcm_socket,
    name: *const libc::c_char,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut type_0: xcm_attr_type = 0 as xcm_attr_type;
    let rc: libc::c_int = xcm_attr_get(s, name, &mut type_0, value, capacity);
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if type_0 as libc::c_uint != xcm_attr_type_bin as libc::c_int as libc::c_uint {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_list_len(
    s: *mut xcm_socket,
    name: *const libc::c_char,
) -> libc::c_int { unsafe {
    let attr_tree: *mut attr_tree = build_attr_tree(s);
    let rc: libc::c_int = attr_tree_get_list_len(
        attr_tree,
        name,
        s as *mut libc::c_void,
    );
    attr_tree_destroy(attr_tree);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf(
    s: *mut xcm_socket,
    type_0: *mut xcm_attr_type,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
    name_fmt: *const libc::c_char,
    args: ...
) -> libc::c_int { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let name: *mut libc::c_char = ut_vasprintf(name_fmt, ap.as_va_list());
    let rc: libc::c_int = xcm_attr_get(s, name, type_0, value, capacity);
    ut_free(name as *mut libc::c_void);
    rc
}}
unsafe extern "C" fn attr_vgetf_with_type(
    s: *mut xcm_socket,
    required_type: xcm_attr_type,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
    name_fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int { unsafe {
    let name: *mut libc::c_char = ut_vasprintf(name_fmt, ap.as_va_list());
    let rc: libc::c_int = attr_get_with_type(
        s,
        name,
        required_type,
        value,
        capacity,
    );
    ut_free(name as *mut libc::c_void);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_bool(
    s: *mut xcm_socket,
    value: *mut bool,
    name_fmt: *const libc::c_char,
    args: ...
) -> libc::c_int { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_bool,
        value as *mut libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
        name_fmt,
        ap.as_va_list(),
    );
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_int64(
    s: *mut xcm_socket,
    value: *mut libc::c_long,
    name_fmt: *const libc::c_char,
    args: ...
) -> libc::c_int { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_int64,
        value as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
        name_fmt,
        ap.as_va_list(),
    );
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_double(
    s: *mut xcm_socket,
    value: *mut libc::c_double,
    name_fmt: *const libc::c_char,
    args: ...
) -> libc::c_int { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_double,
        value as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        name_fmt,
        ap.as_va_list(),
    );
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_str(
    s: *mut xcm_socket,
    value: *mut libc::c_char,
    capacity: libc::c_ulong,
    name_fmt: *const libc::c_char,
    args: ...
) -> libc::c_int { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_str,
        value as *mut libc::c_void,
        capacity,
        name_fmt,
        ap.as_va_list(),
    );
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_getf_bin(
    s: *mut xcm_socket,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
    name_fmt: *const libc::c_char,
    args: ...
) -> libc::c_int { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let rc: libc::c_int = attr_vgetf_with_type(
        s,
        xcm_attr_type_bin,
        value,
        capacity,
        name_fmt,
        ap.as_va_list(),
    );
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_get_all(
    s: *mut xcm_socket,
    cb: xcm_attr_cb,
    cb_data: *mut libc::c_void,
) { unsafe {
    let attr_tree: *mut attr_tree = build_attr_tree(s);
    attr_tree_get_all(attr_tree, cb, cb_data);
    attr_tree_destroy(attr_tree);
}}
