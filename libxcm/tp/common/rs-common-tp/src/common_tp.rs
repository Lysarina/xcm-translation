#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(extern_types)]

use std::process::abort;
use libc::{memcpy, memset, sockaddr, sockaddr_storage, sockaddr_in, sockaddr_in6};

use xcm_rust_common::xcm_tp::xcm_socket;
use rs_log::*;
use rs_xcm_addr::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_ip_to_sockaddr(
    xcm_ip: *const xcm_addr_ip,
    port: libc::c_ushort,
    scope: libc::c_long,
    sockaddr: *mut sockaddr,
) { unsafe {
    memset(
        sockaddr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_storage>(),
    );
    if (*xcm_ip).family as libc::c_int == 2 as libc::c_int {
        let sockaddr4: *mut sockaddr_in = sockaddr as *mut sockaddr_in;
        (*sockaddr4).sin_family = 2 as libc::c_int as libc::c_ushort;
        (*sockaddr4).sin_addr.s_addr = (*xcm_ip).addr.ip4;
        (*sockaddr4).sin_port = port;
    } else {
        if (*xcm_ip).family as libc::c_int != 10 as libc::c_int {
            log_console_conf(1 as libc::c_int != 0);
            if log_is_enabled(log_type_error) {
                __log_event(
                    log_type_error,
                    b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/tp/common/common_tp.c\0"
                        as *const u8 as *const libc::c_char,
                    29 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"tp_ip_to_sockaddr\0"))
                        .as_ptr(),
                    std::ptr::null_mut::<xcm_socket>(),
                    b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                    b"xcm_ip->family == 10\0" as *const u8 as *const libc::c_char,
                );
            }
            abort();
        }
        let sockaddr6: *mut sockaddr_in6 = sockaddr as *mut sockaddr_in6;
        (*sockaddr6).sin6_family = 10 as libc::c_int as libc::c_ushort;
        (*sockaddr6).sin6_port = port;
        (*sockaddr6).sin6_scope_id = scope as libc::c_uint;
        memcpy(
            ((*sockaddr6).sin6_addr.s6_addr).as_mut_ptr()
                as *mut libc::c_void,
            ((*xcm_ip).addr.ip6).as_ptr() as *const libc::c_void,
            16,
        );
    };
}}
unsafe extern "C" fn sockaddr_to_ip(
    sock_addr: *mut sockaddr_storage,
    xcm_ip: *mut xcm_addr_ip,
    port: *mut libc::c_ushort,
) { unsafe {
    (*xcm_ip).family = (*sock_addr).ss_family;
    match (*sock_addr).ss_family as libc::c_int {
        2 => {
            let sockaddr4: *mut sockaddr_in = sock_addr as *mut sockaddr_in;
            (*xcm_ip).addr.ip4 = (*sockaddr4).sin_addr.s_addr;
            *port = (*sockaddr4).sin_port;
        }
        10 => {
            let sockaddr6: *mut sockaddr_in6 = sock_addr as *mut sockaddr_in6;
            memcpy(
                ((*xcm_ip).addr.ip6).as_mut_ptr() as *mut libc::c_void,
                ((*sockaddr6).sin6_addr.s6_addr).as_mut_ptr()
                as *mut libc::c_void,
                16,
            );
            *port = (*sockaddr6).sin6_port;
        }
        _ => {
            if 0 as libc::c_int == 0 {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/tp/common/common_tp.c\0"
                            as *const u8 as *const libc::c_char,
                        57 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 15],
                            &[libc::c_char; 15],
                        >(b"sockaddr_to_ip\0"))
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
    };
}}
unsafe extern "C" fn sockaddr_to_host(
    sock_addr: *mut sockaddr_storage,
    xcm_host: *mut xcm_addr_host,
    port: *mut libc::c_ushort,
) { unsafe {
    (*xcm_host).type_0 = xcm_addr_type_ip;
    sockaddr_to_ip(sock_addr, &mut (*xcm_host).c2rust_unnamed.ip, port);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_sockaddr_to_sctp_addr(
    sock_addr: *mut sockaddr_storage,
    xcm_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) { unsafe {
    let mut xcm_host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    sockaddr_to_host(sock_addr, &mut xcm_host, &mut port);
    let rc: libc::c_int = xcm_addr_make_sctp(
        &xcm_host as *const xcm_addr_host,
        port,
        xcm_addr,
        capacity,
    );
    if rc != 0 as libc::c_int {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/tp/common/common_tp.c\0"
                    as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"tp_sockaddr_to_sctp_addr\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_sockaddr_to_btcp_addr(
    sock_addr: *mut sockaddr_storage,
    xcm_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) { unsafe {
    let mut xcm_host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    sockaddr_to_host(sock_addr, &mut xcm_host, &mut port);
    let rc: libc::c_int = xcm_addr_make_btcp(
        &xcm_host,
        port,
        xcm_addr,
        capacity,
    );
    if rc != 0 as libc::c_int {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/tp/common/common_tp.c\0"
                    as *const u8 as *const libc::c_char,
                89 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"tp_sockaddr_to_btcp_addr\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_sockaddr_to_btls_addr(
    sock_addr: *mut sockaddr_storage,
    xcm_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) { unsafe {
    let mut xcm_host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    sockaddr_to_host(sock_addr, &mut xcm_host, &mut port);
    let rc: libc::c_int = xcm_addr_make_btls(
        &xcm_host,
        port,
        xcm_addr,
        capacity,
    );
    if rc != 0 as libc::c_int {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/tp/common/common_tp.c\0"
                    as *const u8 as *const libc::c_char,
                101 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"tp_sockaddr_to_btls_addr\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btcp_to_tcp(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_btcp(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_tcp(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcp_to_btcp(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_tcp(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btcp(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btcp_to_btls(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_btcp(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btls(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btls_to_btcp(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_btls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btcp(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btls_to_tls(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_btls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_tls(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tls_to_btls(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_tls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btls(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utls_to_tls(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_utls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_tls(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tls_to_utls(
    xtls_addr: *const libc::c_char,
    ytls_addr: *mut libc::c_char,
    capacity: libc::c_ulong,
) -> libc::c_int { unsafe {
    let mut host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: libc::c_ushort = 0;
    if xcm_addr_parse_tls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_utls(&host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_fd_events_name(
    events: libc::c_int,
) -> *const libc::c_char {
    if events & (1 as libc::c_int) << 0 as libc::c_int != 0
        && events & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        b"readable and writable\0" as *const u8 as *const libc::c_char
    } else if events & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        return b"readable\0" as *const u8 as *const libc::c_char
    } else if events & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return b"writeable\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_so_condition_name(
    condition: libc::c_int,
) -> *const libc::c_char {
    if condition == 0 as libc::c_int {
        b"nothing\0" as *const u8 as *const libc::c_char
    } else if condition == (1 as libc::c_int) << 2 as libc::c_int {
        return b"acceptable\0" as *const u8 as *const libc::c_char
    } else if condition
        == (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int
    {
        return b"sendable and receivable\0" as *const u8 as *const libc::c_char
    } else if condition == (1 as libc::c_int) << 1 as libc::c_int {
        return b"sendable\0" as *const u8 as *const libc::c_char
    } else if condition == (1 as libc::c_int) << 0 as libc::c_int {
        return b"receivable\0" as *const u8 as *const libc::c_char
    } else {
        return b"invalid\0" as *const u8 as *const libc::c_char
    }
}
