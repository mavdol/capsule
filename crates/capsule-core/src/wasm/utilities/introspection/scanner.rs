use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::wasm::utilities::introspection::javascript::extract_js_task_configs;
use crate::wasm::utilities::introspection::python::extract_python_task_configs;

const IGNORED_DIRS: &[&str] = &[
    "node_modules",
    "__pycache__",
    ".git",
    ".capsule",
    "dist",
    "build",
    ".venv",
    "venv",
    "target",
];

pub fn scan_python_tasks(source_dir: &Path) -> Option<HashMap<String, serde_json::Value>> {
    let files = collect_source_files(source_dir, &["py"]);

    if files.is_empty() {
        return None;
    }

    let all_tasks = files
        .par_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(path).ok()?;
            extract_python_task_configs(&source)
        })
        .reduce(HashMap::new, |mut acc, tasks| {
            acc.extend(tasks);
            acc
        });

    if all_tasks.is_empty() {
        None
    } else {
        Some(all_tasks)
    }
}

pub fn scan_js_tasks(source_dir: &Path) -> Option<HashMap<String, serde_json::Value>> {
    let files = collect_source_files(source_dir, &["js", "ts", "mjs", "mts"]);

    if files.is_empty() {
        return None;
    }

    let all_tasks = files
        .par_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(path).ok()?;
            let is_typescript = path
                .extension()
                .is_some_and(|ext| ext == "ts" || ext == "mts");
            extract_js_task_configs(&source, is_typescript)
        })
        .reduce(HashMap::new, |mut acc, tasks| {
            acc.extend(tasks);
            acc
        });

    if all_tasks.is_empty() {
        None
    } else {
        Some(all_tasks)
    }
}

fn collect_source_files(source_dir: &Path, extensions: &[&str]) -> Vec<std::path::PathBuf> {
    WalkDir::new(source_dir)
        .into_iter()
        .filter_entry(|entry| {
            if entry.file_type().is_dir() {
                let name = entry.file_name().to_string_lossy();
                return !IGNORED_DIRS.iter().any(|ignored| name == *ignored);
            }
            true
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if !entry.file_type().is_file() {
                return false;
            }
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| extensions.contains(&ext))
        })
        .map(|entry| entry.into_path())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("capsule_test_{}", nanoid::nanoid!(8)));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_scan_python_multiple_files() {
        let temp_dir = create_temp_dir();

        fs::write(
            temp_dir.join("main.py"),
            r#"
@task(name="main")
def main():
    pass
"#,
        )
        .unwrap();

        let tasks_dir = temp_dir.join("tasks");
        fs::create_dir_all(&tasks_dir).unwrap();

        fs::write(
            tasks_dir.join("compute.py"),
            r#"
@task(name="heavy_compute", compute="HIGH", ram="HIGH")
def heavy_compute():
    pass
"#,
        )
        .unwrap();

        fs::write(
            tasks_dir.join("io_tasks.py"),
            r#"
@task(name="fetch_data", timeout="30s")
def fetch_data():
    pass

@task(name="save_data", allowed_files=["./data"])
def save_data():
    pass
"#,
        )
        .unwrap();

        let tasks = scan_python_tasks(&temp_dir).unwrap();

        assert_eq!(tasks.len(), 4);
        assert!(tasks.contains_key("main"));
        assert!(tasks.contains_key("heavy_compute"));
        assert!(tasks.contains_key("fetch_data"));
        assert!(tasks.contains_key("save_data"));

        assert_eq!(tasks["heavy_compute"]["compute"], "HIGH");
        assert_eq!(tasks["fetch_data"]["timeout"], "30s");
        assert_eq!(
            tasks["save_data"]["allowed_files"],
            serde_json::json!(["./data"])
        );

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_scan_js_typescript_files() {
        let temp_dir = create_temp_dir();

        fs::write(
            temp_dir.join("main.ts"),
            r#"
export const main = task({ name: "main", compute: "HIGH" }, (): string => {
    return "hello";
});
"#,
        )
        .unwrap();

        fs::write(
            temp_dir.join("utils.js"),
            r#"
const helper = task({ name: "helper" }, () => {
    return 42;
});
"#,
        )
        .unwrap();

        let tasks = scan_js_tasks(&temp_dir).unwrap();

        assert_eq!(tasks.len(), 2);
        assert!(tasks.contains_key("main"));
        assert!(tasks.contains_key("helper"));
        assert_eq!(tasks["main"]["compute"], "HIGH");

        fs::remove_dir_all(&temp_dir).ok();
    }
}
