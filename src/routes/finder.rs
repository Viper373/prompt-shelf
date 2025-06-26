use anyhow::{Ok, Result};
use std::path::{Path, PathBuf};

pub fn find_prompt(prompt_id: &str, version: &str, commit_id: &str) -> Result<PathBuf> {
    let dir = Path::new("/data").join(prompt_id).join(version);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(commit_id).with_extension("json");
    Ok(path)
}
