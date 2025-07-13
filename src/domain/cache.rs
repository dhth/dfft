use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct FileCache {
    pub cache: HashMap<String, Arc<str>>,
}

impl FileCache {
    pub fn insert<T>(&mut self, path: T, contents: T) -> Option<Arc<str>>
    where
        T: AsRef<str>,
    {
        let arc_contents: Arc<str> = contents.as_ref().into();
        self.cache.insert(path.as_ref().to_string(), arc_contents)
    }

    pub fn remove(&mut self, path: &str) {
        self.cache.remove(path);
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}
