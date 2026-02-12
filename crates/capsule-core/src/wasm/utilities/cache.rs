use std::path::Path;

pub fn generate_wasm_filename(source_path: &Path) -> String {
    let stem = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("capsule");

    let path_str = source_path.to_string_lossy();
    let hash = blake3::hash(path_str.as_bytes()).to_hex();
    let hash_str = hash.as_str();
    let short_hash = &hash_str[..8];

    format!("{stem}_{short_hash}.wasm")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_wasm_filename_basic() {
        let path = PathBuf::from("/project/main.ts");
        let filename = generate_wasm_filename(&path);
        assert!(filename.starts_with("main_"));
        assert!(filename.ends_with(".wasm"));
        assert_eq!(filename.len(), "main_".len() + 8 + ".wasm".len());
    }

    #[test]
    fn test_generate_wasm_filename_deterministic() {
        let path = PathBuf::from("/project/main.ts");
        let a = generate_wasm_filename(&path);
        let b = generate_wasm_filename(&path);
        assert_eq!(a, b);
    }

    #[test]
    fn test_generate_wasm_filename_different_dirs() {
        let a = generate_wasm_filename(&PathBuf::from("/project/src/main.ts"));
        let b = generate_wasm_filename(&PathBuf::from("/project/tasks/main.ts"));
        assert_ne!(a, b);
        assert!(a.starts_with("main_"));
        assert!(b.starts_with("main_"));
    }

    #[test]
    fn test_generate_wasm_filename_different_files() {
        let a = generate_wasm_filename(&PathBuf::from("/project/main.ts"));
        let b = generate_wasm_filename(&PathBuf::from("/project/other.ts"));
        assert_ne!(a, b);
        assert!(a.starts_with("main_"));
        assert!(b.starts_with("other_"));
    }
}
