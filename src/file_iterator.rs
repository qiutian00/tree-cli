use std::{fs, io};
use std::collections::VecDeque;
use std::fs::DirEntry;
use std::path::Path;

use globset::GlobMatcher;

use crate::pojo::FileItem;

#[derive(Debug)]
pub struct FileIteratorConfig {
    pub show_hidden: bool,
    pub max_level: usize,
    pub include_glob: Option<GlobMatcher>,
}

#[derive(Debug)]
pub struct FileIterator {
    queue: VecDeque<FileItem>,
    config: FileIteratorConfig,
}

impl FileIterator {
    pub fn new(path: &Path, config: FileIteratorConfig) -> FileIterator {
        let mut queue = VecDeque::new();
        queue.push_back(FileItem::new(path, 0, true));
        FileIterator { queue, config }
    }

    fn is_glob_included(&self, file_name: &str) -> bool {
        if let Some(ref glob) = self.config.include_glob {
            glob.is_match(file_name)
        } else {
            true
        }
    }

    fn is_included(&self, name: &str, is_dir: bool) -> bool {
        if !self.config.show_hidden && name.starts_with('.') {
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
            .into_iter().collect::<io::Result<Vec<_>>>()
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
            if item.is_dir() && item.level < self.config.max_level {
                self.push_dir(&item);
            }

            Some(item)
        } else {
            None
        }
    }
}