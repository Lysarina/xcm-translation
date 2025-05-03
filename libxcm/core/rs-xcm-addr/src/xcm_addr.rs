#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(extern_types)]

use std::process::abort;
use libc::{__errno_location, snprintf, strtol, memcpy, strcpy,
    strncpy, strcmp, strchr, strrchr, strlen};

use rs_log::*;
use rs_xcm_dns::*;

use xcm_rust_common::xcm_tp::xcm_socket;

unsafe extern "C" {
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    // fn __errno_location() -> *mut libc::c_int;
    static in6addr_any: in6_addr;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    // fn snprintf(
    //     _: *mut libc::c_char,
    //     _: libc::c_ulong,
    //     _: *const libc::c_char,
    //     _: ...
    // ) -> libc::c_int;
    // fn strtol(
    //     _: *const libc::c_char,
    //     _: *mut *mut libc::c_char,
    //     _: libc::c_int,
    // ) -> libc::c_long;
    // fn abort() -> !;
    // fn memcpy(
    //     _: *mut libc::c_void,
    //     _: *const libc::c_void,
    //     _: libc::c_ulong,
    // ) -> *mut libc::c_void;
    // fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    // fn strncpy(
    //     _: *mut libc::c_char,
    //     _: *const libc::c_char,
    //     _: libc::c_ulong,
    // ) -> *mut libc::c_char;
    // fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    // fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    // fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    // fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub const _ISspace: libc::c_uint = 8192;

// pub type C2RustUnnamed_2 = libc::c_uint;
// pub const _ISalnum: C2RustUnnamed_2 = 8;
// pub const _ISpunct: C2RustUnnamed_2 = 4;
// pub const _IScntrl: C2RustUnnamed_2 = 2;
// pub const _ISblank: C2RustUnnamed_2 = 1;
// pub const _ISgraph: C2RustUnnamed_2 = 32768;
// pub const _ISprint: C2RustUnnamed_2 = 16384;
// pub const _ISxdigit: C2RustUnnamed_2 = 4096;
// pub const _ISdigit: C2RustUnnamed_2 = 2048;
// pub const _ISalpha: C2RustUnnamed_2 = 1024;
// pub const _ISlower: C2RustUnnamed_2 = 512;
// pub const _ISupper: C2RustUnnamed_2 = 256;
unsafe extern "C" fn supports_tls() -> bool {
    1 as libc::c_int != 0
}
unsafe extern "C" fn supports_sctp() -> bool {
    0 as libc::c_int != 0
}
unsafe extern "C" fn is_valid_addr(
    xcm_addr_s: *const libc::c_char,
    require_supported: bool,
) -> bool { unsafe {
    let mut host: xcm_addr_host; 
    // = xcm_addr_host {
    //     type_0: xcm_addr_type_name,
    //     c2rust_unnamed: C2RustUnnamed_1 {
    //         ip: xcm_addr_ip {
    //             family: 0,
    //             addr: C2RustUnnamed_0 { ip4: 0 },
    //         },
    //     },
    // };
    let mut port: uint16_t; //= 0;
    let mut ux_name: [libc::c_char; 579]; //= [0; 579];
    let mut proto: [libc::c_char; 32] = [0; 32];
    let mut rc: libc::c_int; //= -(1 as libc::c_int);
    let mut _oerrno: libc::c_int = *__errno_location();
    rc = xcm_addr_parse_proto(
        xcm_addr_s,
        proto.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    if rc >= 0 as libc::c_int {
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
        if (supports_sctp() as libc::c_int != 0 || !require_supported) && strcmp(b"sctp\0" as *const u8 as *const libc::c_char, proto.as_mut_ptr()) == 0 as libc::c_int {
            rc = xcm_addr_parse_sctp(xcm_addr_s, &mut host, &mut port);
        }
    }
    *__errno_location() = _oerrno;
    rc == 0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_is_valid(xcm_addr_s: *const libc::c_char) -> bool { unsafe {
    is_valid_addr(xcm_addr_s, 0 as libc::c_int != 0)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_is_supported(
    xcm_addr_s: *const libc::c_char,
) -> bool { unsafe {
    is_valid_addr(xcm_addr_s, 1 as libc::c_int != 0)
}}
unsafe extern "C" fn has_space(s: *const libc::c_char) -> bool { unsafe {
    let mut i: usize = 0;
    while i < strlen(s) {
        if *(*__ctype_b_loc()).offset(*s.add(i) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            return true;
        }
        i += 1;
    }
    false
}}
unsafe extern "C" fn proto_addr_parse(
    addr_s: *const libc::c_char,
    proto: *mut libc::c_char,
    proto_capacity: size_t,
    proto_addr: *mut libc::c_char,
    proto_addr_capacity: size_t,
) -> libc::c_int { unsafe {
    let proto_addr_start: *const libc::c_char; //= std::ptr::null::<libc::c_char>();
    let proto_sep: *const libc::c_char; // = std::ptr::null::<libc::c_char>();
    let proto_len: size_t; // = 0;
    if !(strlen(addr_s)
        > (32 + 512 + 32 + 2)
            as usize || has_space(addr_s))
    {
        proto_sep = strchr(addr_s, ':' as i32);
        if !proto_sep.is_null() {
            proto_len = proto_sep.offset_from(addr_s) as libc::c_long as size_t;
            if proto_len <= 32 as libc::c_int as libc::c_ulong {
                if proto_len < proto_capacity {
                    proto_addr_start = addr_s
                        .offset(proto_len as isize)
                        .offset(1 as libc::c_int as isize);
                    if strlen(proto_addr_start) < proto_addr_capacity as usize {
                        strncpy(proto, addr_s, proto_len as usize);
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
    -(1 as libc::c_int)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_proto(
    addr_s: *const libc::c_char,
    proto: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    let mut proto_addr: [libc::c_char; 579] = [0; 579];
    proto_addr_parse(
        addr_s,
        proto,
        capacity,
        proto_addr.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 579]>() as libc::c_ulong,
    )
}}
unsafe extern "C" fn addr_parse_ux_uxf(
    ux_proto: *const libc::c_char,
    ux_addr_s: *const libc::c_char,
    ux_name: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
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
    if strcmp(proto.as_mut_ptr(), ux_proto) != 0
        || strlen(name.as_mut_ptr()) > 107
        || strlen(name.as_mut_ptr()) == 0
    {
        *__errno_location() = 22;
        return -1;
    }
    if strlen(name.as_mut_ptr()) >= capacity as usize {
        *__errno_location() = 36;
        return -1;
    }
    strcpy(ux_name, name.as_mut_ptr());
    0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_ux(
    ux_addr_s: *const libc::c_char,
    ux_name: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    addr_parse_ux_uxf(
        b"ux\0" as *const u8 as *const libc::c_char,
        ux_addr_s,
        ux_name,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_uxf(
    uxf_addr_s: *const libc::c_char,
    uxf_name: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    addr_parse_ux_uxf(
        b"uxf\0" as *const u8 as *const libc::c_char,
        uxf_addr_s,
        uxf_name,
        capacity,
    )
}}
unsafe extern "C" fn host_parse(
    host_s: *const libc::c_char,
    host: *mut xcm_addr_host,
) -> libc::c_int { unsafe {
    if host_s.is_null() || strlen(host_s) == 0 {
        *__errno_location() = libc::EINVAL;
        return -1;
    }

    let len = strlen(host_s);
    let first = *host_s;
    let last = *host_s.add(len - 1);

    // IPv6 in brackets: [::1]
    if first == b'[' as i8 && last == b']' as i8 && len >= 2 {
        let ip6_len = len - 2; // Remove brackets
        let mut ip6_s = vec![0 as libc::c_char; ip6_len + 1];
        strncpy(ip6_s.as_mut_ptr(), host_s.add(1), ip6_len);
        *ip6_s.as_mut_ptr().add(ip6_len) = 0;

        if strcmp(ip6_s.as_ptr(), c"*".as_ptr() as *const libc::c_char) == 0 {
            memcpy(
                (*host).c2rust_unnamed.ip.addr.ip6.as_mut_ptr() as *mut libc::c_void,
                in6addr_any.__in6_u.__u6_addr8.as_ptr() as *const libc::c_void,
                16,
            );
        } else {
            let mut addr = in6_addr {
                __in6_u: C2RustUnnamed { __u6_addr8: [0; 16] },
            };
            if inet_pton(libc::AF_INET6, ip6_s.as_ptr(), &mut addr as *mut _ as *mut libc::c_void)
                != 1
            {
                *__errno_location() = libc::EINVAL;
                return -1;
            }
            memcpy(
                (*host).c2rust_unnamed.ip.addr.ip6.as_mut_ptr() as *mut libc::c_void,
                addr.__in6_u.__u6_addr8.as_ptr() as *const libc::c_void,
                16,
            );
        }

        (*host).type_0 = xcm_addr_type_ip;
        (*host).c2rust_unnamed.ip.family = libc::AF_INET6 as sa_family_t;
        return 0;
    }

    // IPv4 wildcard: "*"
    if strcmp(host_s, c"*".as_ptr() as *const libc::c_char) == 0 {
        (*host).type_0 = xcm_addr_type_ip;
        (*host).c2rust_unnamed.ip.family = libc::AF_INET as sa_family_t;
        (*host).c2rust_unnamed.ip.addr.ip4 = 0;
        return 0;
    }

    // IPv4 address
    let mut addr = in_addr { s_addr: 0 };
    if inet_pton(libc::AF_INET, host_s, &mut addr as *mut _ as *mut libc::c_void) == 1 {
        (*host).type_0 = xcm_addr_type_ip;
        (*host).c2rust_unnamed.ip.family = libc::AF_INET as sa_family_t;
        (*host).c2rust_unnamed.ip.addr.ip4 = addr.s_addr;
        return 0;
    }

    // DNS name
    if xcm_dns_is_valid_name(host_s) {
        (*host).type_0 = xcm_addr_type_name;
        strcpy((*host).c2rust_unnamed.name.as_mut_ptr(), host_s);
        return 0;
    }

    *__errno_location() = libc::EINVAL;
    -1
}}
unsafe extern "C" fn host_port_parse(
    proto: *const libc::c_char,
    addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    let mut actual_proto: [libc::c_char; 33] = [0; 33];
    let mut paddr: [libc::c_char; 579] = [0; 579];

    // Parse protocol and address string
    if proto_addr_parse(
        addr_s,
        actual_proto.as_mut_ptr(),
        actual_proto.len() as libc::c_ulong,
        paddr.as_mut_ptr(),
        paddr.len() as libc::c_ulong,
    ) < 0
    {
        return -1;
    }

    // Protocol mismatch
    if strcmp(proto, actual_proto.as_ptr()) != 0 {
        *__errno_location() = libc::EINVAL;
        return -1;
    }

    // Find last colon (port separator)
    let port_sep = strrchr(paddr.as_ptr(), ':' as i32);
    if port_sep.is_null() {
        *__errno_location() = libc::EINVAL;
        return -1;
    }

    // Extract port string
    let port_start = port_sep.add(1);
    let mut end: *mut libc::c_char = std::ptr::null_mut();
    let lport = strtol(port_start, &mut end, 10);

    if *end != 0 || !(0..=65535).contains(&lport) {
        *__errno_location() = libc::EINVAL;
        return -1;
    }

    // Null-terminate host part
    let host_start = paddr.as_mut_ptr();
    let host_len = port_sep.offset_from(host_start) as usize;

    if host_len == 0 || host_len > 512 {
        *__errno_location() = libc::EINVAL;
        return -1;
    }

    *host_start.add(host_len) = 0;

    // Parse host
    if host_parse(host_start, host) < 0 {
        return -1;
    }

    *port = ntohs(lport as uint16_t);
    0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_utls(
    utls_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    host_port_parse(
        b"utls\0" as *const u8 as *const libc::c_char,
        utls_addr_s,
        host,
        port,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_tls(
    tls_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    host_port_parse(
        b"tls\0" as *const u8 as *const libc::c_char,
        tls_addr_s,
        host,
        port,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_tcp(
    tcp_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    host_port_parse(
        b"tcp\0" as *const u8 as *const libc::c_char,
        tcp_addr_s,
        host,
        port,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_sctp(
    sctp_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    host_port_parse(
        b"sctp\0" as *const u8 as *const libc::c_char,
        sctp_addr_s,
        host,
        port,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_btcp(
    btcp_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    host_port_parse(
        b"btcp\0" as *const u8 as *const libc::c_char,
        btcp_addr_s,
        host,
        port,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_btls(
    btls_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut uint16_t,
) -> libc::c_int { unsafe {
    host_port_parse(
        b"btls\0" as *const u8 as *const libc::c_char,
        btls_addr_s,
        host,
        port,
    )
}}
unsafe extern "C" fn name_port_make(
    proto: *const libc::c_char,
    domain_name: *const libc::c_char,
    port: uint16_t,
    addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    let rc: libc::c_int = snprintf(
        addr_s,
        capacity as usize,
        b"%s%c%s%c%d\0" as *const u8 as *const libc::c_char,
        proto,
        ':' as i32,
        domain_name,
        ':' as i32,
        ntohs(port) as libc::c_int,
    );
    if rc as libc::c_ulong == capacity {
        *__errno_location() = 36;
        return -1;
    }
    0
}}
unsafe extern "C" fn ip_port_make(
    proto: *const libc::c_char,
    ip: *const xcm_addr_ip,
    port: uint16_t,
    addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
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
    let rc: libc::c_int = if (*ip).family as libc::c_int == 2 as libc::c_int {
        snprintf(
            addr_s,
            capacity as usize,
            b"%s%c%s%c%d\0" as *const u8 as *const libc::c_char,
            proto,
            ':' as i32,
            ip_s.as_mut_ptr(),
            ':' as i32,
            ntohs(port) as libc::c_int,
        )
    } else {
        snprintf(
            addr_s,
            capacity as usize,
            b"%s%c%c%s%c%c%d\0" as *const u8 as *const libc::c_char,
            proto,
            ':' as i32,
            '[' as i32,
            ip_s.as_mut_ptr(),
            ']' as i32,
            ':' as i32,
            ntohs(port) as libc::c_int,
        )
    };
    if rc as libc::c_ulong == capacity {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
unsafe extern "C" fn host_port_make(
    proto: *const libc::c_char,
    host: *const xcm_addr_host,
    port: uint16_t,
    addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_utls(
    host: *const xcm_addr_host,
    port: uint16_t,
    utls_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    host_port_make(
        b"utls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        utls_addr_s,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_tls(
    host: *const xcm_addr_host,
    port: uint16_t,
    tls_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    host_port_make(
        b"tls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        tls_addr_s,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_tcp(
    host: *const xcm_addr_host,
    port: uint16_t,
    tcp_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    host_port_make(
        b"tcp\0" as *const u8 as *const libc::c_char,
        host,
        port,
        tcp_addr_s,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_sctp(
    host: *const xcm_addr_host,
    port: uint16_t,
    sctp_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    host_port_make(
        b"sctp\0" as *const u8 as *const libc::c_char,
        host,
        port,
        sctp_addr_s,
        capacity,
    )
}}
unsafe extern "C" fn addr_make_ux_uxf(
    ux_proto: *const libc::c_char,
    ux_name: *const libc::c_char,
    ux_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    if strlen(ux_name) > (107) {
        *__errno_location() = 22;
        return -1;
    }
    let rc: libc::c_int = snprintf(
        ux_addr_s,
        capacity as usize,
        b"%s%c%s\0" as *const u8 as *const libc::c_char,
        ux_proto,
        ':' as i32,
        ux_name,
    );
    if rc as libc::c_ulong == capacity {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_ux(
    ux_name: *const libc::c_char,
    ux_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    addr_make_ux_uxf(
        b"ux\0" as *const u8 as *const libc::c_char,
        ux_name,
        ux_addr_s,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_uxf(
    uxf_name: *const libc::c_char,
    uxf_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    addr_make_ux_uxf(
        b"uxf\0" as *const u8 as *const libc::c_char,
        uxf_name,
        uxf_addr_s,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_btcp(
    host: *const xcm_addr_host,
    port: libc::c_ushort,
    btcp_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    host_port_make(
        b"btcp\0" as *const u8 as *const libc::c_char,
        host,
        port,
        btcp_addr_s,
        capacity,
    )
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_make_btls(
    host: *const xcm_addr_host,
    port: libc::c_ushort,
    btls_addr_s: *mut libc::c_char,
    capacity: size_t,
) -> libc::c_int { unsafe {
    host_port_make(
        b"btls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        btls_addr_s,
        capacity,
    )
}}
