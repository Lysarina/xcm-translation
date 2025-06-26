#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xcm_dns_is_valid_name(name: *const libc::c_char) -> bool {

    if name.is_null() {
        return false;
    }
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };

    let name_bytes = c_str.to_bytes();

    if name_bytes.len() > 253 {
        return false;
    }

    static DNS_NAME_RE: once_cell::sync::Lazy<regex::bytes::Regex> = once_cell::sync::Lazy::new(|| {
        regex::bytes::Regex::new(r"(?i)^[a-z0-9\-]+(\.[a-z0-9\-]+\.?)*$").expect("invalid regex")

    });

    DNS_NAME_RE.is_match(name_bytes)
}

