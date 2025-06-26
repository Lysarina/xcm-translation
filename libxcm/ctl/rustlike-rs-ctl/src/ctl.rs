#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc,
    clippy::while_immutable_condition
)]

use std::process::abort;
use libc::{__errno_location, sockaddr, strerror, memcpy, strcmp,
    memset, strcpy, sockaddr_un, EPOLLIN, SOCK_NONBLOCK, SOCK_SEQPACKET,
    MSG_NOSIGNAL, EPOLLOUT, socket, bind, getsockname, send, recv,
    listen, getpid, unlink, stat};

use rs_common_ctl::{ctl_get_dir, ctl_derive_path};
use rs_log::*;
use rs_util::*;
use xcm_rust_common::attr_tree_mod::*;
use xcm_rust_common::xcm_tp::*;
use xcm_rust_common::xcm_attr::*;
use rs_xpoll::*;

unsafe extern "C" {
    // These are from xcm.rs, cannot import w/ Rust because cyclic
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
}
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
    pub int64_value: libc::c_long,
    pub str_value: [libc::c_char; 512],
    pub any_value: [libc::c_uchar; 512],
}
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
unsafe extern "C" fn create_ux(s: *mut xcm_socket) -> libc::c_int { unsafe {
    let mut ctl_dir: [libc::c_char; 108] = [0; 108];
    ctl_get_dir(
        ctl_dir.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    let mut st: stat = std::mem::zeroed();
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
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint != 0o40000 as libc::c_int as libc::c_uint
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
        
        sockaddr_un {
            sun_family: 1 as libc::c_int as libc::c_ushort,
            sun_path: [0; 108],
        }
    };
    ctl_derive_path(
        ctl_dir.as_mut_ptr(),
        getpid(),
        (*s).sock_id,
        (addr.sun_path).as_mut_ptr(),
        108 as libc::c_int as size_t,
    );
    unlink((addr.sun_path).as_mut_ptr());
    let server_fd: libc::c_int = socket(
        1 as libc::c_int,
        SOCK_SEQPACKET as libc::c_int | SOCK_NONBLOCK as libc::c_int,
        0 as libc::c_int,
    );
    if server_fd >= 0 as libc::c_int {
        if bind(
            server_fd,
            &mut addr as *mut sockaddr_un as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong as libc::c_uint,
        ) >= 0 as libc::c_int
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
    -(1 as libc::c_int)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_create(socket_0: *mut xcm_socket) -> *mut ctl { unsafe {
    let mut _oerrno: libc::c_int = *__errno_location();
    let server_fd: libc::c_int = create_ux(socket_0);
    *__errno_location() = _oerrno;
    if server_fd < 0 as libc::c_int {
        return std::ptr::null_mut::<ctl>();
    }
    let ctl: *mut ctl = ut_calloc(::core::mem::size_of::<ctl>() as libc::c_ulong)
        as *mut ctl;
    (*ctl).socket = socket_0;
    (*ctl).server_fd = server_fd;
    (*ctl)
        .server_fd_reg_id = xpoll_fd_reg_add(
        (*(*ctl).socket).xpoll,
        (*ctl).server_fd,
        EPOLLIN as libc::c_int,
    );
    ctl
}}
unsafe extern "C" fn remove_client(ctl: *mut ctl, client_idx: libc::c_int) { unsafe {
    let rclient: *mut client = &mut *((*ctl).clients)
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
            ::core::mem::size_of::<client>(),
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_destroy(ctl: *mut ctl, owner: bool) { unsafe {
    if !ctl.is_null() {
        let mut _oerrno: libc::c_int = *__errno_location();
        while (*ctl).num_clients > 0 as libc::c_int {
            remove_client(ctl, 0 as libc::c_int);
        }
        let mut laddr: sockaddr_un = sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108],
        };
        let mut laddr_len: libc::c_uint = ::core::mem::size_of::<sockaddr_un>()
            as libc::c_ulong as libc::c_uint;
        let rc: libc::c_int = getsockname(
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
}}
unsafe extern "C" fn is_sensitive(attr_name: *const libc::c_char) -> bool { unsafe {
    strcmp(attr_name, b"tls.key\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
}}
unsafe extern "C" fn clear_attr(attr: *mut ctl_proto_attr) { unsafe {
    memset(
        ((*attr).c2rust_unnamed.any_value).as_mut_ptr() as *mut libc::c_void,
        0,
        512_usize,
    );
}}
unsafe extern "C" fn process_get_attr(
    socket_0: *mut xcm_socket,
    req: *mut ctl_proto_get_attr_req,
    response: *mut ctl_proto_msg,
) { unsafe {
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
    let cfm: *mut ctl_proto_get_attr_cfm = &mut (*response)
        .c2rust_unnamed
        .get_attr_cfm;
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut rc: libc::c_int = xcm_attr_get(
        socket_0,
        ((*req).attr_name).as_mut_ptr(),
        &mut (*cfm).attr.value_type,
        &mut (*cfm).attr.c2rust_unnamed.any_value as *mut [libc::c_uchar; 512]
            as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong,
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
}}
unsafe extern "C" fn add_attr(
    attr_name: *const libc::c_char,
    type_0: xcm_attr_type,
    value: *mut libc::c_void,
    len: size_t,
    data: *mut libc::c_void,
) { unsafe {
    if is_sensitive(attr_name) {
        return;
    }
    let cfm: *mut ctl_proto_get_all_attr_cfm = data
        as *mut ctl_proto_get_all_attr_cfm;
    let attr: *mut ctl_proto_attr = &mut *((*cfm).attrs)
        .as_mut_ptr()
        .offset((*cfm).attrs_len as isize) as *mut ctl_proto_attr;
    (*cfm).attrs_len = ((*cfm).attrs_len).wrapping_add(1);
    if (*cfm).attrs_len >= 64 as libc::c_int as libc::c_ulong {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                211 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_attr\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"cfm->attrs_len < (64)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    strcpy(((*attr).name).as_mut_ptr(), attr_name);
    (*attr).value_type = type_0;
    if (*attr).value_len >= ::core::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
                    as *const u8 as *const libc::c_char,
                216 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_attr\0"))
                    .as_ptr(),
                std::ptr::null_mut::<xcm_socket>(),
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
        len as usize,
    );
    (*attr).value_len = len;
}}
unsafe extern "C" fn process_get_all_attr(
    socket_0: *mut xcm_socket,
    response: *mut ctl_proto_msg,
) { unsafe {
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
    let cfm: *mut ctl_proto_get_all_attr_cfm = &mut (*response)
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
}}
unsafe extern "C" fn client_send(
    client: *mut client,
    ctl: *mut ctl,
) -> libc::c_int { unsafe {
    let mut _oerrno: libc::c_int = *__errno_location();
    let rc: libc::c_int = send(
        (*client).fd,
        &mut (*client).pending_response as *mut ctl_proto_msg as *const libc::c_void,
        ::core::mem::size_of::<ctl_proto_msg>(),
        MSG_NOSIGNAL as libc::c_int,
    ) as libc::c_int;
    let send_errno: libc::c_int = *__errno_location();
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
    0 as libc::c_int
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_receive(
    client: *mut client,
    ctl: *mut ctl,
) -> libc::c_int { unsafe {
    if !ut_is_readable((*client).fd) {
        return 0;
    }

    let mut rc = -1;
    let req = ut_malloc(std::mem::size_of::<ctl_proto_msg>() as libc::c_ulong)
        as *mut ctl_proto_msg;

    let saved_errno = *__errno_location();
    let recv_rc = recv(
        (*client).fd,
        req as *mut libc::c_void,
        std::mem::size_of::<ctl_proto_msg>(),
        0,
    );
    let recv_errno = *__errno_location();
    *__errno_location() = saved_errno;

    if recv_rc < 0 {
        if recv_errno == libc::EAGAIN {
            rc = 0;
        } else if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                c"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c".as_ptr() as *const _,
                273,
                c"client_receive".as_ptr() as *const _,
                (*ctl).socket,
                c"Error talking to control client on fd %d; errno %d (%s).".as_ptr() as *const _,
                (*client).fd,
                recv_errno,
                strerror(recv_errno),
            );
        }
    } else if recv_rc == 0 {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                c"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c".as_ptr() as *const _,
                276,
                c"client_receive".as_ptr() as *const _,
                (*ctl).socket,
                c"Client disconnected.".as_ptr() as *const _,
            );
        }
    } else if recv_rc as usize != std::mem::size_of::<ctl_proto_msg>() {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                c"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c".as_ptr() as *const _,
                279,
                c"client_receive".as_ptr() as *const _,
                (*ctl).socket,
                c"Received malformed control message from client.".as_ptr() as *const _,
            );
        }
    } else {
        (*client).is_response_pending = true;
        xpoll_fd_reg_mod(
            (*(*ctl).socket).xpoll,
            (*client).fd_reg_id,
            EPOLLOUT,
        );

        let res = &mut (*client).pending_response;
        match (*req).type_0 as libc::c_uint {
            0 => {
                process_get_attr((*ctl).socket, &mut (*req).c2rust_unnamed.get_attr_req, res);
                rc = 0;
            }
            3 => {
                process_get_all_attr((*ctl).socket, res);
                rc = 0;
            }
            _ => {
                if log_is_enabled(log_type_debug) {
                    __log_event(
                        log_type_debug,
                        c"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c".as_ptr() as *const _,
                        296,
                        c"client_receive".as_ptr() as *const _,
                        (*ctl).socket,
                        c"Received malformed control message from client.".as_ptr() as *const _,
                    );
                }
                (*client).is_response_pending = false;
            }
        }
    }

    ut_free(req as *mut libc::c_void);
    rc
}}


// unsafe extern "C" fn client_receive(
//     client: *mut client,
//     ctl: *mut ctl,
// ) -> libc::c_int { unsafe {
//     let res: *mut ctl_proto_msg;// = std::ptr::null_mut::<ctl_proto_msg>();
//     let current_block: u64;
//     if !ut_is_readable((*client).fd) {
//         return 0 as libc::c_int;
//     }
//     let mut rc: libc::c_int = -(1 as libc::c_int);
//     let req: *mut ctl_proto_msg = ut_malloc(
//         ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong,
//     ) as *mut ctl_proto_msg;
//     let mut _oerrno: libc::c_int = *__errno_location();
//     let recv_rc: libc::c_int = recv(
//         (*client).fd,
//         req as *mut libc::c_void,
//         ::core::mem::size_of::<ctl_proto_msg>(),
//         0 as libc::c_int,
//     ) as libc::c_int;
//     let recv_errno: libc::c_int = *__errno_location();
//     *__errno_location() = _oerrno;
//     if recv_rc < 0 as libc::c_int {
//         if recv_errno == 11 as libc::c_int {
//             rc = 0 as libc::c_int;
//         } else if log_is_enabled(log_type_debug) {
//             __log_event(
//                 log_type_debug,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
//                     as *const u8 as *const libc::c_char,
//                 273 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 15],
//                     &[libc::c_char; 15],
//                 >(b"client_receive\0"))
//                     .as_ptr(),
//                 (*ctl).socket,
//                 b"Error talking to control client on fd %d; errno %d (%s).\0"
//                     as *const u8 as *const libc::c_char,
//                 (*client).fd,
//                 recv_errno,
//                 strerror(recv_errno),
//             );
//         }
//     } else if recv_rc == 0 as libc::c_int {
//         if log_is_enabled(log_type_debug) {
//             __log_event(
//                 log_type_debug,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
//                     as *const u8 as *const libc::c_char,
//                 276 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 15],
//                     &[libc::c_char; 15],
//                 >(b"client_receive\0"))
//                     .as_ptr(),
//                 (*ctl).socket,
//                 b"Client disconnected.\0" as *const u8 as *const libc::c_char,
//             );
//         }
//     } else if recv_rc as libc::c_ulong
//         != ::core::mem::size_of::<ctl_proto_msg>() as libc::c_ulong
//     {
//         if log_is_enabled(log_type_debug) {
//             __log_event(
//                 log_type_debug,
//                 b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
//                     as *const u8 as *const libc::c_char,
//                 279 as libc::c_int,
//                 (*::core::mem::transmute::<
//                     &[u8; 15],
//                     &[libc::c_char; 15],
//                 >(b"client_receive\0"))
//                     .as_ptr(),
//                 (*ctl).socket,
//                 b"Received malformed control message from client.\0" as *const u8
//                     as *const libc::c_char,
//             );
//         }
//     } else {
//         (*client).is_response_pending = 1 as libc::c_int != 0;
//         xpoll_fd_reg_mod(
//             (*(*ctl).socket).xpoll,
//             (*client).fd_reg_id,
//             EPOLLOUT as libc::c_int,
//         );
//         res = &mut (*client).pending_response;
//         match (*req).type_0 as libc::c_uint {
//             0 => {
//                 process_get_attr(
//                     (*ctl).socket,
//                     &mut (*req).c2rust_unnamed.get_attr_req,
//                     res,
//                 );
//                 current_block = 15897653523371991391;
//             }
//             3 => {
//                 process_get_all_attr((*ctl).socket, res);
//                 current_block = 15897653523371991391;
//             }
//             _ => {
//                 if log_is_enabled(log_type_debug) {
//                     __log_event(
//                         log_type_debug,
//                         b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/ctl/ctl.c\0"
//                             as *const u8 as *const libc::c_char,
//                         296 as libc::c_int,
//                         (*::core::mem::transmute::<
//                             &[u8; 15],
//                             &[libc::c_char; 15],
//                         >(b"client_receive\0"))
//                             .as_ptr(),
//                         (*ctl).socket,
//                         b"Received malformed control message from client.\0" as *const u8
//                             as *const libc::c_char,
//                     );
//                 }
//                 (*client).is_response_pending = 0 as libc::c_int != 0;
//                 current_block = 59788533810856093;
//             }
//         }
//         match current_block {
//             59788533810856093 => {}
//             _ => {
//                 rc = 0 as libc::c_int;
//             }
//         }
//     }
//     ut_free(req as *mut libc::c_void);
//     rc
// }}
unsafe extern "C" fn process_client(
    client: *mut client,
    ctl: *mut ctl,
) -> libc::c_int { unsafe {
    if (*client).is_response_pending {
        client_send(client, ctl)
    } else {
        client_receive(client, ctl)
    }
}}
unsafe extern "C" fn accept_client(ctl: *mut ctl) { unsafe {
    if !ut_is_readable((*ctl).server_fd) {
        return;
    }
    let client_fd: libc::c_int = ut_accept(
        (*ctl).server_fd,
        std::ptr::null_mut::<sockaddr>(),
        std::ptr::null_mut::<libc::c_uint>(),
        SOCK_NONBLOCK as libc::c_int as libc::c_uint,
    );
    if client_fd < 0 as libc::c_int {
        if *__errno_location() != 11 as libc::c_int && log_is_enabled(log_type_debug) {
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
        return;
    }
    let nclient: *mut client = &mut *((*ctl).clients)
        .as_mut_ptr()
        .offset((*ctl).num_clients as isize) as *mut client;
    (*ctl).num_clients += 1;
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctl_process(ctl: *mut ctl) { unsafe {
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut i: libc::c_int = 0;
    while i < (*ctl).num_clients {
        if process_client(&mut *((*ctl).clients).as_mut_ptr().offset(i as isize), ctl)
            < 0 as libc::c_int
        {
            remove_client(ctl, i);
            ctl_process(ctl);
        }
        i += 1;
    }
    if (*ctl).num_clients < 2 as libc::c_int {
        accept_client(ctl);
    }
    *__errno_location() = _oerrno;
}}
