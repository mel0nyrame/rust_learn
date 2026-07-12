pub mod basetypes;
pub mod own;
pub mod structs;
pub mod matcher;
pub mod misc;
pub mod collection;

use std::fs;
use std::path::Path;

/// 扫描当前 crate 的 src 目录，返回所有可能的模块名（不包括 lib.rs 和 main.rs）
pub fn scan_modules() -> Vec<String> {
    // 获取当前项目根目录（Cargo 提供的环境变量）
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = Path::new(manifest_dir).join("src");

    let mut modules = Vec::new();
    if let Ok(entries) = fs::read_dir(&src_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        if let Some(stem) = path.file_stem() {
                            let name = stem.to_string_lossy().to_string();
                            // 跳过 lib.rs 和 main.rs（它们不是模块）
                            if name != "lib" && name != "main" {
                                modules.push(name);
                            }
                        }
                    }
                }
            }
        }
    }
    modules
}