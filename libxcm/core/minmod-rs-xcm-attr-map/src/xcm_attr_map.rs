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
unsafe extern "C" {
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    fn abort() -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn ut_memdup(ptr: *const libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
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
pub type xcm_attr_type = libc::c_uint;
pub const xcm_attr_type_double: xcm_attr_type = 5;
pub const xcm_attr_type_bin: xcm_attr_type = 4;
pub const xcm_attr_type_str: xcm_attr_type = 3;
pub const xcm_attr_type_int64: xcm_attr_type = 2;
pub const xcm_attr_type_bool: xcm_attr_type = 1;
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_attr_map {
    pub attrs: attr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_list {
    pub lh_first: *mut attr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr {
    pub name: *mut libc::c_char,
    pub type_0: xcm_attr_type,
    pub value: *mut libc::c_void,
    pub value_len: size_t,
    pub entry: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub le_next: *mut attr,
    pub le_prev: *mut *mut attr,
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
pub type xcm_attr_map_foreach_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
unsafe extern "C" fn assert_valid_len(mut type_0: xcm_attr_type, mut value_len: size_t) {
    match type_0 as libc::c_uint {
        1 => {
            if !(value_len == ::core::mem::size_of::<bool>() as libc::c_ulong) {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                            as *const u8 as *const libc::c_char,
                        24 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"assert_valid_len\0"))
                            .as_ptr(),
                        0 as *mut xcm_socket,
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"value_len == sizeof(_Bool)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
        }
        2 => {
            if !(value_len == ::core::mem::size_of::<int64_t>() as libc::c_ulong) {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                            as *const u8 as *const libc::c_char,
                        27 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"assert_valid_len\0"))
                            .as_ptr(),
                        0 as *mut xcm_socket,
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"value_len == sizeof(int64_t)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
        }
        5 => {
            if !(value_len == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                            as *const u8 as *const libc::c_char,
                        30 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"assert_valid_len\0"))
                            .as_ptr(),
                        0 as *mut xcm_socket,
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"value_len == sizeof(double)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
        }
        3 | 4 | _ => {}
    };
}
unsafe extern "C" fn attr_create(
    mut name: *const libc::c_char,
    mut type_0: xcm_attr_type,
    mut value: *const libc::c_void,
    mut value_len: size_t,
) -> *mut attr {
    assert_valid_len(type_0, value_len);
    let mut attr: *mut attr = ut_malloc(::core::mem::size_of::<attr>() as libc::c_ulong)
        as *mut attr;
    *attr = {
        let mut init = attr {
            name: ut_strdup(name),
            type_0: type_0,
            value: ut_memdup(value, value_len),
            value_len: value_len,
            entry: C2RustUnnamed {
                le_next: 0 as *mut attr,
                le_prev: 0 as *mut *mut attr,
            },
        };
        init
    };
    return attr;
}
unsafe extern "C" fn attr_destroy(mut attr: *mut attr) {
    if !attr.is_null() {
        ut_free((*attr).name as *mut libc::c_void);
        ut_free((*attr).value);
        ut_free(attr as *mut libc::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_create() -> *mut xcm_attr_map {
    let mut attr_map: *mut xcm_attr_map = ut_malloc(
        ::core::mem::size_of::<xcm_attr_map>() as libc::c_ulong,
    ) as *mut xcm_attr_map;
    (*attr_map).attrs.lh_first = 0 as *mut attr;
    return attr_map;
}
unsafe extern "C" fn copy_attr_cb(
    mut attr_name: *const libc::c_char,
    mut attr_type: xcm_attr_type,
    mut attr_value: *const libc::c_void,
    mut attr_value_len: size_t,
    mut user: *mut libc::c_void,
) {
    let mut copy: *mut xcm_attr_map = user as *mut xcm_attr_map;
    xcm_attr_map_add(copy, attr_name, attr_type, attr_value, attr_value_len);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_clone(
    mut original: *const xcm_attr_map,
) -> *mut xcm_attr_map {
    let mut copy: *mut xcm_attr_map = xcm_attr_map_create();
    xcm_attr_map_foreach(
        original,
        Some(
            copy_attr_cb
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    xcm_attr_type,
                    *const libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        copy as *mut libc::c_void,
    );
    return copy;
}
unsafe extern "C" fn lookup_attr(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> *mut attr {
    let mut attr: *mut attr = 0 as *mut attr;
    attr = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        if strcmp((*attr).name, attr_name) == 0 as libc::c_int {
            return attr;
        }
        attr = (*attr).entry.le_next;
    }
    return 0 as *mut attr;
}
unsafe extern "C" fn lookup_attr_with_type(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut type_0: xcm_attr_type,
) -> *const attr {
    let mut attr: *mut attr = 0 as *mut attr;
    attr = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        if strcmp((*attr).name, attr_name) == 0 as libc::c_int {
            return if (*attr).type_0 as libc::c_uint == type_0 as libc::c_uint {
                attr
            } else {
                0 as *mut attr
            };
        }
        attr = (*attr).entry.le_next;
    }
    return 0 as *const attr;
}
unsafe extern "C" fn lookup_value_with_type(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut type_0: xcm_attr_type,
) -> *const libc::c_void {
    let mut attr: *const attr = lookup_attr_with_type(attr_map, attr_name, type_0);
    if attr.is_null() {
        return 0 as *const libc::c_void;
    }
    return (*attr).value;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_type: xcm_attr_type,
    mut attr_value: *const libc::c_void,
    mut attr_value_len: size_t,
) {
    if !(!attr_name.is_null() && !attr_value.is_null()) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                    as *const u8 as *const libc::c_char,
                136 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"xcm_attr_map_add\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_name && attr_value\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    xcm_attr_map_del(attr_map, attr_name);
    let mut attr: *mut attr = attr_create(
        attr_name,
        attr_type,
        attr_value,
        attr_value_len,
    );
    (*attr).entry.le_next = (*attr_map).attrs.lh_first;
    if !((*attr).entry.le_next).is_null() {
        (*(*attr_map).attrs.lh_first).entry.le_prev = &mut (*attr).entry.le_next;
    }
    (*attr_map).attrs.lh_first = attr;
    (*attr).entry.le_prev = &mut (*attr_map).attrs.lh_first;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_bool(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_value: bool,
) {
    xcm_attr_map_add(
        attr_map,
        attr_name,
        xcm_attr_type_bool,
        &mut attr_value as *mut bool as *const libc::c_void,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_int64(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_value: int64_t,
) {
    xcm_attr_map_add(
        attr_map,
        attr_name,
        xcm_attr_type_int64,
        &mut attr_value as *mut int64_t as *const libc::c_void,
        ::core::mem::size_of::<int64_t>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_double(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_value: libc::c_double,
) {
    xcm_attr_map_add(
        attr_map,
        attr_name,
        xcm_attr_type_double,
        &mut attr_value as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_str(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_value: *const libc::c_char,
) {
    xcm_attr_map_add(
        attr_map,
        attr_name,
        xcm_attr_type_str,
        attr_value as *const libc::c_void,
        (strlen(attr_value)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_bin(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_value: *const libc::c_void,
    mut attr_value_len: size_t,
) {
    xcm_attr_map_add(attr_map, attr_name, xcm_attr_type_bin, attr_value, attr_value_len);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_all(
    mut dst_map: *mut xcm_attr_map,
    mut src_map: *const xcm_attr_map,
) {
    if dst_map != src_map as *mut xcm_attr_map {
        xcm_attr_map_foreach(
            src_map,
            Some(
                copy_attr_cb
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        xcm_attr_type,
                        *const libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            dst_map as *mut libc::c_void,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
    mut attr_type: *mut xcm_attr_type,
    mut attr_value_len: *mut size_t,
) -> *const libc::c_void {
    let mut attr: *mut attr = lookup_attr(attr_map, attr_name);
    if attr.is_null() {
        return 0 as *const libc::c_void;
    }
    if !attr_type.is_null() {
        *attr_type = (*attr).type_0;
    }
    if !attr_value_len.is_null() {
        *attr_value_len = (*attr).value_len;
    }
    return (*attr).value;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_bool(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> *const bool {
    return lookup_value_with_type(attr_map, attr_name, xcm_attr_type_bool)
        as *const bool;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_int64(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> *const int64_t {
    return lookup_value_with_type(attr_map, attr_name, xcm_attr_type_int64)
        as *const int64_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_double(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> *const libc::c_double {
    return lookup_value_with_type(attr_map, attr_name, xcm_attr_type_double)
        as *const libc::c_double;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_str(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> *const libc::c_char {
    return lookup_value_with_type(attr_map, attr_name, xcm_attr_type_str)
        as *const libc::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_bin(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> *const libc::c_char {
    return lookup_value_with_type(attr_map, attr_name, xcm_attr_type_bin)
        as *const libc::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_exists(
    mut attr_map: *const xcm_attr_map,
    mut attr_name: *const libc::c_char,
) -> bool {
    return !(lookup_attr(attr_map, attr_name)).is_null();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_del(
    mut attr_map: *mut xcm_attr_map,
    mut attr_name: *const libc::c_char,
) {
    let mut attr: *mut attr = lookup_attr(attr_map, attr_name);
    if !attr.is_null() {
        if !((*attr).entry.le_next).is_null() {
            (*(*attr).entry.le_next).entry.le_prev = (*attr).entry.le_prev;
        }
        *(*attr).entry.le_prev = (*attr).entry.le_next;
        attr_destroy(attr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_size(mut attr_map: *const xcm_attr_map) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut attr: *mut attr = 0 as *mut attr;
    attr = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        count = count.wrapping_add(1);
        count;
        attr = (*attr).entry.le_next;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_foreach(
    mut attr_map: *const xcm_attr_map,
    mut cb: xcm_attr_map_foreach_cb,
    mut user: *mut libc::c_void,
) {
    let mut attr: *mut attr = 0 as *mut attr;
    attr = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        cb
            .expect(
                "non-null function pointer",
            )((*attr).name, (*attr).type_0, (*attr).value, (*attr).value_len, user);
        attr = (*attr).entry.le_next;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_equal(
    mut attr_map_a: *const xcm_attr_map,
    mut attr_map_b: *const xcm_attr_map,
) -> bool {
    let mut size_a: size_t = xcm_attr_map_size(attr_map_a);
    let mut size_b: size_t = xcm_attr_map_size(attr_map_b);
    if size_a != size_b {
        return 0 as libc::c_int != 0;
    }
    let mut attr_a: *mut attr = 0 as *mut attr;
    attr_a = (*attr_map_a).attrs.lh_first;
    while !attr_a.is_null() {
        let mut attr_b: *const attr = lookup_attr_with_type(
            attr_map_b,
            (*attr_a).name,
            (*attr_a).type_0,
        );
        if attr_b.is_null() {
            return 0 as libc::c_int != 0;
        }
        if (*attr_a).value_len != (*attr_b).value_len {
            return 0 as libc::c_int != 0;
        }
        if memcmp((*attr_a).value, (*attr_b).value, (*attr_a).value_len)
            != 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        attr_a = (*attr_a).entry.le_next;
    }
    return 1 as libc::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_destroy(mut attr_map: *mut xcm_attr_map) {
    if !attr_map.is_null() {
        let mut attr: *mut attr = 0 as *mut attr;
        loop {
            attr = (*attr_map).attrs.lh_first;
            if attr.is_null() {
                break;
            }
            if !((*attr).entry.le_next).is_null() {
                (*(*attr).entry.le_next).entry.le_prev = (*attr).entry.le_prev;
            }
            *(*attr).entry.le_prev = (*attr).entry.le_next;
            attr_destroy(attr);
        }
        ut_free(attr_map as *mut libc::c_void);
    }
}
