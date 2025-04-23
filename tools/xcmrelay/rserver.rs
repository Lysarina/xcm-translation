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
    pub type xcm_attr_map;
    pub type xcm_socket;
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn xcm_attr_map_clone(original: *const xcm_attr_map) -> *mut xcm_attr_map;
    fn xcm_attr_map_add_bool(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: bool,
    );
    fn xcm_attr_map_destroy(attr_map: *mut xcm_attr_map);
    fn xcm_connect_a(
        remote_addr: *const libc::c_char,
        attrs: *const xcm_attr_map,
    ) -> *mut xcm_socket;
    fn xcm_server_a(
        local_addr: *const libc::c_char,
        attrs: *const xcm_attr_map,
    ) -> *mut xcm_socket;
    fn xcm_close(socket: *mut xcm_socket) -> libc::c_int;
    fn xcm_accept_a(
        server_socket: *mut xcm_socket,
        attrs: *const xcm_attr_map,
    ) -> *mut xcm_socket;
    fn xcm_await(socket: *mut xcm_socket, condition: libc::c_int) -> libc::c_int;
    fn xcm_fd(socket: *mut xcm_socket) -> libc::c_int;
    fn xcm_finish(socket: *mut xcm_socket) -> libc::c_int;
    fn xcm_attr_get_str(
        socket: *mut xcm_socket,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        capacity: size_t,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn xrelay_create(
        conn0: *mut xcm_socket,
        conn1: *mut xcm_socket,
        err_cb: xrelay_err_cb,
        cb_data: *mut libc::c_void,
        event_base: *mut event_base,
    ) -> *mut xrelay;
    fn xrelay_destroy(relay: *mut xrelay);
    fn xrelay_start(relay: *mut xrelay) -> libc::c_int;
    fn xrelay_stop(relay: *mut xrelay);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ut_malloc(size: size_t) -> *mut libc::c_void;
    fn ut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn ut_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type event_callback_fn = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rserver {
    pub server_socket: *mut xcm_socket,
    pub server_conn_attrs: *mut xcm_attr_map,
    pub client_addr: *mut libc::c_char,
    pub client_conn_attrs: *mut xcm_attr_map,
    pub server_socket_event: event,
    pub running: bool,
    pub relays: xrelay_list,
    pub fatal_cb: rserver_fatal_cb,
    pub fatal_cb_data: *mut libc::c_void,
    pub event_base: *mut event_base,
}
pub type rserver_fatal_cb = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xrelay_list {
    pub lh_first: *mut xrelay,
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
pub type xfwd_err_cb = Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut libc::c_void) -> (),
>;
pub type xrelay_err_cb = Option::<
    unsafe extern "C" fn(
        *mut xrelay,
        libc::c_int,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> (),
>;
#[no_mangle]
pub unsafe extern "C" fn rserver_create(
    mut server_addr: *const libc::c_char,
    mut server_attrs: *const xcm_attr_map,
    mut server_conn_attrs: *const xcm_attr_map,
    mut client_addr: *const libc::c_char,
    mut client_conn_attrs: *const xcm_attr_map,
    mut fatal_cb: rserver_fatal_cb,
    mut fatal_cb_data: *mut libc::c_void,
    mut event_base: *mut event_base,
) -> *mut rserver {
    let mut attrs: *mut xcm_attr_map = xcm_attr_map_clone(server_attrs);
    xcm_attr_map_add_bool(
        attrs,
        b"xcm.blocking\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
    let mut server_socket: *mut xcm_socket = xcm_server_a(server_addr, attrs);
    xcm_attr_map_destroy(attrs);
    if server_socket.is_null() {
        perror(b"Unable to create server socket\0" as *const u8 as *const libc::c_char);
        return 0 as *mut rserver;
    }
    let mut server: *mut rserver = ut_malloc(
        ::core::mem::size_of::<rserver>() as libc::c_ulong,
    ) as *mut rserver;
    *server = {
        let mut init = rserver {
            server_socket: server_socket,
            server_conn_attrs: xcm_attr_map_clone(server_conn_attrs),
            client_addr: ut_strdup(client_addr),
            client_conn_attrs: xcm_attr_map_clone(client_conn_attrs),
            server_socket_event: event {
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
            running: false,
            relays: xrelay_list {
                lh_first: 0 as *mut xrelay,
            },
            fatal_cb: fatal_cb,
            fatal_cb_data: fatal_cb_data,
            event_base: event_base,
        };
        init
    };
    xcm_attr_map_add_bool(
        (*server).client_conn_attrs,
        b"xcm.blocking\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
    (*server).relays.lh_first = 0 as *mut xrelay;
    return server;
}
#[no_mangle]
pub unsafe extern "C" fn rserver_destroy(mut server: *mut rserver) {
    if !server.is_null() {
        rserver_stop(server);
        xcm_close((*server).server_socket);
        xcm_attr_map_destroy((*server).server_conn_attrs);
        xcm_attr_map_destroy((*server).client_conn_attrs);
        ut_free((*server).client_addr as *mut libc::c_void);
        ut_free(server as *mut libc::c_void);
    }
}
unsafe extern "C" fn rserver_num_relays(mut server: *mut rserver) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut relay: *mut xrelay = 0 as *mut xrelay;
    relay = (*server).relays.lh_first;
    while !relay.is_null() {
        count = count.wrapping_add(1);
        count;
        relay = (*relay).entry.le_next;
    }
    return count;
}
unsafe extern "C" fn rserver_terminate_relay(
    mut relay: *mut xrelay,
    mut reason: libc::c_int,
    mut msg: *const libc::c_char,
    mut cb_data: *mut libc::c_void,
) {
    let mut server: *mut rserver = cb_data as *mut rserver;
    xrelay_stop(relay);
    if rserver_num_relays(server) == 10000 as libc::c_int as size_t {
        xcm_await((*server).server_socket, (1 as libc::c_int) << 2 as libc::c_int);
    }
    if !((*relay).entry.le_next).is_null() {
        (*(*relay).entry.le_next).entry.le_prev = (*relay).entry.le_prev;
    }
    *(*relay).entry.le_prev = (*relay).entry.le_next;
    xrelay_destroy(relay);
}
unsafe extern "C" fn rserver_accept(
    mut fd: libc::c_int,
    mut ev: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut server: *mut rserver = arg as *mut rserver;
    if rserver_num_relays(server) == 10000 as libc::c_int as size_t {
        xcm_finish((*server).server_socket);
        return;
    }
    let mut server_conn: *mut xcm_socket = xcm_accept_a(
        (*server).server_socket,
        (*server).server_conn_attrs,
    );
    if server_conn.is_null() {
        return;
    }
    let mut client_conn: *mut xcm_socket = xcm_connect_a(
        (*server).client_addr,
        (*server).client_conn_attrs,
    );
    if client_conn.is_null() {
        xcm_close(server_conn);
        return;
    }
    let mut server_service: [libc::c_char; 128] = [0; 128];
    let mut client_service: [libc::c_char; 128] = [0; 128];
    if xcm_attr_get_str(
        server_conn,
        b"xcm.service\0" as *const u8 as *const libc::c_char,
        server_service.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    ) < 0 as libc::c_int
        || xcm_attr_get_str(
            client_conn,
            b"xcm.service\0" as *const u8 as *const libc::c_char,
            client_service.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Unable to retrieve XCM service from connection socket: %s\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        xcm_close(client_conn);
        xcm_close(server_conn);
        if ((*server).fatal_cb).is_some() {
            ((*server).fatal_cb)
                .expect("non-null function pointer")((*server).fatal_cb_data);
        }
        return;
    }
    if strcmp(server_service.as_mut_ptr(), client_service.as_mut_ptr())
        != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Server connection is of type \"%s\", while client connection is of incompatible type \"%s\".\n\0"
                as *const u8 as *const libc::c_char,
            server_service.as_mut_ptr(),
            client_service.as_mut_ptr(),
        );
        xcm_close(client_conn);
        xcm_close(server_conn);
        if ((*server).fatal_cb).is_some() {
            ((*server).fatal_cb)
                .expect("non-null function pointer")((*server).fatal_cb_data);
        }
        return;
    }
    let mut new_relay: *mut xrelay = xrelay_create(
        server_conn,
        client_conn,
        Some(
            rserver_terminate_relay
                as unsafe extern "C" fn(
                    *mut xrelay,
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
        server as *mut libc::c_void,
        (*server).event_base,
    );
    if xrelay_start(new_relay) < 0 as libc::c_int {
        xrelay_destroy(new_relay);
        return;
    }
    (*new_relay).entry.le_next = (*server).relays.lh_first;
    if !((*new_relay).entry.le_next).is_null() {
        (*(*server).relays.lh_first).entry.le_prev = &mut (*new_relay).entry.le_next;
    }
    (*server).relays.lh_first = new_relay;
    (*new_relay).entry.le_prev = &mut (*server).relays.lh_first;
    if rserver_num_relays(server) == 10000 as libc::c_int as size_t {
        xcm_await((*server).server_socket, 0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rserver_start(mut server: *mut rserver) -> libc::c_int {
    if !(*server).running {
        xcm_await((*server).server_socket, (1 as libc::c_int) << 2 as libc::c_int);
        event_assign(
            &mut (*server).server_socket_event,
            (*server).event_base,
            xcm_fd((*server).server_socket),
            (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
            Some(
                rserver_accept
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            server as *mut libc::c_void,
        );
        event_add(&mut (*server).server_socket_event, 0 as *const timeval);
        (*server).running = 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rserver_stop(mut server: *mut rserver) {
    if (*server).running {
        xcm_await((*server).server_socket, 0 as libc::c_int);
        event_del(&mut (*server).server_socket_event);
        (*server).running = 0 as libc::c_int != 0;
    }
}
