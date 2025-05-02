#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
unsafe extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
pub type xcm_attr_type = libc::c_uint;
pub const xcm_attr_type_double: xcm_attr_type = 5;
pub const xcm_attr_type_bin: xcm_attr_type = 4;
pub const xcm_attr_type_str: xcm_attr_type = 3;
pub const xcm_attr_type_int64: xcm_attr_type = 2;
pub const xcm_attr_type_bool: xcm_attr_type = 1;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_attr_type_name(
    mut type_0: xcm_attr_type,
) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        1 => return b"bool\0" as *const u8 as *const libc::c_char,
        2 => return b"int64\0" as *const u8 as *const libc::c_char,
        5 => return b"double\0" as *const u8 as *const libc::c_char,
        3 => return b"string\0" as *const u8 as *const libc::c_char,
        4 => return b"binary\0" as *const u8 as *const libc::c_char,
        _ => return b"invalid\0" as *const u8 as *const libc::c_char,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_attr_str_value(
    mut type_0: xcm_attr_type,
    mut value: *const libc::c_void,
    mut len: size_t,
    mut buf: *mut libc::c_char,
    mut capacity: size_t,
) {
    match type_0 as libc::c_uint {
        1 => {
            if *(value as *mut bool) {
                strcpy(buf, b"true\0" as *const u8 as *const libc::c_char);
            } else {
                strcpy(buf, b"false\0" as *const u8 as *const libc::c_char);
            }
        }
        2 => {
            snprintf(
                buf,
                capacity,
                b"%ld\0" as *const u8 as *const libc::c_char,
                *(value as *const int64_t),
            );
        }
        5 => {
            snprintf(
                buf,
                capacity,
                b"%f\0" as *const u8 as *const libc::c_char,
                *(value as *const libc::c_double),
            );
        }
        3 => {
            snprintf(
                buf,
                capacity,
                b"\"%s\"\0" as *const u8 as *const libc::c_char,
                value as *const libc::c_char,
            );
            *buf
                .offset(
                    capacity.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        4 => {
            if len == 0 as libc::c_int as libc::c_ulong {
                strcpy(
                    buf,
                    b"<zero-length binary data>\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let mut offset: size_t = 0 as libc::c_int as size_t;
                let mut i: libc::c_int = 0;
                let mut value_bin: *const uint8_t = value as *const uint8_t;
                i = 0 as libc::c_int;
                while (i as libc::c_ulong) < len {
                    let mut left: size_t = capacity.wrapping_sub(offset);
                    if left < 4 as libc::c_int as libc::c_ulong {
                        snprintf(
                            buf,
                            capacity,
                            b"<%zd bytes of data>\0" as *const u8 as *const libc::c_char,
                            len,
                        );
                        break;
                    } else {
                        if i != 0 as libc::c_int {
                            *buf.offset(offset as isize) = ':' as i32 as libc::c_char;
                            offset = offset.wrapping_add(1);
                            offset;
                        }
                        snprintf(
                            buf.offset(offset as isize),
                            capacity.wrapping_sub(offset),
                            b"%02x\0" as *const u8 as *const libc::c_char,
                            *value_bin.offset(i as isize) as libc::c_int,
                        );
                        offset = (offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        i += 1;
                        i;
                    }
                }
                *buf.offset(offset as isize) = '\0' as i32 as libc::c_char;
            }
        }
        _ => {}
    };
}
