use std::path::{Component, Path, PathBuf};

fn utf8_to_path(input: &[u8]) -> Option<&Path> {
    std::str::from_utf8(input).ok().map(Path::new)
}

fn path_to_bytes(path: &Path) -> Vec<u8> {
    path.to_string_lossy().into_owned().into_bytes()
}

pub fn join(base: &[u8], relative: &[u8]) -> Vec<u8> {
    let base_path = utf8_to_path(base).unwrap_or_else(|| Path::new(""));
    let relative_path = utf8_to_path(relative).unwrap_or_else(|| Path::new(""));
    let joined = base_path.join(relative_path);
    path_to_bytes(&joined)
}

pub fn extension(path: &[u8]) -> Option<Vec<u8>> {
    let path = utf8_to_path(path)?;
    path.extension()
        .map(|ext| ext.to_string_lossy().into_owned().into_bytes())
}

pub fn exists(path: &[u8]) -> bool {
    utf8_to_path(path)
        .map(|path| path.exists())
        .unwrap_or(false)
}

pub fn normalize(path: &[u8]) -> Vec<u8> {
    let source = utf8_to_path(path).unwrap_or_else(|| Path::new(""));
    let mut normalized = PathBuf::new();

    if source.is_absolute() {
        normalized.push("/");
    }

    for component in source.components() {
        match component {
            Component::Prefix(prefix) => {
                normalized.push(prefix.as_os_str());
            }
            Component::RootDir => {
                // Root already handled above for Unix paths.
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if normalized.components().next_back().is_some() {
                    normalized.pop();
                } else {
                    normalized.push("..");
                }
            }
            Component::Normal(part) => {
                normalized.push(part);
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        normalized.push(".");
    }

    path_to_bytes(&normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_combines_paths() {
        assert_eq!(join(b"/tmp", b"app/log.txt"), b"/tmp/app/log.txt".to_vec());
    }

    #[test]
    fn extension_returns_file_extension() {
        assert_eq!(extension(b"/tmp/app.log"), Some(b"log".to_vec()));
    }

    #[test]
    fn exists_returns_false_for_invalid_utf8() {
        assert!(!exists(&[0xff, 0xff]));
    }

    #[test]
    fn normalize_simplifies_paths() {
        assert_eq!(normalize(b"/tmp/../var/./log"), b"/var/log".to_vec());
    }
}
