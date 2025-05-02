#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

use libc::{abort, strerror, epoll_event, EPOLLIN, epoll_create1, epoll_ctl, __errno_location};

use xcm_rust_common::xcm_tp::xcm_socket;
use rs_active_fd::{active_fd_get, active_fd_put};
use rs_util::{ut_malloc, ut_realloc, ut_free, ut_fatal, ut_close};
use rs_log::{__log_event, log_is_enabled, log_console_conf, log_type_debug, log_type_error};

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
#[inline]
unsafe extern "C" fn log_xpoll_fd_event_str(
    event: libc::c_int,
) -> *const libc::c_char {
    match event {
        5 => b"readable and writable\0" as *const u8 as *const libc::c_char,
        1 => b"readable\0" as *const u8 as *const libc::c_char,
        4 => b"writable\0" as *const u8 as *const libc::c_char,
        _ => b"none\0" as *const u8 as *const libc::c_char,
    }
}
unsafe extern "C" fn log_xpoll_ring_str(ringing: bool) -> *const libc::c_char {
    if ringing {
        b"ringing\0" as *const u8 as *const libc::c_char
    } else {
        b"idle\0" as *const u8 as *const libc::c_char
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_create(log_ref: *mut libc::c_void) -> *mut xpoll { unsafe {
    let epoll_fd: libc::c_int = epoll_create1(0);
    if epoll_fd < 0 {
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
        return std::ptr::null_mut::<xpoll>();
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
    let xpoll: *mut xpoll = ut_malloc(
        ::core::mem::size_of::<xpoll>() as libc::c_ulong,
    ) as *mut xpoll;
    *xpoll = {
        
        xpoll {
            epoll_fd,
            fd_regs: std::ptr::null_mut::<xpoll_fd_reg>(),
            fd_regs_capacity: 0,
            num_fd_regs: 0,
            active_fd: -1,
            active_fd_reg_id: -1,
            bell_regs: std::ptr::null_mut::<xpoll_bell_reg>(),
            bell_regs_capacity: 0,
            num_bell_regs: 0,
            log_ref,
        }
    };
    xpoll
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_destroy(xpoll: *mut xpoll) { unsafe {
    if !xpoll.is_null() {
        ut_close((*xpoll).epoll_fd);
        if (*xpoll).active_fd >= 0 {
            active_fd_put((*xpoll).active_fd);
        }
        ut_free((*xpoll).fd_regs as *mut libc::c_void);
        ut_free((*xpoll).bell_regs as *mut libc::c_void);
        ut_free(xpoll as *mut libc::c_void);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_get_fd(xpoll: *mut xpoll) -> libc::c_int { unsafe {
    (*xpoll).epoll_fd
}}
unsafe extern "C" fn regs_extend_capacity(
    xpoll: *mut xpoll,
    new_capacity: libc::c_int,
) { unsafe {
    (*xpoll)
        .fd_regs = ut_realloc(
        (*xpoll).fd_regs as *mut libc::c_void,
        (new_capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xpoll_fd_reg>() as libc::c_ulong),
    ) as *mut xpoll_fd_reg;
    let mut i: libc::c_int = (*xpoll).fd_regs_capacity;
    while i < new_capacity {
        (*((*xpoll).fd_regs).offset(i as isize)).fd = -1;
        i += 1;
    }
    (*xpoll).fd_regs_capacity = new_capacity;
}}
unsafe extern "C" fn find_fd(xpoll: *mut xpoll, fd: libc::c_int) -> libc::c_int { unsafe {
    let mut i: libc::c_int = 0;
    while i < (*xpoll).fd_regs_capacity {
        let reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs).offset(i as isize)
            as *mut xpoll_fd_reg;
        if (*reg).fd == fd {
            return i;
        }
        i += 1;
    }
    -1
}}
unsafe extern "C" fn has_fd(xpoll: *mut xpoll, fd: libc::c_int) -> bool { unsafe {
    find_fd(xpoll, fd) >= 0
}}
unsafe extern "C" fn find_free_fd_reg_idx(xpoll: *mut xpoll) -> libc::c_int { unsafe {
    find_fd(xpoll, -1)
}}
unsafe extern "C" fn next_capacity(current_capacity: libc::c_int) -> libc::c_int {
    (current_capacity + 1) * 2
}
unsafe extern "C" fn allocate_fd_reg_idx(xpoll: *mut xpoll) -> libc::c_int { unsafe {
    let new_reg_idx: libc::c_int;
    if (*xpoll).num_fd_regs == (*xpoll).fd_regs_capacity {
        new_reg_idx = (*xpoll).fd_regs_capacity;
        regs_extend_capacity(xpoll, next_capacity((*xpoll).fd_regs_capacity));
    } else {
        new_reg_idx = find_free_fd_reg_idx(xpoll);
    }
    (*xpoll).num_fd_regs += 1;
    new_reg_idx
}}
unsafe extern "C" fn deallocate_fd_reg_idx(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
) { unsafe {
    let reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs).offset(reg_idx as isize)
        as *mut xpoll_fd_reg;
    (*reg).fd = -1;
    (*xpoll).num_fd_regs -= 1;
}}
unsafe extern "C" fn reg_epoll_mod(
    xpoll: *mut xpoll,
    reg: *mut xpoll_fd_reg,
    new_event: libc::c_int,
) { unsafe {
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
    if (*reg).event == 0 && new_event != 0 {
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
            
            epoll_event {
                events: new_event as libc::c_uint,
                u64: 0
            }
        };
        let rc: libc::c_int = epoll_ctl(
            (*xpoll).epoll_fd,
            1,
            (*reg).fd,
            &mut nevent,
        );
        if rc != 0 {
            log_console_conf(true);
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
                    std::ptr::null_mut::<xcm_socket>(),
                    b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                    b"rc == 0\0" as *const u8 as *const libc::c_char,
                );
            }
            abort();
        }
    } else if (*reg).event != 0 && new_event != 0 {
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
            
            epoll_event {
                events: new_event as libc::c_uint,
                u64: 0
            }
        };
        if epoll_ctl((*xpoll).epoll_fd, 3, (*reg).fd, &mut nevent_0)
            < 0
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
        let rc_0: libc::c_int = epoll_ctl(
            (*xpoll).epoll_fd,
            2,
            (*reg).fd,
            std::ptr::null_mut::<epoll_event>(),
        );
        let epoll_errno: libc::c_int = *__errno_location();
        *__errno_location() = _oerrno;
        if rc_0 < 0
            && (epoll_errno != 9 && epoll_errno != 2 
                && epoll_errno != 1)
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_add(
    xpoll: *mut xpoll,
    fd: libc::c_int,
    event: libc::c_int,
) -> libc::c_int { unsafe {
    if fd < 0 {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"fd >= 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    if has_fd(xpoll, fd) {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"!has_fd(xpoll, fd)\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    let new_reg_idx: libc::c_int = allocate_fd_reg_idx(xpoll);
    let new_reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs)
        .offset(new_reg_idx as isize) as *mut xpoll_fd_reg;
    *new_reg = {
        
        xpoll_fd_reg {
            fd,
            event: 0,
        }
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
    new_reg_idx
}}
unsafe extern "C" fn get_fd_reg(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
) -> *mut xpoll_fd_reg { unsafe {
    if !(reg_idx >= 0 && reg_idx < (*xpoll).fd_regs_capacity) {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg_idx >= 0 && reg_idx < xpoll->fd_regs_capacity\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let reg: *mut xpoll_fd_reg = &mut *((*xpoll).fd_regs).offset(reg_idx as isize)
        as *mut xpoll_fd_reg;
    if (*reg).fd < 0 {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg->fd >= 0\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    reg
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_mod(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
    new_event: libc::c_int,
) { unsafe {
    let reg: *mut xpoll_fd_reg = get_fd_reg(xpoll, reg_idx);
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_del(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
) { unsafe {
    let reg: *mut xpoll_fd_reg = get_fd_reg(xpoll, reg_idx);
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
    reg_epoll_mod(xpoll, reg, 0);
    deallocate_fd_reg_idx(xpoll, reg_idx);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_fd_reg_del_if_valid(
    xpoll: *mut xpoll,
    reg_id: libc::c_int,
) { unsafe {
    if reg_id >= 0 {
        xpoll_fd_reg_del(xpoll, reg_id);
    }
}}
unsafe extern "C" fn bell_regs_extend_capacity(
    xpoll: *mut xpoll,
    new_capacity: libc::c_int,
) { unsafe {
    (*xpoll)
        .bell_regs = ut_realloc(
        (*xpoll).bell_regs as *mut libc::c_void,
        (new_capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xpoll_bell_reg>() as libc::c_ulong),
    ) as *mut xpoll_bell_reg;
    let mut i: libc::c_int = (*xpoll).bell_regs_capacity;
    while i < new_capacity {
        (*((*xpoll).bell_regs).offset(i as isize)).free = true;
        i += 1;
    }
    (*xpoll).bell_regs_capacity = new_capacity;
}}
unsafe extern "C" fn find_free_bell_reg_idx(xpoll: *mut xpoll) -> libc::c_int { unsafe {
    let mut i: libc::c_int = 0;
    while i < (*xpoll).bell_regs_capacity {
        let reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs).offset(i as isize)
            as *mut xpoll_bell_reg;
        if (*reg).free {
            return i;
        }
        i += 1;
    }
    -1
}}
unsafe extern "C" fn allocate_bell_reg_idx(xpoll: *mut xpoll) -> libc::c_int { unsafe {
    let new_reg_idx: libc::c_int;
    if (*xpoll).num_bell_regs == (*xpoll).bell_regs_capacity {
        new_reg_idx = (*xpoll).bell_regs_capacity;
        bell_regs_extend_capacity(xpoll, next_capacity((*xpoll).bell_regs_capacity));
    } else {
        new_reg_idx = find_free_bell_reg_idx(xpoll);
    }
    let reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(new_reg_idx as isize) as *mut xpoll_bell_reg;
    (*reg).free = false;
    (*xpoll).num_bell_regs += 1;
    new_reg_idx
}}
unsafe extern "C" fn deallocate_bell_reg_idx(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
) { unsafe {
    let reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(reg_idx as isize) as *mut xpoll_bell_reg;
    (*reg).free = true;
    (*xpoll).num_bell_regs -= 1;
}}
unsafe extern "C" fn has_ringing_bell(xpoll: *mut xpoll) -> bool { unsafe {
    let mut i: libc::c_int = 0;
    while i < (*xpoll).bell_regs_capacity {
        let reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs).offset(i as isize)
            as *mut xpoll_bell_reg;
        if !(*reg).free && (*reg).ringing {
            return true;
        }
        i += 1;
    }
    false
}}
unsafe extern "C" fn update_active_fd(xpoll: *mut xpoll) { unsafe {
    if (*xpoll).num_bell_regs == 0
        && (*xpoll).active_fd >= 0
    {
        xpoll_fd_reg_del(xpoll, (*xpoll).active_fd_reg_id);
        (*xpoll).active_fd_reg_id = -1;
        active_fd_put((*xpoll).active_fd);
        (*xpoll).active_fd = -1;
    } else if (*xpoll).num_bell_regs > 0
        && (*xpoll).active_fd < 0
    {
        (*xpoll).active_fd = active_fd_get();
        (*xpoll)
            .active_fd_reg_id = xpoll_fd_reg_add(
            xpoll,
            (*xpoll).active_fd,
            0,
        );
    }
    if (*xpoll).active_fd >= 0 {
        let event: libc::c_int = if has_ringing_bell(xpoll) {
            EPOLLIN as libc::c_int
        } else {
            0
        };
        xpoll_fd_reg_mod(xpoll, (*xpoll).active_fd_reg_id, event);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_add(
    xpoll: *mut xpoll,
    ringing: bool,
) -> libc::c_int { unsafe {
    let new_reg_idx: libc::c_int = allocate_bell_reg_idx(xpoll);
    let new_reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
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
    new_reg_idx
}}
unsafe extern "C" fn get_bell_reg(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
) -> *mut xpoll_bell_reg { unsafe {
    if !(reg_idx >= 0 && reg_idx < (*xpoll).bell_regs_capacity) {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"reg_idx >= 0 && reg_idx < xpoll->bell_regs_capacity\0" as *const u8
                    as *const libc::c_char,
            );
        }
        abort();
    }
    let reg: *mut xpoll_bell_reg = &mut *((*xpoll).bell_regs)
        .offset(reg_idx as isize) as *mut xpoll_bell_reg;
    if (*reg).free {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
                b"Assertion \"%s\" failed.\n\0" as *const u8 as *const libc::c_char,
                b"!reg->free\0" as *const u8 as *const libc::c_char,
            );
        }
        abort();
    }
    reg
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_mod(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
    ringing: bool,
) { unsafe {
    let reg: *mut xpoll_bell_reg = get_bell_reg(xpoll, reg_idx);
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
    if (*reg).ringing != ringing {
        (*reg).ringing = ringing;
        update_active_fd(xpoll);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_del(
    xpoll: *mut xpoll,
    reg_idx: libc::c_int,
) { unsafe {
    let reg: *mut xpoll_bell_reg = get_bell_reg(xpoll, reg_idx);
    if reg.is_null() {
        log_console_conf(true);
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
                std::ptr::null_mut::<xcm_socket>(),
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
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xpoll_bell_reg_del_if_valid(
    xpoll: *mut xpoll,
    reg_id: libc::c_int,
) { unsafe {
    if reg_id >= 0 {
        xpoll_bell_reg_del(xpoll, reg_id);
    }
}}
