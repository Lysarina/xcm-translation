use std::collections::LinkedList;
use std::os::raw::c_int;
use std::sync::{Mutex, OnceLock};

#[repr(C)]
pub struct ActiveFd {
    fd: c_int,
    cnt: c_int,
}

static ACTIVE_FD_LIST: OnceLock<Mutex<LinkedList<ActiveFd>>> = OnceLock::new();

#[no_mangle]
pub extern "C" fn active_fd_get() -> c_int {
    const MAX_USERS_PER_FD: c_int = 100;

    let list = ACTIVE_FD_LIST.get_or_init(|| Mutex::new(LinkedList::new()));
    let mut list = list.lock().unwrap();

    // Try to reuse an existing fd
    for entry in list.iter_mut() {
        if entry.cnt < MAX_USERS_PER_FD {
            entry.cnt += 1;
            return entry.fd;
        }
    }

    // Create a new one
    let fd = unsafe { libc::eventfd(1, libc::EFD_NONBLOCK) };
    if fd < 0 {
        return -1;
    }

    list.push_front(ActiveFd { fd, cnt: 1 });
    fd
}

#[no_mangle]
pub extern "C" fn active_fd_put(fd: c_int) {
    let list = ACTIVE_FD_LIST.get().unwrap();
    let mut list = list.lock().unwrap();

    let mut found = false;

    list.iter_mut().for_each(|entry| {
        if entry.fd == fd {
            entry.cnt -= 1;
            found = true;
        }
    });

    // Remove entries with cnt == 0
    // Unstable feature
    // list.retain(|entry| {
    //     if entry.fd == fd && entry.cnt == 0 {
    //         unsafe {
    //             libc::close(fd);
    //         }
    //         false
    //     } else {
    //         true
    //     }
    // });

    list.iter_mut().filter(|entry| entry.cnt == 0).for_each(|entry| {
        unsafe {
            libc::close(entry.fd);
        }
    });

    if !found {
        panic!("active_fd_put: unknown fd {}", fd);
    }
}
