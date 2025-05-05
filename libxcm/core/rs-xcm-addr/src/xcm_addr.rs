#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

use std::process::abort;
use std::ffi::CStr;
use std::ptr;
use libc::{__errno_location, snprintf, strcpy,
    strncpy, strcmp, strchr, strlen, ntohs, in6addr_any,
    in_addr, in6_addr, EINVAL, AF_INET, AF_INET6};

use rs_log::*;
use rs_xcm_dns::*;
use xcm_rust_common::xcm_tp::xcm_socket;

unsafe extern "C" {
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: libc::c_uint,
    ) -> *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [libc::c_uchar; 16],
    pub __u6_addr16: [libc::c_ushort; 8],
    pub __u6_addr32: [libc::c_uint; 4],
}
pub type xcm_addr_type = libc::c_uint;
pub const xcm_addr_type_ip: xcm_addr_type = 1;
pub const xcm_addr_type_name: xcm_addr_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcm_addr_ip {
    pub family: libc::c_ushort,
    pub addr: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ip4: libc::c_uint,
    pub ip6: [libc::c_uchar; 16],
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
unsafe extern "C" fn supports_tls() -> bool {
    true
}
unsafe extern "C" fn supports_sctp() -> bool {
    false
}
unsafe extern "C" fn is_valid_addr(
    xcm_addr_s: *const libc::c_char,
    require_supported: bool,
) -> bool { unsafe {
    let mut host: xcm_addr_host;
    let mut port: libc::c_ushort;
    let mut ux_name: [libc::c_char; 579];
    let mut proto: [libc::c_char; 32] = [0; 32];
    let mut rc: libc::c_int;
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
    proto_capacity: libc::c_ulong,
    proto_addr: *mut libc::c_char,
    proto_addr_capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let proto_addr_start: *const libc::c_char;
    let proto_sep: *const libc::c_char;
    let proto_len: libc::c_ulong;
    if !(strlen(addr_s)
        > (32 + 512 + 32 + 2)
            as usize || has_space(addr_s))
    {
        proto_sep = strchr(addr_s, ':' as i32);
        if !proto_sep.is_null() {
            proto_len = proto_sep.offset_from(addr_s) as libc::c_long as libc::c_ulong;
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
        *__errno_location() = EINVAL;
        return -1;
    }

    let host_cstr = CStr::from_ptr(host_s);
    let host_str = match host_cstr.to_str() {
        Ok(s) => s,
        Err(_) => {
            *__errno_location() = EINVAL;
            return -1;
        }
    };

    if host_str.starts_with('[') {
        if !host_str.ends_with(']') || !host_str.len() >= 2 { // invalid format
            *__errno_location() = EINVAL;
            return -1;
        }
        let inner = &host_str[1..host_str.len() - 1]; // string w/o []

        if inner == "*" { // if string is just wildcard
            ptr::copy_nonoverlapping( // copy address "any" into host
                in6addr_any.s6_addr.as_ptr(), // src
                (*host).c2rust_unnamed.ip.addr.ip6.as_mut_ptr(), // dest
                16,
            );
        } else {
            let mut addr = in6_addr { s6_addr: [0; 16] };
            let inner_cstr = match std::ffi::CString::new(inner) {
                Ok(s) => s,
                Err(_) => {
                    *__errno_location() = EINVAL;
                    return -1;
                }
            };
            if inet_pton(AF_INET6, inner_cstr.as_ptr(), &mut addr as *mut _ as *mut _) != 1 {
                *__errno_location() = EINVAL;
                return -1;
            }
            ptr::copy_nonoverlapping(
                addr.s6_addr.as_ptr(),
                (*host).c2rust_unnamed.ip.addr.ip6.as_mut_ptr(),
                16,
            );
        }

        (*host).type_0 = xcm_addr_type_ip;
        (*host).c2rust_unnamed.ip.family = AF_INET6 as libc::c_ushort;
        return 0;
    }

    if host_str == "*" {
        (*host).type_0 = xcm_addr_type_ip;
        (*host).c2rust_unnamed.ip.family = AF_INET as libc::c_ushort;
        (*host).c2rust_unnamed.ip.addr.ip4 = 0;
        return 0;
    }

    let mut addr = in_addr { s_addr: 0 };
    if inet_pton(AF_INET, host_s, &mut addr as *mut _ as *mut _) == 1 {
        (*host).type_0 = xcm_addr_type_ip;
        (*host).c2rust_unnamed.ip.family = AF_INET as libc::c_ushort;
        (*host).c2rust_unnamed.ip.addr.ip4 = addr.s_addr;
        return 0;
    }

    if xcm_dns_is_valid_name(host_s) {
        (*host).type_0 = xcm_addr_type_name;
        let src = CStr::from_ptr(host_s);
        let bytes = src.to_bytes_with_nul();
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            (*host).c2rust_unnamed.name.as_mut_ptr() as *mut u8,
            bytes.len(),
        );
        // strcpy((*host).c2rust_unnamed.name.as_mut_ptr(), host_s);
        return 0;
    }

    *__errno_location() = libc::EINVAL;
    -1
}}
unsafe extern "C" fn host_port_parse(
    proto: *const libc::c_char,
    addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut libc::c_ushort,
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

    if strcmp(proto, actual_proto.as_ptr()) != 0 {
        *__errno_location() = libc::EINVAL;
        return -1;
    }

    let paddr_cstr = CStr::from_ptr(paddr.as_ptr());
    let paddr_str = match paddr_cstr.to_str() {
        Ok(s) => s,
        Err(_) => {
            return -1;
        }
    };

    // Find last colon
    let colon_idx = match paddr_str.rfind(':') {
        Some(i) => i,
        None => {
            *__errno_location() = EINVAL;
            return -1;
        }
    };

    let (host_str, port_str) = paddr_str.split_at(colon_idx);
    let port_str = &port_str[1..]; // Skip colon

    // Transform port num CStr => actual number
    let lport: libc::c_ushort = match port_str.parse() {
        Ok(p) => p,
        _ => {
            *__errno_location() = EINVAL;
            return -1;
        }
    };
    // Don't need to check if lport is in bounds because of type limits

    if host_str.is_empty() || host_str.len() > 512 {
        *__errno_location() = EINVAL;
        return -1;
    }

    // Copy host string into a C string buffer
    let mut host_buf = Vec::with_capacity(host_str.len() + 1);
    host_buf.extend_from_slice(host_str.as_bytes());
    host_buf.push(0); // null-terminate

    if host_parse(host_buf.as_ptr() as *const libc::c_char, host) < 0 {
        return -1;
    }

    *port = ntohs(lport);
    0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_addr_parse_utls(
    utls_addr_s: *const libc::c_char,
    host: *mut xcm_addr_host,
    port: *mut libc::c_ushort,
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
    port: *mut libc::c_ushort,
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
    port: *mut libc::c_ushort,
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
    port: *mut libc::c_ushort,
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
    port: *mut libc::c_ushort,
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
    port: *mut libc::c_ushort,
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
    port: libc::c_ushort,
    addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
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
    port: libc::c_ushort,
    addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut ip_s: [libc::c_char; 46] = [0; 46];
    if (inet_ntop(
        (*ip).family as libc::c_int,
        &(*ip).addr.ip4 as *const libc::c_uint as *const libc::c_void,
        ip_s.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as libc::c_uint,
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
    port: libc::c_ushort,
    addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
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
    port: libc::c_ushort,
    utls_addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
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
    port: libc::c_ushort,
    tls_addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
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
    port: libc::c_ushort,
    tcp_addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
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
    port: libc::c_ushort,
    sctp_addr_s: *mut libc::c_char,
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
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
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    host_port_make(
        b"btls\0" as *const u8 as *const libc::c_char,
        host,
        port,
        btls_addr_s,
        capacity,
    )
}}
