use std::collections::VecDeque;
use crate::file_iterator::FileIterator;

use crate::pojo::{FileItem};

pub struct FilteredIterator {
    pub current: FileIterator,
    cache: VecDeque<FileItem>,
    skip: bool,
    next_item: Option<FileItem>,
}

impl FilteredIterator {
    pub fn new(iterator: FileIterator) -> Self {
        FilteredIterator {
            current: iterator,
            cache: VecDeque::new(),
            skip: false,
            next_item: None,
        }
    }

    pub fn skip_filter(&mut self) {
        self.skip = true;
    }

    /// Remove previous directories from cache that shouldn't be
    /// shown because they are empty.
    fn remove_empty_directories_from_cache(&mut self, item: &FileItem) {
        while let Some(last) = self.cache.pop_back() {
            if last.level < item.level {
                self.cache.push_back(last);
                break;
            }
        }
    }
}

impl Iterator for FilteredIterator {
    type Item = FileItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip {
            return self.current.next();
        }

        if let Some(cache_item) = self.cache.pop_front() {
            return Some(cache_item);
        }

        if let Some(next_item) = self.next_item.take() {
            return Some(next_item);
        }

        while let Some(item) = self.current.next() {
            self.remove_empty_directories_from_cache(&item);

            if item.is_dir() {
                self.cache.push_back(item)
            } else {
                // If the cache already contains a folder, start emptying cache, and
                // save the item.
                return if let Some(cache_front) = self.cache.pop_front() {
                    self.next_item = Some(item);
                    Some(cache_front)
                } else {
                    Some(item)
                };
            }
        }

        None
    }
}
