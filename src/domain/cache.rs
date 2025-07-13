use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct FileCache {
    cache: HashMap<String, Arc<str>>,
}

impl FileCache {
    pub fn insert<P, C>(&mut self, path: P, contents: C) -> Option<Arc<str>>
    where
        P: AsRef<str>,
        C: AsRef<str>,
    {
        let arc_contents: Arc<str> = contents.as_ref().into();
        self.cache.insert(path.as_ref().to_string(), arc_contents)
    }

    pub fn remove<P>(&mut self, path: P) -> Option<Arc<str>>
    where
        P: AsRef<str>,
    {
        self.cache.remove(path.as_ref())
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
