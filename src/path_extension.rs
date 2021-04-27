use std::fs::{self};
use std::io::Result;
use std::path::{Path, PathBuf};

use path_absolutize::Absolutize;
use pathdiff::diff_paths;
use shellexpand::tilde;

pub trait PathExtension {
    fn is_symbolic(&self) -> bool;
    fn actually_exists(&self) -> bool;
    fn absolutize(&self) -> Result<PathBuf>;
    fn relative_to<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;
}

impl PathExtension for Path {
    fn is_symbolic(&self) -> bool {
        return if self.actually_exists() {
            match self
                .symlink_metadata()
                .map(|metadata| metadata.file_type().is_symlink())
            {
                Ok(result) => result,
                Err(_) => false,
            }
        } else {
            false
        };
    }

    fn actually_exists(&self) -> bool {
        return self.exists() || fs::read_link(&self).is_ok();
    }

    fn absolutize(&self) -> Result<PathBuf> {
        let path = tilde(self.to_str().unwrap());
        let path = Absolutize::absolutize(Path::new(path.as_ref()))?.to_path_buf();
        return Ok(path);
    }

    fn relative_to<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        let path = PathExtension::absolutize(path.as_ref()).ok()?;
        return diff_paths(&self, &path);
    }
}

impl PathExtension for PathBuf {
    fn is_symbolic(&self) -> bool {
        self.as_path().is_symbolic()
    }

    fn actually_exists(&self) -> bool {
        self.as_path().actually_exists()
    }

    fn absolutize(&self) -> Result<PathBuf> {
        PathExtension::absolutize(self.as_path())
    }

    fn relative_to<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        self.as_path().relative_to(path)
    }
}
