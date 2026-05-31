use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    pub size: u64,
    pub permissions: u32,
    pub modified_unix: i64,
}

fn bytes_to_str(input: &[u8]) -> Result<&str, i32> {
    std::str::from_utf8(input).map_err(|_| libc::EINVAL)
}

pub fn stat(path: &[u8]) -> Result<FileMetadata, i32> {
    let path = bytes_to_str(path)?;
    let metadata = fs::metadata(path).map_err(|err| err.raw_os_error().unwrap_or(-1))?;

    let modified_unix = metadata
        .modified()
        .map_err(|err| err.raw_os_error().unwrap_or(-1))?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| libc::EINVAL)?
        .as_secs() as i64;

    #[cfg(unix)]
    let permissions = metadata.permissions().mode();
    #[cfg(not(unix))]
    let permissions = 0;

    Ok(FileMetadata {
        size: metadata.len(),
        permissions,
        modified_unix,
    })
}

pub fn size(path: &[u8]) -> Result<u64, i32> {
    Ok(stat(path)?.size)
}

pub fn permissions(path: &[u8]) -> Result<u32, i32> {
    Ok(stat(path)?.permissions)
}

pub fn modified_time(path: &[u8]) -> Result<i64, i32> {
    Ok(stat(path)?.modified_unix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::NamedTempFile;

    #[test]
    fn stat_returns_metadata_for_existing_file() {
        let temp = NamedTempFile::new().expect("failed to create temp file");
        let path = temp.path().to_string_lossy().into_owned();

        let metadata = stat(path.as_bytes()).expect("stat failed");
        assert!(metadata.size >= 0);
    }

    #[test]
    fn size_returns_file_length() {
        let mut temp = NamedTempFile::new().expect("failed to create temp file");
        std::io::Write::write_all(&mut temp, b"hello").expect("write failed");
        let path = temp.path().to_string_lossy().into_owned();

        assert_eq!(size(path.as_bytes()).unwrap(), 5);
    }
}
