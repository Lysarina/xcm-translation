// Rustlike
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

use rs_attr_path::attr_path_is_valid_key;
use xcm_rust_common::*;

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
        size_t,
    ) -> libc::c_int,
>;
pub type attr_set = Option::<
    unsafe extern "C" fn(
        *mut xcm_socket,
        *mut libc::c_void,
        *const libc::c_void,
        size_t,
    ) -> libc::c_int,
>;
pub type attr_dict_foreach_cb = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut attr_node, *mut libc::c_void) -> (),
>;
pub type attr_list_foreach_cb = Option::<
    unsafe extern "C" fn(size_t, *mut attr_node, *mut libc::c_void) -> (),
>;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value(
    mut s: *mut xcm_socket,
    mut context: *mut libc::c_void,
    mut type_0: xcm_attr_type,
    mut set: attr_set,
    mut get: attr_get,
) -> *mut attr_node {
    let mut node: *mut attr_node = ut_malloc(
        ::core::mem::size_of::<attr_node>() as libc::c_ulong,
    ) as *mut attr_node;
    *node = {
        let mut init = attr_node {
            type_0: attr_node_type_value,
            c2rust_unnamed: C2RustUnnamed {
                value: {
                    let mut init = attr_node_value {
                        type_0: type_0,
                        s: s,
                        context: context,
                        set: set,
                        get: get,
                    };
                    init
                },
            },
        };
        init
    };
    return node;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_get_value_type(
    mut value_node: *const attr_node,
) -> xcm_attr_type {
    if !((*value_node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                70 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"attr_node_value_get_value_type\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    return (*value_node).c2rust_unnamed.value.type_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_is_readable(
    mut value_node: *const attr_node,
) -> bool {
    if !((*value_node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"attr_node_value_is_readable\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    return ((*value_node).c2rust_unnamed.value.get).is_some();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_is_writable(
    mut value_node: *const attr_node,
) -> bool {
    if !((*value_node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                84 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"attr_node_value_is_writable\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    return ((*value_node).c2rust_unnamed.value.set).is_some();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_set(
    mut value_node: *const attr_node,
    mut value: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    if !((*value_node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                92 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_node_value_set\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    return ((*value_node).c2rust_unnamed.value.set)
        .expect(
            "non-null function pointer",
        )(
        (*value_node).c2rust_unnamed.value.s,
        (*value_node).c2rust_unnamed.value.context,
        value,
        len,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_get(
    mut value_node: *const attr_node,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
) -> libc::c_int {
    if !((*value_node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_node_value_get\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    return ((*value_node).c2rust_unnamed.value.get)
        .expect(
            "non-null function pointer",
        )(
        (*value_node).c2rust_unnamed.value.s,
        (*value_node).c2rust_unnamed.value.context,
        value,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict() -> *mut attr_node {
    let mut dict: *mut attr_node = ut_malloc(
        ::core::mem::size_of::<attr_node>() as libc::c_ulong,
    ) as *mut attr_node;
    (*dict).type_0 = attr_node_type_dict;
    (*dict).c2rust_unnamed.dict.tqh_first = 0 as *mut attr_node_dict_elem;
    (*dict).c2rust_unnamed.dict.tqh_last = &mut (*dict).c2rust_unnamed.dict.tqh_first;
    return dict;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_add_key(
    mut dict: *mut attr_node,
    mut name: *const libc::c_char,
    mut attr: *mut attr_node,
) {
    if attr_node_dict_has_key(dict, name) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                123 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_node_dict_add_key\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"!attr_node_dict_has_key(dict, name)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut elem: *mut attr_node_dict_elem = ut_malloc(
        ::core::mem::size_of::<attr_node_dict_elem>() as libc::c_ulong,
    ) as *mut attr_node_dict_elem;
    if !attr_path_is_valid_key(name) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                128 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_node_dict_add_key\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_path_is_valid_key(name)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    (*elem).key = ut_strdup(name);
    (*elem).node = attr;
    (*elem).entry.tqe_next = 0 as *mut attr_node_dict_elem;
    (*elem).entry.tqe_prev = (*dict).c2rust_unnamed.dict.tqh_last;
    *(*dict).c2rust_unnamed.dict.tqh_last = elem;
    (*dict).c2rust_unnamed.dict.tqh_last = &mut (*elem).entry.tqe_next;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_has_key(
    mut dict: *mut attr_node,
    mut key: *const libc::c_char,
) -> bool {
    return !(attr_node_dict_get_key(dict, key)).is_null();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_size(mut dict: *mut attr_node) -> size_t {
    if !((*dict).type_0 as libc::c_uint
        == attr_node_type_dict as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                143 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_node_dict_size\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"dict->type == attr_node_type_dict\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
    elem = (*dict).c2rust_unnamed.dict.tqh_first;
    while !elem.is_null() {
        count = count.wrapping_add(1);
        count;
        elem = (*elem).entry.tqe_next;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_get_key(
    mut dict: *mut attr_node,
    mut key: *const libc::c_char,
) -> *mut attr_node {
    if !((*dict).type_0 as libc::c_uint
        == attr_node_type_dict as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                156 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_node_dict_get_key\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"dict->type == attr_node_type_dict\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
    elem = (*dict).c2rust_unnamed.dict.tqh_first;
    while !elem.is_null() {
        if strcmp((*elem).key, key) == 0 as libc::c_int {
            return (*elem).node;
        }
        elem = (*elem).entry.tqe_next;
    }
    return 0 as *mut attr_node;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_foreach(
    mut dict: *mut attr_node,
    mut cb: attr_dict_foreach_cb,
    mut cb_data: *mut libc::c_void,
) {
    if !((*dict).type_0 as libc::c_uint
        == attr_node_type_dict as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                169 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_node_dict_foreach\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"dict->type == attr_node_type_dict\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
    elem = (*dict).c2rust_unnamed.dict.tqh_first;
    while !elem.is_null() {
        cb.expect("non-null function pointer")((*elem).key, (*elem).node, cb_data);
        elem = (*elem).entry.tqe_next;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list() -> *mut attr_node {
    let mut list: *mut attr_node = ut_malloc(
        ::core::mem::size_of::<attr_node>() as libc::c_ulong,
    ) as *mut attr_node;
    (*list).type_0 = attr_node_type_list;
    (*list).c2rust_unnamed.list.tqh_first = 0 as *mut attr_node_list_elem;
    (*list).c2rust_unnamed.list.tqh_last = &mut (*list).c2rust_unnamed.list.tqh_first;
    return list;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_append(
    mut list: *mut attr_node,
    mut attr: *mut attr_node,
) {
    if !((*list).type_0 as libc::c_uint
        == attr_node_type_list as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                189 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"attr_node_list_append\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"list->type == attr_node_type_list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut elem: *mut attr_node_list_elem = ut_malloc(
        ::core::mem::size_of::<attr_node_list_elem>() as libc::c_ulong,
    ) as *mut attr_node_list_elem;
    (*elem).node = attr;
    (*elem).entry.tqe_next = 0 as *mut attr_node_list_elem;
    (*elem).entry.tqe_prev = (*list).c2rust_unnamed.list.tqh_last;
    *(*list).c2rust_unnamed.list.tqh_last = elem;
    (*list).c2rust_unnamed.list.tqh_last = &mut (*elem).entry.tqe_next;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_len(mut list: *mut attr_node) -> size_t {
    if !((*list).type_0 as libc::c_uint
        == attr_node_type_list as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                201 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"attr_node_list_len\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"list->type == attr_node_type_list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut elem: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
    elem = (*list).c2rust_unnamed.list.tqh_first;
    while !elem.is_null() {
        count = count.wrapping_add(1);
        count;
        elem = (*elem).entry.tqe_next;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_get_index(
    mut list: *mut attr_node,
    mut index: size_t,
) -> *mut attr_node {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut elem: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
    elem = (*list).c2rust_unnamed.list.tqh_first;
    while !elem.is_null() {
        let fresh0 = count;
        count = count.wrapping_add(1);
        if fresh0 == index {
            return (*elem).node;
        }
        elem = (*elem).entry.tqe_next;
    }
    return 0 as *mut attr_node;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_foreach(
    mut list: *mut attr_node,
    mut cb: attr_list_foreach_cb,
    mut cb_data: *mut libc::c_void,
) {
    if !((*list).type_0 as libc::c_uint
        == attr_node_type_list as libc::c_int as libc::c_uint)
    {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
                    as *const u8 as *const libc::c_char,
                228 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_node_list_foreach\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"list->type == attr_node_type_list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut index: size_t = 0 as libc::c_int as size_t;
    let mut elem: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
    elem = (*list).c2rust_unnamed.list.tqh_first;
    while !elem.is_null() {
        let fresh1 = index;
        index = index.wrapping_add(1);
        cb.expect("non-null function pointer")(fresh1, (*elem).node, cb_data);
        elem = (*elem).entry.tqe_next;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_get_type(
    mut node: *const attr_node,
) -> attr_node_type {
    return (*node).type_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_is_value(mut node: *const attr_node) -> bool {
    return (*node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_is_dict(mut node: *const attr_node) -> bool {
    return (*node).type_0 as libc::c_uint
        == attr_node_type_dict as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_is_list(mut node: *const attr_node) -> bool {
    return (*node).type_0 as libc::c_uint
        == attr_node_type_list as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_destroy(mut node: *mut attr_node) {
    if !node.is_null() {
        if (*node).type_0 as libc::c_uint
            == attr_node_type_dict as libc::c_int as libc::c_uint
        {
            let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
            loop {
                elem = (*node).c2rust_unnamed.dict.tqh_first;
                if elem.is_null() {
                    break;
                }
                if !((*elem).entry.tqe_next).is_null() {
                    (*(*elem).entry.tqe_next).entry.tqe_prev = (*elem).entry.tqe_prev;
                } else {
                    (*node).c2rust_unnamed.dict.tqh_last = (*elem).entry.tqe_prev;
                }
                *(*elem).entry.tqe_prev = (*elem).entry.tqe_next;
                ut_free((*elem).key as *mut libc::c_void);
                attr_node_destroy((*elem).node);
                ut_free(elem as *mut libc::c_void);
            }
        } else if (*node).type_0 as libc::c_uint
            == attr_node_type_list as libc::c_int as libc::c_uint
        {
            let mut elem_0: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
            loop {
                elem_0 = (*node).c2rust_unnamed.list.tqh_first;
                if elem_0.is_null() {
                    break;
                }
                if !((*elem_0).entry.tqe_next).is_null() {
                    (*(*elem_0).entry.tqe_next)
                        .entry
                        .tqe_prev = (*elem_0).entry.tqe_prev;
                } else {
                    (*node).c2rust_unnamed.list.tqh_last = (*elem_0).entry.tqe_prev;
                }
                *(*elem_0).entry.tqe_prev = (*elem_0).entry.tqe_next;
                attr_node_destroy((*elem_0).node);
                ut_free(elem_0 as *mut libc::c_void);
            }
        }
        ut_free(node as *mut libc::c_void);
    }
}


// // Original (c2rust?)
// #![allow(
//     dead_code,
//     mutable_transmutes,
//     non_camel_case_types,
//     non_snake_case,
//     non_upper_case_globals,
//     unused_assignments,
//     unused_mut
// )]
// #![feature(extern_types)]

// use rs_attr_path::attr_path_is_valid_key;
// unsafe extern "C" {
//     pub type ctl;
//     pub type xpoll;
//     pub type attr_tree;
//     fn abort() -> !;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn ut_malloc(size: size_t) -> *mut libc::c_void;
//     fn ut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
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
//     // fn attr_path_is_valid_key(key: *const libc::c_char) -> bool;
// }
// pub type __int64_t = libc::c_long;
// pub type __uint64_t = libc::c_ulong;
// pub type size_t = libc::c_ulong;
// pub type int64_t = __int64_t;
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
// pub type uint64_t = __uint64_t;
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
// pub type log_type = libc::c_uint;
// pub const log_type_error: log_type = 1;
// pub const log_type_debug: log_type = 0;
// pub type xcm_attr_type = libc::c_uint;
// pub const xcm_attr_type_double: xcm_attr_type = 5;
// pub const xcm_attr_type_bin: xcm_attr_type = 4;
// pub const xcm_attr_type_str: xcm_attr_type = 3;
// pub const xcm_attr_type_int64: xcm_attr_type = 2;
// pub const xcm_attr_type_bool: xcm_attr_type = 1;
// pub type attr_node_type = libc::c_uint;
// pub const attr_node_type_list: attr_node_type = 2;
// pub const attr_node_type_dict: attr_node_type = 1;
// pub const attr_node_type_value: attr_node_type = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct attr_node {
//     pub type_0: attr_node_type,
//     pub c2rust_unnamed: C2RustUnnamed,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed {
//     pub value: attr_node_value,
//     pub dict: attr_node_dict,
//     pub list: attr_node_list,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct attr_node_list {
//     pub tqh_first: *mut attr_node_list_elem,
//     pub tqh_last: *mut *mut attr_node_list_elem,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct attr_node_list_elem {
//     pub node: *mut attr_node,
//     pub entry: C2RustUnnamed_0,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_0 {
//     pub tqe_next: *mut attr_node_list_elem,
//     pub tqe_prev: *mut *mut attr_node_list_elem,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct attr_node_dict {
//     pub tqh_first: *mut attr_node_dict_elem,
//     pub tqh_last: *mut *mut attr_node_dict_elem,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct attr_node_dict_elem {
//     pub key: *mut libc::c_char,
//     pub node: *mut attr_node,
//     pub entry: C2RustUnnamed_1,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_1 {
//     pub tqe_next: *mut attr_node_dict_elem,
//     pub tqe_prev: *mut *mut attr_node_dict_elem,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct attr_node_value {
//     pub type_0: xcm_attr_type,
//     pub s: *mut xcm_socket,
//     pub context: *mut libc::c_void,
//     pub set: attr_set,
//     pub get: attr_get,
// }
// pub type attr_get = Option::<
//     unsafe extern "C" fn(
//         *mut xcm_socket,
//         *mut libc::c_void,
//         *mut libc::c_void,
//         size_t,
//     ) -> libc::c_int,
// >;
// pub type attr_set = Option::<
//     unsafe extern "C" fn(
//         *mut xcm_socket,
//         *mut libc::c_void,
//         *const libc::c_void,
//         size_t,
//     ) -> libc::c_int,
// >;
// pub type attr_dict_foreach_cb = Option::<
//     unsafe extern "C" fn(*const libc::c_char, *mut attr_node, *mut libc::c_void) -> (),
// >;
// pub type attr_list_foreach_cb = Option::<
//     unsafe extern "C" fn(size_t, *mut attr_node, *mut libc::c_void) -> (),
// >;
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_value(
//     mut s: *mut xcm_socket,
//     mut context: *mut libc::c_void,
//     mut type_0: xcm_attr_type,
//     mut set: attr_set,
//     mut get: attr_get,
// ) -> *mut attr_node {
//     let mut node: *mut attr_node = ut_malloc(
//         ::core::mem::size_of::<attr_node>() as libc::c_ulong,
//     ) as *mut attr_node;
//     *node = {
//         let mut init = attr_node {
//             type_0: attr_node_type_value,
//             c2rust_unnamed: C2RustUnnamed {
//                 value: {
//                     let mut init = attr_node_value {
//                         type_0: type_0,
//                         s: s,
//                         context: context,
//                         set: set,
//                         get: get,
//                     };
//                     init
//                 },
//             },
//         };
//         init
//     };
//     return node;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_value_get_value_type(
//     mut value_node: *const attr_node,
// ) -> xcm_attr_type {
//     if !((*value_node).type_0 as libc::c_uint
//         == attr_node_type_value as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 70 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 31],
//                     &[libc::c_char; 31],
//                 >(b"attr_node_value_get_value_type\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"value_node->type == attr_node_type_value\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     return (*value_node).c2rust_unnamed.value.type_0;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_value_is_readable(
//     mut value_node: *const attr_node,
// ) -> bool {
//     if !((*value_node).type_0 as libc::c_uint
//         == attr_node_type_value as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 77 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 28],
//                     &[libc::c_char; 28],
//                 >(b"attr_node_value_is_readable\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"value_node->type == attr_node_type_value\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     return ((*value_node).c2rust_unnamed.value.get).is_some();
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_value_is_writable(
//     mut value_node: *const attr_node,
// ) -> bool {
//     if !((*value_node).type_0 as libc::c_uint
//         == attr_node_type_value as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 84 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 28],
//                     &[libc::c_char; 28],
//                 >(b"attr_node_value_is_writable\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"value_node->type == attr_node_type_value\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     return ((*value_node).c2rust_unnamed.value.set).is_some();
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_value_set(
//     mut value_node: *const attr_node,
//     mut value: *const libc::c_void,
//     mut len: size_t,
// ) -> libc::c_int {
//     if !((*value_node).type_0 as libc::c_uint
//         == attr_node_type_value as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 92 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 20],
//                     &[libc::c_char; 20],
//                 >(b"attr_node_value_set\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"value_node->type == attr_node_type_value\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     return ((*value_node).c2rust_unnamed.value.set)
//         .expect(
//             "non-null function pointer",
//         )(
//         (*value_node).c2rust_unnamed.value.s,
//         (*value_node).c2rust_unnamed.value.context,
//         value,
//         len,
//     );
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_value_get(
//     mut value_node: *const attr_node,
//     mut value: *mut libc::c_void,
//     mut capacity: size_t,
// ) -> libc::c_int {
//     if !((*value_node).type_0 as libc::c_uint
//         == attr_node_type_value as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 102 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 20],
//                     &[libc::c_char; 20],
//                 >(b"attr_node_value_get\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"value_node->type == attr_node_type_value\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     return ((*value_node).c2rust_unnamed.value.get)
//         .expect(
//             "non-null function pointer",
//         )(
//         (*value_node).c2rust_unnamed.value.s,
//         (*value_node).c2rust_unnamed.value.context,
//         value,
//         capacity,
//     );
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_dict() -> *mut attr_node {
//     let mut dict: *mut attr_node = ut_malloc(
//         ::core::mem::size_of::<attr_node>() as libc::c_ulong,
//     ) as *mut attr_node;
//     (*dict).type_0 = attr_node_type_dict;
//     (*dict).c2rust_unnamed.dict.tqh_first = 0 as *mut attr_node_dict_elem;
//     (*dict).c2rust_unnamed.dict.tqh_last = &mut (*dict).c2rust_unnamed.dict.tqh_first;
//     return dict;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_dict_add_key(
//     mut dict: *mut attr_node,
//     mut name: *const libc::c_char,
//     mut attr: *mut attr_node,
// ) {
//     if attr_node_dict_has_key(dict, name) {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 123 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 23],
//                     &[libc::c_char; 23],
//                 >(b"attr_node_dict_add_key\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"!attr_node_dict_has_key(dict, name)\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut elem: *mut attr_node_dict_elem = ut_malloc(
//         ::core::mem::size_of::<attr_node_dict_elem>() as libc::c_ulong,
//     ) as *mut attr_node_dict_elem;
//     if !attr_path_is_valid_key(name) {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 128 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 23],
//                     &[libc::c_char; 23],
//                 >(b"attr_node_dict_add_key\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"attr_path_is_valid_key(name)\0" as *const u8 as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     (*elem).key = ut_strdup(name);
//     (*elem).node = attr;
//     (*elem).entry.tqe_next = 0 as *mut attr_node_dict_elem;
//     (*elem).entry.tqe_prev = (*dict).c2rust_unnamed.dict.tqh_last;
//     *(*dict).c2rust_unnamed.dict.tqh_last = elem;
//     (*dict).c2rust_unnamed.dict.tqh_last = &mut (*elem).entry.tqe_next;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_dict_has_key(
//     mut dict: *mut attr_node,
//     mut key: *const libc::c_char,
// ) -> bool {
//     return !(attr_node_dict_get_key(dict, key)).is_null();
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_dict_size(mut dict: *mut attr_node) -> size_t {
//     if !((*dict).type_0 as libc::c_uint
//         == attr_node_type_dict as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 143 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 20],
//                     &[libc::c_char; 20],
//                 >(b"attr_node_dict_size\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"dict->type == attr_node_type_dict\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut count: size_t = 0 as libc::c_int as size_t;
//     let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
//     elem = (*dict).c2rust_unnamed.dict.tqh_first;
//     while !elem.is_null() {
//         count = count.wrapping_add(1);
//         count;
//         elem = (*elem).entry.tqe_next;
//     }
//     return count;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_dict_get_key(
//     mut dict: *mut attr_node,
//     mut key: *const libc::c_char,
// ) -> *mut attr_node {
//     if !((*dict).type_0 as libc::c_uint
//         == attr_node_type_dict as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 156 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 23],
//                     &[libc::c_char; 23],
//                 >(b"attr_node_dict_get_key\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"dict->type == attr_node_type_dict\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
//     elem = (*dict).c2rust_unnamed.dict.tqh_first;
//     while !elem.is_null() {
//         if strcmp((*elem).key, key) == 0 as libc::c_int {
//             return (*elem).node;
//         }
//         elem = (*elem).entry.tqe_next;
//     }
//     return 0 as *mut attr_node;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_dict_foreach(
//     mut dict: *mut attr_node,
//     mut cb: attr_dict_foreach_cb,
//     mut cb_data: *mut libc::c_void,
// ) {
//     if !((*dict).type_0 as libc::c_uint
//         == attr_node_type_dict as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 169 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 23],
//                     &[libc::c_char; 23],
//                 >(b"attr_node_dict_foreach\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"dict->type == attr_node_type_dict\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
//     elem = (*dict).c2rust_unnamed.dict.tqh_first;
//     while !elem.is_null() {
//         cb.expect("non-null function pointer")((*elem).key, (*elem).node, cb_data);
//         elem = (*elem).entry.tqe_next;
//     }
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_list() -> *mut attr_node {
//     let mut list: *mut attr_node = ut_malloc(
//         ::core::mem::size_of::<attr_node>() as libc::c_ulong,
//     ) as *mut attr_node;
//     (*list).type_0 = attr_node_type_list;
//     (*list).c2rust_unnamed.list.tqh_first = 0 as *mut attr_node_list_elem;
//     (*list).c2rust_unnamed.list.tqh_last = &mut (*list).c2rust_unnamed.list.tqh_first;
//     return list;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_list_append(
//     mut list: *mut attr_node,
//     mut attr: *mut attr_node,
// ) {
//     if !((*list).type_0 as libc::c_uint
//         == attr_node_type_list as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 189 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 22],
//                     &[libc::c_char; 22],
//                 >(b"attr_node_list_append\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"list->type == attr_node_type_list\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut elem: *mut attr_node_list_elem = ut_malloc(
//         ::core::mem::size_of::<attr_node_list_elem>() as libc::c_ulong,
//     ) as *mut attr_node_list_elem;
//     (*elem).node = attr;
//     (*elem).entry.tqe_next = 0 as *mut attr_node_list_elem;
//     (*elem).entry.tqe_prev = (*list).c2rust_unnamed.list.tqh_last;
//     *(*list).c2rust_unnamed.list.tqh_last = elem;
//     (*list).c2rust_unnamed.list.tqh_last = &mut (*elem).entry.tqe_next;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_list_len(mut list: *mut attr_node) -> size_t {
//     if !((*list).type_0 as libc::c_uint
//         == attr_node_type_list as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 201 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 19],
//                     &[libc::c_char; 19],
//                 >(b"attr_node_list_len\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"list->type == attr_node_type_list\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut count: size_t = 0 as libc::c_int as size_t;
//     let mut elem: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
//     elem = (*list).c2rust_unnamed.list.tqh_first;
//     while !elem.is_null() {
//         count = count.wrapping_add(1);
//         count;
//         elem = (*elem).entry.tqe_next;
//     }
//     return count;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_list_get_index(
//     mut list: *mut attr_node,
//     mut index: size_t,
// ) -> *mut attr_node {
//     let mut count: size_t = 0 as libc::c_int as size_t;
//     let mut elem: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
//     elem = (*list).c2rust_unnamed.list.tqh_first;
//     while !elem.is_null() {
//         let fresh0 = count;
//         count = count.wrapping_add(1);
//         if fresh0 == index {
//             return (*elem).node;
//         }
//         elem = (*elem).entry.tqe_next;
//     }
//     return 0 as *mut attr_node;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_list_foreach(
//     mut list: *mut attr_node,
//     mut cb: attr_list_foreach_cb,
//     mut cb_data: *mut libc::c_void,
// ) {
//     if !((*list).type_0 as libc::c_uint
//         == attr_node_type_list as libc::c_int as libc::c_uint)
//     {
//         log_console_conf(1 as libc::c_int != 0);
//         if log_is_enabled(log_type_error) {
//             __log_event(
//                 log_type_error,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_node.c\0"
//                     as *const u8 as *const libc::c_char,
//                 228 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 23],
//                     &[libc::c_char; 23],
//                 >(b"attr_node_list_foreach\0"))
//                     .as_ptr(),
//                 0 as *mut xcm_socket,
//                 b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
//                 b"list->type == attr_node_type_list\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//         abort();
//     }
//     let mut index: size_t = 0 as libc::c_int as size_t;
//     let mut elem: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
//     elem = (*list).c2rust_unnamed.list.tqh_first;
//     while !elem.is_null() {
//         let fresh1 = index;
//         index = index.wrapping_add(1);
//         cb.expect("non-null function pointer")(fresh1, (*elem).node, cb_data);
//         elem = (*elem).entry.tqe_next;
//     }
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_get_type(
//     mut node: *const attr_node,
// ) -> attr_node_type {
//     return (*node).type_0;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_is_value(mut node: *const attr_node) -> bool {
//     return (*node).type_0 as libc::c_uint
//         == attr_node_type_value as libc::c_int as libc::c_uint;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_is_dict(mut node: *const attr_node) -> bool {
//     return (*node).type_0 as libc::c_uint
//         == attr_node_type_dict as libc::c_int as libc::c_uint;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_is_list(mut node: *const attr_node) -> bool {
//     return (*node).type_0 as libc::c_uint
//         == attr_node_type_list as libc::c_int as libc::c_uint;
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn attr_node_destroy(mut node: *mut attr_node) {
//     if !node.is_null() {
//         if (*node).type_0 as libc::c_uint
//             == attr_node_type_dict as libc::c_int as libc::c_uint
//         {
//             let mut elem: *mut attr_node_dict_elem = 0 as *mut attr_node_dict_elem;
//             loop {
//                 elem = (*node).c2rust_unnamed.dict.tqh_first;
//                 if elem.is_null() {
//                     break;
//                 }
//                 if !((*elem).entry.tqe_next).is_null() {
//                     (*(*elem).entry.tqe_next).entry.tqe_prev = (*elem).entry.tqe_prev;
//                 } else {
//                     (*node).c2rust_unnamed.dict.tqh_last = (*elem).entry.tqe_prev;
//                 }
//                 *(*elem).entry.tqe_prev = (*elem).entry.tqe_next;
//                 ut_free((*elem).key as *mut libc::c_void);
//                 attr_node_destroy((*elem).node);
//                 ut_free(elem as *mut libc::c_void);
//             }
//         } else if (*node).type_0 as libc::c_uint
//             == attr_node_type_list as libc::c_int as libc::c_uint
//         {
//             let mut elem_0: *mut attr_node_list_elem = 0 as *mut attr_node_list_elem;
//             loop {
//                 elem_0 = (*node).c2rust_unnamed.list.tqh_first;
//                 if elem_0.is_null() {
//                     break;
//                 }
//                 if !((*elem_0).entry.tqe_next).is_null() {
//                     (*(*elem_0).entry.tqe_next)
//                         .entry
//                         .tqe_prev = (*elem_0).entry.tqe_prev;
//                 } else {
//                     (*node).c2rust_unnamed.list.tqh_last = (*elem_0).entry.tqe_prev;
//                 }
//                 *(*elem_0).entry.tqe_prev = (*elem_0).entry.tqe_next;
//                 attr_node_destroy((*elem_0).node);
//                 ut_free(elem_0 as *mut libc::c_void);
//             }
//         }
//         ut_free(node as *mut libc::c_void);
//     }
// }
