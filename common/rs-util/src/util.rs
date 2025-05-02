#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]
#![feature(c_variadic, extern_types)]

use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Read};
use libc::{accept4, timespec, pollfd, timeval, sockaddr,
    pthread_mutexattr_t, pthread_mutex_t, FILE, DIR, dirent,
    stat, PATH_MAX};

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
        1,
        4,
        &mut socket_errno as *mut libc::c_int as *mut libc::c_void,
        &mut len,
    ) < 0
    {
        return -1;
    }
    if socket_errno != 0 {
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
    poll(&mut pfd, 1, 0);
    *__errno_location() = _errno;
    if pfd.revents & 4 != 0
        || pfd.revents & 8 != 0
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
    let mut self_net_ns = [0 as libc::c_char; PATH_MAX as usize];

    // Format: /proc/<tid>/ns/net
    snprintf(
        self_net_ns.as_mut_ptr(),
        self_net_ns.len() as libc::c_ulong,
        c"/proc/%d/ns/net".as_ptr() as *const libc::c_char,
        ut_gettid(),
    );

    let mut self_ns_st: stat = std::mem::zeroed();
    if stat(self_net_ns.as_ptr(), &mut self_ns_st) < 0 {
        return -1;
    }

    let ns_dir = opendir(c"/run/netns".as_ptr() as *const libc::c_char);
    if ns_dir.is_null() {
        if *__errno_location() == libc::ENOENT {
            *name = 0; // Set to empty string
            return 0;
        }
        return -1;
    }

    *__errno_location() = 0;
    let mut rc = -1;

    loop {
        let entry = readdir(ns_dir);
        if entry.is_null() {
            break;
        }

        let d_name = (*entry).d_name.as_ptr();
        if *d_name == 0 {
            continue;
        }

        let path_len = strlen(c"/run/netns".as_ptr() as *const libc::c_char)
            + strlen(d_name)
            + 2; // for '/' and null terminator

        let mut ns_file = vec![0 as libc::c_char; path_len as usize];
        snprintf(
            ns_file.as_mut_ptr(),
            ns_file.len() as libc::c_ulong,
            c"/run/netns/%s".as_ptr() as *const libc::c_char,
            d_name,
        );

        let mut ns_st: stat = std::mem::zeroed();
        if stat(ns_file.as_ptr(), &mut ns_st) < 0 {
            break;
        }

        if self_ns_st.st_dev == ns_st.st_dev && self_ns_st.st_ino == ns_st.st_ino {
            strcpy(name, d_name);
            rc = 0;
            break;
        }
    }

    if rc != 0 && *__errno_location() == 0 {
        *name = 0;
        rc = 0;
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
    let mut capacity: libc::c_ulong = 0;
    let mut len: libc::c_long = 0;

    // Convert the filename from a raw C string to a Rust String
    let c_str = match CStr::from_ptr(filename).to_str() {
        Ok(s) => s,
        Err(_) => return -1, // Error handling if C string is not valid UTF-8
    };
    let file = match File::open(c_str) {
        Ok(f) => f,
        Err(_) => return -1, // Error handling if file can't be opened
    };

    *data = std::ptr::null_mut();

    let mut reader = io::BufReader::new(file);

    loop {
        capacity += 256;
        *data = ut_realloc(
            *data as *mut libc::c_void,
            capacity + spare_capacity,
        ) as *mut libc::c_char;

        // Read data into the buffer
        let buffer = std::slice::from_raw_parts_mut(*data as *mut u8, capacity as usize);
        let bytes_read = match reader.read(&mut buffer[len as usize..]) {
            Ok(b) => b,
            Err(_) => {
                ut_free(*data as *mut libc::c_void);
                *data = std::ptr::null_mut();
                return -1; // Error during reading
            }
        };

        len += bytes_read as libc::c_long;

        // If fewer than the expected number of bytes were read, check for EOF or error
        if bytes_read < 256 {
            break;
        }
    }

    // Return the number of bytes read
    len
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