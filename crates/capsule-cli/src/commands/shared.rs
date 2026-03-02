use std::path::Path;

pub fn load_env_variables(project_root: &Path) -> Result<(), String> {
    let env_files = [".env", ".env.local", ".env.development", ".env.production"];

    for file_name in env_files {
        let file_path = project_root.join(file_name);

        if file_path.exists() {
            dotenvy::from_path(&file_path)
                .map_err(|e| format!("Failed to load {}: {}", file_path.display(), e))?;
        }
    }

    Ok(())
}
