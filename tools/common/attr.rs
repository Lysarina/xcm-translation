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
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcm_attr_map;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
    fn ut_load_file(
        filename: *const libc::c_char,
        data: *mut *mut libc::c_char,
    ) -> ssize_t;
    fn ut_die(fmt: *const libc::c_char, _: ...) -> !;
    fn xcm_attr_map_add_bool(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: bool,
    );
    fn xcm_attr_map_add_int64(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: int64_t,
    );
    fn xcm_attr_map_add_double(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: libc::c_double,
    );
    fn xcm_attr_map_add_str(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: *const libc::c_char,
    );
    fn xcm_attr_map_add_bin(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: *const libc::c_void,
        attr_value_len: size_t,
    );
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
unsafe extern "C" fn parse_str_attr(
    mut s: *const libc::c_char,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut name_end: *const libc::c_char = strchr(s, '=' as i32);
    if name_end.is_null() {
        fprintf(
            stderr,
            b"Invalid attribute format. '%c' is missing.\n\0" as *const u8
                as *const libc::c_char,
            '=' as i32,
        );
        exit(1 as libc::c_int);
    }
    let mut name_len: size_t = name_end.offset_from(s) as libc::c_long as size_t;
    if name_len > 64 as libc::c_int as size_t {
        fprintf(
            stderr,
            b"Attribute name too long.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    strncpy(name, s, name_len);
    *name.offset(name_len as isize) = '\0' as i32 as libc::c_char;
    let mut value_part: *const libc::c_char = &*s
        .offset(name_len.wrapping_add(1 as libc::c_int as size_t) as isize)
        as *const libc::c_char;
    if strlen(value_part) > 512 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Attribute value too long.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    strcpy(value, value_part);
}
unsafe extern "C" fn parse_int64_attr(
    mut s: *const libc::c_char,
    mut name: *mut libc::c_char,
    mut value: *mut int64_t,
) {
    let mut str_value: [libc::c_char; 513] = [0; 513];
    parse_str_attr(s, name, str_value.as_mut_ptr());
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    *value = strtol(str_value.as_mut_ptr(), &mut end, 10 as libc::c_int);
    if end != str_value.as_mut_ptr().offset(strlen(str_value.as_mut_ptr()) as isize) {
        fprintf(
            stderr,
            b"\"%s\" not an integer.\n\0" as *const u8 as *const libc::c_char,
            str_value.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn parse_double_attr(
    mut s: *const libc::c_char,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_double,
) {
    let mut str_value: [libc::c_char; 513] = [0; 513];
    parse_str_attr(s, name, str_value.as_mut_ptr());
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    *value = strtod(str_value.as_mut_ptr(), &mut end);
    if end != str_value.as_mut_ptr().offset(strlen(str_value.as_mut_ptr()) as isize) {
        fprintf(
            stderr,
            b"\"%s\" not a double.\n\0" as *const u8 as *const libc::c_char,
            str_value.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn parse_bool_attr(
    mut s: *const libc::c_char,
    mut name: *mut libc::c_char,
    mut value: *mut bool,
) {
    let mut str_value: [libc::c_char; 513] = [0; 513];
    parse_str_attr(s, name, str_value.as_mut_ptr());
    if strcmp(str_value.as_mut_ptr(), b"true\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *value = 1 as libc::c_int != 0;
    } else if strcmp(
        str_value.as_mut_ptr(),
        b"false\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        *value = 0 as libc::c_int != 0;
    } else {
        fprintf(
            stderr,
            b"Boolean attributes need to be either 'true' or 'false'.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn attr_parse_bool(
    mut s: *const libc::c_char,
    mut attrs: *mut xcm_attr_map,
) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut value: bool = false;
    parse_bool_attr(s, name.as_mut_ptr(), &mut value);
    xcm_attr_map_add_bool(attrs, name.as_mut_ptr(), value);
}
#[no_mangle]
pub unsafe extern "C" fn attr_parse_int64(
    mut s: *const libc::c_char,
    mut attrs: *mut xcm_attr_map,
) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut value: int64_t = 0;
    parse_int64_attr(s, name.as_mut_ptr(), &mut value);
    xcm_attr_map_add_int64(attrs, name.as_mut_ptr(), value);
}
#[no_mangle]
pub unsafe extern "C" fn attr_parse_double(
    mut s: *const libc::c_char,
    mut attrs: *mut xcm_attr_map,
) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut value: libc::c_double = 0.;
    parse_double_attr(s, name.as_mut_ptr(), &mut value);
    xcm_attr_map_add_double(attrs, name.as_mut_ptr(), value);
}
#[no_mangle]
pub unsafe extern "C" fn attr_parse_str(
    mut s: *const libc::c_char,
    mut attrs: *mut xcm_attr_map,
) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut value: [libc::c_char; 512] = [0; 512];
    parse_str_attr(s, name.as_mut_ptr(), value.as_mut_ptr());
    xcm_attr_map_add_str(attrs, name.as_mut_ptr(), value.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn attr_load_bin_file(
    mut s: *const libc::c_char,
    mut attrs: *mut xcm_attr_map,
) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut filename: [libc::c_char; 512] = [0; 512];
    parse_str_attr(s, name.as_mut_ptr(), filename.as_mut_ptr());
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: ssize_t = ut_load_file(filename.as_mut_ptr(), &mut value);
    if rc < 0 as libc::c_int as ssize_t {
        ut_die(
            b"Error reading \"%s\"\0" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }
    xcm_attr_map_add_bin(
        attrs,
        name.as_mut_ptr(),
        value as *const libc::c_void,
        rc as size_t,
    );
    ut_free(value as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn attr_load_bin_stdin(
    mut s: *const libc::c_char,
    mut attrs: *mut xcm_attr_map,
) {
    if strlen(s) > 64 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Attribute name too long.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut raw_len: uint32_t = 0;
    if fread(
        &mut raw_len as *mut uint32_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
        stdin,
    ) != ::core::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Failed to read length field for \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            s,
        );
        exit(1 as libc::c_int);
    }
    let mut len: uint32_t = __bswap_32(raw_len);
    let mut value: *mut libc::c_char = ut_malloc(len as size_t) as *mut libc::c_char;
    if fread(
        value as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len as libc::c_ulong,
        stdin,
    ) != len as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Failed to read %d bytes of value data for \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            len,
            s,
        );
        exit(1 as libc::c_int);
    }
    xcm_attr_map_add_bin(attrs, s, value as *const libc::c_void, len as size_t);
    ut_free(value as *mut libc::c_void);
}
