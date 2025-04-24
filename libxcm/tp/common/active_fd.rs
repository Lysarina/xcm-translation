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
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ut_mutex_lock(m: *mut pthread_mutex_t);
    fn ut_mutex_unlock(m: *mut pthread_mutex_t);
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
    fn ut_close(fd: libc::c_int);
    fn eventfd(__count: libc::c_uint, __flags: libc::c_int) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct active_fd {
    pub fd: libc::c_int,
    pub cnt: libc::c_int,
    pub elem: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub le_next: *mut active_fd,
    pub le_prev: *mut *mut active_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct active_fd_list {
    pub lh_first: *mut active_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type size_t = libc::c_ulong;
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
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
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
pub const EFD_NONBLOCK: C2RustUnnamed_1 = 2048;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const EFD_CLOEXEC: C2RustUnnamed_1 = 524288;
pub const EFD_SEMAPHORE: C2RustUnnamed_1 = 1;
static mut active_fd_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut active_fds: active_fd_list = {
    let mut init = active_fd_list {
        lh_first: 0 as *const active_fd as *mut active_fd,
    };
    init
};
unsafe extern "C" fn fd_retrieve() -> *mut active_fd {
    let mut active_fd: *mut active_fd = 0 as *mut active_fd;
    active_fd = active_fds.lh_first;
    while !active_fd.is_null() {
        if (*active_fd).cnt < 100 as libc::c_int {
            (*active_fd).cnt += 1;
            (*active_fd).cnt;
            return active_fd;
        }
        active_fd = (*active_fd).elem.le_next;
    }
    return 0 as *mut active_fd;
}
unsafe extern "C" fn fd_create() -> *mut active_fd {
    let mut fd: libc::c_int = eventfd(
        1 as libc::c_int as libc::c_uint,
        EFD_NONBLOCK as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        if log_is_enabled(log_type_debug) {
            __log_event(
                log_type_debug,
                b"/home/ehhjmou/xcm-translation/libxcm/tp/common/active_fd.c\0"
                    as *const u8 as *const libc::c_char,
                49 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"fd_create\0"))
                    .as_ptr(),
                0 as *mut xcm_socket,
                b"Failed to create always-active event fd; errno %d (%s).\0" as *const u8
                    as *const libc::c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut active_fd;
    }
    let mut active_fd: *mut active_fd = ut_malloc(
        ::core::mem::size_of::<active_fd>() as libc::c_ulong,
    ) as *mut active_fd;
    *active_fd = {
        let mut init = active_fd {
            fd: fd,
            cnt: 1 as libc::c_int,
            elem: C2RustUnnamed {
                le_next: 0 as *mut active_fd,
                le_prev: 0 as *mut *mut active_fd,
            },
        };
        init
    };
    (*active_fd).elem.le_next = active_fds.lh_first;
    if !((*active_fd).elem.le_next).is_null() {
        (*active_fds.lh_first).elem.le_prev = &mut (*active_fd).elem.le_next;
    }
    active_fds.lh_first = active_fd;
    (*active_fd).elem.le_prev = &mut active_fds.lh_first;
    if log_is_enabled(log_type_debug) {
        __log_event(
            log_type_debug,
            b"/home/ehhjmou/xcm-translation/libxcm/tp/common/active_fd.c\0" as *const u8
                as *const libc::c_char,
            62 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"fd_create\0"))
                .as_ptr(),
            0 as *mut xcm_socket,
            b"%s always-active event fd %d.\0" as *const u8 as *const libc::c_char,
            b"Created\0" as *const u8 as *const libc::c_char,
            (*active_fd).fd,
        );
    }
    return active_fd;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn active_fd_get() -> libc::c_int {
    ut_mutex_lock(&mut active_fd_lock);
    let mut active_fd: *mut active_fd = fd_retrieve();
    if active_fd.is_null() {
        active_fd = fd_create();
    }
    ut_mutex_unlock(&mut active_fd_lock);
    return if !active_fd.is_null() { (*active_fd).fd } else { -(1 as libc::c_int) };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn active_fd_put(mut fd: libc::c_int) {
    let mut current_block: u64;
    ut_mutex_lock(&mut active_fd_lock);
    let mut active_fd: *mut active_fd = 0 as *mut active_fd;
    active_fd = active_fds.lh_first;
    loop {
        if active_fd.is_null() {
            current_block = 17407779659766490442;
            break;
        }
        if (*active_fd).fd == fd {
            (*active_fd).cnt -= 1;
            (*active_fd).cnt;
            if (*active_fd).cnt == 0 as libc::c_int {
                if !((*active_fd).elem.le_next).is_null() {
                    (*(*active_fd).elem.le_next)
                        .elem
                        .le_prev = (*active_fd).elem.le_prev;
                }
                *(*active_fd).elem.le_prev = (*active_fd).elem.le_next;
                ut_close((*active_fd).fd);
                if log_is_enabled(log_type_debug) {
                    __log_event(
                        log_type_debug,
                        b"/home/ehhjmou/xcm-translation/libxcm/tp/common/active_fd.c\0"
                            as *const u8 as *const libc::c_char,
                        96 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 14],
                            &[libc::c_char; 14],
                        >(b"active_fd_put\0"))
                            .as_ptr(),
                        0 as *mut xcm_socket,
                        b"%s always-active event fd %d.\0" as *const u8
                            as *const libc::c_char,
                        b"Closed\0" as *const u8 as *const libc::c_char,
                        (*active_fd).fd,
                    );
                }
                ut_free(active_fd as *mut libc::c_void);
            }
            current_block = 11951460078704378413;
            break;
        } else {
            active_fd = (*active_fd).elem.le_next;
        }
    }
    match current_block {
        17407779659766490442 => {
            if 0 as libc::c_int == 0 {
                log_console_conf(1 as libc::c_int != 0);
                if log_is_enabled(log_type_error) {
                    __log_event(
                        log_type_error,
                        b"/home/ehhjmou/xcm-translation/libxcm/tp/common/active_fd.c\0"
                            as *const u8 as *const libc::c_char,
                        103 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 14],
                            &[libc::c_char; 14],
                        >(b"active_fd_put\0"))
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
        _ => {}
    }
    ut_mutex_unlock(&mut active_fd_lock);
}
