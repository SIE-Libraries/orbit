use std::env;

fn bytes_to_str(input: &[u8]) -> Result<&str, i32> {
    std::str::from_utf8(input).map_err(|_| libc::EINVAL)
}

pub fn get(key: &[u8]) -> Result<Vec<u8>, i32> {
    let key = bytes_to_str(key)?;
    match env::var(key) {
        Ok(value) => Ok(value.into_bytes()),
        Err(env::VarError::NotPresent) => Err(libc::ENOENT),
        Err(env::VarError::NotUnicode(_)) => Err(libc::EILSEQ),
    }
}

pub fn set(key: &[u8], value: &[u8]) -> Result<(), i32> {
    let key = bytes_to_str(key)?;
    let value = bytes_to_str(value)?;
    if key.is_empty() {
        return Err(libc::EINVAL);
    }
    env::set_var(key, value);
    Ok(())
}

pub fn unset(key: &[u8]) -> Result<(), i32> {
    let key = bytes_to_str(key)?;
    if key.is_empty() {
        return Err(libc::EINVAL);
    }
    env::remove_var(key);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_value_when_present() {
        std::env::set_var("ORBIT_TEST_KEY", "orbit-value");
        assert_eq!(get(b"ORBIT_TEST_KEY").unwrap(), b"orbit-value".to_vec());
    }

    #[test]
    fn get_returns_errno_when_missing() {
        std::env::remove_var("ORBIT_TEST_MISSING");
        assert_eq!(get(b"ORBIT_TEST_MISSING"), Err(libc::ENOENT));
    }

    #[test]
    fn set_succeeds_for_valid_key() {
        assert!(set(b"ORBIT_SET_TEST", b"ok").is_ok());
        assert_eq!(std::env::var("ORBIT_SET_TEST").unwrap(), "ok");
    }
}
