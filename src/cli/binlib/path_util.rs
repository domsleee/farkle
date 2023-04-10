use std::path::PathBuf;

pub fn to_abs_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(path);
    if !abs_path.is_absolute() {
        if let Ok(cwd) = std::env::current_dir() {
            abs_path = cwd.join(path);
        }
    }
    abs_path.iter().collect()
}
