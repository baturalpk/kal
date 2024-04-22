use std::{fs, path::Path};

use anyhow::{anyhow, Ok};

pub fn check_folder(path: &str) -> anyhow::Result<()> {
    let folder = Path::new(path);
    if !folder.is_dir() {
        return Err(anyhow!("the provided backup path is not a directory"));
    }

    let metadata = fs::metadata(path)?;
    if metadata.permissions().readonly() {
        return Err(anyhow!("the provided backup path is readonly"));
    }

    Ok(())
}

pub fn take_backup(source_file: &str, target_folder: &str) -> anyhow::Result<()> {
    let db_file_path = Path::new(source_file);
    let date_suffix = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_file_path =
        Path::new(target_folder).join(format!("kal_backup-[{}].db", date_suffix));

    fs::copy(db_file_path, backup_file_path)?;

    Ok(())
}
