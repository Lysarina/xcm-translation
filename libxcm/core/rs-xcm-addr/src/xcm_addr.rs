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

use rs_xcm_dns::xcm_dns_is_valid_name;

unsafe extern "C" {
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    fn __errno_location() -> *mut libc::c_int;
    static in6addr_any: in6_addr;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    // fn xcm_dns_is_valid_name(name: *const libc::c_char) -> bool;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type xcm_addr_type = libc::c_uint;
pub const xcm_addr_type_ip: xcm_addr_type = 1;
pub const xcm_addr_type_name: xcm_addr_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_addr_ip {
    pub family: sa_family_t,
    pub addr: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ip4: in_addr_t,
    pub ip6: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_addr_host {
    pub type_0: xcm_addr_type,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ip: xcm_addr_ip,
    pub name: [libc::c_char; 254],
}
pub const _ISspace: C2RustUnnamed_2 = 8192;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
unsafe extern "C" fn supports_tls() -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn supports_sctp() -> bool {
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_valid_addr(
    mut xcm_addr_s: *const libc::c_char,
    mut require_supported: bool,
) -> bool {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: uint16_t = 0;
    let mut ux_name: [libc::c_char; 579] = [0; 579];
    let mut proto: [libc::c_char; 32] = [0; 32];
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut _oerrno: libc::c_int = *__errno_location();
    rc = xcm_addr_parse_proto(
        xcm_addr_s,
        proto.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    if !(rc < 0 as libc::c_int) {
        host = xcm_addr_host {
            type_0: xcm_addr_type_name,
            c2rust_unnamed: C2RustUnnamed_1 {
                ip: xcm_addr_ip {
                    family: 0,
                    addr: C2RustUnnamed_0 { ip4: 0 },
                },
            },
        };
        port = 0;
        ux_name = [0; 579];
        rc = -(1 as libc::c_int);
        if strcmp(b"tcp\0" as *const u8 as *const libc::c_char, proto.as_mut_ptr())
            == 0 as libc::c_int
        {
            rc = xcm_addr_parse_tcp(xcm_addr_s, &mut host, &mut port);
        } else if strcmp(
            b"btcp\0" as *const u8 as *const libc::c_char,
            proto.as_mut_ptr(),
        ) == 0 as libc::c_int
        {
            rc = xcm_addr_parse_btcp(xcm_addr_s, &mut host, &mut port);
        } else if strcmp(b"ux\0" as *const u8 as *const libc::c_char, proto.as_mut_ptr())
            == 0 as libc::c_int
        {
            rc = xcm_addr_parse_ux(
                xcm_addr_s,
                ux_name.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 579]>() as libc::c_ulong,
            );
        } else if strcmp(
            b"uxf\0" as *const u8 as *const libc::c_char,
            proto.as_mut_ptr(),
        ) == 0 as libc::c_int
        {
            rc = xcm_addr_parse_uxf(
                xcm_addr_s,
                ux_name.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 579]>() as libc::c_ulong,
            );
        }
        if supports_tls() as libc::c_int != 0 || !require_supported {
            if strcmp(b"utls\0" as *const u8 as *const libc::c_char, proto.as_mut_ptr())
                == 0 as libc::c_int
            {
                rc = xcm_addr_parse_utls(xcm_addr_s, &mut host, &mut port);
            } else if strcmp(
                b"tls\0" as *const u8 as *const libc::c_char,
                proto.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                rc = xcm_addr_parse_tls(xcm_addr_s, &mut host, &mut port);
            } else if strcmp(
                b"btls\0" as *const u8 as *const libc::c_char,
                proto.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                rc = xcm_addr_parse_btls(xcm_addr_s, &mut host, &mut port);
            }
        }
        if supports_sctp() as libc::c_int != 0 || !require_supported {
            if strcmp(b"sctp\0" as *const u8 as *const libc::c_char, proto.as_mut_ptr())
                == 0 as libc::c_int
            {
                rc = xcm_addr_parse_sctp(xcm_addr_s, &mut host, &mut port);
            }
        }
    }
    *__errno_location() = _oerrno;
    return rc == 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_is_valid(mut xcm_addr_s: *const libc::c_char) -> bool {
    return is_valid_addr(xcm_addr_s, 0 as libc::c_int != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_is_supported(
    mut xcm_addr_s: *const libc::c_char,
) -> bool {
    return is_valid_addr(xcm_addr_s, 1 as libc::c_int != 0);
}
unsafe extern "C" fn has_space(mut s: *const libc::c_char) -> bool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < strlen(s) {
        if *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn proto_addr_parse(
    mut addr_s: *const libc::c_char,
    mut proto: *mut libc::c_char,
    mut proto_capacity: size_t,
    mut proto_addr: *mut libc::c_char,
    mut proto_addr_capacity: size_t,
) -> libc::c_int {
    let mut proto_addr_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut proto_sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut proto_len: size_t = 0;
    if !(strlen(addr_s)
        > (32 as libc::c_int + 512 as libc::c_int + 32 as libc::c_int + 2 as libc::c_int)
            as libc::c_ulong || has_space(addr_s) as libc::c_int != 0)
    {
        proto_sep = strchr(addr_s, ':' as i32);
        if !proto_sep.is_null() {
            proto_len = proto_sep.offset_from(addr_s) as libc::c_long as size_t;
            if !(proto_len > 32 as libc::c_int as libc::c_ulong) {
                if !(proto_len >= proto_capacity) {
                    proto_addr_start = addr_s
                        .offset(proto_len as isize)
                        .offset(1 as libc::c_int as isize);
                    if !(strlen(proto_addr_start) >= proto_addr_capacity) {
                        strncpy(proto, addr_s, proto_len);
                        *proto.offset(proto_len as isize) = '\0' as i32 as libc::c_char;
                        strcpy(proto_addr, proto_addr_start);
                        return 0 as libc::c_int;
                    }
                }
                *__errno_location() = 36 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
    }
    *__errno_location() = 22 as libc::c_int;
    return -(1 as libc::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_proto(
    mut addr_s: *const libc::c_char,
    mut proto: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    let mut proto_addr: [libc::c_char; 579] = [0; 579];
    return proto_addr_parse(
        addr_s,
        proto,
        capacity,
        proto_addr.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 579]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn addr_parse_ux_uxf(
    mut ux_proto: *const libc::c_char,
    mut ux_addr_s: *const libc::c_char,
    mut ux_name: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    let mut proto: [libc::c_char; 33] = [0; 33];
    let mut name: [libc::c_char; 579] = [0; 579];
    if proto_addr_parse(
        ux_addr_s,
        proto.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
        name.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 579]>() as libc::c_ulong,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if strcmp(proto.as_mut_ptr(), ux_proto) != 0 as libc::c_int
        || strlen(name.as_mut_ptr())
            > (108 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        || strlen(name.as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if strlen(name.as_mut_ptr()) >= capacity {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    strcpy(ux_name, name.as_mut_ptr());
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_ux(
    mut ux_addr_s: *const libc::c_char,
    mut ux_name: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return addr_parse_ux_uxf(
        b"ux\0" as *const u8 as *const libc::c_char,
        ux_addr_s,
        ux_name,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_uxf(
    mut uxf_addr_s: *const libc::c_char,
    mut uxf_name: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return addr_parse_ux_uxf(
        b"uxf\0" as *const u8 as *const libc::c_char,
        uxf_addr_s,
        uxf_name,
        capacity,
    );
}
unsafe extern "C" fn host_parse(
    mut host_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
) -> libc::c_int {
    let mut addr_0: in_addr = in_addr { s_addr: 0 };
    let mut current_block: u64;
    if !(strlen(host_s) == 0 as libc::c_int as libc::c_ulong) {
        if *host_s.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
            if !(strlen(host_s) < (1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
                || *host_s
                    .offset(
                        (strlen(host_s)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int != ']' as i32)
            {
                let ip6_s_len: size_t = (strlen(host_s))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let vla = ip6_s_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as usize;
                let mut ip6_s: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
                strncpy(
                    ip6_s.as_mut_ptr(),
                    host_s.offset(1 as libc::c_int as isize),
                    ip6_s_len,
                );
                *ip6_s
                    .as_mut_ptr()
                    .offset(ip6_s_len as isize) = '\0' as i32 as libc::c_char;
                let mut addr: in6_addr = in6_addr {
                    __in6_u: C2RustUnnamed {
                        __u6_addr8: [0; 16],
                    },
                };
                if strcmp(ip6_s.as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    memcpy(
                        ((*host).c2rust_unnamed.ip.addr.ip6).as_mut_ptr()
                            as *mut libc::c_void,
                        (in6addr_any.__in6_u.__u6_addr8).as_ptr() as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    );
                    current_block = 14523784380283086299;
                } else if inet_pton(
                    10 as libc::c_int,
                    ip6_s.as_mut_ptr(),
                    &mut addr as *mut in6_addr as *mut libc::c_void,
                ) == 1 as libc::c_int
                {
                    memcpy(
                        ((*host).c2rust_unnamed.ip.addr.ip6).as_mut_ptr()
                            as *mut libc::c_void,
                        (addr.__in6_u.__u6_addr8).as_mut_ptr() as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    );
                    current_block = 14523784380283086299;
                } else {
                    current_block = 13846943762415835442;
                }
                match current_block {
                    13846943762415835442 => {}
                    _ => {
                        (*host).type_0 = xcm_addr_type_ip;
                        (*host)
                            .c2rust_unnamed
                            .ip
                            .family = 10 as libc::c_int as sa_family_t;
                        return 0 as libc::c_int;
                    }
                }
            }
        } else {
            if strcmp(host_s, b"*\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*host).type_0 = xcm_addr_type_ip;
                (*host).c2rust_unnamed.ip.family = 2 as libc::c_int as sa_family_t;
                (*host).c2rust_unnamed.ip.addr.ip4 = 0 as libc::c_int as in_addr_t;
                return 0 as libc::c_int;
            }
            addr_0 = in_addr { s_addr: 0 };
            if inet_pton(
                2 as libc::c_int,
                host_s,
                &mut addr_0 as *mut in_addr as *mut libc::c_void,
            ) == 1 as libc::c_int
            {
                (*host).type_0 = xcm_addr_type_ip;
                (*host).c2rust_unnamed.ip.family = 2 as libc::c_int as sa_family_t;
                (*host).c2rust_unnamed.ip.addr.ip4 = addr_0.s_addr;
                return 0 as libc::c_int;
            }
            if xcm_dns_is_valid_name(host_s) {
                (*host).type_0 = xcm_addr_type_name;
                strcpy(((*host).c2rust_unnamed.name).as_mut_ptr(), host_s);
                return 0 as libc::c_int;
            }
        }
    }
    *__errno_location() = 22 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn host_port_parse(
    mut proto: *const libc::c_char,
    mut addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    let mut port_sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut port_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lport: libc::c_int = 0;
    let mut host_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_len: size_t = 0;
    let mut current_block: u64;
    let mut actual_proto: [libc::c_char; 33] = [0; 33];
    let mut paddr: [libc::c_char; 579] = [0; 579];
    if !(proto_addr_parse(
        addr_s,
        actual_proto.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
        paddr.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 579]>() as libc::c_ulong,
    ) < 0 as libc::c_int)
    {
        if strcmp(proto, actual_proto.as_mut_ptr()) != 0 as libc::c_int {
            current_block = 10107677687531613869;
        } else {
            port_sep = strrchr(paddr.as_mut_ptr(), ':' as i32);
            if port_sep.is_null() {
                current_block = 10107677687531613869;
            } else {
                port_start = port_sep.offset(1 as libc::c_int as isize);
                end = 0 as *mut libc::c_char;
                lport = strtol(port_start, &mut end, 10 as libc::c_int) as libc::c_int;
                if *end.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                    current_block = 10107677687531613869;
                } else if lport < 0 as libc::c_int || lport > 65535 as libc::c_int {
                    current_block = 10107677687531613869;
                } else {
                    host_start = paddr.as_mut_ptr();
                    host_len = port_sep.offset_from(paddr.as_mut_ptr()) as libc::c_long
                        as size_t;
                    if host_len > 512 as libc::c_int as libc::c_ulong
                        || host_len == 0 as libc::c_int as libc::c_ulong
                    {
                        current_block = 10107677687531613869;
                    } else {
                        *host_start
                            .offset(host_len as isize) = '\0' as i32 as libc::c_char;
                        if host_parse(host_start, host) < 0 as libc::c_int {
                            current_block = 376436631470351228;
                        } else {
                            *port = ntohs(lport as uint16_t);
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        match current_block {
            376436631470351228 => {}
            _ => {
                *__errno_location() = 22 as libc::c_int;
            }
        }
    }
    return -(1 as libc::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_utls(
    mut utls_addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    return host_port_parse(
        b"utls\0" as *const u8 as *const libc::c_char,
        utls_addr_s,
        host,
        port,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_tls(
    mut tls_addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    return host_port_parse(
        b"tls\0" as *const u8 as *const libc::c_char,
        tls_addr_s,
        host,
        port,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_tcp(
    mut tcp_addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    return host_port_parse(
        b"tcp\0" as *const u8 as *const libc::c_char,
        tcp_addr_s,
        host,
        port,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_sctp(
    mut sctp_addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    return host_port_parse(
        b"sctp\0" as *const u8 as *const libc::c_char,
        sctp_addr_s,
        host,
        port,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_btcp(
    mut btcp_addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    return host_port_parse(
        b"btcp\0" as *const u8 as *const libc::c_char,
        btcp_addr_s,
        host,
        port,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_btls(
    mut btls_addr_s: *const libc::c_char,
    mut host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) -> libc::c_int {
    return host_port_parse(
        b"btls\0" as *const u8 as *const libc::c_char,
        btls_addr_s,
        host,
        port,
    );
}
unsafe extern "C" fn name_port_make(
    mut proto: *const libc::c_char,
    mut domain_name: *const libc::c_char,
    mut port: uint16_t,
    mut addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    let mut rc: libc::c_int = snprintf(
        addr_s,
        capacity,
        b"%s%c%s%c%d\0" as *const u8 as *const libc::c_char,
        proto,
        ':' as i32,
        domain_name,
        ':' as i32,
        ntohs(port) as libc::c_int,
    );
    if rc as libc::c_ulong == capacity {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ip_port_make(
    mut proto: *const libc::c_char,
    mut ip: *const xcm_addr_ip,
    mut port: uint16_t,
    mut addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    let mut ip_s: [libc::c_char; 46] = [0; 46];
    if (inet_ntop(
        (*ip).family as libc::c_int,
        &(*ip).addr.ip4 as *const in_addr_t as *const libc::c_void,
        ip_s.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
    ))
        .is_null()
    {
        if *__errno_location() == 28 as libc::c_int {
            *__errno_location() = 36 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = 0;
    if (*ip).family as libc::c_int == 2 as libc::c_int {
        rc = snprintf(
            addr_s,
            capacity,
            b"%s%c%s%c%d\0" as *const u8 as *const libc::c_char,
            proto,
            ':' as i32,
            ip_s.as_mut_ptr(),
            ':' as i32,
            ntohs(port) as libc::c_int,
        );
    } else {
        rc = snprintf(
            addr_s,
            capacity,
            b"%s%c%c%s%c%c%d\0" as *const u8 as *const libc::c_char,
            proto,
            ':' as i32,
            '[' as i32,
            ip_s.as_mut_ptr(),
            ']' as i32,
            ':' as i32,
            ntohs(port) as libc::c_int,
        );
    }
    if rc as libc::c_ulong == capacity {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn host_port_make(
    mut proto: *const libc::c_char,
    mut host: *const xcm_addr_host,
    mut port: uint16_t,
    mut addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    match (*host).type_0 as libc::c_uint {
        0 => {
            return name_port_make(
                proto,
                ((*host).c2rust_unnamed.name).as_ptr(),
                port,
                addr_s,
                capacity,
            );
        }
        1 => {
            return ip_port_make(
                proto,
                &(*host).c2rust_unnamed.ip,
                port,
                addr_s,
                capacity,
            );
        }
        _ => {
            if 0 as libc::c_int == 0 {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xcm_addr.c\0"
                            as *const u8 as *const libc::c_char,
                        399 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 15],
                            &[libc::c_char; 15],
                        >(b"host_port_make\0"))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_utls(
    mut host: *const xcm_addr_host,
    mut port: uint16_t,
    mut utls_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return host_port_make(
        b"utls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        utls_addr_s,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_tls(
    mut host: *const xcm_addr_host,
    mut port: uint16_t,
    mut tls_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return host_port_make(
        b"tls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        tls_addr_s,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_tcp(
    mut host: *const xcm_addr_host,
    mut port: uint16_t,
    mut tcp_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return host_port_make(
        b"tcp\0" as *const u8 as *const libc::c_char,
        host,
        port,
        tcp_addr_s,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_sctp(
    mut host: *const xcm_addr_host,
    mut port: uint16_t,
    mut sctp_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return host_port_make(
        b"sctp\0" as *const u8 as *const libc::c_char,
        host,
        port,
        sctp_addr_s,
        capacity,
    );
}
unsafe extern "C" fn addr_make_ux_uxf(
    mut ux_proto: *const libc::c_char,
    mut ux_name: *const libc::c_char,
    mut ux_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    if strlen(ux_name) > (108 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = snprintf(
        ux_addr_s,
        capacity,
        b"%s%c%s\0" as *const u8 as *const libc::c_char,
        ux_proto,
        ':' as i32,
        ux_name,
    );
    if rc as libc::c_ulong == capacity {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_ux(
    mut ux_name: *const libc::c_char,
    mut ux_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return addr_make_ux_uxf(
        b"ux\0" as *const u8 as *const libc::c_char,
        ux_name,
        ux_addr_s,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_uxf(
    mut uxf_name: *const libc::c_char,
    mut uxf_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return addr_make_ux_uxf(
        b"uxf\0" as *const u8 as *const libc::c_char,
        uxf_name,
        uxf_addr_s,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_btcp(
    mut host: *const xcm_addr_host,
    mut port: libc::c_ushort,
    mut btcp_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return host_port_make(
        b"btcp\0" as *const u8 as *const libc::c_char,
        host,
        port,
        btcp_addr_s,
        capacity,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_btls(
    mut host: *const xcm_addr_host,
    mut port: libc::c_ushort,
    mut btls_addr_s: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
    return host_port_make(
        b"btls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        btls_addr_s,
        capacity,
    );
}
