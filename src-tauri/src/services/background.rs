use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::services::storage::StoragePaths;

const MAX_BACKGROUND_SIZE: u64 = 15 * 1024 * 1024;

pub fn import_custom_background(paths: &StoragePaths, source: &Path) -> Result<PathBuf, String> {
    let size = fs::metadata(source)
        .map_err(|e| format!("读取图片信息失败：{e}"))?
        .len();
    if size == 0 || size > MAX_BACKGROUND_SIZE {
        return Err("图片大小必须在 1 字节到 15 MB 之间".into());
    }

    let bytes = fs::read(source).map_err(|e| format!("读取背景图片失败：{e}"))?;
    if bytes.is_empty() || bytes.len() as u64 > MAX_BACKGROUND_SIZE {
        return Err("图片大小必须在 1 字节到 15 MB 之间".into());
    }
    let extension = detect_extension(&bytes).ok_or("仅支持 PNG、JPG 和 WebP 图片")?;
    let directory = paths.data().join("backgrounds");
    fs::create_dir_all(&directory).map_err(|e| format!("创建背景图片目录失败：{e}"))?;

    let destination = directory.join(format!("custom.{extension}"));
    let temporary = directory.join(format!("custom.{extension}.tmp"));
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)
        .map_err(|e| format!("创建背景图片文件失败：{e}"))?;
    if let Err(error) = file.write_all(&bytes).and_then(|()| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(&temporary);
        return Err(format!("保存背景图片失败：{error}"));
    }
    drop(file);
    fs::rename(&temporary, &destination).map_err(|e| {
        let _ = fs::remove_file(&temporary);
        format!("更新背景图片失败：{e}")
    })?;

    for stale_extension in ["png", "jpg", "webp"] {
        if stale_extension != extension {
            let stale = directory.join(format!("custom.{stale_extension}"));
            let _ = fs::remove_file(stale);
        }
    }

    Ok(destination)
}

pub fn find_custom_background(paths: &StoragePaths) -> Option<PathBuf> {
    ["png", "jpg", "webp"]
        .into_iter()
        .map(|extension| {
            paths
                .data()
                .join("backgrounds")
                .join(format!("custom.{extension}"))
        })
        .filter(|path| path.is_file())
        .max_by_key(|path| {
            fs::metadata(path)
                .and_then(|metadata| metadata.modified())
                .unwrap_or(std::time::UNIX_EPOCH)
        })
}

fn detect_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if bytes.starts_with(b"\xff\xd8\xff") {
        Some("jpg")
    } else if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_supported_image_and_replaces_previous_format() {
        let root = std::env::temp_dir().join(format!(
            "frpcpilot-background-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let paths = StoragePaths { root: root.clone() };
        let source = root.join("source-image.bin");
        let png = b"\x89PNG\r\n\x1a\nimage-data";
        fs::write(&source, png).unwrap();

        let saved = import_custom_background(&paths, &source).unwrap();

        assert_eq!(saved, paths.data().join("backgrounds/custom.png"));
        assert_eq!(fs::read(&saved).unwrap(), png);
        assert_eq!(find_custom_background(&paths), Some(saved.clone()));

        let jpeg = b"\xff\xd8\xffimage-data";
        fs::write(&source, jpeg).unwrap();
        let replaced = import_custom_background(&paths, &source).unwrap();
        assert_eq!(replaced, paths.data().join("backgrounds/custom.jpg"));
        assert!(!saved.exists());
        assert_eq!(fs::read(&replaced).unwrap(), jpeg);
        assert_eq!(find_custom_background(&paths), Some(replaced));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_non_image_files() {
        let root = std::env::temp_dir().join(format!(
            "frpcpilot-background-invalid-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let paths = StoragePaths { root: root.clone() };
        let source = root.join("not-an-image");
        fs::write(&source, b"not an image").unwrap();

        assert!(import_custom_background(&paths, &source).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
