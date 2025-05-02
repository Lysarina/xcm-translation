#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

use xcm_rust_common::*;
use xcm_rust_common::c_functions::*;
use rs_util::*;
use rs_log::*;
use rs_attr_tree::attr_get;
use rs_attr_tree::attr_set;

use xcm_rust_common::xcm_tp::*;
use xcm_rust_common::xcm_attr::*;
use xcm_rust_common::xpoll_mod::*;


unsafe extern "C" { fn attr_tree_add_value_node(
            tree: *mut attr_tree,
            path: *const libc::c_char,
            s: *mut xcm_socket,
            context: *mut libc::c_void,
            type_0: xcm_attr_type,
            set: attr_set,
            get: attr_get,
        );
    }



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



use std::sync::{OnceLock, Mutex};
static NEXT_ID: OnceLock<Mutex<i64>> = OnceLock::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn get_next_sock_id() -> int64_t {
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
    // let s = Box::into_raw(Box::new(core::mem::zeroed::<xcm_socket>()));

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
            .wrapping_add((256 as libc::c_int / 4 as libc::c_int) as uint64_t);
    } else {
        (*s).skipped_ctl_calls = ((*s).skipped_ctl_calls).wrapping_add(1);
    }
    if (*s).skipped_ctl_calls > 256 as libc::c_int as uint64_t {
        do_ctl(s);
        (*s).skipped_ctl_calls = 0 as libc::c_int as uint64_t;
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
    len: size_t,
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
    capacity: size_t,
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
pub unsafe extern "C" fn xcm_tp_socket_max_msg(conn_s: *mut xcm_socket) -> size_t { unsafe {
    ((*(*(*conn_s).proto).ops).max_msg)
        .expect("non-null function pointer")(conn_s)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_socket_get_cnt(
    conn_s: *mut xcm_socket,
    cnt: xcm_tp_cnt,
) -> int64_t { unsafe {
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
    capacity: size_t,
) -> libc::c_int { unsafe {
    let len: size_t = strlen(value);
    if len >= capacity {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    strcpy(buf as *mut libc::c_char, value);
    len.wrapping_add(1 as libc::c_int as size_t) as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_set_bool_attr(
    buf: *const libc::c_void,
    mut _len: size_t,
    value: *mut bool,
) { unsafe {
    memcpy(
        value as *mut libc::c_void,
        buf,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_bool_attr(
    mut value: bool,
    buf: *mut libc::c_void,
    mut _capacity: size_t,
) -> libc::c_int { unsafe {
    memcpy(
        buf,
        &mut value as *mut bool as *const libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    );
    ::core::mem::size_of::<bool>() as libc::c_ulong as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_set_double_attr(
    buf: *const libc::c_void,
    _len: size_t,
    value: *mut libc::c_double,
) { unsafe {
    memcpy(
        value as *mut libc::c_void,
        buf,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_double_attr(
    mut value: libc::c_double,
    buf: *mut libc::c_void,
    _capacity: size_t,
) -> libc::c_int { unsafe {
    memcpy(
        buf,
        &mut value as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_tp_get_bin_attr(
    value: *const libc::c_char,
    len: size_t,
    buf: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if len > capacity {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    memcpy(buf, value as *const libc::c_void, len);
    len as libc::c_int
}}
unsafe extern "C" fn get_type_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    xcm_tp_get_str_attr(xcm_tp_socket_type_name((*s).type_0), value, capacity)
}}
unsafe extern "C" fn get_transport_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    xcm_tp_get_str_attr(xcm_tp_socket_get_transport(s), value, capacity)
}}
unsafe extern "C" fn set_service_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *const libc::c_void,
    _len: size_t,
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
    capacity: size_t,
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
    capacity: size_t,
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
    _len: size_t,
) -> libc::c_int { unsafe {
    xcm_tp_socket_set_local_addr(s, value as *const libc::c_char)
}}
unsafe extern "C" fn get_local_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    addr_to_attr(xcm_local_addr(s), value, capacity)
}}
unsafe extern "C" fn get_remote_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    addr_to_attr(xcm_remote_addr(s), value, capacity)
}}
unsafe extern "C" fn set_blocking_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *const libc::c_void,
    len: size_t,
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
    capacity: size_t,
) -> libc::c_int { unsafe {
    xcm_tp_get_bool_attr((*s).is_blocking, value, capacity)
}}
unsafe extern "C" fn get_max_msg_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if (*s).type_0 as libc::c_uint != xcm_socket_type_conn as libc::c_int as libc::c_uint
    {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut max_msg: int64_t = ((*(*(*s).proto).ops).max_msg)
        .expect("non-null function pointer")(s) as int64_t;
    memcpy(
        value,
        &mut max_msg as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_app_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_app_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_app_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_app_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_lower_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_lower_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_lower_bytes_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_lower_bytes);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_app_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_app_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_app_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_app_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_to_lower_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_lower_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
}}
unsafe extern "C" fn get_from_lower_msgs_attr(
    s: *mut xcm_socket,
    _context: *mut libc::c_void,
    value: *mut libc::c_void,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_lower_msgs);
    memcpy(
        value,
        &mut cnt_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
    ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int
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
                    size_t,
                ) -> libc::c_int,
        ),
        Some(
            get_blocking_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
                ) -> libc::c_int,
        ),
        Some(
            get_service_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
                ) -> libc::c_int,
        ),
        Some(
            get_local_attr
                as unsafe extern "C" fn(
                    *mut xcm_socket,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
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
                    size_t,
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
                    size_t,
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
                    size_t,
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
static mut num_protos: size_t = 0 as libc::c_int as size_t;
#[unsafe(no_mangle)]
#[allow(static_mut_refs)] //Had to add this, the return statement gave error otherwise
pub unsafe extern "C" fn xcm_tp_proto_by_name(
    proto_name: *const libc::c_char,
) -> *mut xcm_tp_proto { unsafe {
    let mut i: libc::c_int = 0;
    while (i as size_t) < num_protos {
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
    if num_protos >= 8 as libc::c_int as size_t {
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
    if strlen(proto_name) > 32 as libc::c_int as libc::c_ulong {
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
    if strlen(proto_name) > 32 as libc::c_int as libc::c_ulong {
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
// #![allow(
//     dead_code,
//     mutable_transmutes,
//     non_camel_case_types,
//     non_snake_case,
//     non_upper_case_globals,
//     unused_assignments,
//     unused_mut,
//     static_mut_refs
// )]
// #![feature(extern_types)]
// unsafe extern "C" {
//     pub type ctl;
//     pub type xpoll;
//     pub type attr_tree;
//     fn __errno_location() -> *mut libc::c_int;
//     fn xcm_set_blocking(socket: *mut xcm_socket, should_block: bool) -> libc::c_int;
//     fn xcm_remote_addr(conn_socket: *mut xcm_socket) -> *const libc::c_char;
//     fn xcm_local_addr(socket: *mut xcm_socket) -> *const libc::c_char;
//     fn attr_tree_add_value_node(
//         tree: *mut attr_tree,
//         path: *const libc::c_char,
//         s: *mut xcm_socket,
//         context: *mut libc::c_void,
//         type_0: xcm_attr_type,
//         set: attr_set,
//         get: attr_get,
//     );
//     fn abort() -> !;
//     fn memcpy(
//         _: *mut libc::c_void,
//         _: *const libc::c_void,
//         _: libc::c_ulong,
//     ) -> *mut libc::c_void;
//     fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
//     fn ut_mutex_lock(m: *mut pthread_mutex_t);
//     fn ut_mutex_unlock(m: *mut pthread_mutex_t);
//     fn ut_calloc(size: size_t) -> *mut libc::c_void;
//     fn ut_free(ptr: *mut libc::c_void);
//     fn log_console_conf(enabled: bool);
//     fn log_is_enabled(type_0: log_type) -> bool;
//     fn __log_event(
//         type_0: log_type,
//         file: *const libc::c_char,
//         line: libc::c_int,
//         function: *const libc::c_char,
//         s: *mut xcm_socket,
//         format: *const libc::c_char,
//         _: ...
//     );
//     fn xcm_addr_parse_proto(
//         addr_s: *const libc::c_char,
//         proto: *mut libc::c_char,
//         capacity: size_t,
//     ) -> libc::c_int;
//     fn ctl_create(socket: *mut xcm_socket) -> *mut ctl;
//     fn ctl_destroy(ctl: *mut ctl, owner: bool);
//     fn ctl_process(ctl: *mut ctl);
// }
// pub type size_t = libc::c_ulong;
// pub type __int64_t = libc::c_long;
// pub type __uint64_t = libc::c_ulong;
// pub type int64_t = __int64_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct __pthread_internal_list {
//     pub __prev: *mut __pthread_internal_list,
//     pub __next: *mut __pthread_internal_list,
// }
// pub type __pthread_list_t = __pthread_internal_list;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct __pthread_mutex_s {
//     pub __lock: libc::c_int,
//     pub __count: libc::c_uint,
//     pub __owner: libc::c_int,
//     pub __nusers: libc::c_uint,
//     pub __kind: libc::c_int,
//     pub __spins: libc::c_short,
//     pub __elision: libc::c_short,
//     pub __list: __pthread_list_t,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union pthread_mutex_t {
//     pub __data: __pthread_mutex_s,
//     pub __size: [libc::c_char; 40],
//     pub __align: libc::c_long,
// }
// pub type xcm_attr_type = libc::c_uint;
// pub const xcm_attr_type_double: xcm_attr_type = 5;
// pub const xcm_attr_type_bin: xcm_attr_type = 4;
// pub const xcm_attr_type_str: xcm_attr_type = 3;
// pub const xcm_attr_type_int64: xcm_attr_type = 2;
// pub const xcm_attr_type_bool: xcm_attr_type = 1;
// pub type uint64_t = __uint64_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct xcm_socket {
//     pub proto: *const xcm_tp_proto,
//     pub type_0: xcm_socket_type,
//     pub sock_id: int64_t,
//     pub auto_enable_ctl: bool,
//     pub auto_update: bool,
//     pub is_blocking: bool,
//     pub xpoll: *mut xpoll,
//     pub condition: libc::c_int,
//     pub ctl: *mut ctl,
//     pub skipped_ctl_calls: uint64_t,
// }
// pub type xcm_socket_type = libc::c_uint;
// pub const xcm_socket_type_server: xcm_socket_type = 1;
// pub const xcm_socket_type_conn: xcm_socket_type = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct xcm_tp_proto {
//     pub name: [libc::c_char; 33],
//     pub ops: *const xcm_tp_ops,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct xcm_tp_ops {
//     pub init: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *mut xcm_socket) -> libc::c_int,
//     >,
//     pub connect: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *const libc::c_char) -> libc::c_int,
//     >,
//     pub server: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *const libc::c_char) -> libc::c_int,
//     >,
//     pub close: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
//     pub cleanup: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
//     pub accept: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *mut xcm_socket) -> libc::c_int,
//     >,
//     pub send: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *const libc::c_void, size_t) -> libc::c_int,
//     >,
//     pub receive: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *mut libc::c_void, size_t) -> libc::c_int,
//     >,
//     pub update: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
//     pub finish: Option::<unsafe extern "C" fn(*mut xcm_socket) -> libc::c_int>,
//     pub get_transport: Option::<
//         unsafe extern "C" fn(*mut xcm_socket) -> *const libc::c_char,
//     >,
//     pub get_remote_addr: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, bool) -> *const libc::c_char,
//     >,
//     pub get_local_addr: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, bool) -> *const libc::c_char,
//     >,
//     pub set_local_addr: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *const libc::c_char) -> libc::c_int,
//     >,
//     pub max_msg: Option::<unsafe extern "C" fn(*mut xcm_socket) -> size_t>,
//     pub get_cnt: Option::<unsafe extern "C" fn(*mut xcm_socket, xcm_tp_cnt) -> int64_t>,
//     pub enable_ctl: Option::<unsafe extern "C" fn(*mut xcm_socket) -> ()>,
//     pub attr_populate: Option::<
//         unsafe extern "C" fn(*mut xcm_socket, *mut attr_tree) -> (),
//     >,
//     pub priv_size: Option::<unsafe extern "C" fn(xcm_socket_type) -> size_t>,
// }
// pub type xcm_tp_cnt = libc::c_uint;
// pub const xcm_tp_cnt_from_lower_msgs: xcm_tp_cnt = 7;
// pub const xcm_tp_cnt_to_lower_msgs: xcm_tp_cnt = 6;
// pub const xcm_tp_cnt_from_app_msgs: xcm_tp_cnt = 5;
// pub const xcm_tp_cnt_to_app_msgs: xcm_tp_cnt = 4;
// pub const xcm_tp_cnt_from_lower_bytes: xcm_tp_cnt = 3;
// pub const xcm_tp_cnt_to_lower_bytes: xcm_tp_cnt = 2;
// pub const xcm_tp_cnt_from_app_bytes: xcm_tp_cnt = 1;
// pub const xcm_tp_cnt_to_app_bytes: xcm_tp_cnt = 0;
// pub type attr_set = Option::<
//     unsafe extern "C" fn(
//         *mut xcm_socket,
//         *mut libc::c_void,
//         *const libc::c_void,
//         size_t,
//     ) -> libc::c_int,
// >;
// pub type attr_get = Option::<
//     unsafe extern "C" fn(
//         *mut xcm_socket,
//         *mut libc::c_void,
//         *mut libc::c_void,
//         size_t,
//     ) -> libc::c_int,
// >;
// pub type log_type = libc::c_uint;
// pub const log_type_error: log_type = 1;
// pub const log_type_debug: log_type = 0;
// pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
// pub type C2RustUnnamed = libc::c_uint;
// pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
// pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
// pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
// pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
// pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
// pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
// pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_type_name(
//     mut socket_type: xcm_socket_type,
// ) -> *const libc::c_char {
//     match socket_type as libc::c_uint {
//         0 => return b"connection\0" as *const u8 as *const libc::c_char,
//         1 => return b"server\0" as *const u8 as *const libc::c_char,
//         _ => {
//             if 0 as libc::c_int == 0 {
//                 log_console_conf(1 as libc::c_int != 0);
//                 if log_is_enabled(log_type_error) {
//                     __log_event(
//                         log_type_error,
//                         b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0"
//                             as *const u8 as *const libc::c_char,
//                         25 as libc::c_int,
//                         (*::core::mem::transmute::<
//                             &[u8; 24],
//                             &[libc::c_char; 24],
//                         >(b"xcm_tp_socket_type_name\0"))
//                             .as_ptr(),
//                         0 as *mut xcm_socket,
//                         b"Assertion \"%s\" failed.\n\0" as *const u8
//                             as *const libc::c_char,
//                         b"0\0" as *const u8 as *const libc::c_char,
//                     );
//                 }
//                 abort();
//             }
//         }
//     }
//     panic!("Reached end of non-void function without returning");
// }
// static mut next_id_lock: pthread_mutex_t = pthread_mutex_t {
//     __data: {
//         let mut init = __pthread_mutex_s {
//             __lock: 0 as libc::c_int,
//             __count: 0 as libc::c_int as libc::c_uint,
//             __owner: 0 as libc::c_int,
//             __nusers: 0 as libc::c_int as libc::c_uint,
//             __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
//             __spins: 0 as libc::c_int as libc::c_short,
//             __elision: 0 as libc::c_int as libc::c_short,
//             __list: {
//                 let mut init = __pthread_internal_list {
//                     __prev: 0 as *const __pthread_internal_list
//                         as *mut __pthread_internal_list,
//                     __next: 0 as *const __pthread_internal_list
//                         as *mut __pthread_internal_list,
//                 };
//                 init
//             },
//         };
//         init
//     },
// };
// static mut next_id: int64_t = 0 as libc::c_int as int64_t;
// unsafe extern "C" fn get_next_sock_id() -> int64_t {
//     let mut nid: int64_t = 0;
//     ut_mutex_lock(&mut next_id_lock);
//     let fresh0 = next_id;
//     next_id = next_id + 1;
//     nid = fresh0;
//     ut_mutex_unlock(&mut next_id_lock);
//     return nid;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_create(
//     mut proto: *const xcm_tp_proto,
//     mut type_0: xcm_socket_type,
//     mut xpoll: *mut xpoll,
//     mut auto_enable_ctl: bool,
//     mut auto_update: bool,
//     mut is_blocking: bool,
// ) -> *mut xcm_socket {
//     let mut priv_size: size_t = ((*(*proto).ops).priv_size)
//         .expect("non-null function pointer")(type_0);
//     let mut s: *mut xcm_socket = ut_calloc(
//         (::core::mem::size_of::<xcm_socket>() as libc::c_ulong).wrapping_add(priv_size),
//     ) as *mut xcm_socket;
//     (*s).proto = proto;
//     (*s).type_0 = type_0;
//     (*s).auto_enable_ctl = auto_enable_ctl;
//     (*s).auto_update = auto_update;
//     (*s).is_blocking = is_blocking;
//     (*s).xpoll = xpoll;
//     (*s).sock_id = get_next_sock_id();
//     (*s).condition = 0 as libc::c_int;
//     (*s).ctl = 0 as *mut ctl;
//     return s;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_destroy(mut s: *mut xcm_socket) {
//     ut_free(s as *mut libc::c_void);
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_init(
//     mut s: *mut xcm_socket,
//     mut parent: *mut xcm_socket,
// ) -> libc::c_int {
//     return ((*(*(*s).proto).ops).init).expect("non-null function pointer")(s, parent);
// }
// unsafe extern "C" fn do_ctl(mut s: *mut xcm_socket) {
//     if !((*s).ctl).is_null() {
//         ctl_process((*s).ctl);
//     }
// }
// unsafe extern "C" fn consider_ctl(
//     mut s: *mut xcm_socket,
//     mut permanently_failed_op: bool,
//     mut temporarly_failed_op: bool,
// ) {
//     if ((*s).ctl).is_null() {
//         return;
//     }
//     if permanently_failed_op {
//         return;
//     }
//     if temporarly_failed_op {
//         (*s)
//             .skipped_ctl_calls = ((*s).skipped_ctl_calls)
//             .wrapping_add((256 as libc::c_int / 4 as libc::c_int) as uint64_t);
//     } else {
//         (*s).skipped_ctl_calls = ((*s).skipped_ctl_calls).wrapping_add(1);
//         (*s).skipped_ctl_calls;
//     }
//     if (*s).skipped_ctl_calls > 256 as libc::c_int as uint64_t {
//         do_ctl(s);
//         (*s).skipped_ctl_calls = 0 as libc::c_int as uint64_t;
//     }
// }
// unsafe extern "C" fn consider_auto_update(mut s: *mut xcm_socket) {
//     if (*s).auto_update {
//         xcm_tp_socket_update(s);
//     }
// }
// unsafe extern "C" fn consider_auto_enable_ctl(mut s: *mut xcm_socket) {
//     if (*s).auto_enable_ctl {
//         xcm_tp_socket_enable_ctl(s);
//     }
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_connect(
//     mut s: *mut xcm_socket,
//     mut remote_addr: *const libc::c_char,
// ) -> libc::c_int {
//     do_ctl(s);
//     let mut rc: libc::c_int = ((*(*(*s).proto).ops).connect)
//         .expect("non-null function pointer")(s, remote_addr);
//     if rc == 0 as libc::c_int {
//         consider_auto_enable_ctl(s);
//         consider_auto_update(s);
//     }
//     return rc;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_server(
//     mut s: *mut xcm_socket,
//     mut local_addr: *const libc::c_char,
// ) -> libc::c_int {
//     do_ctl(s);
//     let mut rc: libc::c_int = ((*(*(*s).proto).ops).server)
//         .expect("non-null function pointer")(s, local_addr);
//     if rc == 0 as libc::c_int {
//         consider_auto_enable_ctl(s);
//         consider_auto_update(s);
//     }
//     return rc;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_close(mut s: *mut xcm_socket) {
//     if !s.is_null() {
//         ctl_destroy((*s).ctl, 1 as libc::c_int != 0);
//         ((*(*(*s).proto).ops).close).expect("non-null function pointer")(s);
//     }
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_cleanup(mut s: *mut xcm_socket) {
//     if !s.is_null() {
//         ctl_destroy((*s).ctl, 0 as libc::c_int != 0);
//         ((*(*(*s).proto).ops).cleanup).expect("non-null function pointer")(s);
//     }
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_accept(
//     mut conn_s: *mut xcm_socket,
//     mut server_s: *mut xcm_socket,
// ) -> libc::c_int {
//     let mut rc: libc::c_int = ((*(*(*conn_s).proto).ops).accept)
//         .expect("non-null function pointer")(conn_s, server_s);
//     if rc == 0 as libc::c_int {
//         consider_auto_enable_ctl(conn_s);
//         consider_auto_update(conn_s);
//     }
//     consider_ctl(
//         server_s,
//         rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
//         rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
//     );
//     xcm_tp_socket_update(server_s);
//     return rc;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_send(
//     mut s: *mut xcm_socket,
//     mut buf: *const libc::c_void,
//     mut len: size_t,
// ) -> libc::c_int {
//     let mut rc: libc::c_int = ((*(*(*s).proto).ops).send)
//         .expect("non-null function pointer")(s, buf, len);
//     consider_ctl(
//         s,
//         rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
//         rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
//     );
//     consider_auto_update(s);
//     return rc;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_receive(
//     mut s: *mut xcm_socket,
//     mut buf: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     let mut rc: libc::c_int = ((*(*(*s).proto).ops).receive)
//         .expect("non-null function pointer")(s, buf, capacity);
//     consider_ctl(
//         s,
//         rc == 0 as libc::c_int
//             || rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
//         rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
//     );
//     consider_auto_update(s);
//     return rc;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_update(mut s: *mut xcm_socket) {
//     ((*(*(*s).proto).ops).update).expect("non-null function pointer")(s);
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_finish(mut s: *mut xcm_socket) -> libc::c_int {
//     let mut rc: libc::c_int = ((*(*(*s).proto).ops).finish)
//         .expect("non-null function pointer")(s);
//     consider_ctl(
//         s,
//         rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int,
//         rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int,
//     );
//     consider_auto_update(s);
//     return rc;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_get_transport(
//     mut s: *mut xcm_socket,
// ) -> *const libc::c_char {
//     if ((*(*(*s).proto).ops).get_transport).is_some() {
//         return ((*(*(*s).proto).ops).get_transport)
//             .expect("non-null function pointer")(s)
//     } else {
//         return ((*(*s).proto).name).as_ptr()
//     };
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_is_bytestream(mut s: *mut xcm_socket) -> bool {
//     return ((*(*(*s).proto).ops).max_msg).is_none();
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_get_remote_addr(
//     mut conn_s: *mut xcm_socket,
//     mut suppress_tracing: bool,
// ) -> *const libc::c_char {
//     return ((*(*(*conn_s).proto).ops).get_remote_addr)
//         .expect("non-null function pointer")(conn_s, suppress_tracing);
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_set_local_addr(
//     mut s: *mut xcm_socket,
//     mut local_addr: *const libc::c_char,
// ) -> libc::c_int {
//     if ((*(*(*s).proto).ops).set_local_addr).is_some() {
//         return ((*(*(*s).proto).ops).set_local_addr)
//             .expect("non-null function pointer")(s, local_addr)
//     } else {
//         *__errno_location() = 13 as libc::c_int;
//         return -(1 as libc::c_int);
//     };
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_get_local_addr(
//     mut s: *mut xcm_socket,
//     mut suppress_tracing: bool,
// ) -> *const libc::c_char {
//     return ((*(*(*s).proto).ops).get_local_addr)
//         .expect("non-null function pointer")(s, suppress_tracing);
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_max_msg(mut conn_s: *mut xcm_socket) -> size_t {
//     return ((*(*(*conn_s).proto).ops).max_msg)
//         .expect("non-null function pointer")(conn_s);
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_get_cnt(
//     mut conn_s: *mut xcm_socket,
//     mut cnt: xcm_tp_cnt,
// ) -> int64_t {
//     return ((*(*(*conn_s).proto).ops).get_cnt)
//         .expect("non-null function pointer")(conn_s, cnt);
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_enable_ctl(mut s: *mut xcm_socket) {
//     if ((*(*(*s).proto).ops).enable_ctl).is_some() {
//         ((*(*(*s).proto).ops).enable_ctl).expect("non-null function pointer")(s);
//     } else {
//         (*s).ctl = ctl_create(s);
//     };
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_socket_attr_populate(
//     mut s: *mut xcm_socket,
//     mut attr_tree: *mut attr_tree,
// ) {
//     if ((*(*(*s).proto).ops).attr_populate).is_some() {
//         ((*(*(*s).proto).ops).attr_populate)
//             .expect("non-null function pointer")(s, attr_tree);
//     }
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_get_str_attr(
//     mut value: *const libc::c_char,
//     mut buf: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     let mut len: size_t = strlen(value);
//     if len >= capacity {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     strcpy(buf as *mut libc::c_char, value);
//     return len.wrapping_add(1 as libc::c_int as size_t) as libc::c_int;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_set_bool_attr(
//     mut buf: *const libc::c_void,
//     mut len: size_t,
//     mut value: *mut bool,
// ) {
//     memcpy(
//         value as *mut libc::c_void,
//         buf,
//         ::core::mem::size_of::<bool>() as libc::c_ulong,
//     );
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_get_bool_attr(
//     mut value: bool,
//     mut buf: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     memcpy(
//         buf,
//         &mut value as *mut bool as *const libc::c_void,
//         ::core::mem::size_of::<bool>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<bool>() as libc::c_ulong as libc::c_int;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_set_double_attr(
//     mut buf: *const libc::c_void,
//     mut len: size_t,
//     mut value: *mut libc::c_double,
// ) {
//     memcpy(
//         value as *mut libc::c_void,
//         buf,
//         ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
//     );
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_get_double_attr(
//     mut value: libc::c_double,
//     mut buf: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     memcpy(
//         buf,
//         &mut value as *mut libc::c_double as *const libc::c_void,
//         ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_get_bin_attr(
//     mut value: *const libc::c_char,
//     mut len: size_t,
//     mut buf: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if len > capacity {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     memcpy(buf, value as *const libc::c_void, len);
//     return len as libc::c_int;
// }
// unsafe extern "C" fn get_type_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     return xcm_tp_get_str_attr(xcm_tp_socket_type_name((*s).type_0), value, capacity);
// }
// unsafe extern "C" fn get_transport_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     return xcm_tp_get_str_attr(xcm_tp_socket_get_transport(s), value, capacity);
// }
// unsafe extern "C" fn set_service_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *const libc::c_void,
//     mut len: size_t,
// ) -> libc::c_int {
//     if strcmp(value as *const libc::c_char, b"any\0" as *const u8 as *const libc::c_char)
//         == 0 as libc::c_int
//     {
//         return 0 as libc::c_int;
//     }
//     let mut actual: *const libc::c_char = if xcm_tp_socket_is_bytestream(s)
//         as libc::c_int != 0
//     {
//         b"bytestream\0" as *const u8 as *const libc::c_char
//     } else {
//         b"messaging\0" as *const u8 as *const libc::c_char
//     };
//     if strcmp(actual, value as *const libc::c_char) == 0 as libc::c_int {
//         return 0 as libc::c_int;
//     }
//     *__errno_location() = 22 as libc::c_int;
//     return -(1 as libc::c_int);
// }
// unsafe extern "C" fn get_service_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     let mut service: *const libc::c_char = if xcm_tp_socket_is_bytestream(s)
//         as libc::c_int != 0
//     {
//         b"bytestream\0" as *const u8 as *const libc::c_char
//     } else {
//         b"messaging\0" as *const u8 as *const libc::c_char
//     };
//     return xcm_tp_get_str_attr(service, value, capacity);
// }
// unsafe extern "C" fn addr_to_attr(
//     mut addr: *const libc::c_char,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if addr.is_null() {
//         *__errno_location() = 2 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     return xcm_tp_get_str_attr(addr, value, capacity);
// }
// unsafe extern "C" fn set_local_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *const libc::c_void,
//     mut len: size_t,
// ) -> libc::c_int {
//     return xcm_tp_socket_set_local_addr(s, value as *const libc::c_char);
// }
// unsafe extern "C" fn get_local_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     return addr_to_attr(xcm_local_addr(s), value, capacity);
// }
// unsafe extern "C" fn get_remote_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     return addr_to_attr(xcm_remote_addr(s), value, capacity);
// }
// unsafe extern "C" fn set_blocking_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *const libc::c_void,
//     mut len: size_t,
// ) -> libc::c_int {
//     let mut is_blocking: bool = false;
//     xcm_tp_set_bool_attr(value, len, &mut is_blocking);
//     if xcm_set_blocking(s, is_blocking) < 0 as libc::c_int {
//         return -(1 as libc::c_int);
//     }
//     return 0 as libc::c_int;
// }
// unsafe extern "C" fn get_blocking_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     return xcm_tp_get_bool_attr((*s).is_blocking, value, capacity);
// }
// unsafe extern "C" fn get_max_msg_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if (*s).type_0 as libc::c_uint != xcm_socket_type_conn as libc::c_int as libc::c_uint
//     {
//         *__errno_location() = 2 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut max_msg: int64_t = ((*(*(*s).proto).ops).max_msg)
//         .expect("non-null function pointer")(s) as int64_t;
//     memcpy(
//         value,
//         &mut max_msg as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_to_app_bytes_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_app_bytes);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_from_app_bytes_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_app_bytes);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_to_lower_bytes_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_lower_bytes);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_from_lower_bytes_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_lower_bytes);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_to_app_msgs_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_app_msgs);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_from_app_msgs_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_app_msgs);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_to_lower_msgs_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_to_lower_msgs);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn get_from_lower_msgs_attr(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if capacity < ::core::mem::size_of::<int64_t>() as libc::c_ulong {
//         *__errno_location() = 75 as libc::c_int;
//         return -(1 as libc::c_int);
//     }
//     let mut cnt_value: int64_t = xcm_tp_socket_get_cnt(s, xcm_tp_cnt_from_lower_msgs);
//     memcpy(
//         value,
//         &mut cnt_value as *mut int64_t as *const libc::c_void,
//         ::core::mem::size_of::<int64_t>() as libc::c_ulong,
//     );
//     return ::core::mem::size_of::<int64_t>() as libc::c_ulong as libc::c_int;
// }
// unsafe extern "C" fn populate_common(mut s: *mut xcm_socket, mut tree: *mut attr_tree) {
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.blocking\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_bool,
//         Some(
//             set_blocking_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *const libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//         Some(
//             get_blocking_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.type\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_str,
//         None,
//         Some(
//             get_type_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.transport\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_str,
//         None,
//         Some(
//             get_transport_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.service\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_str,
//         Some(
//             set_service_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *const libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//         Some(
//             get_service_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
// }
// unsafe extern "C" fn populate_msg_cnt(mut s: *mut xcm_socket, mut tree: *mut attr_tree) {
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.to_app_msgs\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_to_app_msgs_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.to_app_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_to_app_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.from_app_msgs\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_from_app_msgs_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.from_app_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_from_app_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.to_lower_msgs\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_to_lower_msgs_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.to_lower_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_to_lower_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.from_lower_msgs\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_from_lower_msgs_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.from_lower_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_from_lower_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
// }
// unsafe extern "C" fn populate_bytestream_cnt(
//     mut s: *mut xcm_socket,
//     mut tree: *mut attr_tree,
// ) {
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.to_app_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_to_app_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.from_app_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_from_app_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.to_lower_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_to_lower_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.from_lower_bytes\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_from_lower_bytes_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
// }
// unsafe extern "C" fn populate_common_conn(
//     mut s: *mut xcm_socket,
//     mut tree: *mut attr_tree,
// ) {
//     populate_common(s, tree);
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.local_addr\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_str,
//         Some(
//             set_local_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *const libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//         Some(
//             get_local_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.remote_addr\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_str,
//         None,
//         Some(
//             get_remote_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
// }
// unsafe extern "C" fn populate_msg_conn(
//     mut s: *mut xcm_socket,
//     mut tree: *mut attr_tree,
// ) {
//     populate_common_conn(s, tree);
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.max_msg_size\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_int64,
//         None,
//         Some(
//             get_max_msg_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
//     populate_msg_cnt(s, tree);
// }
// unsafe extern "C" fn populate_bytestream_conn(
//     mut s: *mut xcm_socket,
//     mut tree: *mut attr_tree,
// ) {
//     populate_common_conn(s, tree);
//     populate_bytestream_cnt(s, tree);
// }
// unsafe extern "C" fn populate_server(mut s: *mut xcm_socket, mut tree: *mut attr_tree) {
//     populate_common(s, tree);
//     attr_tree_add_value_node(
//         tree,
//         b"xcm.local_addr\0" as *const u8 as *const libc::c_char,
//         s,
//         0 as *mut libc::c_void,
//         xcm_attr_type_str,
//         None,
//         Some(
//             get_local_attr
//                 as unsafe extern "C" fn(
//                     *mut xcm_socket,
//                     *mut libc::c_void,
//                     *mut libc::c_void,
//                     size_t,
//                 ) -> libc::c_int,
//         ),
//     );
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_common_attr_populate(
//     mut s: *mut xcm_socket,
//     mut tree: *mut attr_tree,
// ) {
//     match (*s).type_0 as libc::c_uint {
//         0 => {
//             if xcm_tp_socket_is_bytestream(s) {
//                 populate_bytestream_conn(s, tree);
//             } else {
//                 populate_msg_conn(s, tree);
//             }
//         }
//         1 => {
//             populate_server(s, tree);
//         }
//         _ => {
//             if 0 as libc::c_int == 0 {
//                 log_console_conf(1 as libc::c_int != 0);
//                 if log_is_enabled(log_type_error) {
//                     __log_event(
//                         log_type_error,
//                         b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0"
//                             as *const u8 as *const libc::c_char,
//                         569 as libc::c_int,
//                         (*::core::mem::transmute::<
//                             &[u8; 28],
//                             &[libc::c_char; 28],
//                         >(b"xcm_tp_common_attr_populate\0"))
//                             .as_ptr(),
//                         0 as *mut xcm_socket,
//                         b"Assertion \"%s\" failed.\n\0" as *const u8
//                             as *const libc::c_char,
//                         b"0\0" as *const u8 as *const libc::c_char,
//                     );
//                 }
//                 abort();
//             }
//         }
//     };
// }
// static mut protos: [xcm_tp_proto; 8] = [xcm_tp_proto {
//     name: [0; 33],
//     ops: 0 as *const xcm_tp_ops,
// }; 8];
// static mut num_protos: size_t = 0 as libc::c_int as size_t;
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_proto_by_name(
//     mut proto_name: *const libc::c_char,
// ) -> *mut xcm_tp_proto {
//     let mut i: libc::c_int = 0;
//     i = 0 as libc::c_int;
//     while (i as size_t) < num_protos {
//         if strcmp((protos[i as usize].name).as_mut_ptr(), proto_name) == 0 as libc::c_int
//         {
//             return &mut *protos.as_mut_ptr().offset(i as isize) as *mut xcm_tp_proto;
//         }
//         i += 1;
//         i;
//     }
//     return 0 as *mut xcm_tp_proto;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_proto_by_addr(
//     mut addr: *const libc::c_char,
// ) -> *mut xcm_tp_proto {
//     let mut proto_s: [libc::c_char; 33] = [0; 33];
//     if xcm_addr_parse_proto(
//         addr,
//         proto_s.as_mut_ptr(),
//         ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
//     ) < 0 as libc::c_int
//     {
//         return 0 as *mut xcm_tp_proto;
//     }
//     let mut proto: *mut xcm_tp_proto = xcm_tp_proto_by_name(proto_s.as_mut_ptr());
//     if proto.is_null() {
//         *__errno_location() = 92 as libc::c_int;
//         return 0 as *mut xcm_tp_proto;
//     }
//     return proto;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn xcm_tp_register(
//     mut proto_name: *const libc::c_char,
//     mut ops: *const xcm_tp_ops,
// ) {
//     if !(num_protos < 8 as libc::c_int as size_t) {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
//                     as *const libc::c_char,
//                 603 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 16],
//                     &[libc::c_char; 16],
//                 >(b"xcm_tp_register\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"num_protos < (8)\0" as *const u8 as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     if !(strlen(proto_name) <= 32 as libc::c_int as libc::c_ulong) {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
//                     as *const libc::c_char,
//                 604 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 16],
//                     &[libc::c_char; 16],
//                 >(b"xcm_tp_register\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"strlen(proto_name) <= (32)\0" as *const u8 as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     if !(strlen(proto_name) <= 32 as libc::c_int as libc::c_ulong) {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
//                     as *const libc::c_char,
//                 605 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 16],
//                     &[libc::c_char; 16],
//                 >(b"xcm_tp_register\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"strlen(proto_name) <= (32)\0" as *const u8 as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     if !(xcm_tp_proto_by_name(proto_name)).is_null() {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/ehhjmou/xcm-translation/libxcm/tp/common/xcm_tp.c\0" as *const u8
//                     as *const libc::c_char,
//                 606 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 16],
//                     &[libc::c_char; 16],
//                 >(b"xcm_tp_register\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"xcm_tp_proto_by_name(proto_name) == ((void*)0)\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     strcpy((protos[num_protos as usize].name).as_mut_ptr(), proto_name);
//     protos[num_protos as usize].ops = ops;
//     num_protos = num_protos.wrapping_add(1);
//     num_protos;
// }
