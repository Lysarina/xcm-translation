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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_calloc(size: size_t) -> *mut libc::c_void;
    fn ut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
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
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
pub type uint64_t = __uint64_t;
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
pub type attr_pcomp_type = libc::c_uint;
pub const attr_pcomp_type_index: attr_pcomp_type = 1;
pub const attr_pcomp_type_key: attr_pcomp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_pcomp {
    pub type_0: attr_pcomp_type,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub key: *mut libc::c_char,
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_path {
    pub comps: [*mut attr_pcomp; 64],
    pub num_comps: size_t,
}
unsafe extern "C" fn is_special(mut c: libc::c_char) -> bool {
    match c as libc::c_int {
        91 | 93 | 46 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn is_key_char(mut c: libc::c_char) -> bool {
    // print!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    return !is_special(c);
}
unsafe extern "C" fn attr_path_key_create(
    mut key: *const libc::c_char,
) -> *mut attr_pcomp {
    let mut comp: *mut attr_pcomp = ut_malloc(
        ::core::mem::size_of::<attr_pcomp>() as libc::c_ulong,
    ) as *mut attr_pcomp;
    *comp = {
        let mut init = attr_pcomp {
            type_0: attr_pcomp_type_key,
            c2rust_unnamed: C2RustUnnamed {
                key: ut_strdup(key),
            },
        };
        init
    };
    return comp;
}
unsafe extern "C" fn attr_path_index_create(mut index: size_t) -> *mut attr_pcomp {
    let mut comp: *mut attr_pcomp = ut_malloc(
        ::core::mem::size_of::<attr_pcomp>() as libc::c_ulong,
    ) as *mut attr_pcomp;
    *comp = {
        let mut init = attr_pcomp {
            type_0: attr_pcomp_type_index,
            c2rust_unnamed: C2RustUnnamed { index: index },
        };
        init
    };
    return comp;
}
unsafe extern "C" fn attr_pcomp_destroy(mut comp: *mut attr_pcomp) {
    if !comp.is_null() {
        if (*comp).type_0 as libc::c_uint
            == attr_pcomp_type_key as libc::c_int as libc::c_uint
        {
            ut_free((*comp).c2rust_unnamed.key as *mut libc::c_void);
        }
        ut_free(comp as *mut libc::c_void);
    }
}
unsafe extern "C" fn attr_pcomp_equal(
    mut comp_a: *mut attr_pcomp,
    mut comp_b: *mut attr_pcomp,
) -> bool {
    if (*comp_a).type_0 as libc::c_uint != (*comp_b).type_0 as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if (*comp_a).type_0 as libc::c_uint
        == attr_pcomp_type_key as libc::c_int as libc::c_uint
    {
        return strcmp((*comp_a).c2rust_unnamed.key, (*comp_b).c2rust_unnamed.key)
            == 0 as libc::c_int
    } else {
        return (*comp_a).c2rust_unnamed.index == (*comp_b).c2rust_unnamed.index
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_get_type(
    mut pcomp: *const attr_pcomp,
) -> attr_pcomp_type {
    return (*pcomp).type_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_is_key(mut pcomp: *const attr_pcomp) -> bool {
    return (*pcomp).type_0 as libc::c_uint
        == attr_pcomp_type_key as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_is_index(mut pcomp: *const attr_pcomp) -> bool {
    return (*pcomp).type_0 as libc::c_uint
        == attr_pcomp_type_index as libc::c_int as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_get_key(
    mut pcomp: *const attr_pcomp,
) -> *const libc::c_char {
    if !attr_pcomp_is_key(pcomp) {
        // log_console_conf(true);
        // if log_is_enabled(log_type_error) {
        //     __log_event(
        //         log_type_error,
        //         b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
        //             as *const u8 as *const libc::c_char,
        //         100 as libc::c_int,
        //         (*::core::mem::transmute::<
        //             &[u8; 19],
        //             &[libc::c_char; 19],
        //         >(b"attr_pcomp_get_key\0"))
        //             .as_ptr(),
        //         0 as *mut xcm_socket,
        //         b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
        //         b"attr_pcomp_is_key(pcomp)\0" as *const u8 as *const libc::c_char,
        //     );
        // }
        abort();
    }
    return (*pcomp).c2rust_unnamed.key;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_get_index(mut pcomp: *const attr_pcomp) -> size_t {
    if !attr_pcomp_is_index(pcomp) {
        // log_console_conf(1 as libc::c_int != 0);
        // if log_is_enabled(log_type_error) {
        //     __log_event(
        //         log_type_error,
        //         b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
        //             as *const u8 as *const libc::c_char,
        //         107 as libc::c_int,
        //         (*::core::mem::transmute::<
        //             &[u8; 21],
        //             &[libc::c_char; 21],
        //         >(b"attr_pcomp_get_index\0"))
        //             .as_ptr(),
        //         0 as *mut xcm_socket,
        //         b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
        //         b"attr_pcomp_is_index(pcomp)\0" as *const u8 as *const libc::c_char,
        //     );
        // }
        abort();
    }
    return (*pcomp).c2rust_unnamed.index;
}
unsafe extern "C" fn attr_pcomp_parse_key(
    mut path_str: *const libc::c_char,
    mut comp: *mut *mut attr_pcomp,
) -> libc::c_int {
    let mut key: [libc::c_char; 256] = [0; 256];
    let mut key_len: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut c: libc::c_char = *path_str.offset(key_len as isize);
        if c as libc::c_int == '\0' as i32 || !is_key_char(c) {
            break;
        }
        key[key_len as usize] = c;
        key_len = key_len.wrapping_add(1);
        key_len;
    }
    if key_len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    key[key_len as usize] = '\0' as i32 as libc::c_char;
    *comp = attr_path_key_create(key.as_mut_ptr());
    return key_len as libc::c_int;
}
unsafe extern "C" fn attr_pcomp_parse_index(
    mut path_str: *const libc::c_char,
    mut comp: *mut *mut attr_pcomp,
) -> libc::c_int {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_long = strtol(path_str, &mut end, 10 as libc::c_int);
    if *end as libc::c_int != ']' as i32 || end == path_str as *mut libc::c_char
        || index < 0 as libc::c_int as libc::c_long
        || index == 9223372036854775807 as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    *comp = attr_path_index_create(index as size_t);
    return (end.offset_from(path_str) as libc::c_long + 1 as libc::c_int as libc::c_long)
        as libc::c_int;
}
unsafe extern "C" fn attr_pcomp_parse(
    mut path_str: *const libc::c_char,
    mut comp: *mut *mut attr_pcomp,
) -> libc::c_int {
    // print!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    if strlen(path_str) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut c: libc::c_char = *path_str.offset(0 as libc::c_int as isize);
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if c as libc::c_int == '[' as i32 {
        rc = attr_pcomp_parse_index(path_str.offset(1 as libc::c_int as isize), comp);
    } else if c as libc::c_int == '.' as i32 {
        rc = attr_pcomp_parse_key(path_str.offset(1 as libc::c_int as isize), comp);
    }
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return rc + 1 as libc::c_int;
}
unsafe extern "C" fn attr_pcomp_parse_root(
    mut path_str: *const libc::c_char,
    mut comp: *mut *mut attr_pcomp,
) -> libc::c_int {
    // print!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    return attr_pcomp_parse_key(path_str, comp);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_parse(
    mut path_str: *const libc::c_char,
    mut root: bool,
) -> *mut attr_path {
    if strlen(path_str) > 255 as libc::c_int as libc::c_ulong {
        return 0 as *mut attr_path;
    }
    let mut path: *mut attr_path = ut_calloc(
        ::core::mem::size_of::<attr_path>() as libc::c_ulong,
    ) as *mut attr_path;
    let mut offset: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut comp: *mut *mut attr_pcomp = &mut *((*path).comps)
            .as_mut_ptr()
            .offset((*path).num_comps as isize) as *mut *mut attr_pcomp;
        if offset == strlen(path_str) {
            break;
        }
        let mut rc: libc::c_int = if root as libc::c_int != 0 {
            attr_pcomp_parse_root(path_str, comp)
        } else {
            attr_pcomp_parse(path_str.offset(offset as isize), comp)
        };
        root = 0 as libc::c_int != 0;
        if rc < 0 as libc::c_int {
            attr_path_destroy(path);
            return 0 as *mut attr_path;
        }
        if rc == 0 as libc::c_int {
            break;
        }
        (*path).num_comps = ((*path).num_comps).wrapping_add(1);
        (*path).num_comps;
        offset = (offset as libc::c_ulong).wrapping_add(rc as libc::c_ulong) as size_t
            as size_t;
    }
    return path;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_destroy(mut path: *mut attr_path) {
    if !path.is_null() {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*path).num_comps {
            attr_pcomp_destroy((*path).comps[i as usize]);
            i = i.wrapping_add(1);
            i;
        }
        ut_free(path as *mut libc::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_num_comps(mut path: *const attr_path) -> size_t {
    return (*path).num_comps;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_get_comp(
    mut path: *const attr_path,
    mut comp_num: size_t,
) -> *const attr_pcomp {
    if !(comp_num < (*path).num_comps) {
        // log_console_conf(1 as libc::c_int != 0);
        // if log_is_enabled(log_type_error) {
        //     __log_event(
        //         log_type_error,
        //         b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
        //             as *const u8 as *const libc::c_char,
        //         242 as libc::c_int,
        //         (*::core::mem::transmute::<
        //             &[u8; 19],
        //             &[libc::c_char; 19],
        //         >(b"attr_path_get_comp\0"))
        //             .as_ptr(),
        //         0 as *mut xcm_socket,
        //         b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
        //         b"comp_num < path->num_comps\0" as *const u8 as *const libc::c_char,
        //     );
        // }
        abort();
    }
    return (*path).comps[comp_num as usize];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_len(
    mut path: *const attr_path,
    mut root: bool,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*path).num_comps {
        let mut comp: *mut attr_pcomp = (*path).comps[i as usize];
        if root {
            if !((*comp).type_0 as libc::c_uint
                == attr_pcomp_type_key as libc::c_int as libc::c_uint)
            {
                // log_console_conf(1 as libc::c_int != 0);
                // if log_is_enabled(log_type_error) {
                //     __log_event(
                //         log_type_error,
                //         b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
                //             as *const u8 as *const libc::c_char,
                //         255 as libc::c_int,
                //         (*::core::mem::transmute::<
                //             &[u8; 14],
                //             &[libc::c_char; 14],
                //         >(b"attr_path_len\0"))
                //             .as_ptr(),
                //         0 as *mut xcm_socket,
                //         b"Assertion \"%s\" failed.\n\0" as *const u8
                //             as *const libc::c_char,
                //         b"comp->type == attr_pcomp_type_key\0" as *const u8
                //             as *const libc::c_char,
                //     );
                // }
                abort();
            }
            len = (len as libc::c_ulong).wrapping_add(strlen((*comp).c2rust_unnamed.key))
                as size_t as size_t;
            root = 0 as libc::c_int != 0;
        } else if (*comp).type_0 as libc::c_uint
            == attr_pcomp_type_key as libc::c_int as libc::c_uint
        {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen((*comp).c2rust_unnamed.key))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        } else {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (snprintf(
                        0 as *mut libc::c_char,
                        0 as libc::c_int as libc::c_ulong,
                        b"%zd\0" as *const u8 as *const libc::c_char,
                        (*comp).c2rust_unnamed.index,
                    ) + 2 as libc::c_int) as libc::c_ulong,
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return len;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_equal(
    mut path_a: *const attr_path,
    mut path_b: *const attr_path,
) -> bool {
    if (*path_a).num_comps != (*path_b).num_comps {
        return 0 as libc::c_int != 0;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*path_a).num_comps {
        if !attr_pcomp_equal((*path_a).comps[i as usize], (*path_b).comps[i as usize]) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_equal_str(
    mut path: *const attr_path,
    mut path_str: *const libc::c_char,
    mut root: bool,
) -> bool {
    let mut other_path: *mut attr_path = attr_path_parse(path_str, root);
    if other_path.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut equal: bool = attr_path_equal(path, other_path);
    attr_path_destroy(other_path);
    return equal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_to_str(
    mut path: *const attr_path,
    mut root: bool,
) -> *mut libc::c_char {
    let mut capacity: size_t = (attr_path_len(path, root))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut str: *mut libc::c_char = ut_malloc(capacity) as *mut libc::c_char;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*path).num_comps {
        let mut comp: *mut attr_pcomp = (*path).comps[i as usize];
        if root {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    snprintf(
                        str,
                        capacity,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*comp).c2rust_unnamed.key,
                    ) as libc::c_ulong,
                ) as size_t as size_t;
            root = 0 as libc::c_int != 0;
        } else if (*comp).type_0 as libc::c_uint
            == attr_pcomp_type_key as libc::c_int as libc::c_uint
        {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    snprintf(
                        str.offset(len as isize),
                        capacity.wrapping_sub(len),
                        b"%c%s\0" as *const u8 as *const libc::c_char,
                        '.' as i32,
                        (*comp).c2rust_unnamed.key,
                    ) as libc::c_ulong,
                ) as size_t as size_t;
        } else {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    snprintf(
                        str.offset(len as isize),
                        capacity.wrapping_sub(len),
                        b"%c%zd%c\0" as *const u8 as *const libc::c_char,
                        '[' as i32,
                        (*comp).c2rust_unnamed.index,
                        ']' as i32,
                    ) as libc::c_ulong,
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    *str.offset(len as isize) = '\0' as i32 as libc::c_char;
    return str;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_is_valid_key(mut key: *const libc::c_char) -> bool {
    if strlen(key) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < strlen(key) {
        let mut c: libc::c_char = *key.offset(i as isize);
        if !is_key_char(c) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int != 0;
}
