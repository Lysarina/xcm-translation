#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type event_base;
    pub type xcm_socket;
    fn __errno_location() -> *mut libc::c_int;
    fn event_assign(
        _: *mut event,
        _: *mut event_base,
        _: libc::c_int,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn xcm_close(socket: *mut xcm_socket) -> libc::c_int;
    fn xcm_send(
        conn_socket: *mut xcm_socket,
        buf: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn xcm_receive(
        conn_socket: *mut xcm_socket,
        buf: *mut libc::c_void,
        capacity: size_t,
    ) -> libc::c_int;
    fn xcm_await(socket: *mut xcm_socket, condition: libc::c_int) -> libc::c_int;
    fn xcm_fd(socket: *mut xcm_socket) -> libc::c_int;
    fn xcm_finish(socket: *mut xcm_socket) -> libc::c_int;
    fn xcm_set_blocking(socket: *mut xcm_socket, should_block: bool) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_free(ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xcm_attr_get_int64(
        socket: *mut xcm_socket,
        name: *const libc::c_char,
        value: *mut int64_t,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_4,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub evcb_callback: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
    >,
    pub evcb_selfcb: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
    pub evcb_evfinalize: Option::<
        unsafe extern "C" fn(*mut event, *mut libc::c_void) -> (),
    >,
    pub evcb_cbfinalize: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
pub type event_callback_fn = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
>;
pub type xfwd_err_cb = Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xfwd {
    pub event_base: *mut event_base,
    pub err_cb: xfwd_err_cb,
    pub err_cb_data: *mut libc::c_void,
    pub src_conn: *mut xcm_socket,
    pub dst_conn: *mut xcm_socket,
    pub src_condition: *mut libc::c_int,
    pub dst_condition: *mut libc::c_int,
    pub src_event: event,
    pub dst_event: event,
    pub data: *mut libc::c_char,
    pub data_capacity: libc::c_int,
    pub data_len: libc::c_int,
    pub running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xrelay {
    pub err_cb: xrelay_err_cb,
    pub err_cb_data: *mut libc::c_void,
    pub fwd0: xfwd,
    pub fwd1: xfwd,
    pub cond0: libc::c_int,
    pub cond1: libc::c_int,
    pub running: bool,
    pub entry: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub le_next: *mut xrelay,
    pub le_prev: *mut *mut xrelay,
}
pub type xrelay_err_cb = Option::<
    unsafe extern "C" fn(
        *mut xrelay,
        libc::c_int,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> (),
>;
unsafe extern "C" fn get_required_capacity(
    mut src_conn: *mut xcm_socket,
    mut dst_conn: *mut xcm_socket,
) -> libc::c_int {
    let mut src_max_msg: int64_t = 0;
    let mut src_rc: libc::c_int = xcm_attr_get_int64(
        src_conn,
        b"xcm.max_msg_size\0" as *const u8 as *const libc::c_char,
        &mut src_max_msg,
    );
    let mut dst_max_msg: int64_t = 0;
    let mut dst_rc: libc::c_int = xcm_attr_get_int64(
        dst_conn,
        b"xcm.max_msg_size\0" as *const u8 as *const libc::c_char,
        &mut dst_max_msg,
    );
    let mut max_msg: libc::c_int = 0;
    if src_rc as libc::c_ulong == ::core::mem::size_of::<int64_t>() as libc::c_ulong
        && dst_rc as libc::c_ulong == ::core::mem::size_of::<int64_t>() as libc::c_ulong
    {
        max_msg = ({
            let mut _a: int64_t = src_max_msg;
            let mut _b: int64_t = dst_max_msg;
            if _a < _b { _a } else { _b }
        }) as libc::c_int;
    } else {
        max_msg = 65535 as libc::c_int;
    }
    return max_msg;
}
unsafe extern "C" fn xfwd_init(
    mut relay: *mut xfwd,
    mut src_conn: *mut xcm_socket,
    mut dst_conn: *mut xcm_socket,
    mut src_condition: *mut libc::c_int,
    mut dst_condition: *mut libc::c_int,
    mut err_cb: xfwd_err_cb,
    mut cb_data: *mut libc::c_void,
    mut event_base: *mut event_base,
) {
    *relay = {
        let mut init = xfwd {
            event_base: event_base,
            err_cb: err_cb,
            err_cb_data: cb_data,
            src_conn: src_conn,
            dst_conn: dst_conn,
            src_condition: src_condition,
            dst_condition: dst_condition,
            src_event: event {
                ev_evcallback: event_callback {
                    evcb_active_next: C2RustUnnamed_7 {
                        tqe_next: 0 as *mut event_callback,
                        tqe_prev: 0 as *mut *mut event_callback,
                    },
                    evcb_flags: 0,
                    evcb_pri: 0,
                    evcb_closure: 0,
                    evcb_cb_union: C2RustUnnamed_6 {
                        evcb_callback: None,
                    },
                    evcb_arg: 0 as *mut libc::c_void,
                },
                ev_timeout_pos: C2RustUnnamed_4 {
                    ev_next_with_common_timeout: C2RustUnnamed_5 {
                        tqe_next: 0 as *mut event,
                        tqe_prev: 0 as *mut *mut event,
                    },
                },
                ev_fd: 0,
                ev_base: 0 as *mut event_base,
                ev_: C2RustUnnamed {
                    ev_io: C2RustUnnamed_2 {
                        ev_io_next: C2RustUnnamed_3 {
                            le_next: 0 as *mut event,
                            le_prev: 0 as *mut *mut event,
                        },
                        ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                    },
                },
                ev_events: 0,
                ev_res: 0,
                ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
            },
            dst_event: event {
                ev_evcallback: event_callback {
                    evcb_active_next: C2RustUnnamed_7 {
                        tqe_next: 0 as *mut event_callback,
                        tqe_prev: 0 as *mut *mut event_callback,
                    },
                    evcb_flags: 0,
                    evcb_pri: 0,
                    evcb_closure: 0,
                    evcb_cb_union: C2RustUnnamed_6 {
                        evcb_callback: None,
                    },
                    evcb_arg: 0 as *mut libc::c_void,
                },
                ev_timeout_pos: C2RustUnnamed_4 {
                    ev_next_with_common_timeout: C2RustUnnamed_5 {
                        tqe_next: 0 as *mut event,
                        tqe_prev: 0 as *mut *mut event,
                    },
                },
                ev_fd: 0,
                ev_base: 0 as *mut event_base,
                ev_: C2RustUnnamed {
                    ev_io: C2RustUnnamed_2 {
                        ev_io_next: C2RustUnnamed_3 {
                            le_next: 0 as *mut event,
                            le_prev: 0 as *mut *mut event,
                        },
                        ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                    },
                },
                ev_events: 0,
                ev_res: 0,
                ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
            },
            data: 0 as *mut libc::c_char,
            data_capacity: 0,
            data_len: 0,
            running: false,
        };
        init
    };
    (*relay).data_capacity = get_required_capacity(src_conn, dst_conn);
    (*relay).data = ut_malloc((*relay).data_capacity as size_t) as *mut libc::c_char;
}
unsafe extern "C" fn xfwd_deinit(mut relay: *mut xfwd) {
    if !relay.is_null() {
        xfwd_stop(relay);
        ut_free((*relay).data as *mut libc::c_void);
    }
}
unsafe extern "C" fn set_condition(
    mut conn: *mut xcm_socket,
    mut condition: libc::c_int,
) {
    let mut rc: libc::c_int = xcm_await(conn, condition);
    if rc == 0 as libc::c_int {} else {
        __assert_fail(
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void set_condition(struct xcm_socket *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5126: {
        if rc == 0 as libc::c_int {} else {
            __assert_fail(
                b"rc == 0\0" as *const u8 as *const libc::c_char,
                b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void set_condition(struct xcm_socket *, int)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn add_condition(
    mut conn: *mut xcm_socket,
    mut condition: *mut libc::c_int,
    mut flag: libc::c_int,
) {
    *condition |= flag;
    set_condition(conn, *condition);
}
unsafe extern "C" fn del_condition(
    mut conn: *mut xcm_socket,
    mut condition: *mut libc::c_int,
    mut flag: libc::c_int,
) {
    *condition &= !flag;
    set_condition(conn, *condition);
}
unsafe extern "C" fn xfwd_handle_err(
    mut relay: *mut xfwd,
    mut msg: *const libc::c_char,
) {
    ((*relay).err_cb)
        .expect(
            "non-null function pointer",
        )(-(1 as libc::c_int), msg, (*relay).err_cb_data);
}
unsafe extern "C" fn xfwd_handle_term(mut relay: *mut xfwd) {
    ((*relay).err_cb)
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, 0 as *const libc::c_char, (*relay).err_cb_data);
}
unsafe extern "C" fn xfwd_await_input(mut relay: *mut xfwd) {
    add_condition(
        (*relay).src_conn,
        (*relay).src_condition,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    del_condition(
        (*relay).dst_conn,
        (*relay).dst_condition,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
}
unsafe extern "C" fn xfwd_await_output(mut relay: *mut xfwd) {
    add_condition(
        (*relay).dst_conn,
        (*relay).dst_condition,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    del_condition(
        (*relay).src_conn,
        (*relay).src_condition,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
}
unsafe extern "C" fn xfwd_send(mut relay: *mut xfwd) {
    let mut rc: libc::c_int = xcm_send(
        (*relay).dst_conn,
        (*relay).data as *const libc::c_void,
        (*relay).data_len as size_t,
    );
    if rc < 0 as libc::c_int {
        if *__errno_location() == 32 as libc::c_int
            || *__errno_location() == 104 as libc::c_int
        {
            xfwd_handle_term(relay);
        } else if *__errno_location() != 11 as libc::c_int {
            xfwd_handle_err(
                relay,
                b"Error sending to XCM\0" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    if rc == 0 as libc::c_int {
        (*relay).data_len = 0 as libc::c_int;
    } else {
        (*relay).data_len -= rc;
    }
    if (*relay).data_len == 0 as libc::c_int {
        xfwd_await_input(relay);
    } else {
        memmove(
            (*relay).data as *mut libc::c_void,
            ((*relay).data).offset(rc as isize) as *const libc::c_void,
            (*relay).data_len as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn xfwd_receive(mut relay: *mut xfwd) {
    let mut rc: libc::c_int = xcm_receive(
        (*relay).src_conn,
        (*relay).data as *mut libc::c_void,
        (*relay).data_capacity as size_t,
    );
    if rc < 0 as libc::c_int {
        if *__errno_location() != 11 as libc::c_int {
            xfwd_handle_err(
                relay,
                b"Error receiving from XCM\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if rc == 0 as libc::c_int {
        xfwd_handle_term(relay);
    } else {
        (*relay).data_len = rc;
        xfwd_await_output(relay);
    };
}
unsafe extern "C" fn xfwd_active(
    mut fd: libc::c_int,
    mut ev: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut relay: *mut xfwd = arg as *mut xfwd;
    if (*relay).running {} else {
        __assert_fail(
            b"relay->running\0" as *const u8 as *const libc::c_char,
            b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void xfwd_active(int, short, void *)\0"))
                .as_ptr(),
        );
    }
    'c_5780: {
        if (*relay).running {} else {
            __assert_fail(
                b"relay->running\0" as *const u8 as *const libc::c_char,
                b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void xfwd_active(int, short, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut awaits_input: bool = (*relay).data_len == 0 as libc::c_int;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if awaits_input {
        if fd == xcm_fd((*relay).src_conn) {
            xfwd_receive(relay);
        } else {
            rc = xcm_finish((*relay).dst_conn);
        }
    } else if fd == xcm_fd((*relay).dst_conn) {
        xfwd_send(relay);
    } else {
        rc = xcm_finish((*relay).src_conn);
    }
    if rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int {
        xfwd_handle_err(relay, 0 as *const libc::c_char);
    }
}
unsafe extern "C" fn xfwd_start(mut relay: *mut xfwd) -> libc::c_int {
    if !(*relay).running {
        if xcm_set_blocking((*relay).src_conn, 0 as libc::c_int != 0) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if xcm_set_blocking((*relay).dst_conn, 0 as libc::c_int != 0) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        let mut src_fd: libc::c_int = xcm_fd((*relay).src_conn);
        if src_fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"src_fd >= 0\0" as *const u8 as *const libc::c_char,
                b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"int xfwd_start(struct xfwd *)\0"))
                    .as_ptr(),
            );
        }
        'c_5920: {
            if src_fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"src_fd >= 0\0" as *const u8 as *const libc::c_char,
                    b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
                    180 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 30],
                        &[libc::c_char; 30],
                    >(b"int xfwd_start(struct xfwd *)\0"))
                        .as_ptr(),
                );
            }
        };
        event_assign(
            &mut (*relay).src_event,
            (*relay).event_base,
            src_fd,
            (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
            Some(
                xfwd_active
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            relay as *mut libc::c_void,
        );
        event_add(&mut (*relay).src_event, 0 as *const timeval);
        let mut dst_fd: libc::c_int = xcm_fd((*relay).dst_conn);
        if dst_fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"dst_fd >= 0\0" as *const u8 as *const libc::c_char,
                b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
                187 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"int xfwd_start(struct xfwd *)\0"))
                    .as_ptr(),
            );
        }
        'c_5841: {
            if dst_fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"dst_fd >= 0\0" as *const u8 as *const libc::c_char,
                    b"tools/xcmrelay/xrelay.c\0" as *const u8 as *const libc::c_char,
                    187 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 30],
                        &[libc::c_char; 30],
                    >(b"int xfwd_start(struct xfwd *)\0"))
                        .as_ptr(),
                );
            }
        };
        event_assign(
            &mut (*relay).dst_event,
            (*relay).event_base,
            dst_fd,
            (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
            Some(
                xfwd_active
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            relay as *mut libc::c_void,
        );
        event_add(&mut (*relay).dst_event, 0 as *const timeval);
        if (*relay).data_len == 0 as libc::c_int {
            xfwd_await_input(relay);
        } else {
            xfwd_await_output(relay);
        }
        (*relay).running = 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xfwd_stop(mut relay: *mut xfwd) {
    if (*relay).running {
        event_del(&mut (*relay).src_event);
        event_del(&mut (*relay).dst_event);
        del_condition(
            (*relay).src_conn,
            (*relay).src_condition,
            (1 as libc::c_int) << 0 as libc::c_int,
        );
        del_condition(
            (*relay).dst_conn,
            (*relay).dst_condition,
            (1 as libc::c_int) << 1 as libc::c_int,
        );
        (*relay).running = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn xrelay_fwd_term(
    mut reason: libc::c_int,
    mut msg: *const libc::c_char,
    mut cb_data: *mut libc::c_void,
) {
    let mut relay: *mut xrelay = cb_data as *mut xrelay;
    ((*relay).err_cb)
        .expect("non-null function pointer")(relay, reason, msg, (*relay).err_cb_data);
}
#[no_mangle]
pub unsafe extern "C" fn xrelay_create(
    mut conn0: *mut xcm_socket,
    mut conn1: *mut xcm_socket,
    mut err_cb: xrelay_err_cb,
    mut cb_data: *mut libc::c_void,
    mut event_base: *mut event_base,
) -> *mut xrelay {
    let mut relay: *mut xrelay = ut_malloc(
        ::core::mem::size_of::<xrelay>() as libc::c_ulong,
    ) as *mut xrelay;
    *relay = {
        let mut init = xrelay {
            err_cb: err_cb,
            err_cb_data: cb_data,
            fwd0: xfwd {
                event_base: 0 as *mut event_base,
                err_cb: None,
                err_cb_data: 0 as *mut libc::c_void,
                src_conn: 0 as *mut xcm_socket,
                dst_conn: 0 as *mut xcm_socket,
                src_condition: 0 as *mut libc::c_int,
                dst_condition: 0 as *mut libc::c_int,
                src_event: event {
                    ev_evcallback: event_callback {
                        evcb_active_next: C2RustUnnamed_7 {
                            tqe_next: 0 as *mut event_callback,
                            tqe_prev: 0 as *mut *mut event_callback,
                        },
                        evcb_flags: 0,
                        evcb_pri: 0,
                        evcb_closure: 0,
                        evcb_cb_union: C2RustUnnamed_6 {
                            evcb_callback: None,
                        },
                        evcb_arg: 0 as *mut libc::c_void,
                    },
                    ev_timeout_pos: C2RustUnnamed_4 {
                        ev_next_with_common_timeout: C2RustUnnamed_5 {
                            tqe_next: 0 as *mut event,
                            tqe_prev: 0 as *mut *mut event,
                        },
                    },
                    ev_fd: 0,
                    ev_base: 0 as *mut event_base,
                    ev_: C2RustUnnamed {
                        ev_io: C2RustUnnamed_2 {
                            ev_io_next: C2RustUnnamed_3 {
                                le_next: 0 as *mut event,
                                le_prev: 0 as *mut *mut event,
                            },
                            ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                        },
                    },
                    ev_events: 0,
                    ev_res: 0,
                    ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                },
                dst_event: event {
                    ev_evcallback: event_callback {
                        evcb_active_next: C2RustUnnamed_7 {
                            tqe_next: 0 as *mut event_callback,
                            tqe_prev: 0 as *mut *mut event_callback,
                        },
                        evcb_flags: 0,
                        evcb_pri: 0,
                        evcb_closure: 0,
                        evcb_cb_union: C2RustUnnamed_6 {
                            evcb_callback: None,
                        },
                        evcb_arg: 0 as *mut libc::c_void,
                    },
                    ev_timeout_pos: C2RustUnnamed_4 {
                        ev_next_with_common_timeout: C2RustUnnamed_5 {
                            tqe_next: 0 as *mut event,
                            tqe_prev: 0 as *mut *mut event,
                        },
                    },
                    ev_fd: 0,
                    ev_base: 0 as *mut event_base,
                    ev_: C2RustUnnamed {
                        ev_io: C2RustUnnamed_2 {
                            ev_io_next: C2RustUnnamed_3 {
                                le_next: 0 as *mut event,
                                le_prev: 0 as *mut *mut event,
                            },
                            ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                        },
                    },
                    ev_events: 0,
                    ev_res: 0,
                    ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                },
                data: 0 as *mut libc::c_char,
                data_capacity: 0,
                data_len: 0,
                running: false,
            },
            fwd1: xfwd {
                event_base: 0 as *mut event_base,
                err_cb: None,
                err_cb_data: 0 as *mut libc::c_void,
                src_conn: 0 as *mut xcm_socket,
                dst_conn: 0 as *mut xcm_socket,
                src_condition: 0 as *mut libc::c_int,
                dst_condition: 0 as *mut libc::c_int,
                src_event: event {
                    ev_evcallback: event_callback {
                        evcb_active_next: C2RustUnnamed_7 {
                            tqe_next: 0 as *mut event_callback,
                            tqe_prev: 0 as *mut *mut event_callback,
                        },
                        evcb_flags: 0,
                        evcb_pri: 0,
                        evcb_closure: 0,
                        evcb_cb_union: C2RustUnnamed_6 {
                            evcb_callback: None,
                        },
                        evcb_arg: 0 as *mut libc::c_void,
                    },
                    ev_timeout_pos: C2RustUnnamed_4 {
                        ev_next_with_common_timeout: C2RustUnnamed_5 {
                            tqe_next: 0 as *mut event,
                            tqe_prev: 0 as *mut *mut event,
                        },
                    },
                    ev_fd: 0,
                    ev_base: 0 as *mut event_base,
                    ev_: C2RustUnnamed {
                        ev_io: C2RustUnnamed_2 {
                            ev_io_next: C2RustUnnamed_3 {
                                le_next: 0 as *mut event,
                                le_prev: 0 as *mut *mut event,
                            },
                            ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                        },
                    },
                    ev_events: 0,
                    ev_res: 0,
                    ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                },
                dst_event: event {
                    ev_evcallback: event_callback {
                        evcb_active_next: C2RustUnnamed_7 {
                            tqe_next: 0 as *mut event_callback,
                            tqe_prev: 0 as *mut *mut event_callback,
                        },
                        evcb_flags: 0,
                        evcb_pri: 0,
                        evcb_closure: 0,
                        evcb_cb_union: C2RustUnnamed_6 {
                            evcb_callback: None,
                        },
                        evcb_arg: 0 as *mut libc::c_void,
                    },
                    ev_timeout_pos: C2RustUnnamed_4 {
                        ev_next_with_common_timeout: C2RustUnnamed_5 {
                            tqe_next: 0 as *mut event,
                            tqe_prev: 0 as *mut *mut event,
                        },
                    },
                    ev_fd: 0,
                    ev_base: 0 as *mut event_base,
                    ev_: C2RustUnnamed {
                        ev_io: C2RustUnnamed_2 {
                            ev_io_next: C2RustUnnamed_3 {
                                le_next: 0 as *mut event,
                                le_prev: 0 as *mut *mut event,
                            },
                            ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                        },
                    },
                    ev_events: 0,
                    ev_res: 0,
                    ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                },
                data: 0 as *mut libc::c_char,
                data_capacity: 0,
                data_len: 0,
                running: false,
            },
            cond0: 0,
            cond1: 0,
            running: false,
            entry: C2RustUnnamed_8 {
                le_next: 0 as *mut xrelay,
                le_prev: 0 as *mut *mut xrelay,
            },
        };
        init
    };
    xfwd_init(
        &mut (*relay).fwd0,
        conn0,
        conn1,
        &mut (*relay).cond0,
        &mut (*relay).cond1,
        Some(
            xrelay_fwd_term
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
        relay as *mut libc::c_void,
        event_base,
    );
    xfwd_init(
        &mut (*relay).fwd1,
        conn1,
        conn0,
        &mut (*relay).cond1,
        &mut (*relay).cond0,
        Some(
            xrelay_fwd_term
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
        relay as *mut libc::c_void,
        event_base,
    );
    return relay;
}
#[no_mangle]
pub unsafe extern "C" fn xrelay_start(mut relay: *mut xrelay) -> libc::c_int {
    if xfwd_start(&mut (*relay).fwd0) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if xfwd_start(&mut (*relay).fwd1) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xrelay_stop(mut relay: *mut xrelay) {
    xfwd_stop(&mut (*relay).fwd0);
    xfwd_stop(&mut (*relay).fwd1);
}
#[no_mangle]
pub unsafe extern "C" fn xrelay_destroy(mut relay: *mut xrelay) {
    if !relay.is_null() {
        let mut conn0: *mut xcm_socket = (*relay).fwd0.src_conn;
        let mut conn1: *mut xcm_socket = (*relay).fwd0.dst_conn;
        xfwd_deinit(&mut (*relay).fwd0);
        xfwd_deinit(&mut (*relay).fwd1);
        xcm_close(conn0);
        xcm_close(conn1);
        ut_free(relay as *mut libc::c_void);
    }
}
