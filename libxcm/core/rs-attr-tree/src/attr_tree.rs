#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    dead_code,
    unused_mut,
    unsafe_code,
    clippy::missing_safety_doc,
)]
#![feature(extern_types)]

use std::process::abort;
use libc::{strerror, __errno_location, strlen};

use rs_attr_node::*;
use rs_attr_path::*;
use rs_log_attr_tree::*;
use rs_util::*;
use rs_log::*;

use xcm_rust_common::xcm_tp::xcm_socket;
use xcm_rust_common::xcm_attr::*;
use xcm_rust_common::attr_node_mod::*;
// use xcm_rust_common::attr_node_mod::{attr_node, C2RustUnnamed, attr_node_list,
//     attr_node_list_elem, C2RustUnnamed_0, C2RustUnnamed_1, attr_node_dict,
//     attr_node_dict_elem, attr_node_value, attr_get, attr_set, attr_node_type_value,
//     attr_node_type_dict, attr_node_type_list, attr_list_foreach_cb, attr_dict_foreach_cb,
//     attr_node_type};
use xcm_rust_common::attr_tree_mod::*;

unsafe extern "C" {

    // pub type ctl;
    //pub type xpoll;

    // fn log_attr_str_value(
    //     type_0: xcm_attr_type,
    //     value: *const libc::c_void,
    //     len: libc::c_ulong,
    //     buf: *mut libc::c_char,
    //     capacity: libc::c_ulong,
    // );
    // fn log_attr_type_name(type_0: xcm_attr_type) -> *const libc::c_char;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct foreach_param {
    pub path: *const libc::c_char,
    pub cb: xcm_attr_cb,
    pub cb_data: *mut libc::c_void,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_create() -> *mut attr_tree { unsafe {
    let mut tree: *mut attr_tree = ut_malloc(
        ::core::mem::size_of::<attr_tree>() as libc::c_ulong,
    ) as *mut attr_tree;
    *tree = {
        
        attr_tree {
            root: attr_node_dict(),
        }
    };
    tree
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_destroy(mut tree: *mut attr_tree) { unsafe {
    if !tree.is_null() {
        attr_node_destroy((*tree).root);
        ut_free(tree as *mut libc::c_void);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_root(mut tree: *mut attr_tree) -> *mut attr_node { unsafe {
    (*tree).root
}}
unsafe extern "C" fn ensure_containers(
    mut tree: *mut attr_tree,
    mut path: *const attr_path,
) -> *mut attr_node { unsafe {
    let mut i: libc::c_ulong = 0;
    let mut container: *mut attr_node = (*tree).root;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (attr_path_num_comps(path)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut comp: *const attr_pcomp = attr_path_get_comp(path, i);
        let mut next_comp: *const attr_pcomp = attr_path_get_comp(
            path,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        let mut contained_type: attr_pcomp_type = attr_pcomp_get_type(next_comp);
        let mut next_container: *mut attr_node = std::ptr::null_mut::<attr_node>();
        if attr_node_is_dict(container) {
            let mut key: *const libc::c_char = attr_pcomp_get_key(comp);
            next_container = attr_node_dict_get_key(container, key);
            if next_container.is_null() {
                next_container = if contained_type as libc::c_uint
                    == attr_pcomp_type_key as libc::c_int as libc::c_uint
                {
                    attr_node_dict()
                } else {
                    attr_node_list()
                };
                attr_node_dict_add_key(container, key, next_container);
            }
        } else {
            if !attr_node_is_list(container) {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                            as *const u8 as *const libc::c_char,
                        65 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 18],
                            &[libc::c_char; 18],
                        >(b"ensure_containers\0"))
                            .as_ptr(),
                        std::ptr::null_mut::<xcm_socket>(),
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"attr_node_is_list(container)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
            let mut index: libc::c_ulong = attr_pcomp_get_index(comp);
            let mut list_len: libc::c_ulong = attr_node_list_len(container);
            if index < list_len {
                next_container = attr_node_list_get_index(container, index);
            } else {
                if index != list_len {
                    log_console_conf(1 as libc::c_int != 0);
                    if log_is_enabled(log_type_error) {
                        __log_event(
                            log_type_error,
                            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                                as *const u8 as *const libc::c_char,
                            75 as libc::c_int,
                            (*::core::mem::transmute::<
                                &[u8; 18],
                                &[libc::c_char; 18],
                            >(b"ensure_containers\0"))
                                .as_ptr(),
                            std::ptr::null_mut::<xcm_socket>(),
                            b"Assertion \"%s\" failed.\n\0" as *const u8
                                as *const libc::c_char,
                            b"index == list_len\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    abort();
                }
                next_container = if contained_type as libc::c_uint
                    == attr_pcomp_type_key as libc::c_int as libc::c_uint
                {
                    attr_node_dict()
                } else {
                    attr_node_list()
                };
                attr_node_list_append(container, next_container);
            }
        }
        container = next_container;
        i = i.wrapping_add(1);
    }
    container
}}
unsafe extern "C" fn add_node(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut node: *mut attr_node,
) { unsafe {
    let mut path: *mut attr_path = attr_path_parse(path_str, 1 as libc::c_int != 0);
    if !attr_pcomp_is_key(attr_path_get_comp(path, 0 as libc::c_int as libc::c_ulong)) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                94 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_node\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_pcomp_is_key(attr_path_get_comp(path, 0))\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut container: *mut attr_node = ensure_containers(tree, path);
    let mut last: libc::c_ulong = (attr_path_num_comps(path))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut comp: *const attr_pcomp = attr_path_get_comp(path, last);
    if attr_pcomp_is_index(comp) {
        attr_node_list_append(container, node);
    } else {
        let mut key: *const libc::c_char = attr_pcomp_get_key(comp);
        attr_node_dict_add_key(container, key, node);
    }
    attr_path_destroy(path);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_add_value_node(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut s: *mut xcm_socket,
    mut context: *mut libc::c_void,
    mut type_0: xcm_attr_type,
    mut set: attr_set,
    mut get: attr_get,
) { unsafe {
    let mut value_node: *mut attr_node = attr_node_value(s, context, type_0, set, get);
    add_node(tree, path_str, value_node);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_add_list_node(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
) -> *mut attr_node { unsafe {
    let mut list_node: *mut attr_node = attr_node_list();
    add_node(tree, path_str, list_node);
    list_node
}}
unsafe extern "C" fn valid_set_attr_len(
    mut type_0: xcm_attr_type,
    mut len: libc::c_ulong,
) -> bool { unsafe {
    match type_0 as libc::c_uint {
        1 => return len == ::core::mem::size_of::<bool>() as libc::c_ulong,
        2 => return len == ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
        5 => return len == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        3 => return len > 0 as libc::c_int as libc::c_ulong,
        4 => return 1 as libc::c_int != 0,
        _ => {
            if 0 as libc::c_int == 0 {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                            as *const u8 as *const libc::c_char,
                        146 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"valid_set_attr_len\0"))
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
    }
    panic!("Reached end of non-void function without returning");
}}
unsafe extern "C" fn node_lookup(
    mut root: *mut attr_node,
    mut path: *const attr_path,
) -> *mut attr_node { unsafe {
    let mut i: libc::c_ulong = 0;
    let mut node: *mut attr_node = root;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < attr_path_num_comps(path) {
        let mut next: *mut attr_node = std::ptr::null_mut::<attr_node>();
        let mut comp: *const attr_pcomp = attr_path_get_comp(path, i);
        if attr_pcomp_is_key(comp) as libc::c_int != 0
            && attr_node_is_dict(node) as libc::c_int != 0
        {
            let mut key: *const libc::c_char = attr_pcomp_get_key(comp);
            next = attr_node_dict_get_key(node, key);
        } else if attr_pcomp_is_index(comp) as libc::c_int != 0
            && attr_node_is_list(node) as libc::c_int != 0
        {
            let mut index: libc::c_ulong = attr_pcomp_get_index(comp);
            if index < attr_node_list_len(node) {
                next = attr_node_list_get_index(node, index);
            }
        }
        if next.is_null() {
            return std::ptr::null_mut::<attr_node>();
        }
        node = next;
        i = i.wrapping_add(1);
    }
    node
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_set_value(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut type_0: xcm_attr_type,
    mut value: *const libc::c_void,
    mut len: libc::c_ulong,
    mut log_ref: *mut libc::c_void,
) -> libc::c_int { unsafe {
    if !valid_set_attr_len(type_0, len) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                183 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attempt to set attribute \"%s\" to value with invalid length %zd bytes.\0"
                    as *const u8 as *const libc::c_char,
                path_str,
                len,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut path: *mut attr_path = attr_path_parse(path_str, 1 as libc::c_int != 0);
    if path.is_null() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                191 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute \"%s\" has invalid syntax.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut value_node: *mut attr_node = node_lookup((*tree).root, path);
    attr_path_destroy(path);
    if value_node.is_null() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                201 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute \"%s\" does not exist.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !attr_node_is_value(value_node) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                207 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute at \"%s\" is a dictionary or list.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 13 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !attr_node_value_is_writable(value_node) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                213 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute is not writable.\0" as *const u8 as *const libc::c_char,
            );
        }
        *__errno_location() = 13 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if attr_node_value_get_value_type(value_node) as libc::c_uint
        != type_0 as libc::c_uint
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                220 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute is of type %s, but new value of type %s.\0" as *const u8
                    as *const libc::c_char,
                log_attr_type_name(attr_node_get_type(value_node) as xcm_attr_type),
                log_attr_type_name(type_0),
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = attr_node_value_set(value_node, value, len);
    if rc < 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                228 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_set_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Failed to set attribute value; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_value(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut type_0: *mut xcm_attr_type,
    mut value: *mut libc::c_void,
    mut capacity: libc::c_ulong,
    mut log_ref: *mut libc::c_void,
) -> libc::c_int { unsafe {
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                as *const u8 as *const libc::c_char,
            239 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"attr_tree_get_value\0"))
                .as_ptr(),
            log_ref as *mut xcm_socket,
            b"Application getting attribute \"%s\".\0" as *const u8
                as *const libc::c_char,
            path_str,
        );
    }
    let mut path: *mut attr_path = attr_path_parse(path_str, 1 as libc::c_int != 0);
    if path.is_null() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                244 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_get_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute \"%s\" has invalid syntax.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut value_node: *mut attr_node = node_lookup((*tree).root, path);
    attr_path_destroy(path);
    if value_node.is_null() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                254 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_get_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute \"%s\" does not exist.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !attr_node_is_value(value_node) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                260 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_get_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute at \"%s\" is a dictionary or list.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 13 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !attr_node_value_is_readable(value_node) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                266 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_get_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute is not readable.\0" as *const u8 as *const libc::c_char,
            );
        }
        *__errno_location() = 13 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut value_type: xcm_attr_type = attr_node_value_get_value_type(value_node);
    if !type_0.is_null() {
        *type_0 = value_type;
    }
    let mut rc: libc::c_int = attr_node_value_get(value_node, value, capacity);
    if rc < 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                277 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"attr_tree_get_value\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute retrieval failed; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    let mut value_s: [libc::c_char; 4096] = [0; 4096];
    log_attr_str_value(
        value_type,
        value,
        rc as libc::c_ulong,
        value_s.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                as *const u8 as *const libc::c_char,
            281 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"attr_tree_get_value\0"))
                .as_ptr(),
            log_ref as *mut xcm_socket,
            b"Attribute \"%s\" has the value %s.\0" as *const u8 as *const libc::c_char,
            path_str,
            value_s.as_mut_ptr(),
        );
    }
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_list_len(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut log_ref: *mut libc::c_void,
) -> libc::c_int { unsafe {
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                as *const u8 as *const libc::c_char,
            289 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"attr_tree_get_list_len\0"))
                .as_ptr(),
            log_ref as *mut xcm_socket,
            b"Application retrieving list length of attribute \"%s\".\0" as *const u8
                as *const libc::c_char,
            path_str,
        );
    }
    let mut path: *mut attr_path = attr_path_parse(path_str, 1 as libc::c_int != 0);
    if path.is_null() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                294 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_tree_get_list_len\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute \"%s\" has invalid syntax.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut list_node: *mut attr_node = node_lookup((*tree).root, path);
    attr_path_destroy(path);
    if list_node.is_null() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                304 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_tree_get_list_len\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute \"%s\" does not exist.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !attr_node_is_list(list_node) {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                310 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"attr_tree_get_list_len\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Attribute at \"%s\" is not a list.\0" as *const u8
                    as *const libc::c_char,
                path_str,
            );
        }
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut len: libc::c_int = attr_node_list_len(list_node) as libc::c_int;
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                as *const u8 as *const libc::c_char,
            319 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"attr_tree_get_list_len\0"))
                .as_ptr(),
            log_ref as *mut xcm_socket,
            b"Length of \"%s\" is %d.\0" as *const u8 as *const libc::c_char,
            path_str,
            len,
        );
    }
    len
}}
unsafe extern "C" fn visit_value(
    mut path: *const libc::c_char,
    mut value_node: *const attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    if !attr_node_value_is_readable(value_node) {
        return;
    }
    let mut value_capacity: libc::c_ulong = 256 as libc::c_int as libc::c_ulong;
    let mut value: *mut libc::c_char = ut_malloc(value_capacity) as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    loop {
        rc = attr_node_value_get(value_node, value as *mut libc::c_void, value_capacity);
        if !(rc < 0 as libc::c_int && *__errno_location() == 75 as libc::c_int) {
            break;
        }
        value_capacity = (value_capacity as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_ulong as libc::c_ulong;
        value = ut_realloc(value as *mut libc::c_void, value_capacity)
            as *mut libc::c_char;
    }
    if rc >= 0 as libc::c_int {
        cb
            .expect(
                "non-null function pointer",
            )(
            path,
            attr_node_value_get_value_type(value_node),
            value as *mut libc::c_void,
            rc as libc::c_ulong,
            cb_data,
        );
    }
    ut_free(value as *mut libc::c_void);
}}
unsafe extern "C" fn foreach_dict_key(
    mut key: *const libc::c_char,
    mut node: *mut attr_node,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    let mut data: *mut foreach_param = cb_data as *mut foreach_param;
    let mut root: bool = strlen((*data).path) as libc::c_ulong == 0;
    let mut key_path: *mut libc::c_char = if root as libc::c_int != 0 {
        ut_strdup(key)
    } else {
        ut_asprintf(
            b"%s%c%s\0" as *const u8 as *const libc::c_char,
            (*data).path,
            '.' as i32,
            key,
        )
    };
    visit_node(key_path, node, (*data).cb, (*data).cb_data);
    ut_free(key_path as *mut libc::c_void);
}}
unsafe extern "C" fn visit_dict(
    mut path: *const libc::c_char,
    mut dict: *mut attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    let mut param: foreach_param = {
        
        foreach_param {
            path,
            cb,
            cb_data,
        }
    };
    attr_node_dict_foreach(
        dict,
        Some(
            foreach_dict_key
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut attr_node,
                    *mut libc::c_void,
                ) -> (),
        ),
        &mut param as *mut foreach_param as *mut libc::c_void,
    );
}}
unsafe extern "C" fn foreach_list_index(
    mut index: libc::c_ulong,
    mut node: *mut attr_node,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    let mut data: *mut foreach_param = cb_data as *mut foreach_param;
    let mut index_path: *mut libc::c_char = ut_asprintf(
        b"%s%c%zd%c\0" as *const u8 as *const libc::c_char,
        (*data).path,
        '[' as i32,
        index,
        ']' as i32,
    );
    visit_node(index_path, node, (*data).cb, (*data).cb_data);
    ut_free(index_path as *mut libc::c_void);
}}
unsafe extern "C" fn visit_list(
    mut path: *const libc::c_char,
    mut list: *mut attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    let mut param: foreach_param = {
        
        foreach_param {
            path,
            cb,
            cb_data,
        }
    };
    attr_node_list_foreach(
        list,
        Some(
            foreach_list_index
                as unsafe extern "C" fn(libc::c_ulong, *mut attr_node, *mut libc::c_void) -> (),
        ),
        &mut param as *mut foreach_param as *mut libc::c_void,
    );
}}
unsafe extern "C" fn visit_node(
    mut path: *const libc::c_char,
    mut node: *mut attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    match attr_node_get_type(node) as libc::c_uint {
        0 => {
            visit_value(path, node, cb, cb_data);
        }
        1 => {
            visit_dict(path, node, cb, cb_data);
        }
        2 => {
            visit_list(path, node, cb, cb_data);
        }
        _ => {}
    };
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_all(
    mut tree: *mut attr_tree,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) { unsafe {
    visit_node(b"\0" as *const u8 as *const libc::c_char, (*tree).root, cb, cb_data);
}}
