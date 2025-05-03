// Rustlike
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(extern_types)]

use std::process::abort;
use libc::strcmp;

use xcm_rust_common::xcm_tp::*;
use xcm_rust_common::attr_node_mod::*;
use xcm_rust_common::xcm_attr::*;
use rs_attr_path::attr_path_is_valid_key;
use rs_util::*;
use rs_log::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value(
    s: *mut xcm_socket,
    context: *mut libc::c_void,
    type_0: xcm_attr_type,
    set: attr_set,
    get: attr_get,
) -> *mut attr_node { unsafe {
    let node: *mut attr_node = ut_malloc(
        ::core::mem::size_of::<attr_node>() as libc::c_ulong,
    ) as *mut attr_node;
    *node = {
        
        attr_node {
            type_0: attr_node_type_value,
            c2rust_unnamed: C2RustUnnamed {
                value: {
                    
                    attr_node_value {
                        type_0,
                        s,
                        context,
                        set,
                        get,
                    }
                },
            },
        }
    };
    node
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_get_value_type(
    value_node: *const attr_node,
) -> xcm_attr_type { unsafe {
    if (*value_node).type_0 as libc::c_uint != attr_node_type_value as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    (*value_node).c2rust_unnamed.value.type_0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_is_readable(
    value_node: *const attr_node,
) -> bool { unsafe {
    if (*value_node).type_0 as libc::c_uint != attr_node_type_value as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    ((*value_node).c2rust_unnamed.value.get).is_some()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_is_writable(
    value_node: *const attr_node,
) -> bool { unsafe {
    if (*value_node).type_0 as libc::c_uint != attr_node_type_value as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    ((*value_node).c2rust_unnamed.value.set).is_some()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_set(
    value_node: *const attr_node,
    value: *const libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int { unsafe {
    if (*value_node).type_0 as libc::c_uint != attr_node_type_value as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    ((*value_node).c2rust_unnamed.value.set)
        .expect(
            "non-null function pointer",
        )(
        (*value_node).c2rust_unnamed.value.s,
        (*value_node).c2rust_unnamed.value.context,
        value,
        len,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_value_get(
    value_node: *const attr_node,
    value: *mut libc::c_void,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    if (*value_node).type_0 as libc::c_uint != attr_node_type_value as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"value_node->type == attr_node_type_value\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    ((*value_node).c2rust_unnamed.value.get)
        .expect(
            "non-null function pointer",
        )(
        (*value_node).c2rust_unnamed.value.s,
        (*value_node).c2rust_unnamed.value.context,
        value,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict() -> *mut attr_node { unsafe {
    let dict: *mut attr_node = ut_malloc(
        ::core::mem::size_of::<attr_node>() as libc::c_ulong,
    ) as *mut attr_node;
    (*dict).type_0 = attr_node_type_dict;
    (*dict).c2rust_unnamed.dict.tqh_first = std::ptr::null_mut::<attr_node_dict_elem>();
    (*dict).c2rust_unnamed.dict.tqh_last = &mut (*dict).c2rust_unnamed.dict.tqh_first;
    dict
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_add_key(
    dict: *mut attr_node,
    name: *const libc::c_char,
    attr: *mut attr_node,
) { unsafe {
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"!attr_node_dict_has_key(dict, name)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let elem: *mut attr_node_dict_elem = ut_malloc(
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_path_is_valid_key(name)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    (*elem).key = ut_strdup(name);
    (*elem).node = attr;
    (*elem).entry.tqe_next = std::ptr::null_mut::<attr_node_dict_elem>();
    (*elem).entry.tqe_prev = (*dict).c2rust_unnamed.dict.tqh_last;
    *(*dict).c2rust_unnamed.dict.tqh_last = elem;
    (*dict).c2rust_unnamed.dict.tqh_last = &mut (*elem).entry.tqe_next;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_has_key(
    dict: *mut attr_node,
    key: *const libc::c_char,
) -> bool { unsafe {
    !(attr_node_dict_get_key(dict, key)).is_null()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_size(dict: *mut attr_node) -> libc::c_ulong { unsafe {
    if (*dict).type_0 as libc::c_uint != attr_node_type_dict as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"dict->type == attr_node_type_dict\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    // let mut elem: *mut attr_node_dict_elem = std::ptr::null_mut::<attr_node_dict_elem>();
    let mut elem: *mut attr_node_dict_elem = (*dict).c2rust_unnamed.dict.tqh_first;
    // elem = (*dict).c2rust_unnamed.dict.tqh_first;
    while !elem.is_null() {
        count = count.wrapping_add(1);
        elem = (*elem).entry.tqe_next;
    }
    count
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_get_key(
    dict: *mut attr_node,
    key: *const libc::c_char,
) -> *mut attr_node { unsafe {
    if (*dict).type_0 as libc::c_uint != attr_node_type_dict as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"dict->type == attr_node_type_dict\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut elem = (*dict).c2rust_unnamed.dict.tqh_first;
    while !elem.is_null() {
        if strcmp((*elem).key, key) == 0 as libc::c_int {
            return (*elem).node;
        }
        elem = (*elem).entry.tqe_next;
    }
    std::ptr::null_mut::<attr_node>()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_dict_foreach(
    dict: *mut attr_node,
    cb: attr_dict_foreach_cb,
    cb_data: *mut libc::c_void,
) { unsafe {
    if (*dict).type_0 as libc::c_uint != attr_node_type_dict as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"dict->type == attr_node_type_dict\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut elem = (*dict).c2rust_unnamed.dict.tqh_first;
    while !elem.is_null() {
        cb.expect("non-null function pointer")((*elem).key, (*elem).node, cb_data);
        elem = (*elem).entry.tqe_next;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list() -> *mut attr_node { unsafe {
    let list: *mut attr_node = ut_malloc(
        ::core::mem::size_of::<attr_node>() as libc::c_ulong,
    ) as *mut attr_node;
    (*list).type_0 = attr_node_type_list;
    (*list).c2rust_unnamed.list.tqh_first = std::ptr::null_mut::<attr_node_list_elem>();
    (*list).c2rust_unnamed.list.tqh_last = &mut (*list).c2rust_unnamed.list.tqh_first;
    list
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_append(
    list: *mut attr_node,
    attr: *mut attr_node,
) { unsafe {
    if (*list).type_0 as libc::c_uint != attr_node_type_list as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"list->type == attr_node_type_list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let elem: *mut attr_node_list_elem = ut_malloc(
        ::core::mem::size_of::<attr_node_list_elem>() as libc::c_ulong,
    ) as *mut attr_node_list_elem;
    (*elem).node = attr;
    (*elem).entry.tqe_next = std::ptr::null_mut::<attr_node_list_elem>();
    (*elem).entry.tqe_prev = (*list).c2rust_unnamed.list.tqh_last;
    *(*list).c2rust_unnamed.list.tqh_last = elem;
    (*list).c2rust_unnamed.list.tqh_last = &mut (*elem).entry.tqe_next;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_len(list: *mut attr_node) -> libc::c_ulong { unsafe {
    if (*list).type_0 as libc::c_uint != attr_node_type_list as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"list->type == attr_node_type_list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut elem = (*list).c2rust_unnamed.list.tqh_first;
    while !elem.is_null() {
        count = count.wrapping_add(1);
        elem = (*elem).entry.tqe_next;
    }
    count
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_get_index(
    list: *mut attr_node,
    index: libc::c_ulong,
) -> *mut attr_node { unsafe {
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut elem = (*list).c2rust_unnamed.list.tqh_first;
    while !elem.is_null() {
        let fresh0 = count;
        count = count.wrapping_add(1);
        if fresh0 == index {
            return (*elem).node;
        }
        elem = (*elem).entry.tqe_next;
    }
    std::ptr::null_mut::<attr_node>()
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_list_foreach(
    list: *mut attr_node,
    cb: attr_list_foreach_cb,
    cb_data: *mut libc::c_void,
) { unsafe {
    if (*list).type_0 as libc::c_uint != attr_node_type_list as libc::c_int as libc::c_uint
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"list->type == attr_node_type_list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut index: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut elem = (*list).c2rust_unnamed.list.tqh_first;
    while !elem.is_null() {
        let fresh1 = index;
        index = index.wrapping_add(1);
        cb.expect("non-null function pointer")(fresh1, (*elem).node, cb_data);
        elem = (*elem).entry.tqe_next;
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_get_type(
    node: *const attr_node,
) -> attr_node_type { unsafe {
    (*node).type_0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_is_value(node: *const attr_node) -> bool { unsafe {
    (*node).type_0 as libc::c_uint
        == attr_node_type_value as libc::c_int as libc::c_uint
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_is_dict(node: *const attr_node) -> bool { unsafe {
    (*node).type_0 as libc::c_uint
        == attr_node_type_dict as libc::c_int as libc::c_uint
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_is_list(node: *const attr_node) -> bool { unsafe {
    (*node).type_0 as libc::c_uint
        == attr_node_type_list as libc::c_int as libc::c_uint
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_node_destroy(node: *mut attr_node) { unsafe {
    if !node.is_null() {
        if (*node).type_0 as libc::c_uint
            == attr_node_type_dict as libc::c_int as libc::c_uint
        {
            // let mut elem: *mut attr_node_dict_elem = std::ptr::null_mut::<attr_node_dict_elem>();
            loop {
                let elem = (*node).c2rust_unnamed.dict.tqh_first;
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
            // let mut elem_0: *mut attr_node_list_elem = std::ptr::null_mut::<attr_node_list_elem>();
            loop {
                let elem_0 = (*node).c2rust_unnamed.list.tqh_first;
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
}}