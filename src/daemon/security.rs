#![cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
use tracing::debug;
use walkdir::WalkDir;

/// 遍历目录检查安全性
/// - 跳过软链接
/// - 文件/目录 UID 必须匹配 target_uid
/// - 目录权限：组/其他用户不可写
pub fn check_owner_and_permissions(dir: &str, target_uid: u32) -> bool {
    let mut all_match = true;

    for entry in WalkDir::new(dir).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                debug!("Failed to read entry: {}", e);
                all_match = false;
                continue;
            }
        };

        let path = entry.path();

        // 使用 symlink_metadata 避免跟随软链接
        let metadata = match std::fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => {
                debug!("Failed to get metadata for {}: {}", path.display(), e);
                all_match = false;
                continue;
            }
        };

        // 跳过软链接
        if metadata.file_type().is_symlink() {
            debug!("Skipping symlink: {}", path.display());
            continue;
        }

        // 检查 UID
        if metadata.uid() != target_uid {
            debug!(
                "File/Directory {} has owner UID={} which does not match target UID={}",
                path.display(),
                metadata.uid(),
                target_uid
            );
            all_match = false;
        }

        // 如果是目录，检查权限
        if metadata.is_dir() {
            let mode = metadata.mode();
            if mode & 0o022 != 0 {
                debug!(
                    "Directory {} has writable permissions for group/others (mode={:o})",
                    path.display(),
                    mode & 0o777
                );
                all_match = false;
            }
        }
    }

    all_match
}
