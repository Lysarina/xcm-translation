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
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rserver;
    fn xcm_attr_map_create() -> *mut xcm_attr_map;
    fn xcm_attr_map_add_str(
        attr_map: *mut xcm_attr_map,
        attr_name: *const libc::c_char,
        attr_value: *const libc::c_char,
    );
    fn xcm_attr_map_destroy(attr_map: *mut xcm_attr_map);
    fn attr_parse_bool(s: *const libc::c_char, attrs: *mut xcm_attr_map);
    fn attr_parse_int64(s: *const libc::c_char, attrs: *mut xcm_attr_map);
    fn attr_parse_double(s: *const libc::c_char, attrs: *mut xcm_attr_map);
    fn attr_parse_str(s: *const libc::c_char, attrs: *mut xcm_attr_map);
    fn attr_load_bin_file(s: *const libc::c_char, attrs: *mut xcm_attr_map);
    fn attr_load_bin_stdin(s: *const libc::c_char, attrs: *mut xcm_attr_map);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn event_base_new() -> *mut event_base;
    fn event_base_dispatch(_: *mut event_base) -> libc::c_int;
    fn event_base_free(_: *mut event_base);
    fn event_base_loopbreak(_: *mut event_base) -> libc::c_int;
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
    fn rserver_create(
        server_addr: *const libc::c_char,
        server_attrs: *const xcm_attr_map,
        server_conn_attrs: *const xcm_attr_map,
        client_addr: *const libc::c_char,
        client_conn_attrs: *const xcm_attr_map,
        fatal_cb: rserver_fatal_cb,
        fatal_cb_data: *mut libc::c_void,
        event_base: *mut event_base,
    ) -> *mut rserver;
    fn rserver_destroy(server: *mut rserver);
    fn rserver_start(rserver: *mut rserver) -> libc::c_int;
    fn rserver_stop(rserver: *mut rserver);
    fn exit(_: libc::c_int) -> !;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn xcm_addr_is_valid(xcm_addr_s: *const libc::c_char) -> bool;
    fn xcm_addr_is_supported(xcm_addr_s: *const libc::c_char) -> bool;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type rserver_fatal_cb = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
unsafe extern "C" fn usage(mut name: *const libc::c_char) {
    printf(
        b"Usage: %s [OPTIONS] <server-addr> <client-addr>\n\0" as *const u8
            as *const libc::c_char,
        name,
    );
    printf(b"OPTIONS:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -b <name>=(true|false)  Set boolean attribute on connections to the client\n                         address.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -i <name>=<value>       Set integer attribute on connections to the client\n                         address.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -d <name>=<value>       Set double-precision floating point attribute on\n                         connections to client address.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -s <name>=<value>       Set string attribute on connections to client address.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -f <name>=<filename>    Set binary attribute on connections to the contents\n                         of <filename>.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -r <name>               Read binary connection attribute from stdin. The\n                         value data must be preceded by a 32-bit length field\n                         in network byte order.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -x                      One subsequent -b, -i, -d, -s, -f, or -r switch\n                         configures a server socket attribute.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -y                      One subsequent -b, -i, -d, -s, -f, or -r switch\n                         configures a server-side connection socket attribute.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -h                      Prints this text.\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn on_signal(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut event_base: *mut event_base = arg as *mut event_base;
    event_base_loopbreak(event_base);
}
static mut exit_code: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn handle_fatal(mut cb_data: *mut libc::c_void) {
    let mut event_base: *mut event_base = cb_data as *mut event_base;
    event_base_loopbreak(event_base);
    exit_code = 1 as libc::c_int;
}
unsafe extern "C" fn check_addr(mut addr: *const libc::c_char) {
    if !xcm_addr_is_valid(addr) {
        printf(
            b"\"%s\" is not a valid XCM address.\n\0" as *const u8
                as *const libc::c_char,
            addr,
        );
        exit(1 as libc::c_int);
    } else if !xcm_addr_is_supported(addr) {
        printf(
            b"\"%s\" is not supported by the XCM library.\n\0" as *const u8
                as *const libc::c_char,
            addr,
        );
        exit(1 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut client_conn_attrs: *mut xcm_attr_map = xcm_attr_map_create();
    let mut server_attrs: *mut xcm_attr_map = xcm_attr_map_create();
    let mut server_conn_attrs: *mut xcm_attr_map = xcm_attr_map_create();
    let mut attrs: *mut xcm_attr_map = client_conn_attrs;
    loop {
        c = getopt(argc, argv, b"b:i:d:s:f:r:xyh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                attr_parse_bool(optarg, attrs);
                attrs = client_conn_attrs;
            }
            105 => {
                attr_parse_int64(optarg, attrs);
                attrs = client_conn_attrs;
            }
            100 => {
                attr_parse_double(optarg, attrs);
                attrs = client_conn_attrs;
            }
            115 => {
                attr_parse_str(optarg, attrs);
                attrs = client_conn_attrs;
            }
            102 => {
                attr_load_bin_file(optarg, attrs);
                attrs = client_conn_attrs;
            }
            114 => {
                attr_load_bin_stdin(optarg, attrs);
                attrs = client_conn_attrs;
            }
            120 => {
                attrs = server_attrs;
            }
            121 => {
                attrs = server_conn_attrs;
            }
            104 => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(0 as libc::c_int);
            }
            _ => {}
        }
    }
    let mut num_args: libc::c_int = argc - optind;
    if num_args != 2 as libc::c_int {
        usage(*argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    if attrs == server_attrs {
        fprintf(
            stderr,
            b"-x specified without subsequent -b, -i, -d, -s or -f.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if attrs == server_conn_attrs {
        fprintf(
            stderr,
            b"-y specified without subsequent -b, -i, -d, -s or -f.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut server_addr: *const libc::c_char = *argv.offset(optind as isize);
    let mut client_addr: *const libc::c_char = *argv
        .offset((optind + 1 as libc::c_int) as isize);
    check_addr(server_addr);
    check_addr(client_addr);
    xcm_attr_map_add_str(
        client_conn_attrs,
        b"xcm.service\0" as *const u8 as *const libc::c_char,
        b"any\0" as *const u8 as *const libc::c_char,
    );
    xcm_attr_map_add_str(
        server_attrs,
        b"xcm.service\0" as *const u8 as *const libc::c_char,
        b"any\0" as *const u8 as *const libc::c_char,
    );
    let mut event_base: *mut event_base = event_base_new();
    let mut sigint_event: event = event {
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
    };
    event_assign(
        &mut sigint_event,
        event_base,
        2 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            on_signal
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        event_base as *mut libc::c_void,
    );
    event_add(&mut sigint_event, 0 as *const timeval);
    let mut sighup_event: event = event {
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
    };
    event_assign(
        &mut sighup_event,
        event_base,
        1 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            on_signal
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        event_base as *mut libc::c_void,
    );
    event_add(&mut sighup_event, 0 as *const timeval);
    let mut sigterm_event: event = event {
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
    };
    event_assign(
        &mut sigterm_event,
        event_base,
        15 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            on_signal
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        event_base as *mut libc::c_void,
    );
    event_add(&mut sigterm_event, 0 as *const timeval);
    let mut rserver: *mut rserver = rserver_create(
        server_addr,
        server_attrs,
        server_conn_attrs,
        client_addr,
        client_conn_attrs,
        Some(handle_fatal as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        event_base as *mut libc::c_void,
        event_base,
    );
    if rserver.is_null() {
        exit(1 as libc::c_int);
    }
    xcm_attr_map_destroy(server_attrs);
    xcm_attr_map_destroy(server_conn_attrs);
    xcm_attr_map_destroy(client_conn_attrs);
    if rserver_start(rserver) < 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    event_base_dispatch(event_base);
    rserver_stop(rserver);
    rserver_destroy(rserver);
    event_del(&mut sigint_event);
    event_del(&mut sighup_event);
    event_del(&mut sigterm_event);
    event_base_free(event_base);
    exit(exit_code);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
