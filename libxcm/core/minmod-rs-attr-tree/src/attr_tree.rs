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

use rs_attr_path::*;
use rs_attr_node::*;
use rs_log_attr_tree::log_attr_type_name;

unsafe extern "C" {
    pub type attr_pcomp;
    pub type attr_path;
    pub type ctl;
    pub type xpoll;
    pub type attr_node;
    fn attr_pcomp_get_type(pcomp: *const attr_pcomp) -> attr_pcomp_type;
    fn attr_pcomp_is_key(pcomp: *const attr_pcomp) -> bool;
    fn attr_pcomp_is_index(pcomp: *const attr_pcomp) -> bool;
    fn attr_pcomp_get_key(pcomp: *const attr_pcomp) -> *const libc::c_char;
    fn attr_pcomp_get_index(pcomp: *const attr_pcomp) -> size_t;
    fn attr_path_parse(path_str: *const libc::c_char, root: bool) -> *mut attr_path;
    fn attr_path_destroy(path: *mut attr_path);
    fn attr_path_num_comps(path: *const attr_path) -> size_t;
    fn attr_path_get_comp(path: *const attr_path, comp_num: size_t) -> *const attr_pcomp;
    fn __errno_location() -> *mut libc::c_int;
    fn attr_node_value(
        s: *mut xcm_socket,
        context: *mut libc::c_void,
        type_0: xcm_attr_type,
        set: attr_set,
        get: attr_get,
    ) -> *mut attr_node;
    fn attr_node_value_get_value_type(value_node: *const attr_node) -> xcm_attr_type;
    fn attr_node_value_is_readable(value_node: *const attr_node) -> bool;
    fn attr_node_value_is_writable(value_node: *const attr_node) -> bool;
    fn attr_node_value_set(
        value_node: *const attr_node,
        value: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn attr_node_value_get(
        value_node: *const attr_node,
        value: *mut libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn attr_node_dict() -> *mut attr_node;
    fn attr_node_dict_add_key(
        dict: *mut attr_node,
        key: *const libc::c_char,
        attr_node: *mut attr_node,
    );
    fn attr_node_dict_get_key(
        dict: *mut attr_node,
        key: *const libc::c_char,
    ) -> *mut attr_node;
    fn attr_node_dict_foreach(
        list: *mut attr_node,
        cb: attr_dict_foreach_cb,
        cb_data: *mut libc::c_void,
    );
    fn attr_node_list() -> *mut attr_node;
    fn attr_node_list_append(list: *mut attr_node, attr: *mut attr_node);
    fn attr_node_list_len(list: *mut attr_node) -> size_t;
    fn attr_node_list_get_index(list: *mut attr_node, index: size_t) -> *mut attr_node;
    fn attr_node_list_foreach(
        list: *mut attr_node,
        cb: attr_list_foreach_cb,
        cb_data: *mut libc::c_void,
    );
    fn attr_node_get_type(node: *const attr_node) -> attr_node_type;
    fn attr_node_is_value(node: *const attr_node) -> bool;
    fn attr_node_is_dict(node: *const attr_node) -> bool;
    fn attr_node_is_list(node: *const attr_node) -> bool;
    fn attr_node_destroy(attr_node: *mut attr_node);
    fn abort() -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn ut_free(ptr: *mut libc::c_void);
    fn ut_asprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
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
    fn log_attr_str_value(
        type_0: xcm_attr_type,
        value: *const libc::c_void,
        len: size_t,
        buf: *mut libc::c_char,
        capacity: size_t,
    );
    // fn log_attr_type_name(type_0: xcm_attr_type) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type attr_pcomp_type = libc::c_uint;
pub const attr_pcomp_type_index: attr_pcomp_type = 1;
pub const attr_pcomp_type_key: attr_pcomp_type = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_tree {
    pub root: *mut attr_node,
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
pub type xcm_attr_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type attr_node_type = libc::c_uint;
pub const attr_node_type_list: attr_node_type = 2;
pub const attr_node_type_dict: attr_node_type = 1;
pub const attr_node_type_value: attr_node_type = 0;
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
pub type attr_dict_foreach_cb = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut attr_node, *mut libc::c_void) -> (),
>;
pub type attr_list_foreach_cb = Option::<
    unsafe extern "C" fn(size_t, *mut attr_node, *mut libc::c_void) -> (),
>;
pub type log_type = libc::c_uint;
pub const log_type_error: log_type = 1;
pub const log_type_debug: log_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct foreach_param {
    pub path: *const libc::c_char,
    pub cb: xcm_attr_cb,
    pub cb_data: *mut libc::c_void,
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_create() -> *mut attr_tree {
    let mut tree: *mut attr_tree = ut_malloc(
        ::core::mem::size_of::<attr_tree>() as libc::c_ulong,
    ) as *mut attr_tree;
    *tree = {
        let mut init = attr_tree {
            root: attr_node_dict(),
        };
        init
    };
    return tree;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_destroy(mut tree: *mut attr_tree) {
    if !tree.is_null() {
        attr_node_destroy((*tree).root);
        ut_free(tree as *mut libc::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_root(mut tree: *mut attr_tree) -> *mut attr_node {
    return (*tree).root;
}
unsafe extern "C" fn ensure_containers(
    mut tree: *mut attr_tree,
    mut path: *const attr_path,
) -> *mut attr_node {
    let mut i: size_t = 0;
    let mut container: *mut attr_node = (*tree).root;
    i = 0 as libc::c_int as size_t;
    while i < (attr_path_num_comps(path)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut comp: *const attr_pcomp = attr_path_get_comp(path, i);
        let mut next_comp: *const attr_pcomp = attr_path_get_comp(
            path,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        let mut contained_type: attr_pcomp_type = attr_pcomp_get_type(next_comp);
        let mut next_container: *mut attr_node = 0 as *mut attr_node;
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
                        0 as *mut xcm_socket,
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"attr_node_is_list(container)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
            let mut index: size_t = attr_pcomp_get_index(comp);
            let mut list_len: size_t = attr_node_list_len(container);
            if index < list_len {
                next_container = attr_node_list_get_index(container, index);
            } else {
                if !(index == list_len) {
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
                            0 as *mut xcm_socket,
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
        i;
    }
    return container;
}
unsafe extern "C" fn add_node(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut node: *mut attr_node,
) {
    let mut path: *mut attr_path = attr_path_parse(path_str, 1 as libc::c_int != 0);
    if !attr_pcomp_is_key(attr_path_get_comp(path, 0 as libc::c_int as size_t)) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_tree.c\0"
                    as *const u8 as *const libc::c_char,
                94 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_node\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_pcomp_is_key(attr_path_get_comp(path, 0))\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut container: *mut attr_node = ensure_containers(tree, path);
    let mut last: size_t = (attr_path_num_comps(path))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut comp: *const attr_pcomp = attr_path_get_comp(path, last);
    if attr_pcomp_is_index(comp) {
        attr_node_list_append(container, node);
    } else {
        let mut key: *const libc::c_char = attr_pcomp_get_key(comp);
        attr_node_dict_add_key(container, key, node);
    }
    attr_path_destroy(path);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_add_value_node(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut s: *mut xcm_socket,
    mut context: *mut libc::c_void,
    mut type_0: xcm_attr_type,
    mut set: attr_set,
    mut get: attr_get,
) {
    let mut value_node: *mut attr_node = attr_node_value(s, context, type_0, set, get);
    add_node(tree, path_str, value_node);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_add_list_node(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
) -> *mut attr_node {
    let mut list_node: *mut attr_node = attr_node_list();
    add_node(tree, path_str, list_node);
    return list_node;
}
unsafe extern "C" fn valid_set_attr_len(
    mut type_0: xcm_attr_type,
    mut len: size_t,
) -> bool {
    match type_0 as libc::c_uint {
        1 => return len == ::core::mem::size_of::<bool>() as libc::c_ulong,
        2 => return len == ::core::mem::size_of::<int64_t>() as libc::c_ulong,
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
                        0 as *mut xcm_socket,
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
}
unsafe extern "C" fn node_lookup(
    mut root: *mut attr_node,
    mut path: *const attr_path,
) -> *mut attr_node {
    let mut i: size_t = 0;
    let mut node: *mut attr_node = root;
    i = 0 as libc::c_int as size_t;
    while i < attr_path_num_comps(path) {
        let mut next: *mut attr_node = 0 as *mut attr_node;
        let mut comp: *const attr_pcomp = attr_path_get_comp(path, i);
        if attr_pcomp_is_key(comp) as libc::c_int != 0
            && attr_node_is_dict(node) as libc::c_int != 0
        {
            let mut key: *const libc::c_char = attr_pcomp_get_key(comp);
            next = attr_node_dict_get_key(node, key);
        } else if attr_pcomp_is_index(comp) as libc::c_int != 0
            && attr_node_is_list(node) as libc::c_int != 0
        {
            let mut index: size_t = attr_pcomp_get_index(comp);
            if index < attr_node_list_len(node) {
                next = attr_node_list_get_index(node, index);
            }
        }
        if next.is_null() {
            return 0 as *mut attr_node;
        }
        node = next;
        i = i.wrapping_add(1);
        i;
    }
    return node;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_set_value(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut type_0: xcm_attr_type,
    mut value: *const libc::c_void,
    mut len: size_t,
    mut log_ref: *mut libc::c_void,
) -> libc::c_int {
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
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_value(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut type_0: *mut xcm_attr_type,
    mut value: *mut libc::c_void,
    mut capacity: size_t,
    mut log_ref: *mut libc::c_void,
) -> libc::c_int {
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
        rc as size_t,
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
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_list_len(
    mut tree: *mut attr_tree,
    mut path_str: *const libc::c_char,
    mut log_ref: *mut libc::c_void,
) -> libc::c_int {
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
    return len;
}
unsafe extern "C" fn visit_value(
    mut path: *const libc::c_char,
    mut value_node: *const attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) {
    if !attr_node_value_is_readable(value_node) {
        return;
    }
    let mut value_capacity: size_t = 256 as libc::c_int as size_t;
    let mut value: *mut libc::c_char = ut_malloc(value_capacity) as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    loop {
        rc = attr_node_value_get(value_node, value as *mut libc::c_void, value_capacity);
        if !(rc < 0 as libc::c_int && *__errno_location() == 75 as libc::c_int) {
            break;
        }
        value_capacity = (value_capacity as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
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
            rc as size_t,
            cb_data,
        );
    }
    ut_free(value as *mut libc::c_void);
}
unsafe extern "C" fn foreach_dict_key(
    mut key: *const libc::c_char,
    mut node: *mut attr_node,
    mut cb_data: *mut libc::c_void,
) {
    let mut data: *mut foreach_param = cb_data as *mut foreach_param;
    let mut root: bool = strlen((*data).path) == 0 as libc::c_int as libc::c_ulong;
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
}
unsafe extern "C" fn visit_dict(
    mut path: *const libc::c_char,
    mut dict: *mut attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) {
    let mut param: foreach_param = {
        let mut init = foreach_param {
            path: path,
            cb: cb,
            cb_data: cb_data,
        };
        init
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
}
unsafe extern "C" fn foreach_list_index(
    mut index: size_t,
    mut node: *mut attr_node,
    mut cb_data: *mut libc::c_void,
) {
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
}
unsafe extern "C" fn visit_list(
    mut path: *const libc::c_char,
    mut list: *mut attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) {
    let mut param: foreach_param = {
        let mut init = foreach_param {
            path: path,
            cb: cb,
            cb_data: cb_data,
        };
        init
    };
    attr_node_list_foreach(
        list,
        Some(
            foreach_list_index
                as unsafe extern "C" fn(size_t, *mut attr_node, *mut libc::c_void) -> (),
        ),
        &mut param as *mut foreach_param as *mut libc::c_void,
    );
}
unsafe extern "C" fn visit_node(
    mut path: *const libc::c_char,
    mut node: *mut attr_node,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) {
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
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_tree_get_all(
    mut tree: *mut attr_tree,
    mut cb: xcm_attr_cb,
    mut cb_data: *mut libc::c_void,
) {
    visit_node(b"\0" as *const u8 as *const libc::c_char, (*tree).root, cb, cb_data);
}
