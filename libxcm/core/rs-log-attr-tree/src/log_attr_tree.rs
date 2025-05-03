#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

use libc::{snprintf, strcpy};

use xcm_rust_common::xcm_attr::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_attr_type_name(
    type_0: xcm_attr_type,
) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        1 => b"bool\0" as *const u8 as *const libc::c_char,
        2 => b"int64\0" as *const u8 as *const libc::c_char,
        5 => b"double\0" as *const u8 as *const libc::c_char,
        3 => b"string\0" as *const u8 as *const libc::c_char,
        4 => b"binary\0" as *const u8 as *const libc::c_char,
        _ => b"invalid\0" as *const u8 as *const libc::c_char,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_attr_str_value(
    type_0: xcm_attr_type,
    value: *const libc::c_void,
    len: libc::c_ulong,
    buf: *mut libc::c_char,
    capacity: libc::c_ulong,
) { unsafe {
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
                capacity as usize,
                b"%ld\0" as *const u8 as *const libc::c_char,
                *(value as *const libc::c_long),
            );
        }
        5 => {
            snprintf(
                buf,
                capacity as usize,
                b"%f\0" as *const u8 as *const libc::c_char,
                *(value as *const libc::c_double),
            );
        }
        3 => {
            snprintf(
                buf,
                capacity as usize,
                b"\"%s\"\0" as *const u8 as *const libc::c_char,
                value as *const libc::c_char,
            );
            *buf
                .offset(
                    capacity.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        4 => {
            if len == 0 {
                strcpy(
                    buf,
                    b"<zero-length binary data>\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let mut offset: libc::c_ulong = 0;
                let mut i: libc::c_ulong = 0;
                let value_bin: *const libc::c_uchar = value as *const libc::c_uchar;
                while i < len {
                    let left: libc::c_ulong = capacity.wrapping_sub(offset);
                    if left < 4 {
                        snprintf(
                            buf,
                            capacity as usize,
                            b"<%zd bytes of data>\0" as *const u8 as *const libc::c_char,
                            len,
                        );
                        break;
                    } else {
                        if i != 0 {
                            *buf.offset(offset as isize) = ':' as i32 as libc::c_char;
                            offset = offset.wrapping_add(1);
                        }
                        snprintf(
                            buf.offset(offset as isize),
                            capacity.wrapping_sub(offset) as usize,
                            b"%02x\0" as *const u8 as *const libc::c_char,
                            *value_bin.offset(i as isize) as libc::c_int,
                        );
                        offset = (offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_ulong
                            as libc::c_ulong;
                        i += 1;
                    }
                }
                *buf.offset(offset as isize) = '\0' as i32 as libc::c_char;
            }
        }
        _ => {}
    };
}}
