#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(extern_types)]

use xcm_rust_common::c_functions::*;
use xcm_rust_common::xcm_tp::*;
use rs_util::*;
use rs_log::*;

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
    pub index: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_path {
    pub comps: [*mut attr_pcomp; 64],
    pub num_comps: libc::c_ulong,
}
unsafe extern "C" fn is_special(c: libc::c_char) -> bool {
    match c as libc::c_int {
        91 | 93 | 46 => 1 as libc::c_int != 0,
        _ => 0 as libc::c_int != 0,
    }
}
unsafe extern "C" fn is_key_char(c: libc::c_char) -> bool { unsafe {
    !is_special(c)
}}
unsafe extern "C" fn attr_path_key_create(
    key: *const libc::c_char,
) -> *mut attr_pcomp { unsafe {
    let comp: *mut attr_pcomp = ut_malloc(
        ::core::mem::size_of::<attr_pcomp>() as libc::c_ulong,
    ) as *mut attr_pcomp;
    *comp = {
        
        attr_pcomp {
            type_0: attr_pcomp_type_key,
            c2rust_unnamed: C2RustUnnamed {
                key: ut_strdup(key),
            },
        }
    };
    comp
}}
unsafe extern "C" fn attr_path_index_create(index: libc::c_ulong) -> *mut attr_pcomp { unsafe {
    let comp: *mut attr_pcomp = ut_malloc(
        ::core::mem::size_of::<attr_pcomp>() as libc::c_ulong,
    ) as *mut attr_pcomp;
    *comp = {
        
        attr_pcomp {
            type_0: attr_pcomp_type_index,
            c2rust_unnamed: C2RustUnnamed { index },
        }
    };
    comp
}}
unsafe extern "C" fn attr_pcomp_destroy(comp: *mut attr_pcomp) { unsafe {
    if !comp.is_null() {
        if (*comp).type_0 as libc::c_uint
            == attr_pcomp_type_key as libc::c_int as libc::c_uint
        {
            ut_free((*comp).c2rust_unnamed.key as *mut libc::c_void);
        }
        ut_free(comp as *mut libc::c_void);
    }
}}
unsafe extern "C" fn attr_pcomp_equal(
    comp_a: *mut attr_pcomp,
    comp_b: *mut attr_pcomp,
) -> bool { unsafe {
    if (*comp_a).type_0 as libc::c_uint != (*comp_b).type_0 as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if (*comp_a).type_0 as libc::c_uint
        == attr_pcomp_type_key as libc::c_int as libc::c_uint
    {
        strcmp((*comp_a).c2rust_unnamed.key, (*comp_b).c2rust_unnamed.key)
            == 0 as libc::c_int
    } else {
        (*comp_a).c2rust_unnamed.index == (*comp_b).c2rust_unnamed.index
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_get_type(
    pcomp: *const attr_pcomp,
) -> attr_pcomp_type { unsafe {
    (*pcomp).type_0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_is_key(pcomp: *const attr_pcomp) -> bool { unsafe {
    (*pcomp).type_0 as libc::c_uint
        == attr_pcomp_type_key as libc::c_int as libc::c_uint
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_is_index(pcomp: *const attr_pcomp) -> bool { unsafe {
    (*pcomp).type_0 as libc::c_uint
        == attr_pcomp_type_index as libc::c_int as libc::c_uint
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_get_key(
    pcomp: *const attr_pcomp,
) -> *const libc::c_char { unsafe {
    if !attr_pcomp_is_key(pcomp) {
        log_console_conf(true);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
                    as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"attr_pcomp_get_key\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_pcomp_is_key(pcomp)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    (*pcomp).c2rust_unnamed.key
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_pcomp_get_index(pcomp: *const attr_pcomp) -> libc::c_ulong { unsafe {
    if !attr_pcomp_is_index(pcomp) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
                    as *const u8 as *const libc::c_char,
                107 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"attr_pcomp_get_index\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_pcomp_is_index(pcomp)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    (*pcomp).c2rust_unnamed.index
}}
unsafe extern "C" fn attr_pcomp_parse_key(
    path_str: *const libc::c_char,
    comp: *mut *mut attr_pcomp,
) -> libc::c_int { unsafe {
    let mut key: [libc::c_char; 256] = [0; 256];
    let mut key_len: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    loop {
        let c: libc::c_char = *path_str.offset(key_len as isize);
        if c as libc::c_int == '\0' as i32 || !is_key_char(c) {
            break;
        }
        key[key_len as usize] = c;
        key_len = key_len.wrapping_add(1);
    }
    if key_len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    key[key_len as usize] = '\0' as i32 as libc::c_char;
    *comp = attr_path_key_create(key.as_mut_ptr());
    key_len as libc::c_int
}}
unsafe extern "C" fn attr_pcomp_parse_index(
    path_str: *const libc::c_char,
    comp: *mut *mut attr_pcomp,
) -> libc::c_int { unsafe {
    let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let index: libc::c_long = strtol(path_str, &mut end, 10 as libc::c_int);
    if *end as libc::c_int != ']' as i32 || end == path_str as *mut libc::c_char
        || index < 0 as libc::c_int as libc::c_long
        || index == 9223372036854775807 as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    *comp = attr_path_index_create(index as libc::c_ulong);
    (end.offset_from(path_str) as libc::c_long + 1 as libc::c_int as libc::c_long)
        as libc::c_int
}}
unsafe extern "C" fn attr_pcomp_parse(
    path_str: *const libc::c_char,
    comp: *mut *mut attr_pcomp,
) -> libc::c_int { unsafe {
    // print!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    if strlen(path_str) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let c: libc::c_char = *path_str.offset(0 as libc::c_int as isize);
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if c as libc::c_int == '[' as i32 {
        rc = attr_pcomp_parse_index(path_str.offset(1 as libc::c_int as isize), comp);
    } else if c as libc::c_int == '.' as i32 {
        rc = attr_pcomp_parse_key(path_str.offset(1 as libc::c_int as isize), comp);
    }
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    rc + 1 as libc::c_int
}}
unsafe extern "C" fn attr_pcomp_parse_root(
    path_str: *const libc::c_char,
    comp: *mut *mut attr_pcomp,
) -> libc::c_int { unsafe {
    // print!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    attr_pcomp_parse_key(path_str, comp)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_parse(
    path_str: *const libc::c_char,
    mut root: bool,
) -> *mut attr_path { unsafe {
    if strlen(path_str) > 255 as libc::c_int as libc::c_ulong {
        return std::ptr::null_mut::<attr_path>();
    }
    let path: *mut attr_path = ut_calloc(
        ::core::mem::size_of::<attr_path>() as libc::c_ulong,
    ) as *mut attr_path;
    let mut offset: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    loop {
        let comp: *mut *mut attr_pcomp = &mut *((*path).comps)
            .as_mut_ptr()
            .offset((*path).num_comps as isize) as *mut *mut attr_pcomp;
        if offset == strlen(path_str) {
            break;
        }
        let rc: libc::c_int = if root as libc::c_int != 0 {
            attr_pcomp_parse_root(path_str, comp)
        } else {
            attr_pcomp_parse(path_str.offset(offset as isize), comp)
        };
        root = 0 as libc::c_int != 0;
        if rc < 0 as libc::c_int {
            attr_path_destroy(path);
            return std::ptr::null_mut::<attr_path>();
        }
        if rc == 0 as libc::c_int {
            break;
        }
        (*path).num_comps = ((*path).num_comps).wrapping_add(1);
        offset = (offset as libc::c_ulong).wrapping_add(rc as libc::c_ulong) as libc::c_ulong
            as libc::c_ulong;
    }
    path
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_destroy(path: *mut attr_path) { unsafe {
    if !path.is_null() {
        let mut i: libc::c_ulong = 0;
        while i < (*path).num_comps {
            attr_pcomp_destroy((*path).comps[i as usize]);
            i = i.wrapping_add(1);
        }
        ut_free(path as *mut libc::c_void);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_num_comps(path: *const attr_path) -> libc::c_ulong { unsafe {
    (*path).num_comps
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_get_comp(
    path: *const attr_path,
    comp_num: libc::c_ulong,
) -> *const attr_pcomp { unsafe {
    if comp_num >= (*path).num_comps {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
                    as *const u8 as *const libc::c_char,
                242 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"attr_path_get_comp\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"comp_num < path->num_comps\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    (*path).comps[comp_num as usize]
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_len(
    path: *const attr_path,
    mut root: bool,
) -> libc::c_ulong { unsafe {
    let mut len: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_ulong = 0;
    while i < (*path).num_comps {
        let comp: *mut attr_pcomp = (*path).comps[i as usize];
        if root {
            if (*comp).type_0 as libc::c_uint != attr_pcomp_type_key as libc::c_int as libc::c_uint
            {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/attr_path.c\0"
                            as *const u8 as *const libc::c_char,
                        255 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 14],
                            &[libc::c_char; 14],
                        >(b"attr_path_len\0"))
                            .as_ptr(),
                        std::ptr::null_mut::<xcm_socket>(),
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"comp->type == attr_pcomp_type_key\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
            len = (len as libc::c_ulong).wrapping_add(strlen((*comp).c2rust_unnamed.key))
                as libc::c_ulong as libc::c_ulong;
            root = 0 as libc::c_int != 0;
        } else if (*comp).type_0 as libc::c_uint
            == attr_pcomp_type_key as libc::c_int as libc::c_uint
        {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen((*comp).c2rust_unnamed.key))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_ulong as libc::c_ulong;
        } else {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (snprintf(
                        std::ptr::null_mut::<libc::c_char>(),
                        0 as libc::c_int as libc::c_ulong,
                        b"%zd\0" as *const u8 as *const libc::c_char,
                        (*comp).c2rust_unnamed.index,
                    ) + 2 as libc::c_int) as libc::c_ulong,
                ) as libc::c_ulong as libc::c_ulong;
        }
        i = i.wrapping_add(1);
    }
    len
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_equal(
    path_a: *const attr_path,
    path_b: *const attr_path,
) -> bool { unsafe {
    if (*path_a).num_comps != (*path_b).num_comps {
        return 0 as libc::c_int != 0;
    }
    let mut i: libc::c_ulong = 0;
    while i < (*path_a).num_comps {
        if !attr_pcomp_equal((*path_a).comps[i as usize], (*path_b).comps[i as usize]) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    1 as libc::c_int != 0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_equal_str(
    path: *const attr_path,
    path_str: *const libc::c_char,
    root: bool,
) -> bool { unsafe {
    let other_path: *mut attr_path = attr_path_parse(path_str, root);
    if other_path.is_null() {
        return 0 as libc::c_int != 0;
    }
    let equal: bool = attr_path_equal(path, other_path);
    attr_path_destroy(other_path);
    equal
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_to_str(
    path: *const attr_path,
    mut root: bool,
) -> *mut libc::c_char { unsafe {
    let capacity: libc::c_ulong = (attr_path_len(path, root))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut len: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let str: *mut libc::c_char = ut_malloc(capacity) as *mut libc::c_char;
    let mut i: libc::c_ulong = 0;
    while i < (*path).num_comps {
        let comp: *mut attr_pcomp = (*path).comps[i as usize];
        if root {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    snprintf(
                        str,
                        capacity,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*comp).c2rust_unnamed.key,
                    ) as libc::c_ulong,
                ) as libc::c_ulong as libc::c_ulong;
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
                ) as libc::c_ulong as libc::c_ulong;
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
                ) as libc::c_ulong as libc::c_ulong;
        }
        i = i.wrapping_add(1);
    }
    *str.offset(len as isize) = '\0' as i32 as libc::c_char;
    str
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_path_is_valid_key(key: *const libc::c_char) -> bool { unsafe {
    if strlen(key) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    let mut i: libc::c_ulong = 0;
    while i < strlen(key) {
        let c: libc::c_char = *key.offset(i as isize);
        if !is_key_char(c) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    1 as libc::c_int != 0
}}