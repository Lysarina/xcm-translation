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
    fn xcm_addr_parse_utls(
        utls_addr_s: *const libc::c_char,
        host: *mut xcm_addr_host,
        port: *mut uint16_t,
    ) -> libc::c_int;
    fn xcm_addr_parse_tls(
        tls_addr_s: *const libc::c_char,
        host: *mut xcm_addr_host,
        port: *mut uint16_t,
    ) -> libc::c_int;
    fn xcm_addr_parse_tcp(
        tcp_addr_s: *const libc::c_char,
        host: *mut xcm_addr_host,
        port: *mut uint16_t,
    ) -> libc::c_int;
    fn xcm_addr_parse_btcp(
        btcp_addr_s: *const libc::c_char,
        host: *mut xcm_addr_host,
        port: *mut uint16_t,
    ) -> libc::c_int;
    fn xcm_addr_parse_btls(
        btls_addr_s: *const libc::c_char,
        host: *mut xcm_addr_host,
        port: *mut uint16_t,
    ) -> libc::c_int;
    fn xcm_addr_make_utls(
        host: *const xcm_addr_host,
        port: libc::c_ushort,
        utls_addr_s: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_addr_make_tls(
        host: *const xcm_addr_host,
        port: libc::c_ushort,
        tls_addr_s: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_addr_make_tcp(
        host: *const xcm_addr_host,
        port: libc::c_ushort,
        tcp_addr_s: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_addr_make_sctp(
        host: *const xcm_addr_host,
        port: libc::c_ushort,
        sctp_addr_s: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_addr_make_btcp(
        host: *const xcm_addr_host,
        port: libc::c_ushort,
        btcp_addr_s: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_addr_make_btls(
        host: *const xcm_addr_host,
        port: libc::c_ushort,
        btls_addr_s: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
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
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_ip_to_sockaddr(
    mut xcm_ip: *const xcm_addr_ip,
    mut port: uint16_t,
    mut scope: int64_t,
    mut sockaddr: *mut sockaddr,
) {
    memset(
        sockaddr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as libc::c_ulong,
    );
    if (*xcm_ip).family as libc::c_int == 2 as libc::c_int {
        let mut sockaddr4: *mut sockaddr_in = sockaddr as *mut sockaddr_in;
        (*sockaddr4).sin_family = 2 as libc::c_int as sa_family_t;
        (*sockaddr4).sin_addr.s_addr = (*xcm_ip).addr.ip4;
        (*sockaddr4).sin_port = port;
    } else {
        if !((*xcm_ip).family as libc::c_int == 10 as libc::c_int) {
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
                    0 as *mut xcm_socket,
                    b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                    b"xcm_ip->family == 10\0" as *const u8 as *const libc::c_char,
                );
            }
            abort();
        }
        let mut sockaddr6: *mut sockaddr_in6 = sockaddr as *mut sockaddr_in6;
        (*sockaddr6).sin6_family = 10 as libc::c_int as sa_family_t;
        (*sockaddr6).sin6_port = port;
        (*sockaddr6).sin6_scope_id = scope as uint32_t;
        memcpy(
            ((*sockaddr6).sin6_addr.__in6_u.__u6_addr8).as_mut_ptr()
                as *mut libc::c_void,
            ((*xcm_ip).addr.ip6).as_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn sockaddr_to_ip(
    mut sock_addr: *mut sockaddr_storage,
    mut xcm_ip: *mut xcm_addr_ip,
    mut port: *mut uint16_t,
) {
    (*xcm_ip).family = (*sock_addr).ss_family;
    match (*sock_addr).ss_family as libc::c_int {
        2 => {
            let mut sockaddr4: *mut sockaddr_in = sock_addr as *mut sockaddr_in;
            (*xcm_ip).addr.ip4 = (*sockaddr4).sin_addr.s_addr;
            *port = (*sockaddr4).sin_port;
        }
        10 => {
            let mut sockaddr6: *mut sockaddr_in6 = sock_addr as *mut sockaddr_in6;
            memcpy(
                ((*xcm_ip).addr.ip6).as_mut_ptr() as *mut libc::c_void,
                ((*sockaddr6).sin6_addr.__in6_u.__u6_addr8).as_mut_ptr()
                    as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
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
                        0 as *mut xcm_socket,
                        b"Assertion \"%s\" failed.\n\0" as *const u8
                            as *const libc::c_char,
                        b"0\0" as *const u8 as *const libc::c_char,
                    );
                }
                abort();
            }
        }
    };
}
unsafe extern "C" fn sockaddr_to_host(
    mut sock_addr: *mut sockaddr_storage,
    mut xcm_host: *mut xcm_addr_host,
    mut port: *mut uint16_t,
) {
    (*xcm_host).type_0 = xcm_addr_type_ip;
    sockaddr_to_ip(sock_addr, &mut (*xcm_host).c2rust_unnamed.ip, port);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_sockaddr_to_sctp_addr(
    mut sock_addr: *mut sockaddr_storage,
    mut xcm_addr: *mut libc::c_char,
    mut capacity: size_t,
) {
    let mut xcm_host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: uint16_t = 0;
    sockaddr_to_host(sock_addr, &mut xcm_host, &mut port);
    let mut rc: libc::c_int = xcm_addr_make_sctp(
        &mut xcm_host,
        port,
        xcm_addr,
        capacity,
    );
    if !(rc == 0 as libc::c_int) {
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
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_sockaddr_to_btcp_addr(
    mut sock_addr: *mut sockaddr_storage,
    mut xcm_addr: *mut libc::c_char,
    mut capacity: size_t,
) {
    let mut xcm_host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: uint16_t = 0;
    sockaddr_to_host(sock_addr, &mut xcm_host, &mut port);
    let mut rc: libc::c_int = xcm_addr_make_btcp(
        &mut xcm_host,
        port,
        xcm_addr,
        capacity,
    );
    if !(rc == 0 as libc::c_int) {
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
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_sockaddr_to_btls_addr(
    mut sock_addr: *mut sockaddr_storage,
    mut xcm_addr: *mut libc::c_char,
    mut capacity: size_t,
) {
    let mut xcm_host: xcm_addr_host = xcm_addr_host {
        type_0: xcm_addr_type_name,
        c2rust_unnamed: C2RustUnnamed_1 {
            ip: xcm_addr_ip {
                family: 0,
                addr: C2RustUnnamed_0 { ip4: 0 },
            },
        },
    };
    let mut port: uint16_t = 0;
    sockaddr_to_host(sock_addr, &mut xcm_host, &mut port);
    let mut rc: libc::c_int = xcm_addr_make_btls(
        &mut xcm_host,
        port,
        xcm_addr,
        capacity,
    );
    if !(rc == 0 as libc::c_int) {
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
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"rc == 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btcp_to_tcp(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_btcp(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_tcp(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcp_to_btcp(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_tcp(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btcp(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btcp_to_btls(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_btcp(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btls(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btls_to_btcp(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_btls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btcp(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btls_to_tls(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_btls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_tls(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tls_to_btls(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_tls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_btls(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utls_to_tls(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_utls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_tls(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tls_to_utls(
    mut xtls_addr: *const libc::c_char,
    mut ytls_addr: *mut libc::c_char,
    mut capacity: size_t,
) -> libc::c_int {
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
    if xcm_addr_parse_tls(xtls_addr, &mut host, &mut port) != 0 {
        return -(1 as libc::c_int);
    }
    if xcm_addr_make_utls(&mut host, port, ytls_addr, capacity) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_fd_events_name(
    mut events: libc::c_int,
) -> *const libc::c_char {
    if events & (1 as libc::c_int) << 0 as libc::c_int != 0
        && events & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        return b"readable and writable\0" as *const u8 as *const libc::c_char
    } else if events & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        return b"readable\0" as *const u8 as *const libc::c_char
    } else if events & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return b"writeable\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_so_condition_name(
    mut condition: libc::c_int,
) -> *const libc::c_char {
    if condition == 0 as libc::c_int {
        return b"nothing\0" as *const u8 as *const libc::c_char
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
    };
}
