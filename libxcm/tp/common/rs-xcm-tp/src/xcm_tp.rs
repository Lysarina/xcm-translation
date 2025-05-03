#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

use std::sync::{OnceLock, Mutex};
use std::process::abort;
use libc::{strcpy, strlen, strcmp, __errno_location, memcpy};

use rs_util::*;
use rs_log::*;
use rs_xcm_addr::*;
use rs_attr_tree::*;
use xcm_rust_common::{cyclic_declarations::*, xcm_tp::*,
    xcm_attr::*, attr_tree_mod::attr_tree,
    xpoll_mod::xpoll};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_type_name(
    socket_type: xcm_socket_type,
) -> *const libc::c_char {
    match socket_type as libc::c_uint {
        0 => c"connection".as_ptr() as *const libc::c_char,
        1 => c"server".as_ptr() as *const libc::c_char,
        _ => panic!("Assertion failed in xcm_tp_socket_type_name"),
    }
}

static NEXT_ID: OnceLock<Mutex<i64>> = OnceLock::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn get_next_sock_id() -> libc::c_long {
    let counter = NEXT_ID.get_or_init(|| Mutex::new(0));
    let mut guard = counter.lock().unwrap();
    let id = *guard;
    *guard += 1;
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_create(
    proto: *const xcm_tp_proto,
    type_0: xcm_socket_type,
    xpoll: *mut xpoll,
    auto_enable_ctl: bool,
    auto_update: bool,
    is_blocking: bool,
) -> *mut xcm_socket { unsafe {
    let priv_size = ((*(*proto).ops).priv_size)
        .expect("non-null function pointer")(type_0);

    let s = ut_calloc(
        (core::mem::size_of::<xcm_socket>() as libc::c_ulong)
            .wrapping_add(priv_size),
    ).cast::<xcm_socket>();

    *s = xcm_socket {
        proto,
        type_0,
        auto_enable_ctl,
        auto_update,
        is_blocking,
        xpoll,
        sock_id: get_next_sock_id(),
        condition: 0,
        ctl: core::ptr::null_mut(),
        skipped_ctl_calls: 0,
    };
    s
}}



#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_destroy(s: *mut xcm_socket) { unsafe {
    ut_free(s as *mut libc::c_void); //keep ut_free? Think the only other option is libc::free(s as *mut libc::c_void);, but don't know if that is better
    // let s = Box::from_raw(s);
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_init(
    s: *mut xcm_socket,
    parent: *mut xcm_socket,
) -> libc::c_int { unsafe {
    ((*(*(*s).proto).ops).init).expect("non-null function pointer")(s, parent)
}}
unsafe extern "C" fn do_ctl(s: *mut xcm_socket) { unsafe {
    if !((*s).ctl).is_null() {
        ctl_process((*s).ctl);
    }
}}
unsafe extern "C" fn consider_ctl(
    s: *mut xcm_socket,
    permanently_failed_op: bool,
    temporarly_failed_op: bool,
) { unsafe {
    if ((*s).ctl).is_null() {
        return;
    }
    if permanently_failed_op {
        return;
    }
    if temporarly_failed_op {
        (*s)
            .skipped_ctl_calls = ((*s).skipped_ctl_calls)
            .wrapping_add((256 as libc::c_int / 4 as libc::c_int) as libc::c_ulong);
    } else {
        (*s).skipped_ctl_calls = ((*s).skipped_ctl_calls).wrapping_add(1);
    }
    if (*s).skipped_ctl_calls > 256 as libc::c_int as libc::c_ulong {
        do_ctl(s);
        (*s).skipped_ctl_calls = 0 as libc::c_int as libc::c_ulong;
    }
}}
unsafe extern "C" fn consider_auto_update(s: *mut xcm_socket) { unsafe {
    if (*s).auto_update {
        xcm_tp_socket_update(s);
    }
}}
unsafe extern "C" fn consider_auto_enable_ctl(s: *mut xcm_socket) { unsafe {
    if (*s).auto_enable_ctl {
        xcm_tp_socket_enable_ctl(s);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_connect(
    s: *mut xcm_socket,
    remote_addr: *const libc::c_char,
) -> libc::c_int { unsafe {
    do_ctl(s);
    let rc: libc::c_int = ((*(*(*s).proto).ops).connect)
        .expect("non-null function pointer")(s, remote_addr);
    if rc == 0 as libc::c_int {
        consider_auto_enable_ctl(s);
        consider_auto_update(s);
    }
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_server(
    s: *mut xcm_socket,
    local_addr: *const libc::c_char,
) -> libc::c_int { unsafe {
    do_ctl(s);
    let rc: libc::c_int = ((*(*(*s).proto).ops).server)
        .expect("non-null function pointer")(s, local_addr);
    if rc == 0 as libc::c_int {
        consider_auto_enable_ctl(s);
        consider_auto_update(s);
    }
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_close(s: *mut xcm_socket) { unsafe {
    if !s.is_null() {
        ctl_destroy((*s).ctl, 1 as libc::c_int != 0);
        ((*(*(*s).proto).ops).close).expect("non-null function pointer")(s);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_cleanup(s: *mut xcm_socket) { unsafe {
    if !s.is_null() {
        ctl_destroy((*s).ctl, 0 as libc::c_int != 0);
        ((*(*(*s).proto).ops).cleanup).expect("non-null function pointer")(s);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_accept(
    conn_s: *mut xcm_socket,
    server_s: *mut xcm_socket,
) -> libc::c_int { unsafe {
    let rc: libc::c_int = ((*(*(*conn_s).proto).ops).accept)
        .expect("non-null function pointer")(conn_s, server_s);
    if rc == 0 {
        consider_auto_enable_ctl(conn_s);
        consider_auto_update(conn_s);
    }
    consider_ctl(
        server_s,
        rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
        rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
    );
    xcm_tp_socket_update(server_s);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_send(
    s: *mut xcm_socket,
    buf: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
    let rc: libc::c_int = ((*(*(*s).proto).ops).send)
        .expect("non-null function pointer")(s, buf, len);
    consider_ctl(
        s,
        rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
        rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
    );
    consider_auto_update(s);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_receive(
    s: *mut xcm_socket,
    buf: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let rc: libc::c_int = ((*(*(*s).proto).ops).receive)
        .expect("non-null function pointer")(s, buf, capacity);
    consider_ctl(
        s,
        rc == 0 as libc::c_int
            || rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
        rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
    );
    consider_auto_update(s);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_update(s: *mut xcm_socket) { unsafe {
    ((*(*(*s).proto).ops).update).expect("non-null function pointer")(s);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_finish(s: *mut xcm_socket) -> libc::c_int {
    unsafe {
        let rc: libc::c_int = ((*(*(*s).proto).ops).finish)
            .expect("non-null function pointer")(s);
        consider_ctl(
            s,
            rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
            rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
        );
        consider_auto_update(s);
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_get_transport(
    s: *mut xcm_socket,
) -> *const libc::c_char {
    unsafe {
        if ((*(*(*s).proto).ops).get_transport).is_some() {
            ((*(*(*s).proto).ops).get_transport)
                .expect("non-null function pointer")(s)
        } else {
            ((*(*s).proto).name).as_ptr()
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_is_bytestream(s: *mut xcm_socket) -> bool {
    unsafe {((*(*(*s).proto).ops).max_msg).is_none()}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_get_remote_addr(
    conn_s: *mut xcm_socket,
    suppress_tracing: bool,
) -> *const libc::c_char { unsafe {
    ((*(*(*conn_s).proto).ops).get_remote_addr)
        .expect("non-null function pointer")(conn_s, suppress_tracing)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_set_local_addr(
    s: *mut xcm_socket,
    local_addr: *const libc::c_char,
) -> libc::c_int { unsafe {
    if ((*(*(*s).proto).ops).set_local_addr).is_some() {
        ((*(*(*s).proto).ops).set_local_addr)
            .expect("non-null function pointer")(s, local_addr)
    } else {
        *__errno_location() = 13 as libc::c_int;
        -(1 as libc::c_int)
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_get_local_addr(
    s: *mut xcm_socket,
    suppress_tracing: bool,
) -> *const libc::c_char { unsafe {
    ((*(*(*s).proto).ops).get_local_addr)
        .expect("non-null function pointer")(s, suppress_tracing)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_max_msg(conn_s: *mut xcm_socket) -> libc::c_ulong { unsafe {
    ((*(*(*conn_s).proto).ops).max_msg)
        .expect("non-null function pointer")(conn_s)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_get_cnt(
    conn_s: *mut xcm_socket,
    cnt: xcm_tp_cnt,
) -> libc::c_long { unsafe {
    ((*(*(*conn_s).proto).ops).get_cnt)
        .expect("non-null function pointer")(conn_s, cnt)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_enable_ctl(s: *mut xcm_socket) {
    unsafe {
        if ((*(*(*s).proto).ops).enable_ctl).is_some() {
            ((*(*(*s).proto).ops).enable_ctl).expect("non-null function pointer")(s);
        } else {
            (*s).ctl = ctl_create(s);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_attr_populate(
    s: *mut xcm_socket,
    attr_tree: *mut attr_tree,
) { unsafe {
    if ((*(*(*s).proto).ops).attr_populate).is_some() {
        ((*(*(*s).proto).ops).attr_populate)
            .expect("non-null function pointer")(s, attr_tree);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_str_attr(
    value: *const libc::c_char,
    buf: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let len: usize = strlen(value);
    if len >= capacity as usize {
        *__errno_location() = 75;
        return -1;
    }
    strcpy(buf as *mut libc::c_char, value);
    len.wrapping_add(1) as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_set_bool_attr(
    buf: *const libc::c_void,
    mut _len: libc::c_ulong,
    value: *mut bool,
) { unsafe {
    memcpy(
        value as *mut libc::c_void,
        buf,
        ::core::mem::size_of::<bool>(),
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_bool_attr(
    mut value: bool,
    buf: *mut libc::c_void,
    mut _capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    memcpy(
        buf,
        &mut value as *mut bool as *const libc::c_void,
        ::core::mem::size_of::<bool>(),
    );
    ::core::mem::size_of::<bool>() as libc::c_ulong as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_set_double_attr(
    buf: *const libc::c_void,
    _len: libc::c_ulong,
    value: *mut libc::c_double,
) { unsafe {
    memcpy(
        value as *mut libc::c_void,
        buf,
        ::core::mem::size_of::<libc::c_double>(),
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_double_attr(
    mut value: libc::c_double,
    buf: *mut libc::c_void,
    _capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    memcpy(
        buf,
        &mut value as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>(),
    );
    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_bin_attr(
    value: *const libc::c_char,
    len: libc::c_ulong,
    buf: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if len > capacity {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    memcpy(buf, value as *const libc::c_void, len as usize);
    len as libc::c_int
}}
unsafe extern "C" fn get_type_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    xcm_tp_get_str_attr(xcm_tp_socket_type_name((*s).type_0), value, capacity)
}}
unsafe extern "C" fn get_transport_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    xcm_tp_get_str_attr(xcm_tp_socket_get_transport(s), value, capacity)
}}
unsafe extern "C" fn set_service_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *const libc::c_void,
    _len: libc::c_ulong,
) -> libc::c_int { unsafe {
    if strcmp(value as *const libc::c_char, b"any\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let actual: *const libc::c_char = if xcm_tp_socket_is_bytestream(s)
        as libc::c_int != 0
    {
        b"bytestream\0" as *const u8 as *const libc::c_char
    } else {
        b"messaging\0" as *const u8 as *const libc::c_char
    };
    if strcmp(actual, value as *const libc::c_char) == 0 as libc::c_int {
       return 0 as libc::c_int;
    }
    *__errno_location() = 22 as libc::c_int;
    -(1 as libc::c_int)
}}
unsafe extern "C" fn get_service_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let service: *const libc::c_char = if xcm_tp_socket_is_bytestream(s)
        as libc::c_int != 0
    {
        b"bytestream\0" as *const u8 as *const libc::c_char
    } else {
        b"messaging\0" as *const u8 as *const libc::c_char
    };
    xcm_tp_get_str_attr(service, value, capacity)
}}
unsafe extern "C" fn addr_to_attr(
    addr: *const libc::c_char,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if addr.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    xcm_tp_get_str_attr(addr, value, capacity)
}}
unsafe extern "C" fn set_local_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *const libc::c_void,
    _len: libc::c_ulong,
) -> libc::c_int { unsafe {
    xcm_tp_socket_set_local_addr(s, value as *const libc::c_char)
}}
unsafe extern "C" fn get_local_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    addr_to_attr(xcm_local_addr(s), value, capacity)
}}
unsafe extern "C" fn get_remote_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    addr_to_attr(xcm_remote_addr(s), value, capacity)
}}
unsafe extern "C" fn set_blocking_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut is_blocking: bool = false;
    xcm_tp_set_bool_attr(value, len, &mut is_blocking);
    if xcm_set_blocking(s, is_blocking) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
unsafe extern "C" fn get_blocking_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    xcm_tp_get_bool_attr((*s).is_blocking, value, capacity)
}}
unsafe extern "C" fn get_max_msg_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if (*s).type_0 as libc::c_uint != xcm_socket_type_conn as libc::c_int as libc::c_uint
    {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut max_msg: libc::c_long = ((*(*(*s).proto).ops).max_msg)
        .expect("non-null function pointer")(s) as libc::c_long;
    memcpy(
        value,
        &mut max_msg as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_app_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_app_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_app_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_app_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_lower_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_lower_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_lower_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_lower_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_app_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_app_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_app_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_app_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_lower_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_lower_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_lower_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: libc::c_long = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_lower_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>(),
    );
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn populate_common(s: *mut xcm_socket, tree: *mut attr_tree) { unsafe {
    attr_tree_add_value_node(
        tree,
        b"xcm.blocking\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_bool,
        Some(
            set_blocking_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *const libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
        Some(
            get_blocking_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.type\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_str,
        None,
        Some(
            get_type_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.transport\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_str,
        None,
        Some(
            get_transport_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.service\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_str,
        Some(
            set_service_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *const libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
        Some(
            get_service_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
}}
unsafe extern "C" fn populate_msg_cnt(s: *mut xcm_socket, tree: *mut attr_tree) { unsafe {
    attr_tree_add_value_node(
        tree,
        b"xcm.to_app_msgs\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_to_app_msgs_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.to_app_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_to_app_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.from_app_msgs\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_from_app_msgs_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.from_app_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_from_app_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.to_lower_msgs\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_to_lower_msgs_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.to_lower_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_to_lower_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.from_lower_msgs\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_from_lower_msgs_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.from_lower_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_from_lower_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
}}
unsafe extern "C" fn populate_bytestream_cnt(
    s: *mut xcm_socket,
    tree: *mut attr_tree,
) { unsafe {
    attr_tree_add_value_node(
        tree,
        b"xcm.to_app_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_to_app_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.from_app_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_from_app_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.to_lower_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_to_lower_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.from_lower_bytes\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_from_lower_bytes_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
}}
unsafe extern "C" fn populate_common_conn(
    s: *mut xcm_socket,
    tree: *mut attr_tree,
) { unsafe {
    populate_common(s, tree);
    attr_tree_add_value_node(
        tree,
        b"xcm.local_addr\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_str,
        Some(
            set_local_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *const libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
        Some(
            get_local_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    attr_tree_add_value_node(
        tree,
        b"xcm.remote_addr\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_str,
        None,
        Some(
            get_remote_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
}}
unsafe extern "C" fn populate_msg_conn(
    s: *mut xcm_socket,
    tree: *mut attr_tree,
) { unsafe {
    populate_common_conn(s, tree);
    attr_tree_add_value_node(
        tree,
        b"xcm.max_msg_size\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_int64,
        None,
        Some(
            get_max_msg_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
    populate_msg_cnt(s, tree);
}}
unsafe extern "C" fn populate_bytestream_conn(
    s: *mut xcm_socket,
    tree: *mut attr_tree,
) { unsafe {
    populate_common_conn(s, tree);
    populate_bytestream_cnt(s, tree);
}}
unsafe extern "C" fn populate_server(s: *mut xcm_socket, tree: *mut attr_tree) { unsafe {
    populate_common(s, tree);
    attr_tree_add_value_node(
        tree,
        b"xcm.local_addr\0" as *const u8 as *const libc::c_char,
        s,
        std::ptr::null_mut::<libc::c_void>(),
        xcm_attr_type_str,
        None,
        Some(
            get_local_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> libc::c_int,
        ),
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_common_attr_populate(
    s: *mut xcm_socket,
    tree: *mut attr_tree,
) { unsafe {
    match (*s).type_0 as libc::c_uint {
        0 => {
            if xcm_tp_socket_is_bytestream(s) {
                populate_bytestream_conn(s, tree);
            } else {
                populate_msg_conn(s, tree);
            }
        }
        1 => {
            populate_server(s, tree);
        }
        _ => {
            if 0 as libc::c_int == 0 {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0"
                            as *const u8 as *const libc::c_char,
                        569 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 28],
                            &[libc::c_char; 28],
                        >(b"xcm_tp_common_attr_populate\0"))
                            .as_ptr(),
                        std::ptr::null_mut::<xcm_socket>(),
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"0\0" as *const u8 as *const libc::c_char,
                    );
                }
                abort();
            }
        }
    };
}}
static mut protos: [xcm_tp_proto; 8] = [xcm_tp_proto {
    name: [0; 33],
    ops: 0 as *const xcm_tp_ops,
}; 8];
static mut num_protos: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[unsafe(no_mangle)]
#[allow(static_mut_refs)] //Had to add this, the return statement gave error otherwise
pub unsafe extern "C" fn xcm_tp_proto_by_name(
    proto_name: *const libc::c_char,
) -> *mut xcm_tp_proto { unsafe {
    let mut i: libc::c_int = 0;
    while (i as libc::c_ulong) < num_protos {
        if strcmp((protos[i as usize].name).as_mut_ptr(), proto_name) == 0 as libc::c_int
        {
            return &mut *protos.as_mut_ptr().offset(i as isize) as *mut xcm_tp_proto;
        }
        i += 1;
    }
    std::ptr::null_mut::<xcm_tp_proto>()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_proto_by_addr(
    addr: *const libc::c_char,
) -> *mut xcm_tp_proto { unsafe {
    let mut proto_s: [libc::c_char; 33] = [0; 33];
    if xcm_addr_parse_proto(
        addr,
        proto_s.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
    ) < 0 as libc::c_int
    {
        return std::ptr::null_mut::<xcm_tp_proto>();
    }
    let proto: *mut xcm_tp_proto = xcm_tp_proto_by_name(proto_s.as_mut_ptr());
    if proto.is_null() {
        *__errno_location() = 92 as libc::c_int;
        return std::ptr::null_mut::<xcm_tp_proto>();
    }
    proto
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_register(
    proto_name: *const libc::c_char,
    ops: *const xcm_tp_ops,
) { unsafe {
    if num_protos >= 8 as libc::c_int as libc::c_ulong {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
                    as *const libc::c_char,
                603 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"xcm_tp_register\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"num_protos < (8)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    if strlen(proto_name) > 32_usize {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
                    as *const libc::c_char,
                604 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"xcm_tp_register\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"strlen(proto_name) <= (32)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    if strlen(proto_name) > 32_usize {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
                    as *const libc::c_char,
                605 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"xcm_tp_register\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"strlen(proto_name) <= (32)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    if !(xcm_tp_proto_by_name(proto_name)).is_null() {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
                    as *const libc::c_char,
                606 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"xcm_tp_register\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"xcm_tp_proto_by_name(proto_name) == ((void*)0)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    strcpy((protos[num_protos as usize].name).as_mut_ptr(), proto_name);
    protos[num_protos as usize].ops = ops;
    num_protos = num_protos.wrapping_add(1);
}}