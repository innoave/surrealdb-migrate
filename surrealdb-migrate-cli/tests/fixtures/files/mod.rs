use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn list_filenames_in_dir(dir: &Path) -> impl Iterator<Item = String> {
    fs::read_dir(dir)
        .expect("failed to list content of directory")
        .inspect(|entry| eprintln!("{entry:?}"))
        .filter_map(|entry| {
            entry.as_ref().ok().and_then(|entry| {
                entry
                    .path()
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
            })
        })
}
