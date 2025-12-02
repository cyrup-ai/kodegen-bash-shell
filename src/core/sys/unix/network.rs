pub(crate) fn get_hostname() -> std::io::Result<std::ffi::OsString> {
    crate::core::sys::hostname::get()
}
