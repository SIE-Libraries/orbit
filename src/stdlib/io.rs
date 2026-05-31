use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn utf8_to_path(input: &[u8]) -> Option<&Path> {
    std::str::from_utf8(input).ok().map(Path::new)
}

fn path_to_bytes(path: &Path) -> Vec<u8> {
    path.to_string_lossy().into_owned().into_bytes()
}

pub fn read(path: &[u8]) -> Result<Vec<u8>, i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    let mut contents = Vec::new();
    let mut file = fs::File::open(path).map_err(|err| err.raw_os_error().unwrap_or(-1))?;
    file.read_to_end(&mut contents)
        .map_err(|err| err.raw_os_error().unwrap_or(-1))?;
    Ok(contents)
}

pub fn write(path: &[u8], contents: &[u8]) -> Result<(), i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    let mut file = fs::File::create(path).map_err(|err| err.raw_os_error().unwrap_or(-1))?;
    file.write_all(contents)
        .map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn append(path: &[u8], contents: &[u8]) -> Result<(), i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|err| err.raw_os_error().unwrap_or(-1))?;
    file.write_all(contents)
        .map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn rm_all(path: &[u8]) -> Result<(), i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    fs::create_dir_all(path).map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn rmdir(path: &[u8]) -> Result<(), i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    fs::remove_dir(path).map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn rm(path: &[u8]) -> Result<(), i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    fs::remove_file(path).map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn rename(src: &[u8], dest: &[u8]) -> Result<(), i32> {
    let src = utf8_to_path(src).ok_or(libc::EINVAL)?;
    let dest = utf8_to_path(dest).ok_or(libc::EINVAL)?;
    fs::rename(src, dest).map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn copy(src: &[u8], dest: &[u8]) -> Result<u64, i32> {
    let src = utf8_to_path(src).ok_or(libc::EINVAL)?;
    let dest = utf8_to_path(dest).ok_or(libc::EINVAL)?;
    fs::copy(src, dest).map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn file_size(path: &[u8]) -> Result<u64, i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn read_dir(path: &[u8]) -> Result<Vec<Vec<u8>>, i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    let entries = fs::read_dir(path).map_err(|err| err.raw_os_error().unwrap_or(-1))?;
    let mut items = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| err.raw_os_error().unwrap_or(-1))?;
        items.push(path_to_bytes(&entry.path()));
    }
    Ok(items)
}

pub fn chdir(path: &[u8]) -> Result<(), i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    std::env::set_current_dir(path).map_err(|err| err.raw_os_error().unwrap_or(-1))
}

pub fn readonly(path: &[u8]) -> Result<bool, i32> {
    let path = utf8_to_path(path).ok_or(libc::EINVAL)?;
    let meta = fs::metadata(path).map_err(|err| err.raw_os_error().unwrap_or(-1))?;
    Ok(meta.permissions().readonly())
}

pub fn pwd() -> Option<Vec<u8>> {
    std::env::current_dir()
        .ok()
        .map(|path| path_to_bytes(&path))
}

pub fn temp_dir() -> Option<Vec<u8>> {
    let path = std::env::temp_dir();
    Some(path_to_bytes(&path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn read_returns_error_for_invalid_utf8_path() {
        assert_eq!(read(&[0xff, 0xff]), Err(libc::EINVAL));
    }

    #[test]
    fn write_roundtrips_file_contents() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("log.txt");
        write(path.to_string_lossy().as_bytes(), b"hello").unwrap();
        assert_eq!(
            read(path.to_string_lossy().as_bytes()).unwrap(),
            b"hello".to_vec()
        );
    }

    #[test]
    fn append_appends_contents() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("append.txt");
        append(path.to_string_lossy().as_bytes(), b"one\n").unwrap();
        append(path.to_string_lossy().as_bytes(), b"two\n").unwrap();
        let contents = read(path.to_string_lossy().as_bytes()).unwrap();
        assert!(contents.ends_with(b"two\n"));
    }

    #[test]
    fn mkdir_all_creates_nested_directories() {
        let dir = tempdir().unwrap();
        let nested = dir.path().join("a/b/c");
        mkdir_all(nested.to_string_lossy().as_bytes()).unwrap();
        assert!(nested.exists());
    }

    #[test]
    fn rmdir_removes_empty_directory() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("empty");
        mkdir_all(target.to_string_lossy().as_bytes()).unwrap();
        rmdir(target.to_string_lossy().as_bytes()).unwrap();
        assert!(!target.exists());
    }

    #[test]
    fn rm_all_removes_nested_tree() {
        let dir = tempdir().unwrap();
        let nested = dir.path().join("x/y/z");
        mkdir_all(nested.to_string_lossy().as_bytes()).unwrap();
        let file = dir.path().join("x/y/z/f.txt");
        write(file.to_string_lossy().as_bytes(), b"data").unwrap();
        rm_all(dir.path().join("x").to_string_lossy().as_bytes()).unwrap();
        assert!(!dir.path().join("x").exists());
    }

    #[test]
    fn file_size_returns_value_for_existing_file() {
        let dir = tempdir().expect("failed to create temp dir");
        let path = dir.path().join("hello.txt");
        fs::write(&path, b"hello").unwrap();

        assert_eq!(file_size(path.to_string_lossy().as_bytes()).unwrap(), 5);
    }

    #[test]
    fn read_dir_lists_children() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("one.txt")).unwrap();
        File::create(dir.path().join("two.txt")).unwrap();
        let entries = read_dir(dir.path().to_string_lossy().as_bytes()).unwrap();
        assert!(entries.iter().any(|b| b"one.txt".to_vec() == *b));
    }

    #[test]
    fn chdir_and_pwd_work() {
        let dir = tempdir().unwrap();
        let before = pwd().unwrap();
        chdir(dir.path().to_string_lossy().as_bytes()).unwrap();
        let current = pwd().unwrap();
        assert!(current != before);
    }

    #[test]
    fn readonly_detects_permissions() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("r.txt");
        write(file.to_string_lossy().as_bytes(), b"x").unwrap();
        let ro = readonly(file.to_string_lossy().as_bytes()).unwrap();
        assert!(!ro);
    }

    #[test]
    fn temp_dir_returns_temp_directory() {
        assert!(temp_dir().is_some());
    }
}
