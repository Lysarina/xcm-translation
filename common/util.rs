#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ctl;
    pub type xpoll;
    pub type attr_tree;
    pub type __dirstream;
    fn __errno_location() -> *mut libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type va_list = __builtin_va_list;
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
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type nfds_t = libc::c_ulong;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mutex_init(mut m: *mut pthread_mutex_t) {
    let mut rc: libc::c_int = pthread_mutex_init(m, 0 as *const pthread_mutexattr_t);
    if !(rc == 0 as libc::c_int) {
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mutex_lock(mut m: *mut pthread_mutex_t) {
    abort("ut_mutex_lock_trying abort");
    let mut rc: libc::c_int = pthread_mutex_lock(m);
    if !(rc == 0 as libc::c_int) {
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mutex_unlock(mut m: *mut pthread_mutex_t) {
    let mut rc: libc::c_int = pthread_mutex_unlock(m);
    if !(rc == 0 as libc::c_int) {
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_gettid() -> pid_t {
    return syscall(186 as libc::c_int as libc::c_long) as pid_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(size);
    if ptr.is_null() {
        ut_mem_exhausted();
    }
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    ptr = realloc(ptr, size);
    if ptr.is_null() {
        ut_mem_exhausted();
    }
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_calloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ut_malloc(size);
    memset(ptr, 0 as libc::c_int, size);
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(str);
    if copy.is_null() {
        ut_mem_exhausted();
    }
    return copy;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_strndup(
    mut str: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strndup(str, n);
    if copy.is_null() {
        ut_mem_exhausted();
    }
    return copy;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_memdup(
    mut ptr: *const libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut copy: *mut libc::c_void = ut_malloc(size);
    memcpy(copy, ptr, size);
    return copy;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mem_exhausted() -> ! {
    ut_fatal();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_fatal() -> ! {
    abort();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_close(mut fd: libc::c_int) {
    let mut _errno: libc::c_int = *__errno_location();
    close(fd);
    *__errno_location() = _errno;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_close_if_valid(mut fd: libc::c_int) {
    if fd >= 0 as libc::c_int {
        ut_close(fd);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_timeval_to_f(mut tv: *const timeval) -> libc::c_double {
    return (*tv).tv_sec as libc::c_double + (*tv).tv_usec as libc::c_double / 1e6f64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_timespec_to_f(mut ts: *const timespec) -> libc::c_double {
    return (*ts).tv_sec as libc::c_double + (*ts).tv_nsec as libc::c_double / 1e9f64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_f_to_timespec(mut t: libc::c_double, mut ts: *mut timespec) {
    (*ts).tv_sec = t as __time_t;
    (*ts).tv_nsec = ((t - (*ts).tv_sec as libc::c_double) * 1e9f64) as __syscall_slong_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_ftime() -> libc::c_double {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut now);
    return ut_timespec_to_f(&mut now);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_send_all(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut offset: ssize_t = 0 as libc::c_int as ssize_t;
    loop {
        let mut bytes_written: ssize_t = send(
            fd,
            buf.offset(offset as isize),
            count.wrapping_sub(offset as size_t),
            flags,
        );
        if bytes_written < 0 as libc::c_int as ssize_t {
            return -(1 as libc::c_int);
        }
        offset += bytes_written;
        if !((offset as size_t) < count) {
            break;
        }
    }
    return count as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_vaprintf(
    mut buf: *mut libc::c_char,
    mut capacity: size_t,
    mut format: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    let mut len: size_t = strlen(buf);
    let mut used: size_t = len.wrapping_add(1 as libc::c_int as size_t);
    if !(used <= capacity) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
        abort();
    }
    let mut left: size_t = capacity.wrapping_sub(used);
    if left == 0 as libc::c_int as size_t {
        return; 
    }
    let mut rc: libc::c_int = vsnprintf(
        buf.offset(len as isize),
        left,
        format,
        ap.as_va_list(),
    );
    if !(rc >= 0 as libc::c_int) {
        log_console_conf(1 as libc::c_int != 0);
        if log_is_enabled(log_type_error) {
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_aprintf(
    mut buf: *mut libc::c_char,
    mut capacity: size_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    ut_vaprintf(buf, capacity, format, ap.as_va_list());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_vasprintf(
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = vasprintf(&mut str, fmt, ap.as_va_list());
    if rc < 0 as libc::c_int {
        ut_fatal();
    }
    return str;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_asprintf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut str: *mut libc::c_char = ut_vasprintf(fmt, ap.as_va_list());
    return str;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_set_blocking(
    mut fd: libc::c_int,
    mut should_block: bool,
) -> libc::c_int {
    let mut flags: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    if should_block {
        flags &= !(0o4000 as libc::c_int);
    } else {
        flags |= 0o4000 as libc::c_int;
    }
    return fcntl(fd, 4 as libc::c_int, flags);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_is_blocking(mut fd: libc::c_int) -> bool {
    let mut flags: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    return if flags & 0o4000 as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    } != 0;
}
unsafe extern "C" fn socket_error(mut fd: libc::c_int) -> libc::c_int {
    let mut socket_errno: libc::c_int = 0;
    let mut len: socklen_t = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as socklen_t;
    if getsockopt(
        fd,
        1 as libc::c_int,
        4 as libc::c_int,
        &mut socket_errno as *mut libc::c_int as *mut libc::c_void,
        &mut len,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if socket_errno != 0 as libc::c_int {
        *__errno_location() = socket_errno;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_established(mut fd: libc::c_int) -> libc::c_int {
    let mut pfd: pollfd = {
        let mut init = pollfd {
            fd: fd,
            events: 0x4 as libc::c_int as libc::c_short,
            revents: 0,
        };
        init
    };
    let mut _errno: libc::c_int = *__errno_location();
    poll(&mut pfd, 1 as libc::c_int as nfds_t, 0 as libc::c_int);
    *__errno_location() = _errno;
    if pfd.revents as libc::c_int & 0x4 as libc::c_int != 0
        || pfd.revents as libc::c_int & 0x8 as libc::c_int != 0
    {
        return socket_error(fd)
    } else {
        *__errno_location() = 115 as libc::c_int;
        return -(1 as libc::c_int);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_is_readable(mut fd: libc::c_int) -> bool {
    let mut pfd: pollfd = {
        let mut init = pollfd {
            fd: fd,
            events: 0x1 as libc::c_int as libc::c_short,
            revents: 0,
        };
        init
    };
    let mut _oerrno: libc::c_int = *__errno_location();
    let mut rc: libc::c_int = poll(
        &mut pfd,
        1 as libc::c_int as nfds_t,
        0 as libc::c_int,
    );
    *__errno_location() = _oerrno;
    return rc == 1 as libc::c_int
        && pfd.revents as libc::c_int & 0x1 as libc::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_self_net_ns(mut name: *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut self_net_ns: [libc::c_char; 4096] = [0; 4096];
    snprintf(
        self_net_ns.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"/proc/%d/ns/net\0" as *const u8 as *const libc::c_char,
        ut_gettid(),
    );
    let mut self_ns_st: stat = stat {
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
    if stat(self_net_ns.as_mut_ptr(), &mut self_ns_st) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut ns_dir: *mut DIR = opendir(
        b"/run/netns\0" as *const u8 as *const libc::c_char,
    );
    if ns_dir.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            *name.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            return 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    *__errno_location() = 0 as libc::c_int;
    let mut e: *mut dirent = 0 as *mut dirent;
    loop {
        e = readdir(ns_dir);
        if e.is_null() {
            current_block = 17407779659766490442;
            break;
        }
        let vla = (strlen(b"/run/netns\0" as *const u8 as *const libc::c_char))
            .wrapping_add(strlen(((*e).d_name).as_mut_ptr()))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as usize;
        let mut ns_file: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
        snprintf(
            ns_file.as_mut_ptr(),
            (vla * ::core::mem::size_of::<libc::c_char>()) as libc::c_ulong,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            b"/run/netns\0" as *const u8 as *const libc::c_char,
            ((*e).d_name).as_mut_ptr(),
        );
        let mut ns_st: stat = stat {
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
        if stat(ns_file.as_mut_ptr(), &mut ns_st) < 0 as libc::c_int {
            current_block = 1624237813134515024;
            break;
        }
        if !(self_ns_st.st_dev == ns_st.st_dev && self_ns_st.st_ino == ns_st.st_ino) {
            continue;
        }
        strcpy(name, ((*e).d_name).as_mut_ptr());
        rc = 0 as libc::c_int;
        current_block = 1624237813134515024;
        break;
    }
    match current_block {
        17407779659766490442 => {
            if *__errno_location() == 0 as libc::c_int {
                *name.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                rc = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    closedir(ns_dir);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_accept(
    mut sockfd: libc::c_int,
    mut addr: *mut sockaddr,
    mut addrlen: *mut socklen_t,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut rc: libc::c_int = accept4(sockfd, addr, addrlen, flags);
    if rc < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int {
        *__errno_location() = 11 as libc::c_int;
    }
    return rc;
}
unsafe extern "C" fn load_file(
    mut filename: *const libc::c_char,
    mut data: *mut *mut libc::c_char,
    mut spare_capacity: size_t,
) -> ssize_t {
    let mut current_block: u64;
    let mut capacity: size_t = 0 as libc::c_int as size_t;
    let mut len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut f: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        *data = 0 as *mut libc::c_char;
        loop {
            capacity = capacity.wrapping_add(256 as libc::c_int as size_t);
            *data = ut_realloc(
                *data as *mut libc::c_void,
                capacity.wrapping_add(spare_capacity),
            ) as *mut libc::c_char;
            let mut b: size_t = fread(
                (*data).offset(len as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                256 as libc::c_int as libc::c_ulong,
                f,
            );
            len = (len as size_t).wrapping_add(b) as ssize_t as ssize_t;
            if !(b < 256 as libc::c_int as size_t) {
                continue;
            }
            if ferror(f) != 0 {
                current_block = 13661517195515771617;
                break;
            } else {
                current_block = 3276175668257526147;
                break;
            }
        }
        match current_block {
            13661517195515771617 => {
                ut_free(*data as *mut libc::c_void);
                *data = 0 as *mut libc::c_char;
                fclose(f);
            }
            _ => {
                fclose(f);
                return len;
            }
        }
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_load_file(
    mut filename: *const libc::c_char,
    mut data: *mut *mut libc::c_char,
) -> ssize_t {
    return load_file(filename, data, 0 as libc::c_int as size_t);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_load_text_file(
    mut filename: *const libc::c_char,
    mut data: *mut *mut libc::c_char,
) -> ssize_t {
    let mut rc: ssize_t = load_file(filename, data, 1 as libc::c_int as size_t);
    if rc >= 0 as libc::c_int as ssize_t {
        *(*data).offset(rc as isize) = '\0' as i32 as libc::c_char;
        rc += 1;
        rc;
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_die(mut fmt: *const libc::c_char, mut args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    fprintf(stderr, b"FATAL: \0" as *const u8 as *const libc::c_char);
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(
        stderr,
        b": %s.\n\0" as *const u8 as *const libc::c_char,
        strerror(*__errno_location()),
    );
    exit(1 as libc::c_int);
}
