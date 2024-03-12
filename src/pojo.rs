use std::collections::VecDeque;
use std::fs::Metadata;
use std::io::Result;
use std::path::{Path, PathBuf};
use globset::GlobMatcher;

pub struct Config {
    pub colorful: bool,
    pub show_all: bool,
    pub max_level: usize,
    pub include_glob: Option<GlobMatcher>,
}

pub struct DirSummary {
    num_folders: usize,
    num_files: usize,
}

impl DirSummary {
    pub fn default() -> DirSummary {
        DirSummary {
            num_folders: 0,
            num_files: 0,
        }
    }
}

#[derive(Debug)]
pub struct FileItem {
    pub file_name: String,
    pub path: PathBuf,
    pub metadata: Result<Metadata>,
    pub level: usize,
    pub is_last: bool,
}

impl FileItem {
    pub fn new(path: &Path, level: usize, is_last: bool) -> FileItem {
        let metadata = path.symlink_metadata();
        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .or_else(|| path.to_str())
            .unwrap_or("");
        
        FileItem {
            file_name: file_name.to_string(),
            path: path.to_owned(),
            metadata,
            level,
            is_last,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false)
    }
}
