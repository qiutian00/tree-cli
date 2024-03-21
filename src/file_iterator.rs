use std::collections::VecDeque;
use std::fs::{DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::Config;
use globset::GlobMatcher;

#[derive(Debug)]
pub struct FileItem {
    pub file_name: String,
    pub path: PathBuf,
    pub metadata: io::Result<Metadata>,
    pub level: usize,
    pub is_last: bool,
}

impl FileItem {
    pub fn new(path: &Path, level: usize, is_last: bool) -> FileItem {
        let metadata = path.symlink_metadata();
        let file_name = path
            .file_name()
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

#[derive(Debug)]
pub struct FileIterator {
    queue: VecDeque<FileItem>,
    show_hidden: bool,
    max_level: usize,
    include_glob: Option<GlobMatcher>,
}

impl FileIterator {
    pub fn new(path: &Path, config: &Config) -> FileIterator {
        let mut queue = VecDeque::new();
        queue.push_back(FileItem::new(path, 0, true));
        FileIterator {
            queue,
            max_level: config.max_level,
            show_hidden: config.show_all,
            include_glob: config.include_glob.clone(),
        }
    }

    fn is_glob_included(&self, file_name: &str) -> bool {
        if let Some(ref glob) = self.include_glob {
            glob.is_match(file_name)
        } else {
            true
        }
    }

    fn is_included(&self, name: &str, is_dir: bool) -> bool {
        if !self.show_hidden && name.starts_with('.') {
            return false;
        }
        if is_dir {
            true
        } else {
            self.is_glob_included(name)
        }
    }

    fn push_dir(&mut self, item: &FileItem) {
        let err_msg = format!(
            "Couldn't retrieve files in directory: {}",
            item.path.display()
        );
        let mut dir_entries: Vec<DirEntry> = fs::read_dir(&item.path)
            .expect(&err_msg)
            .into_iter()
            .collect::<io::Result<Vec<_>>>()
            .expect(&err_msg);
        dir_entries.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

        let mut entries: Vec<FileItem> = dir_entries
            .iter()
            .map(|e| FileItem::new(&e.path(), item.level + 1, false))
            .filter(|item| self.is_included(&item.file_name, item.is_dir()))
            .collect();

        if let Some(item) = entries.first_mut() {
            item.is_last = true;
        }

        for item in entries {
            self.queue.push_back(item);
        }
    }
}

impl Iterator for FileIterator {
    type Item = FileItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_back() {
            if item.is_dir() && item.level < self.max_level {
                self.push_dir(&item);
            }
            Some(item)
        } else {
            None
        }
    }
}
