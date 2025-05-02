#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(c_variadic, extern_types)]

use std::mem;
use libc::{accept4, timespec, pollfd, timeval, sockaddr,
    pthread_mutexattr_t, pthread_mutex_t, FILE, DIR, dirent,
    stat};

unsafe extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn clock_gettime(__clock_id: libc::c_int, __tp: *mut timespec) -> libc::c_int;
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
        __n: libc::c_ulong,
        __flags: libc::c_int,
    ) -> libc::c_long;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut libc::c_uint,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: libc::c_ulong, __timeout: libc::c_int) -> libc::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mutex_init(m: *mut pthread_mutex_t) { unsafe {
    let rc = pthread_mutex_init(m, std::ptr::null());
    if rc != 0 {
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mutex_lock(m: *mut pthread_mutex_t) { unsafe {
    let rc = pthread_mutex_lock(m);
    if rc != 0 {
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mutex_unlock(m: *mut pthread_mutex_t) { unsafe {
    let rc = pthread_mutex_unlock(m);
    if rc != 0 {
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_gettid() -> libc::c_int { unsafe {
    syscall(186) as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_malloc(size: libc::c_ulong) -> *mut libc::c_void { unsafe {
    let ptr: *mut libc::c_void = malloc(size);
    if ptr.is_null() {
        ut_mem_exhausted();
    }
    ptr
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_realloc(
    mut ptr: *mut libc::c_void,
    size: libc::c_ulong,
) -> *mut libc::c_void { unsafe {
    ptr = realloc(ptr, size);
    if ptr.is_null() {
        ut_mem_exhausted();
    }
    ptr
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_calloc(size: libc::c_ulong) -> *mut libc::c_void { unsafe {
    let ptr: *mut libc::c_void = ut_malloc(size);
    memset(ptr, 0 as libc::c_int, size);
    ptr
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_strdup(str: *const libc::c_char) -> *mut libc::c_char { unsafe {
    let copy: *mut libc::c_char = strdup(str);
    if copy.is_null() {
        ut_mem_exhausted();
    }
    copy
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_strndup(
    str: *const libc::c_char,
    n: libc::c_ulong,
) -> *mut libc::c_char { unsafe {
    let copy: *mut libc::c_char = strndup(str, n);
    if copy.is_null() {
        ut_mem_exhausted();
    }
    copy
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_memdup(
    ptr: *const libc::c_void,
    size: libc::c_ulong,
) -> *mut libc::c_void { unsafe {
    let copy: *mut libc::c_void = ut_malloc(size);
    memcpy(copy, ptr, size);
    copy
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_free(ptr: *mut libc::c_void) { unsafe {
    free(ptr);
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_mem_exhausted() -> ! { unsafe {
    ut_fatal();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_fatal() -> ! { unsafe {
    abort();
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_close(fd: libc::c_int) { unsafe {
    let mut _errno: libc::c_int = *__errno_location();
    close(fd);
    *__errno_location() = _errno;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_close_if_valid(fd: libc::c_int) { unsafe {
    if fd >= 0 {
        ut_close(fd);
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_timeval_to_f(tv: *const timeval) -> libc::c_double { unsafe {
    (*tv).tv_sec as libc::c_double + (*tv).tv_usec as libc::c_double / 1e6f64
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_timespec_to_f(ts: *const timespec) -> libc::c_double { unsafe {
    (*ts).tv_sec as libc::c_double + (*ts).tv_nsec as libc::c_double / 1e9f64
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_f_to_timespec(t: libc::c_double, ts: *mut timespec) { unsafe {
    (*ts).tv_sec = t as libc::c_long;
    (*ts).tv_nsec = ((t - (*ts).tv_sec as libc::c_double) * 1e9f64) as libc::c_long;
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_ftime() -> libc::c_double { unsafe {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1, &mut now);
    ut_timespec_to_f(&now)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_send_all(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    count: libc::c_ulong,
    flags: libc::c_int,
) -> libc::c_int { unsafe {
    let mut offset: libc::c_long = 0 as libc::c_int as libc::c_long;
    loop {
        let bytes_written: libc::c_long = send(
            fd,
            buf.offset(offset as isize),
            count.wrapping_sub(offset as libc::c_ulong),
            flags,
        );
        if bytes_written < 0 as libc::c_int as libc::c_long {
            return -1;
        }
        offset += bytes_written;
        if (offset as libc::c_ulong) >= count {
            break;
        }
    }
    count as libc::c_int
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_vaprintf(
    buf: *mut libc::c_char,
    capacity: libc::c_ulong,
    format: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) { unsafe {
    let len: libc::c_ulong = strlen(buf);
    let used: libc::c_ulong = len.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if used > capacity {
        abort();
    }
    let left: libc::c_ulong = capacity.wrapping_sub(used);
    if left == 0 {
        return;
    }
    let rc: libc::c_int = vsnprintf(
        buf.offset(len as isize),
        left,
        format,
        ap.as_va_list(),
    );
    if rc < 0 {
        abort();
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_aprintf(
    buf: *mut libc::c_char,
    capacity: libc::c_ulong,
    format: *const libc::c_char,
    args: ...
) { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    ut_vaprintf(buf, capacity, format, ap.as_va_list());
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_vasprintf(
    fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut libc::c_char { unsafe {
    let mut str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let rc: libc::c_int = vasprintf(&mut str, fmt, ap.as_va_list());
    if rc < 0 {
        ut_fatal();
    }
    str
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_asprintf(
    fmt: *const libc::c_char,
    args: ...
) -> *mut libc::c_char { unsafe {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let str: *mut libc::c_char = ut_vasprintf(fmt, ap.as_va_list());
    str
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_set_blocking(
    fd: libc::c_int,
    should_block: bool,
) -> libc::c_int { unsafe {
    let mut flags: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    if should_block {
        flags &= !(0o4000 as libc::c_int);
    } else {
        flags |= 0o4000 as libc::c_int;
    }
    fcntl(fd, 4 as libc::c_int, flags)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_is_blocking(fd: libc::c_int) -> bool { unsafe {
    let flags: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    (if flags & 0o4000 as libc::c_int != 0 {
        0
    } else {
        1
    } != 0)
}}
unsafe extern "C" fn socket_error(fd: libc::c_int) -> libc::c_int { unsafe {
    let mut socket_errno: libc::c_int = 0;
    let mut len: libc::c_uint = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as libc::c_uint;
    if getsockopt(
        fd,
        1 as libc::c_int,
        4 as libc::c_int,
        &mut socket_errno as *mut libc::c_int as *mut libc::c_void,
        &mut len,
    ) < 0 as libc::c_int
    {
        return -1;
    }
    if socket_errno != 0 as libc::c_int {
        *__errno_location() = socket_errno;
        return -1;
    }
    0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_established(fd: libc::c_int) -> libc::c_int { unsafe {
    let mut pfd: pollfd = {
        
        pollfd {
            fd,
            events: 0x4 as libc::c_int as libc::c_short,
            revents: 0,
        }
    };
    let mut _errno: libc::c_int = *__errno_location();
    poll(&mut pfd, 1 as libc::c_int as libc::c_ulong, 0 as libc::c_int);
    *__errno_location() = _errno;
    if pfd.revents as libc::c_int & 0x4 as libc::c_int != 0
        || pfd.revents as libc::c_int & 0x8 as libc::c_int != 0
    {
        socket_error(fd)
    } else {
        *__errno_location() = 115; 
        -1
    }
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_is_readable(fd: libc::c_int) -> bool { unsafe {
    let mut pfd: pollfd = {
        
        pollfd {
            fd,
            events: 0x1 as libc::c_int as libc::c_short,
            revents: 0,
        }
    };
    let mut _oerrno: libc::c_int = *__errno_location();
    let rc: libc::c_int = poll(
        &mut pfd,
        1,
        0,
    );
    *__errno_location() = _oerrno;
    rc == 1 && pfd.revents & 1 != 0
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_self_net_ns(name: *mut libc::c_char) -> libc::c_int { unsafe {
    let current_block: u64;
    let mut self_net_ns: [libc::c_char; 4096] = [0; 4096];
    snprintf(
        self_net_ns.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"/proc/%d/ns/net\0" as *const u8 as *const libc::c_char,
        ut_gettid(),
    );
    let mut self_ns_st: stat = mem::zeroed();
    if stat(self_net_ns.as_mut_ptr(), &mut self_ns_st) < 0 as libc::c_int {
        return -1;
    }
    let ns_dir: *mut DIR = opendir(
        b"/run/netns\0" as *const u8 as *const libc::c_char,
    );
    if ns_dir.is_null() {
        if *__errno_location() == 2 {
            *name.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            return 0;
        } else {
            return -1;
        }
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    *__errno_location() = 0 as libc::c_int;
    let mut e: *mut dirent;
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
        let mut ns_st: stat = mem::zeroed(); 

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
    if current_block == 17407779659766490442 && *__errno_location() == 0 as libc::c_int {
        *name.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        rc = 0 as libc::c_int;
    }
    closedir(ns_dir);
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_accept(
    sockfd: libc::c_int,
    addr: *mut sockaddr,
    addrlen: *mut libc::c_uint,
    flags: libc::c_uint,
) -> libc::c_int { unsafe {
    let rc: libc::c_int = accept4(
        sockfd,
        addr,
        addrlen,
        flags as libc::c_int,
    );

    rc
}}
unsafe extern "C" fn load_file(
    filename: *const libc::c_char,
    data: *mut *mut libc::c_char,
    spare_capacity: libc::c_ulong,
) -> libc::c_long { unsafe {
    let current_block: u64;
    let mut capacity: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut len: libc::c_long = 0 as libc::c_int as libc::c_long;
    let f: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        *data = std::ptr::null_mut::<libc::c_char>();
        loop {
            capacity = capacity.wrapping_add(256 as libc::c_int as libc::c_ulong);
            *data = ut_realloc(
                *data as *mut libc::c_void,
                capacity.wrapping_add(spare_capacity),
            ) as *mut libc::c_char;
            let b: libc::c_ulong = fread(
                (*data).offset(len as isize) as *mut libc::c_void,
                1 as libc::c_ulong,
                256 as libc::c_ulong,
                f,
            );
            len = (len as libc::c_ulong).wrapping_add(b) as libc::c_long as libc::c_long;
            if b >= 256 as libc::c_ulong {
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
                *data = std::ptr::null_mut::<libc::c_char>();
                fclose(f);
            }
            _ => {
                fclose(f);
                return len;
            }
        }
    }
    -(1 as libc::c_int) as libc::c_long
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_load_file(
    filename: *const libc::c_char,
    data: *mut *mut libc::c_char,
) -> libc::c_long { unsafe {
    load_file(filename, data, 0)
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_load_text_file(
    filename: *const libc::c_char,
    data: *mut *mut libc::c_char,
) -> libc::c_long { unsafe {
    let mut rc: libc::c_long = load_file(filename, data, 1);
    if rc >= 0 {
        *(*data).offset(rc as isize) = '\0' as i32 as libc::c_char;
        rc += 1;
    }
    rc
}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ut_die(fmt: *const libc::c_char, args: ...) -> ! { unsafe {
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
}}