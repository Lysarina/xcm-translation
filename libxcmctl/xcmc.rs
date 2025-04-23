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
    pub type __dirstream;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_in6;
    pub type sockaddr_in;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn ctl_get_dir(buf: *mut libc::c_char, capacity: size_t);
    fn ctl_derive_path(
        ctl_dir: *const libc::c_char,
        creator_pid: pid_t,
        sock_ref: int64_t,
        buf: *mut libc::c_char,
        capacity: size_t,
    );
    fn ctl_parse_info(
        filename: *const libc::c_char,
        creator_pid: *mut pid_t,
        sock_ref: *mut int64_t,
    ) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type xcm_attr_type = libc::c_uint;
pub const xcm_attr_type_double: xcm_attr_type = 5;
pub const xcm_attr_type_bin: xcm_attr_type = 4;
pub const xcm_attr_type_str: xcm_attr_type = 3;
pub const xcm_attr_type_int64: xcm_attr_type = 2;
pub const xcm_attr_type_bool: xcm_attr_type = 1;
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type xcmc_list_cb = Option::<
    unsafe extern "C" fn(pid_t, int64_t, *mut libc::c_void) -> (),
>;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcmc_session {
    pub fd: libc::c_int,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: __kernel_sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type __kernel_sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub const SOCK_SEQPACKET: __socket_type = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_attr {
    pub name: [libc::c_char; 64],
    pub value_type: xcm_attr_type,
    pub c2rust_unnamed: C2RustUnnamed,
    pub value_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub bool_value: bool,
    pub int64_value: int64_t,
    pub str_value: [libc::c_char; 512],
    pub any_value: [uint8_t; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_get_attr_cfm {
    pub attr: ctl_proto_attr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub get_attr_req: ctl_proto_get_attr_req,
    pub get_attr_cfm: ctl_proto_get_attr_cfm,
    pub get_attr_rej: ctl_proto_generic_rej,
    pub get_all_attr_cfm: ctl_proto_get_all_attr_cfm,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_get_all_attr_cfm {
    pub attrs: [ctl_proto_attr; 64],
    pub attrs_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_generic_rej {
    pub rej_errno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_get_attr_req {
    pub attr_name: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_msg {
    pub type_0: ctl_proto_type,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
pub type ctl_proto_type = libc::c_uint;
pub const ctl_proto_type_get_all_attr_cfm: ctl_proto_type = 4;
pub const ctl_proto_type_get_all_attr_req: ctl_proto_type = 3;
pub const ctl_proto_type_get_attr_rej: ctl_proto_type = 2;
pub const ctl_proto_type_get_attr_cfm: ctl_proto_type = 1;
pub const ctl_proto_type_get_attr_req: ctl_proto_type = 0;
pub const MSG_NOSIGNAL: C2RustUnnamed_1 = 16384;
pub type xcmc_attr_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_1 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_1 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_1 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_1 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_1 = 65536;
pub const MSG_MORE: C2RustUnnamed_1 = 32768;
pub const MSG_ERRQUEUE: C2RustUnnamed_1 = 8192;
pub const MSG_RST: C2RustUnnamed_1 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_1 = 2048;
pub const MSG_SYN: C2RustUnnamed_1 = 1024;
pub const MSG_FIN: C2RustUnnamed_1 = 512;
pub const MSG_WAITALL: C2RustUnnamed_1 = 256;
pub const MSG_EOR: C2RustUnnamed_1 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_1 = 64;
pub const MSG_TRUNC: C2RustUnnamed_1 = 32;
pub const MSG_PROXY: C2RustUnnamed_1 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_1 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_1 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_1 = 4;
pub const MSG_PEEK: C2RustUnnamed_1 = 2;
pub const MSG_OOB: C2RustUnnamed_1 = 1;
#[no_mangle]
pub unsafe extern "C" fn xcmc_list(
    mut cb: xcmc_list_cb,
    mut cb_data: *mut libc::c_void,
) -> libc::c_int {
    let mut ctl_dir: [libc::c_char; 4096] = [0; 4096];
    ctl_get_dir(
        ctl_dir.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    let mut d: *mut DIR = opendir(ctl_dir.as_mut_ptr());
    if d.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        let mut ent: *mut dirent = readdir(d);
        if ent.is_null() {
            break;
        }
        let mut creator_pid: pid_t = 0;
        let mut sock_ref: int64_t = 0;
        if ctl_parse_info(
            ((*ent).d_name).as_mut_ptr(),
            &mut creator_pid,
            &mut sock_ref,
        ) {
            cb.expect("non-null function pointer")(creator_pid, sock_ref, cb_data);
        }
    }
    closedir(d);
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_tmo(mut fd: libc::c_int, mut tmo: useconds_t) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_usec = tmo as __suseconds_t;
    if setsockopt(
        fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tv as *mut timeval as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
        || setsockopt(
            fd,
            1 as libc::c_int,
            21 as libc::c_int,
            &mut tv as *mut timeval as *const libc::c_void,
            ::core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xcmc_open(
    mut creator_pid: pid_t,
    mut sock_ref: int64_t,
) -> *mut xcmc_session {
    let mut addr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut s: *mut xcmc_session = 0 as *mut xcmc_session;
    let mut ctl_dir: [libc::c_char; 4096] = [0; 4096];
    ctl_get_dir(
        ctl_dir.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    let mut path: [libc::c_char; 4096] = [0; 4096];
    ctl_derive_path(
        ctl_dir.as_mut_ptr(),
        creator_pid,
        sock_ref,
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    let mut fd: libc::c_int = 0;
    fd = socket(1 as libc::c_int, SOCK_SEQPACKET as libc::c_int, 0 as libc::c_int);
    if !(fd < 0 as libc::c_int) {
        if !(set_tmo(fd, (300 as libc::c_int * 1000 as libc::c_int) as useconds_t)
            < 0 as libc::c_int)
        {
            addr = {
                let mut init = sockaddr_un {
                    sun_family: 1 as libc::c_int as __kernel_sa_family_t,
                    sun_path: [0; 108],
                };
                init
            };
            strcpy((addr.sun_path).as_mut_ptr(), path.as_mut_ptr());
            if !(connect(
                fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut addr as *mut sockaddr_un as *mut sockaddr,
                },
                ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
            ) < 0 as libc::c_int)
            {
                s = ut_malloc(::core::mem::size_of::<xcmc_session>() as libc::c_ulong)
                    as *mut xcmc_session;
                (*s).fd = fd;
                return s;
            }
        }
        let mut _errno: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = _errno;
    }
    return 0 as *mut xcmc_session;
}
#[no_mangle]
pub unsafe extern "C" fn xcmc_close(mut session: *mut xcmc_session) -> libc::c_int {
    if !session.is_null() {
        let mut fd: libc::c_int = (*session).fd;
        ut_free(session as *mut libc::c_void);
        return close(fd);
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xcmc_attr_get(
    mut session: *mut xcmc_session,
    mut attr_name: *const libc::c_char,
    mut value_type: *mut xcm_attr_type,
    mut attr_value: *mut libc::c_void,
    mut value_capacity: size_t,
) -> libc::c_int {
    if strlen(attr_name) >= 64 as libc::c_int as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut req: ctl_proto_msg = {
        let mut init = ctl_proto_msg {
            type_0: ctl_proto_type_get_attr_req,
            c2rust_unnamed: C2RustUnnamed_0 {
                get_attr_req: ctl_proto_get_attr_req {
                    attr_name: [0; 64],
                },
            },
        };
        init
    };
    strcpy((req.c2rust_unnamed.get_attr_req.attr_name).as_mut_ptr(), attr_name);
    if send(
        (*session).fd,
        &mut req as *mut ctl_proto_msg as *const libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
        MSG_NOSIGNAL as libc::c_int,
    ) as libc::c_ulong != ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    let mut res: ctl_proto_msg = ctl_proto_msg {
        type_0: ctl_proto_type_get_attr_req,
        c2rust_unnamed: C2RustUnnamed_0 {
            get_attr_req: ctl_proto_get_attr_req {
                attr_name: [0; 64],
            },
        },
    };
    if recv(
        (*session).fd,
        &mut res as *mut ctl_proto_msg as *mut libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
        0 as libc::c_int,
    ) as libc::c_ulong != ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    let mut attr: *mut ctl_proto_attr = &mut res.c2rust_unnamed.get_attr_cfm.attr;
    match res.type_0 as libc::c_uint {
        2 => {
            *__errno_location() = res.c2rust_unnamed.get_attr_rej.rej_errno;
            return -(1 as libc::c_int);
        }
        1 => {
            if (*attr).value_len > value_capacity {
                *__errno_location() = 75 as libc::c_int;
                return -(1 as libc::c_int);
            }
            memcpy(
                attr_value,
                &mut (*attr).c2rust_unnamed.any_value as *mut [uint8_t; 512]
                    as *const libc::c_void,
                (*attr).value_len,
            );
            if !value_type.is_null() {
                *value_type = (*attr).value_type;
            }
            return (*attr).value_len as libc::c_int;
        }
        _ => {
            *__errno_location() = 71 as libc::c_int;
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xcmc_attr_get_all(
    mut session: *mut xcmc_session,
    mut cb: xcmc_attr_cb,
    mut cb_data: *mut libc::c_void,
) -> libc::c_int {
    let mut req: ctl_proto_msg = {
        let mut init = ctl_proto_msg {
            type_0: ctl_proto_type_get_all_attr_req,
            c2rust_unnamed: C2RustUnnamed_0 {
                get_attr_req: ctl_proto_get_attr_req {
                    attr_name: [0; 64],
                },
            },
        };
        init
    };
    if send(
        (*session).fd,
        &mut req as *mut ctl_proto_msg as *const libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
        MSG_NOSIGNAL as libc::c_int,
    ) as libc::c_ulong != ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    let mut res: ctl_proto_msg = ctl_proto_msg {
        type_0: ctl_proto_type_get_attr_req,
        c2rust_unnamed: C2RustUnnamed_0 {
            get_attr_req: ctl_proto_get_attr_req {
                attr_name: [0; 64],
            },
        },
    };
    if recv(
        (*session).fd,
        &mut res as *mut ctl_proto_msg as *mut libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
        0 as libc::c_int,
    ) as libc::c_ulong != ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    if res.type_0 as libc::c_uint
        != ctl_proto_type_get_attr_cfm as libc::c_int as libc::c_uint
    {
        *__errno_location() = 71 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cfm: *mut ctl_proto_get_all_attr_cfm = &mut res
        .c2rust_unnamed
        .get_all_attr_cfm;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*cfm).attrs_len {
        let mut attr: *mut ctl_proto_attr = &mut *((*cfm).attrs)
            .as_mut_ptr()
            .offset(i as isize) as *mut ctl_proto_attr;
        cb
            .expect(
                "non-null function pointer",
            )(
            ((*attr).name).as_mut_ptr(),
            (*attr).value_type,
            ((*attr).c2rust_unnamed.any_value).as_mut_ptr() as *mut libc::c_void,
            (*attr).value_len,
            cb_data,
        );
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
