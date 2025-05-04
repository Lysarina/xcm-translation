#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(extern_types)]

use rs_log::*;
use rs_util::*;
use xcm_rust_common::xcm_attr::*;
use xcm_rust_common::xcm_tp::*;


extern "C" {
    fn abort() -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;

}

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
    pub value_len: libc::c_ulong,
    pub entry: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub le_next: *mut attr,
    pub le_prev: *mut *mut attr,
}


pub type xcm_attr_map_foreach_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *const libc::c_void,
        libc::c_ulong,
        *mut libc::c_void,
    ) -> (),
>;
unsafe extern "C" fn assert_valid_len(type_0: xcm_attr_type, value_len: libc::c_ulong) {
    match type_0 as libc::c_uint {
        1 => {
            if value_len != ::core::mem::size_of::<bool>() as libc::c_ulong {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/johan/coding/thesis/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                            as *const u8 as *const libc::c_char,
                        24 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"assert_valid_len\0"))
                            .as_ptr(),
                        std::ptr::null_mut::<xcm_socket>(),
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
            if value_len != ::core::mem::size_of::<libc::c_long>() as libc::c_ulong {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/johan/coding/thesis/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                            as *const u8 as *const libc::c_char,
                        27 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"assert_valid_len\0"))
                            .as_ptr(),
                        std::ptr::null_mut::<xcm_socket>(),
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
            if value_len != ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
            {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/johan/coding/thesis/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                            as *const u8 as *const libc::c_char,
                        30 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"assert_valid_len\0"))
                            .as_ptr(),
                        std::ptr::null_mut::<xcm_socket>(),
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"value_len == sizeof(double)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                abort();
            }
        }
        3 | 4 => {}
        _ => {}
    };
}
unsafe extern "C" fn attr_create(
    name: *const libc::c_char,
    type_0: xcm_attr_type,
    value: *const libc::c_void,
    value_len: libc::c_ulong,
) -> *mut attr {
    assert_valid_len(type_0, value_len);
    let attr: *mut attr = ut_malloc(::core::mem::size_of::<attr>() as libc::c_ulong)
        as *mut attr;
    *attr = {
        
        attr {
            name: ut_strdup(name),
            type_0,
            value: ut_memdup(value, value_len),
            value_len,
            entry: C2RustUnnamed {
                le_next: std::ptr::null_mut::<attr>(),
                le_prev: std::ptr::null_mut::<*mut attr>(),
            },
        }
    };
    attr
}
unsafe extern "C" fn attr_destroy(attr: *mut attr) {
    if !attr.is_null() {
        ut_free((*attr).name as *mut libc::c_void);
        ut_free((*attr).value);
        ut_free(attr as *mut libc::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_create() -> *mut xcm_attr_map {
    let attr_map: *mut xcm_attr_map = ut_malloc(
        ::core::mem::size_of::<xcm_attr_map>() as libc::c_ulong,
    ) as *mut xcm_attr_map;
    (*attr_map).attrs.lh_first = std::ptr::null_mut::<attr>();
    attr_map
}
unsafe extern "C" fn copy_attr_cb(
    attr_name: *const libc::c_char,
    attr_type: xcm_attr_type,
    attr_value: *const libc::c_void,
    attr_value_len: libc::c_ulong,
    user: *mut libc::c_void,
) {
    let copy: *mut xcm_attr_map = user as *mut xcm_attr_map;
    xcm_attr_map_add(copy, attr_name, attr_type, attr_value, attr_value_len);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_clone(
    original: *const xcm_attr_map,
) -> *mut xcm_attr_map {
    let copy: *mut xcm_attr_map = xcm_attr_map_create();
    xcm_attr_map_foreach(
        original,
        Some(
            copy_attr_cb
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    xcm_attr_type,
                    *const libc::c_void,
                    libc::c_ulong,
                    *mut libc::c_void,
                ) -> (),
        ),
        copy as *mut libc::c_void,
    );
    copy
}
unsafe extern "C" fn lookup_attr(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> *mut attr {
    let mut attr : *mut attr = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        if strcmp((*attr).name, attr_name) == 0 as libc::c_int {
            return attr;
        }
        attr = (*attr).entry.le_next;
    }
    std::ptr::null_mut::<attr>()
}
unsafe extern "C" fn lookup_attr_with_type(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
    type_0: xcm_attr_type,
) -> *const attr {
    let mut attr: *mut attr  = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        if strcmp((*attr).name, attr_name) == 0 as libc::c_int {
            return if (*attr).type_0 as libc::c_uint == type_0 as libc::c_uint {
                attr
            } else {
                std::ptr::null_mut::<attr>()
            };
        }
        attr = (*attr).entry.le_next;
    }
    std::ptr::null::<attr>()
}
unsafe extern "C" fn lookup_value_with_type(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
    type_0: xcm_attr_type,
) -> *const libc::c_void {
    let attr: *const attr = lookup_attr_with_type(attr_map, attr_name, type_0);
    if attr.is_null() {
        return std::ptr::null::<libc::c_void>();
    }
    (*attr).value
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add(
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
    attr_type: xcm_attr_type,
    attr_value: *const libc::c_void,
    attr_value_len: libc::c_ulong,
) {
    if !(!attr_name.is_null() && !attr_value.is_null()) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/johan/coding/thesis/xcm-translation/libxcm/core/xcm_attr_map.c\0"
                    as *const u8 as *const libc::c_char,
                136 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"xcm_attr_map_add\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr_name && attr_value\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    xcm_attr_map_del(attr_map, attr_name);
    let attr: *mut attr = attr_create(
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
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
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
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
    mut attr_value: libc::c_long,
) {
    xcm_attr_map_add(
        attr_map,
        attr_name,
        xcm_attr_type_int64,
        &mut attr_value as *mut libc::c_long as *const libc::c_void,
        ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_double(
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
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
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
    attr_value: *const libc::c_char,
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
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
    attr_value: *const libc::c_void,
    attr_value_len: libc::c_ulong,
) {
    xcm_attr_map_add(attr_map, attr_name, xcm_attr_type_bin, attr_value, attr_value_len);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_add_all(
    dst_map: *mut xcm_attr_map,
    src_map: *const xcm_attr_map,
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
                        libc::c_ulong,
                        *mut libc::c_void,
                    ) -> (),
            ),
            dst_map as *mut libc::c_void,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
    attr_type: *mut xcm_attr_type,
    attr_value_len: *mut libc::c_ulong,
) -> *const libc::c_void {
    let attr: *mut attr = lookup_attr(attr_map, attr_name);
    if attr.is_null() {
        return std::ptr::null::<libc::c_void>();
    }
    if !attr_type.is_null() {
        *attr_type = (*attr).type_0;
    }
    if !attr_value_len.is_null() {
        *attr_value_len = (*attr).value_len;
    }
    (*attr).value
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_bool(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> *const bool {
    lookup_value_with_type(attr_map, attr_name, xcm_attr_type_bool)
        as *const bool
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_int64(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> *const libc::c_long {
    lookup_value_with_type(attr_map, attr_name, xcm_attr_type_int64)
        as *const libc::c_long
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_double(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> *const libc::c_double {
    lookup_value_with_type(attr_map, attr_name, xcm_attr_type_double)
        as *const libc::c_double
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_str(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> *const libc::c_char {
    lookup_value_with_type(attr_map, attr_name, xcm_attr_type_str)
        as *const libc::c_char
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_get_bin(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> *const libc::c_char {
    lookup_value_with_type(attr_map, attr_name, xcm_attr_type_bin)
        as *const libc::c_char
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_exists(
    attr_map: *const xcm_attr_map,
    attr_name: *const libc::c_char,
) -> bool {
    !(lookup_attr(attr_map, attr_name)).is_null()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_del(
    attr_map: *mut xcm_attr_map,
    attr_name: *const libc::c_char,
) {
    let attr: *mut attr = lookup_attr(attr_map, attr_name);
    if !attr.is_null() {
        if !((*attr).entry.le_next).is_null() {
            (*(*attr).entry.le_next).entry.le_prev = (*attr).entry.le_prev;
        }
        *(*attr).entry.le_prev = (*attr).entry.le_next;
        attr_destroy(attr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_size(attr_map: *const xcm_attr_map) -> libc::c_ulong {
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut attr: *mut attr = (*attr_map).attrs.lh_first;
    while !attr.is_null() {
        count = count.wrapping_add(1);
        attr = (*attr).entry.le_next;
    }
    count
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_foreach(
    attr_map: *const xcm_attr_map,
    cb: xcm_attr_map_foreach_cb,
    user: *mut libc::c_void,
) {
    let mut attr: *mut attr = (*attr_map).attrs.lh_first;
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
    attr_map_a: *const xcm_attr_map,
    attr_map_b: *const xcm_attr_map,
) -> bool {
    let size_a: libc::c_ulong = xcm_attr_map_size(attr_map_a);
    let size_b: libc::c_ulong = xcm_attr_map_size(attr_map_b);
    if size_a != size_b {
        return 0 as libc::c_int != 0;
    }
    let mut attr_a: *mut attr = (*attr_map_a).attrs.lh_first;
    while !attr_a.is_null() {
        let attr_b: *const attr = lookup_attr_with_type(
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
    1 as libc::c_int != 0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_attr_map_destroy(attr_map: *mut xcm_attr_map) {
    if !attr_map.is_null() {
        let attr: *mut attr = (*attr_map).attrs.lh_first;
        loop {
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
