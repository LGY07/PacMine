use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::Path;
use zip::read::ZipArchive;
use tree_magic_mini;

#[derive(Debug)]
pub struct JarInfo {
    pub main_class: String,
    pub java_version: u16, // 映射后的 Java 版本
}

#[derive(Debug)]
pub enum JarError {
    NotJar,
    NoMainClass,
    ClassNotFound,
    IoError(Error),
}

impl From<Error> for JarError {
    fn from(err: Error) -> Self {
        JarError::IoError(err)
    }
}


/// 根据文件路径获取 MIME 类型（路径传入 &str）
pub fn get_mime_type(path: &str) -> Result<String, ()> {
    // 使用 tree_magic_mini 检测 MIME 类型
    match tree_magic_mini::from_filepath(Path::new(path)) {
        None => Err(()),
        Some(v) => Ok(v.to_string())
    }
}

/// 分析 JAR 文件，获取 Main-Class 和 Java 版本（直接 major_version - 45）
pub fn analyze_jar(jar_path: &str) -> Result<JarInfo, JarError> {
    let file = File::open(jar_path).map_err(|_| JarError::NotJar)?;
    let mut archive = ZipArchive::new(&file).map_err(|_| JarError::NotJar)?;

    // 读取 META-INF/MANIFEST.MF
    let mut manifest_file = archive
        .by_name("META-INF/MANIFEST.MF")
        .map_err(|_| JarError::NoMainClass)?;
    let mut manifest_content = String::new();
    manifest_file.read_to_string(&mut manifest_content)?;

    // 解析 Main-Class
    let main_class = manifest_content
        .lines()
        .find_map(|line| {
            if line.starts_with("Main-Class:") {
                Some(line["Main-Class:".len()..].trim().to_string())
            } else {
                None
            }
        })
        .ok_or(JarError::NoMainClass)?;

    let mut archive = ZipArchive::new(&file).map_err(|_| JarError::NotJar)?;
    // Main-Class 转 class 文件路径
    let class_path = format!("{}.class", main_class.replace('.', "/"));
    let mut class_file = archive
        .by_name(&class_path)
        .map_err(|_| JarError::ClassNotFound)?;

    let mut class_header = [0u8; 8];
    class_file.read_exact(&mut class_header)?;

    // 检查魔术字
    if &class_header[0..4] != &[0xCA, 0xFE, 0xBA, 0xBE] {
        return Err(JarError::NotJar);
    }

    // major version → Java 版本：直接减 45
    let major_version = u16::from_be_bytes([class_header[6], class_header[7]]);
    let java_version = major_version.checked_sub(44).ok_or(JarError::NotJar)?;

    Ok(JarInfo {
        main_class,
        java_version,
    })
}
