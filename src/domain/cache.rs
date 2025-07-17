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

    pub fn remove_directory<P>(&mut self, dir_path: P) -> bool
    where
        P: AsRef<str>,
    {
        let dir_path_str = dir_path.as_ref().trim();
        let normalized_path = dir_path_str.replace('\\', "/");
        if normalized_path.is_empty() || normalized_path == "/" {
            return false;
        }
        let dir_prefix = if normalized_path.ends_with('/') {
            normalized_path
        } else {
            format!("{normalized_path}/")
        };
        let len_before = self.cache.len();
        self.cache.retain(|path, _| !path.starts_with(&dir_prefix));

        self.cache.len() < len_before
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
