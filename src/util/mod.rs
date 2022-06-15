use anyhow::bail;
use log::trace;
use std::path::Path;
use std::{fs, io};

pub mod dfcolor;
mod named_color;
pub use named_color::NamedColor;

pub fn remove_dir_if_exists<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(_) => {}
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {}
            _ => bail!(error),
        },
    }

    Ok(())
}

pub fn copy_all_from_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> anyhow::Result<()> {
    for entry in walkdir::WalkDir::new(&from) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = entry.path().strip_prefix(&from)?;
        let to_path = &to.as_ref().join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(to_path)?;
            trace!("Created directory {:?}.", to_path);
        } else {
            fs::copy(path, to_path)?;
            trace!("Copied file {:?} to {:?}.", path, to_path);
        }
    }

    Ok(())
}
