pub fn get_env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> Option<String> {
    match std::env::var(key) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
