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
    pub type attr_tree;
    fn epoll_create1(__flags: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
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
    fn active_fd_get() -> libc::c_int;
    fn active_fd_put(fd: libc::c_int);
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
    fn ut_fatal() -> !;
    fn ut_close(fd: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
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
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xpoll {
    pub epoll_fd: libc::c_int,
    pub fd_regs: *mut xpoll_fd_reg,
    pub fd_regs_capacity: libc::c_int,
    pub num_fd_regs: libc::c_int,
    pub active_fd: libc::c_int,
    pub active_fd_reg_id: libc::c_int,
    pub bell_regs: *mut xpoll_bell_reg,
    pub bell_regs_capacity: libc::c_int,
    pub num_bell_regs: libc::c_int,
    pub log_ref: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xpoll_bell_reg {
    pub free: bool,
    pub ringing: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xpoll_fd_reg {
    pub fd: libc::c_int,
    pub event: libc::c_int,
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
#[inline]
unsafe extern "C" fn log_xpoll_fd_event_str(
    mut event: libc::c_int,
) -> *const libc::c_char {
    match event {
        5 => return b"readable and writable\0" as *const u8 as *const libc::c_char,
        1 => return b"readable\0" as *const u8 as *const libc::c_char,
        4 => return b"writable\0" as *const u8 as *const libc::c_char,
        _ => return b"none\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn log_xpoll_ring_str(mut ringing: bool) -> *const libc::c_char {
    return if ringing as libc::c_int != 0 {
        b"ringing\0" as *const u8 as *const libc::c_char
    } else {
        b"idle\0" as *const u8 as *const libc::c_char
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_create(mut log_ref: *mut libc::c_void) -> *mut xpoll {
    let mut epoll_fd: libc::c_int = epoll_create1(0 as libc::c_int);
    if epoll_fd < 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                42 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"xpoll_create\0"))
                    .as_ptr(),
                log_ref as *mut xcm_socket,
                b"Failed to create epoll instance; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut xpoll;
    }
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"xpoll_create\0"))
                .as_ptr(),
            log_ref as *mut xcm_socket,
            b"Created xpoll with epoll fd %d.\0" as *const u8 as *const libc::c_char,
            epoll_fd,
        );
    }
    let mut xpoll: *mut xpoll = ut_malloc(
        ::core::mem::size_of::<xpoll>() as libc::c_ulong,
    ) as *mut xpoll;
    *xpoll = {
        let mut init = xpoll {
            epoll_fd: epoll_fd,
            fd_regs: 0 as *mut xpoll_fd_reg,
            fd_regs_capacity: 0,
            num_fd_regs: 0,
            active_fd: -(1 as libc::c_int),
            active_fd_reg_id: -(1 as libc::c_int),
            bell_regs: 0 as *mut xpoll_bell_reg,
            bell_regs_capacity: 0,
            num_bell_regs: 0,
            log_ref: log_ref,
        };
        init
    };
    return xpoll;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_destroy(mut xpoll: *mut xpoll) {
    if !xpoll.is_null() {
        ut_close((*xpoll).epoll_fd);
        if (*xpoll).active_fd >= 0 as libc::c_int {
            active_fd_put((*xpoll).active_fd);
        }
        ut_free((*xpoll).fd_regs as *mut libc::c_void);
        ut_free((*xpoll).bell_regs as *mut libc::c_void);
        ut_free(xpoll as *mut libc::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_get_fd(mut xpoll: *mut xpoll) -> libc::c_int {
    return (*xpoll).epoll_fd;
}
unsafe extern "C" fn regs_extend_capacity(
    mut xpoll: *mut xpoll,
    mut new_capacity: libc::c_int,
) {
    (*xpoll)
        .fd_regs = ut_realloc(
        (*xpoll).fd_regs as *mut libc::c_void,
        (new_capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xpoll_fd_reg>() as libc::c_ulong),
    ) as *mut xpoll_fd_reg;
    let mut i: libc::c_int = 0;
    i = (*xpoll).fd_regs_capacity;
    while i < new_capacity {
        (*((*xpoll).fd_regs).offset(i as isize)).fd = -(1 as libc::c_int);
        i += 1;
        i;
    }
    (*xpoll).fd_regs_capacity = new_capacity;
}
unsafe extern "C" fn find_fd(mut xpoll: *mut xpoll, mut fd: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*xpoll).fd_regs_capacity {
        let mut reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs).offset(i as isize)
            as *mut xpoll_fd_reg;
        if (*reg).fd == fd {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn has_fd(mut xpoll: *mut xpoll, mut fd: libc::c_int) -> bool {
    return find_fd(xpoll, fd) >= 0 as libc::c_int;
}
unsafe extern "C" fn find_free_fd_reg_idx(mut xpoll: *mut xpoll) -> libc::c_int {
    return find_fd(xpoll, -(1 as libc::c_int));
}
unsafe extern "C" fn next_capacity(mut current_capacity: libc::c_int) -> libc::c_int {
    return (current_capacity + 1 as libc::c_int) * 2 as libc::c_int;
}
unsafe extern "C" fn allocate_fd_reg_idx(mut xpoll: *mut xpoll) -> libc::c_int {
    let mut new_reg_idx: libc::c_int = 0;
    if (*xpoll).num_fd_regs == (*xpoll).fd_regs_capacity {
        new_reg_idx = (*xpoll).fd_regs_capacity;
        regs_extend_capacity(xpoll, next_capacity((*xpoll).fd_regs_capacity));
    } else {
        new_reg_idx = find_free_fd_reg_idx(xpoll);
    }
    (*xpoll).num_fd_regs += 1;
    (*xpoll).num_fd_regs;
    return new_reg_idx;
}
unsafe extern "C" fn deallocate_fd_reg_idx(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
) {
    let mut reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs).offset(reg_idx as isize)
        as *mut xpoll_fd_reg;
    (*reg).fd = -(1 as libc::c_int);
    (*xpoll).num_fd_regs -= 1;
    (*xpoll).num_fd_regs;
}
unsafe extern "C" fn reg_epoll_mod(
    mut xpoll: *mut xpoll,
    mut reg: *mut xpoll_fd_reg,
    mut new_event: libc::c_int,
) {
    if (*reg).event == new_event {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                146 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"reg_epoll_mod\0"))
                    .as_ptr(),
                (*xpoll).log_ref as *mut xcm_socket,
                b"fd %d already have event type %s registered in epoll instance %d.\0"
                    as *const u8 as *const libc::c_char,
                (*reg).fd,
                log_xpoll_fd_event_str(new_event),
                (*xpoll).epoll_fd,
            );
        }
        return;
    }
    if (*reg).event == 0 as libc::c_int && new_event != 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                152 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"reg_epoll_mod\0"))
                    .as_ptr(),
                (*xpoll).log_ref as *mut xcm_socket,
                b"Adding fd %d with event type %s to epoll fd %d.\0" as *const u8
                    as *const libc::c_char,
                (*reg).fd,
                log_xpoll_fd_event_str(new_event),
                (*xpoll).epoll_fd,
            );
        }
        let mut nevent: epoll_event = {
            let mut init = epoll_event {
                events: new_event as uint32_t,
                data: epoll_data {
                    ptr: 0 as *mut libc::c_void,
                },
            };
            init
        };
        let mut rc: libc::c_int = epoll_ctl(
            (*xpoll).epoll_fd,
            1 as libc::c_int,
            (*reg).fd,
            &mut nevent,
        );
        if !(rc == 0 as libc::c_int) {
            log_console_conf(1 as libc::c_int != 0);
            if log_is_enabled(log_type_error) {
                __log_event(
                    log_type_error,
                    b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                        as *const u8 as *const libc::c_char,
                    159 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"reg_epoll_mod\0"))
                        .as_ptr(),
                    0 as *mut xcm_socket,
                    b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                    b"rc == 0\0" as *const u8 as *const libc::c_char,
                );
            }
            abort();
        }
    } else if (*reg).event != 0 as libc::c_int && new_event != 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                163 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"reg_epoll_mod\0"))
                    .as_ptr(),
                (*xpoll).log_ref as *mut xcm_socket,
                b"Modifying fd %d with event type %s to epoll fd %d.\0" as *const u8
                    as *const libc::c_char,
                (*reg).fd,
                log_xpoll_fd_event_str(new_event),
                (*xpoll).epoll_fd,
            );
        }
        let mut nevent_0: epoll_event = {
            let mut init = epoll_event {
                events: new_event as uint32_t,
                data: epoll_data {
                    ptr: 0 as *mut libc::c_void,
                },
            };
            init
        };
        if epoll_ctl((*xpoll).epoll_fd, 3 as libc::c_int, (*reg).fd, &mut nevent_0)
            < 0 as libc::c_int
        {
            if log_is_enabled(log_type_error) {
                __log_event(
                    log_type_error,
                    b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                        as *const u8 as *const libc::c_char,
                    171 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"reg_epoll_mod\0"))
                        .as_ptr(),
                    (*xpoll).log_ref as *mut xcm_socket,
                    b"Failed to modify fd %d in epoll instance %d; errno %d (%s).\0"
                        as *const u8 as *const libc::c_char,
                    (*reg).fd,
                    (*xpoll).epoll_fd,
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
            ut_fatal();
        }
    } else {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                175 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"reg_epoll_mod\0"))
                    .as_ptr(),
                (*xpoll).log_ref as *mut xcm_socket,
                b"Deleting fd %d from epoll fd %d.\0" as *const u8
                    as *const libc::c_char,
                (*reg).fd,
                (*xpoll).epoll_fd,
            );
        }
        let mut _oerrno: libc::c_int = *__errno_location();
        let mut rc_0: libc::c_int = epoll_ctl(
            (*xpoll).epoll_fd,
            2 as libc::c_int,
            (*reg).fd,
            0 as *mut epoll_event,
        );
        let mut epoll_errno: libc::c_int = *__errno_location();
        *__errno_location() = _oerrno;
        if rc_0 < 0 as libc::c_int
            && (epoll_errno != 9 as libc::c_int && epoll_errno != 2 as libc::c_int
                && epoll_errno != 1 as libc::c_int)
        {
            if log_is_enabled(log_type_error) {
                __log_event(
                    log_type_error,
                    b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                        as *const u8 as *const libc::c_char,
                    187 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"reg_epoll_mod\0"))
                        .as_ptr(),
                    (*xpoll).log_ref as *mut xcm_socket,
                    b"Failed to delete fd %d from epoll instance %d; errno %d (%s).\0"
                        as *const u8 as *const libc::c_char,
                    (*reg).fd,
                    (*xpoll).epoll_fd,
                    epoll_errno,
                    strerror(epoll_errno),
                );
            }
            ut_fatal();
        }
    }
    (*reg).event = new_event;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_add(
    mut xpoll: *mut xpoll,
    mut fd: libc::c_int,
    mut event: libc::c_int,
) -> libc::c_int {
    if !(fd >= 0 as libc::c_int) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                197 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"xpoll_fd_reg_add\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"fd >= 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    if has_fd(xpoll, fd) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                198 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"xpoll_fd_reg_add\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"!has_fd(xpoll, fd)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    let mut new_reg_idx: libc::c_int = allocate_fd_reg_idx(xpoll);
    let mut new_reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs)
        .offset(new_reg_idx as isize) as *mut xpoll_fd_reg;
    *new_reg = {
        let mut init = xpoll_fd_reg {
            fd: fd,
            event: 0 as libc::c_int,
        };
        init
    };
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"xpoll_fd_reg_add\0"))
                .as_ptr(),
            (*xpoll).log_ref as *mut xcm_socket,
            b"Registering fd %d with event type %s as registration %d in xpoll instance for epoll fd %d\0"
                as *const u8 as *const libc::c_char,
            fd,
            log_xpoll_fd_event_str(event),
            new_reg_idx,
            (*xpoll).epoll_fd,
        );
    }
    reg_epoll_mod(xpoll, new_reg, event);
    return new_reg_idx;
}
unsafe extern "C" fn get_fd_reg(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
) -> *mut xpoll_fd_reg {
    if !(reg_idx >= 0 as libc::c_int && reg_idx < (*xpoll).fd_regs_capacity) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                218 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"get_fd_reg\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg_idx >= 0 && reg_idx < xpoll->fd_regs_capacity\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs).offset(reg_idx as isize)
        as *mut xpoll_fd_reg;
    if !((*reg).fd >= 0 as libc::c_int) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                222 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"get_fd_reg\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg->fd >= 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    return reg;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_mod(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
    mut new_event: libc::c_int,
) {
    let mut reg: *mut xpoll_fd_reg = get_fd_reg(xpoll, reg_idx);
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"xpoll_fd_reg_mod\0"))
                .as_ptr(),
            (*xpoll).log_ref as *mut xcm_socket,
            b"Modifying registration %d fd %d from event type %s to event type %s in xpoll instance for epoll fd %d\0"
                as *const u8 as *const libc::c_char,
            reg_idx,
            (*reg).fd,
            log_xpoll_fd_event_str((*reg).event),
            log_xpoll_fd_event_str(new_event),
            (*xpoll).epoll_fd,
        );
    }
    reg_epoll_mod(xpoll, reg, new_event);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_del(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
) {
    let mut reg: *mut xpoll_fd_reg = get_fd_reg(xpoll, reg_idx);
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"xpoll_fd_reg_del\0"))
                .as_ptr(),
            (*xpoll).log_ref as *mut xcm_socket,
            b"Removing registration with id %d (for fd %d) in xpoll instance for epoll fd %d\0"
                as *const u8 as *const libc::c_char,
            reg_idx,
            (*reg).fd,
            (*xpoll).epoll_fd,
        );
    }
    reg_epoll_mod(xpoll, reg, 0 as libc::c_int);
    deallocate_fd_reg_idx(xpoll, reg_idx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_del_if_valid(
    mut xpoll: *mut xpoll,
    mut reg_id: libc::c_int,
) {
    if reg_id >= 0 as libc::c_int {
        xpoll_fd_reg_del(xpoll, reg_id);
    }
}
unsafe extern "C" fn bell_regs_extend_capacity(
    mut xpoll: *mut xpoll,
    mut new_capacity: libc::c_int,
) {
    (*xpoll)
        .bell_regs = ut_realloc(
        (*xpoll).bell_regs as *mut libc::c_void,
        (new_capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xpoll_bell_reg>() as libc::c_ulong),
    ) as *mut xpoll_bell_reg;
    let mut i: libc::c_int = 0;
    i = (*xpoll).bell_regs_capacity;
    while i < new_capacity {
        (*((*xpoll).bell_regs).offset(i as isize)).free = 1 as libc::c_int != 0;
        i += 1;
        i;
    }
    (*xpoll).bell_regs_capacity = new_capacity;
}
unsafe extern "C" fn find_free_bell_reg_idx(mut xpoll: *mut xpoll) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*xpoll).bell_regs_capacity {
        let mut reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs).offset(i as isize)
            as *mut xpoll_bell_reg;
        if (*reg).free {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn allocate_bell_reg_idx(mut xpoll: *mut xpoll) -> libc::c_int {
    let mut new_reg_idx: libc::c_int = 0;
    if (*xpoll).num_bell_regs == (*xpoll).bell_regs_capacity {
        new_reg_idx = (*xpoll).bell_regs_capacity;
        bell_regs_extend_capacity(xpoll, next_capacity((*xpoll).bell_regs_capacity));
    } else {
        new_reg_idx = find_free_bell_reg_idx(xpoll);
    }
    let mut reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(new_reg_idx as isize) as *mut xpoll_bell_reg;
    (*reg).free = 0 as libc::c_int != 0;
    (*xpoll).num_bell_regs += 1;
    (*xpoll).num_bell_regs;
    return new_reg_idx;
}
unsafe extern "C" fn deallocate_bell_reg_idx(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
) {
    let mut reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(reg_idx as isize) as *mut xpoll_bell_reg;
    (*reg).free = 1 as libc::c_int != 0;
    (*xpoll).num_bell_regs -= 1;
    (*xpoll).num_bell_regs;
}
unsafe extern "C" fn has_ringing_bell(mut xpoll: *mut xpoll) -> bool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*xpoll).bell_regs_capacity {
        let mut reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs).offset(i as isize)
            as *mut xpoll_bell_reg;
        if !(*reg).free && (*reg).ringing as libc::c_int != 0 {
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn update_active_fd(mut xpoll: *mut xpoll) {
    if (*xpoll).num_bell_regs == 0 as libc::c_int
        && (*xpoll).active_fd >= 0 as libc::c_int
    {
        xpoll_fd_reg_del(xpoll, (*xpoll).active_fd_reg_id);
        (*xpoll).active_fd_reg_id = -(1 as libc::c_int);
        active_fd_put((*xpoll).active_fd);
        (*xpoll).active_fd = -(1 as libc::c_int);
    } else if (*xpoll).num_bell_regs > 0 as libc::c_int
        && (*xpoll).active_fd < 0 as libc::c_int
    {
        (*xpoll).active_fd = active_fd_get();
        (*xpoll)
            .active_fd_reg_id = xpoll_fd_reg_add(
            xpoll,
            (*xpoll).active_fd,
            0 as libc::c_int,
        );
    }
    if (*xpoll).active_fd >= 0 as libc::c_int {
        let mut event: libc::c_int = if has_ringing_bell(xpoll) as libc::c_int != 0 {
            EPOLLIN as libc::c_int
        } else {
            0 as libc::c_int
        };
        xpoll_fd_reg_mod(xpoll, (*xpoll).active_fd_reg_id, event);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_add(
    mut xpoll: *mut xpoll,
    mut ringing: bool,
) -> libc::c_int {
    let mut new_reg_idx: libc::c_int = allocate_bell_reg_idx(xpoll);
    let mut new_reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(new_reg_idx as isize) as *mut xpoll_bell_reg;
    (*new_reg).ringing = ringing;
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"xpoll_bell_reg_add\0"))
                .as_ptr(),
            (*xpoll).log_ref as *mut xcm_socket,
            b"Registering bell with state %s as registration %d in xpoll instance for epoll fd %d\0"
                as *const u8 as *const libc::c_char,
            log_xpoll_ring_str(ringing),
            new_reg_idx,
            (*xpoll).epoll_fd,
        );
    }
    update_active_fd(xpoll);
    return new_reg_idx;
}
unsafe extern "C" fn get_bell_reg(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
) -> *mut xpoll_bell_reg {
    if !(reg_idx >= 0 as libc::c_int && reg_idx < (*xpoll).bell_regs_capacity) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                358 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"get_bell_reg\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg_idx >= 0 && reg_idx < xpoll->bell_regs_capacity\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let mut reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(reg_idx as isize) as *mut xpoll_bell_reg;
    if (*reg).free {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                362 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"get_bell_reg\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"!reg->free\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    return reg;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_mod(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
    mut ringing: bool,
) {
    let mut reg: *mut xpoll_bell_reg = get_bell_reg(xpoll, reg_idx);
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            373 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"xpoll_bell_reg_mod\0"))
                .as_ptr(),
            (*xpoll).log_ref as *mut xcm_socket,
            b"Modifying bell registration %d from %s to %s in xpoll instance for epoll fd %d\0"
                as *const u8 as *const libc::c_char,
            reg_idx,
            log_xpoll_ring_str((*reg).ringing),
            log_xpoll_ring_str(ringing),
            (*xpoll).epoll_fd,
        );
    }
    if (*reg).ringing as libc::c_int != ringing as libc::c_int {
        (*reg).ringing = ringing;
        update_active_fd(xpoll);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_del(
    mut xpoll: *mut xpoll,
    mut reg_idx: libc::c_int,
) {
    let mut reg: *mut xpoll_bell_reg = get_bell_reg(xpoll, reg_idx);
    if reg.is_null() {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
            __log_event(
                log_type_error,
                b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                    as *const u8 as *const libc::c_char,
                385 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"xpoll_bell_reg_del\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg != ((void*)0)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/lysarina/skool/exjobb/xcm-translation/libxcm/core/xpoll.c\0"
                as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"xpoll_bell_reg_del\0"))
                .as_ptr(),
            (*xpoll).log_ref as *mut xcm_socket,
            b"Removing bell registration %d in xpoll instance for epoll fd %d\0"
                as *const u8 as *const libc::c_char,
            reg_idx,
            (*xpoll).epoll_fd,
        );
    }
    deallocate_bell_reg_idx(xpoll, reg_idx);
    update_active_fd(xpoll);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_del_if_valid(
    mut xpoll: *mut xpoll,
    mut reg_id: libc::c_int,
) {
    if reg_id >= 0 as libc::c_int {
        xpoll_bell_reg_del(xpoll, reg_id);
    }
}
