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

use rs_common_ctl::ctl_get_dir;
use rs_common_ctl::ctl_derive_path;

unsafe extern "C" {
    pub type xpoll;
    pub type attr_tree;
    fn xpoll_fd_reg_add(
        xpoll: *mut xpoll,
        fd: libc::c_int,
        event: libc::c_int,
    ) -> libc::c_int;
    fn xpoll_fd_reg_mod(xpoll: *mut xpoll, reg_id: libc::c_int, event: libc::c_int);
    fn xpoll_fd_reg_del(xpoll: *mut xpoll, reg_id: libc::c_int);
    // fn ctl_get_dir(buf: *mut libc::c_char, capacity: size_t);
    // fn ctl_derive_path(
    //     ctl_dir: *const libc::c_char,
    //     creator_pid: pid_t,
    //     sock_ref: int64_t,
    //     buf: *mut libc::c_char,
    //     capacity: size_t,
    // );
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
    fn __errno_location() -> *mut libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
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
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_calloc(size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
    fn ut_close(fd: libc::c_int);
    fn ut_is_readable(fd: libc::c_int) -> bool;
    fn ut_accept(
        sockfd: libc::c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn xcm_attr_get(
        socket_0: *mut xcm_socket,
        name: *const libc::c_char,
        type_0: *mut xcm_attr_type,
        value: *mut libc::c_void,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_attr_get_all(
        socket_0: *mut xcm_socket,
        cb: xcm_attr_cb,
        cb_data: *mut libc::c_void,
    );
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
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
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl {
    pub socket: *mut xcm_socket,
    pub server_fd: libc::c_int,
    pub server_fd_reg_id: libc::c_int,
    pub clients: [client; 2],
    pub num_clients: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub fd: libc::c_int,
    pub fd_reg_id: libc::c_int,
    pub is_response_pending: bool,
    pub pending_response: ctl_proto_msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_msg {
    pub type_0: ctl_proto_type,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_attr {
    pub name: [libc::c_char; 64],
    pub value_type: xcm_attr_type,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub value_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub bool_value: bool,
    pub int64_value: int64_t,
    pub str_value: [libc::c_char; 512],
    pub any_value: [uint8_t; 512],
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub type xcm_attr_type = libc::c_uint;
pub const xcm_attr_type_double: xcm_attr_type = 5;
pub const xcm_attr_type_bin: xcm_attr_type = 4;
pub const xcm_attr_type_str: xcm_attr_type = 3;
pub const xcm_attr_type_int64: xcm_attr_type = 2;
pub const xcm_attr_type_bool: xcm_attr_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_generic_rej {
    pub rej_errno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_get_attr_cfm {
    pub attr: ctl_proto_attr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_proto_get_attr_req {
    pub attr_name: [libc::c_char; 64],
}
pub type ctl_proto_type = libc::c_uint;
pub const ctl_proto_type_get_all_attr_cfm: ctl_proto_type = 4;
pub const ctl_proto_type_get_all_attr_req: ctl_proto_type = 3;
pub const ctl_proto_type_get_attr_rej: ctl_proto_type = 2;
pub const ctl_proto_type_get_attr_cfm: ctl_proto_type = 1;
pub const ctl_proto_type_get_attr_req: ctl_proto_type = 0;
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
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: __kernel_sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type __kernel_sa_family_t = libc::c_ushort;
pub type log_type = libc::c_uint;
pub const log_type_error: log_type = 1;
pub const log_type_debug: log_type = 0;
pub type socklen_t = __socklen_t;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type __mode_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __gid_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type xcm_attr_cb = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        xcm_attr_type,
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub const MSG_NOSIGNAL: C2RustUnnamed_1 = 16384;
pub type __socket_type = libc::c_uint;
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
pub const MSG_DONTROUTE: C2RustUnnamed_1 = 4;
pub const MSG_PEEK: C2RustUnnamed_1 = 2;
pub const MSG_OOB: C2RustUnnamed_1 = 1;
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
unsafe extern "C" fn create_ux(mut s: *mut xcm_socket) -> libc::c_int {
    let mut ctl_dir: [libc::c_char; 108] = [0; 108];
    ctl_get_dir(
        ctl_dir.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(ctl_dir.as_mut_ptr(), &mut st) < 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                54 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"create_ux\0"))
                    .as_ptr(),
                s,
                b"Error attempting stat XCM control run directory \"%s\"; errno %d (%s).\0"
                    as *const u8 as *const libc::c_char,
                ctl_dir.as_mut_ptr(),
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                58 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"create_ux\0"))
                    .as_ptr(),
                s,
                b"XCM control run directory \"%s\" is not a directory.\0" as *const u8
                    as *const libc::c_char,
                ctl_dir.as_mut_ptr(),
            );
        }
        return -(1 as libc::c_int);
    }
    let mut addr: sockaddr_un = {
        let mut init = sockaddr_un {
            sun_family: 1 as libc::c_int as __kernel_sa_family_t,
            sun_path: [0; 108],
        };
        init
    };
    ctl_derive_path(
        ctl_dir.as_mut_ptr(),
        getpid(),
        (*s).sock_id,
        (addr.sun_path).as_mut_ptr(),
        108 as libc::c_int as size_t,
    );
    unlink((addr.sun_path).as_mut_ptr());
    let mut server_fd: libc::c_int = 0;
    server_fd = socket(
        1 as libc::c_int,
        SOCK_SEQPACKET as libc::c_int | SOCK_NONBLOCK as libc::c_int,
        0 as libc::c_int,
    );
    if !(server_fd < 0 as libc::c_int) {
        if !(bind(
            server_fd,
            &mut addr as *mut sockaddr_un as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
        ) < 0 as libc::c_int)
        {
            if listen(server_fd, 2 as libc::c_int) < 0 as libc::c_int {
                unlink((addr.sun_path).as_mut_ptr());
            } else {
                if log_is_enabled(log_type_debug) {
                    __log_event(
                        log_type_debug,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                            as *const u8 as *const libc::c_char,
                        81 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 10],
                            &[libc::c_char; 10],
                        >(b"create_ux\0"))
                            .as_ptr(),
                        s,
                        b"Created control UNIX domain socket with fd %d at path \"%s\".\0"
                            as *const u8 as *const libc::c_char,
                        server_fd,
                        (addr.sun_path).as_mut_ptr(),
                    );
                }
                return server_fd;
            }
        }
        ut_close(server_fd);
    }
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"create_ux\0"))
                .as_ptr(),
            s,
            b"Unable to create UNIX domain socket at path \"%s\"; errno %d (%s).\0"
                as *const u8 as *const libc::c_char,
            (addr.sun_path).as_mut_ptr(),
            *__errno_location(),
            strerror(*__errno_location()),
        );
    }
    return -(1 as libc::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_create(mut socket_0: *mut xcm_socket) -> *mut ctl {
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut server_fd: libc::c_int = create_ux(socket_0);
    *__errno_location() = _oerrno;
    if server_fd < 0 as libc::c_int {
        return 0 as *mut ctl;
    }
    let mut ctl: *mut ctl = ut_calloc(::core::mem::size_of::<ctl>() as libc::c_ulong)
        as *mut ctl;
    (*ctl).socket = socket_0;
    (*ctl).server_fd = server_fd;
    (*ctl)
        .server_fd_reg_id = xpoll_fd_reg_add(
        (*(*ctl).socket).xpoll,
        (*ctl).server_fd,
        EPOLLIN as libc::c_int,
    );
    return ctl;
}
unsafe extern "C" fn remove_client(mut ctl: *mut ctl, mut client_idx: libc::c_int) {
    let mut rclient: *mut client = &mut *((*ctl).clients)
        .as_mut_ptr()
        .offset(client_idx as isize) as *mut client;
    xpoll_fd_reg_del((*(*ctl).socket).xpoll, (*rclient).fd_reg_id);
    ut_close((*rclient).fd);
    let last_idx: libc::c_int = (*ctl).num_clients - 1 as libc::c_int;
    if client_idx != last_idx {
        memcpy(
            rclient as *mut libc::c_void,
            &mut *((*ctl).clients).as_mut_ptr().offset(last_idx as isize) as *mut client
                as *const libc::c_void,
            ::core::mem::size_of::<client>() as libc::c_ulong,
        );
    }
    if (*ctl).num_clients == 2 as libc::c_int {
        xpoll_fd_reg_mod(
            (*(*ctl).socket).xpoll,
            (*ctl).server_fd_reg_id,
            EPOLLIN as libc::c_int,
        );
    }
    (*ctl).num_clients -= 1;
    (*ctl).num_clients;
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"remove_client\0"))
                .as_ptr(),
            (*ctl).socket,
            b"Removing client.\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_destroy(mut ctl: *mut ctl, mut owner: bool) {
    if !ctl.is_null() {
        let mut _oerrno: libc::c_int = *__errno_location();
        while (*ctl).num_clients > 0 as libc::c_int {
            remove_client(ctl, 0 as libc::c_int);
        }
        let mut laddr: sockaddr_un = sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108],
        };
        let mut laddr_len: socklen_t = ::core::mem::size_of::<sockaddr_un>()
            as libc::c_ulong as socklen_t;
        let mut rc: libc::c_int = getsockname(
            (*ctl).server_fd,
            &mut laddr as *mut sockaddr_un as *mut sockaddr,
            &mut laddr_len,
        );
        if owner {
            xpoll_fd_reg_del((*(*ctl).socket).xpoll, (*ctl).server_fd_reg_id);
        }
        ut_close((*ctl).server_fd);
        if rc == 0 as libc::c_int && owner as libc::c_int != 0 {
            unlink((laddr.sun_path).as_mut_ptr());
        }
        ut_free(ctl as *mut libc::c_void);
        *__errno_location() = _oerrno;
    }
}
unsafe extern "C" fn is_sensitive(mut attr_name: *const libc::c_char) -> bool {
    return strcmp(attr_name, b"tls.key\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
}
unsafe extern "C" fn clear_attr(mut attr: *mut ctl_proto_attr) {
    memset(
        ((*attr).c2rust_unnamed.any_value).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        512 as libc::c_int as libc::c_ulong,
    );
}
unsafe extern "C" fn process_get_attr(
    mut socket_0: *mut xcm_socket,
    mut req: *mut ctl_proto_get_attr_req,
    mut response: *mut ctl_proto_msg,
) {
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"process_get_attr\0"))
                .as_ptr(),
            socket_0,
            b"Control client attempting to retrieve attribute \"%s\".\0" as *const u8
                as *const libc::c_char,
            ((*req).attr_name).as_mut_ptr(),
        );
    }
    let mut cfm: *mut ctl_proto_get_attr_cfm = &mut (*response)
        .c2rust_unnamed
        .get_attr_cfm;
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut rc: libc::c_int = xcm_attr_get(
        socket_0,
        ((*req).attr_name).as_mut_ptr(),
        &mut (*cfm).attr.value_type,
        &mut (*cfm).attr.c2rust_unnamed.any_value as *mut [uint8_t; 512]
            as *mut libc::c_void,
        ::core::mem::size_of::<[uint8_t; 512]>() as libc::c_ulong,
    );
    let mut attr_errno: libc::c_int = *__errno_location();
    *__errno_location() = _oerrno;
    if is_sensitive(((*req).attr_name).as_mut_ptr()) {
        clear_attr(&mut (*cfm).attr);
        rc = -(1 as libc::c_int);
        attr_errno = 13 as libc::c_int;
    }
    if rc >= 0 as libc::c_int {
        (*response).type_0 = ctl_proto_type_get_attr_cfm;
        (*cfm).attr.value_len = rc as size_t;
    } else {
        (*response).type_0 = ctl_proto_type_get_attr_rej;
        (*response).c2rust_unnamed.get_attr_rej.rej_errno = attr_errno;
    };
}
unsafe extern "C" fn add_attr(
    mut attr_name: *const libc::c_char,
    mut type_0: xcm_attr_type,
    mut value: *mut libc::c_void,
    mut len: size_t,
    mut data: *mut libc::c_void,
) {
    if is_sensitive(attr_name) {
        return;
    }
    let mut cfm: *mut ctl_proto_get_all_attr_cfm = data
        as *mut ctl_proto_get_all_attr_cfm;
    let mut attr: *mut ctl_proto_attr = &mut *((*cfm).attrs)
        .as_mut_ptr()
        .offset((*cfm).attrs_len as isize) as *mut ctl_proto_attr;
    (*cfm).attrs_len = ((*cfm).attrs_len).wrapping_add(1);
    (*cfm).attrs_len;
    if !((*cfm).attrs_len < 64 as libc::c_int as libc::c_ulong) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                211 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_attr\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"cfm->attrs_len < (64)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    strcpy(((*attr).name).as_mut_ptr(), attr_name);
    (*attr).value_type = type_0;
    if !((*attr).value_len < ::core::mem::size_of::<[uint8_t; 512]>() as libc::c_ulong) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                216 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_attr\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"attr->value_len < sizeof(attr->any_value)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    memcpy(
        ((*attr).c2rust_unnamed.any_value).as_mut_ptr() as *mut libc::c_void,
        value,
        len,
    );
    (*attr).value_len = len;
}
unsafe extern "C" fn process_get_all_attr(
    mut socket_0: *mut xcm_socket,
    mut response: *mut ctl_proto_msg,
) {
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"process_get_all_attr\0"))
                .as_ptr(),
            socket_0,
            b"Control client attempting retrieve all attributes.\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut cfm: *mut ctl_proto_get_all_attr_cfm = &mut (*response)
        .c2rust_unnamed
        .get_all_attr_cfm;
    (*cfm).attrs_len = 0 as libc::c_int as size_t;
    xcm_attr_get_all(
        socket_0,
        Some(
            add_attr
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    xcm_attr_type,
                    *mut libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        cfm as *mut libc::c_void,
    );
}
unsafe extern "C" fn client_send(
    mut client: *mut client,
    mut ctl: *mut ctl,
) -> libc::c_int {
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut rc: libc::c_int = send(
        (*client).fd,
        &mut (*client).pending_response as *mut ctl_proto_msg as *const libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
        MSG_NOSIGNAL as libc::c_int,
    ) as libc::c_int;
    let mut send_errno: libc::c_int = *__errno_location();
    *__errno_location() = _oerrno;
    if rc < 0 as libc::c_int {
        if send_errno == 11 as libc::c_int {
            return 0 as libc::c_int;
        }
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                243 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 12],
                    &[libc::c_char; 12],
                >(b"client_send\0"))
                    .as_ptr(),
                (*ctl).socket,
                b"Error talking to control client on fd %d; errno %d (%s).\0"
                    as *const u8 as *const libc::c_char,
                (*client).fd,
                send_errno,
                strerror(send_errno),
            );
        }
        return -(1 as libc::c_int);
    }
    (*client).is_response_pending = 0 as libc::c_int != 0;
    xpoll_fd_reg_mod(
        (*(*ctl).socket).xpoll,
        (*client).fd_reg_id,
        EPOLLIN as libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn client_receive(
    mut client: *mut client,
    mut ctl: *mut ctl,
) -> libc::c_int {
    let mut res: *mut ctl_proto_msg = 0 as *mut ctl_proto_msg;
    let mut current_block: u64;
    if !ut_is_readable((*client).fd) {
        return 0 as libc::c_int;
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut req: *mut ctl_proto_msg = ut_malloc(
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
    ) as *mut ctl_proto_msg;
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut recv_rc: libc::c_int = recv(
        (*client).fd,
        req as *mut libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
        0 as libc::c_int,
    ) as libc::c_int;
    let mut recv_errno: libc::c_int = *__errno_location();
    *__errno_location() = _oerrno;
    if recv_rc < 0 as libc::c_int {
        if recv_errno == 11 as libc::c_int {
            rc = 0 as libc::c_int;
        } else if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                273 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"client_receive\0"))
                    .as_ptr(),
                (*ctl).socket,
                b"Error talking to control client on fd %d; errno %d (%s).\0"
                    as *const u8 as *const libc::c_char,
                (*client).fd,
                recv_errno,
                strerror(recv_errno),
            );
        }
    } else if recv_rc == 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                276 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"client_receive\0"))
                    .as_ptr(),
                (*ctl).socket,
                b"Client disconnected.\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if recv_rc as libc::c_ulong
        != ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong
    {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"client_receive\0"))
                    .as_ptr(),
                (*ctl).socket,
                b"Received malformed control message from client.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        (*client).is_response_pending = 1 as libc::c_int != 0;
        xpoll_fd_reg_mod(
            (*(*ctl).socket).xpoll,
            (*client).fd_reg_id,
            EPOLLOUT as libc::c_int,
        );
        res = &mut (*client).pending_response;
        match (*req).type_0 as libc::c_uint {
            0 => {
                process_get_attr(
                    (*ctl).socket,
                    &mut (*req).c2rust_unnamed.get_attr_req,
                    res,
                );
                current_block = 15897653523371991391;
            }
            3 => {
                process_get_all_attr((*ctl).socket, res);
                current_block = 15897653523371991391;
            }
            _ => {
                if log_is_enabled(log_type_debug) {
                    __log_event(
                        log_type_debug,
                        b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                            as *const u8 as *const libc::c_char,
                        296 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 15],
                            &[libc::c_char; 15],
                        >(b"client_receive\0"))
                            .as_ptr(),
                        (*ctl).socket,
                        b"Received malformed control message from client.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*client).is_response_pending = 0 as libc::c_int != 0;
                current_block = 59788533810856093;
            }
        }
        match current_block {
            59788533810856093 => {}
            _ => {
                rc = 0 as libc::c_int;
            }
        }
    }
    ut_free(req as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn process_client(
    mut client: *mut client,
    mut ctl: *mut ctl,
) -> libc::c_int {
    if (*client).is_response_pending {
        return client_send(client, ctl)
    } else {
        return client_receive(client, ctl)
    };
}
unsafe extern "C" fn accept_client(mut ctl: *mut ctl) {
    if !ut_is_readable((*ctl).server_fd) {
        return;
    }
    let mut client_fd: libc::c_int = ut_accept(
        (*ctl).server_fd,
        0 as *mut sockaddr,
        0 as *mut socklen_t,
        SOCK_NONBLOCK as libc::c_int as libc::c_uint,
    );
    if client_fd < 0 as libc::c_int {
        if *__errno_location() != 11 as libc::c_int {
            if log_is_enabled(log_type_debug) {
                __log_event(
                    log_type_debug,
                    b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                        as *const u8 as *const libc::c_char,
                    326 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"accept_client\0"))
                        .as_ptr(),
                    (*ctl).socket,
                    b"Error accepting new client on control socket; errno %d (%s).\0"
                        as *const u8 as *const libc::c_char,
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
        }
        return;
    }
    let mut nclient: *mut client = &mut *((*ctl).clients)
        .as_mut_ptr()
        .offset((*ctl).num_clients as isize) as *mut client;
    (*ctl).num_clients += 1;
    (*ctl).num_clients;
    (*nclient).fd = client_fd;
    (*nclient)
        .fd_reg_id = xpoll_fd_reg_add(
        (*(*ctl).socket).xpoll,
        (*nclient).fd,
        EPOLLIN as libc::c_int,
    );
    (*nclient).is_response_pending = 0 as libc::c_int != 0;
    if (*ctl).num_clients == 2 as libc::c_int {
        xpoll_fd_reg_mod(
            (*(*ctl).socket).xpoll,
            (*ctl).server_fd_reg_id,
            0 as libc::c_int,
        );
    }
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                as *const u8 as *const libc::c_char,
            340 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"accept_client\0"))
                .as_ptr(),
            (*ctl).socket,
            b"New control client with fd %d accepted; now %d clients connected.\0"
                as *const u8 as *const libc::c_char,
            (*nclient).fd,
            (*ctl).num_clients,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_process(mut ctl: *mut ctl) {
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ctl).num_clients {
        if process_client(&mut *((*ctl).clients).as_mut_ptr().offset(i as isize), ctl)
            < 0 as libc::c_int
        {
            remove_client(ctl, i);
            ctl_process(ctl);
        }
        i += 1;
        i;
    }
    if (*ctl).num_clients < 2 as libc::c_int {
        accept_client(ctl);
    }
    *__errno_location() = _oerrno;
}
