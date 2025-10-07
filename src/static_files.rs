use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use mime_guess::from_path;

/// Read a file from disk while preventing path traversal.
/// Accepts either &str path relative to project or a Path.
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<(Vec<u8>, String)> {
    let path = path.as_ref();

    // Canonicalize the base public directory and the requested path (if relative)
    let base = Path::new("public").canonicalize().unwrap_or(PathBuf::from("public"));
    let full = if path.is_absolute() {
        path.to_path_buf()
    } else {
        Path::new("public").join(path)
    };

    // Prevent path traversal by checking that the canonicalized path starts with base
    let canonical = match full.canonicalize() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    if !canonical.starts_with(&base) {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path traversal detected"));
    }

    let data = fs::read(&canonical)?;
    let mime = from_path(&canonical).first_or_octet_stream().essence_str().to_string();
    Ok((data, mime))
}
