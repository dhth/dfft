use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct FileCache {
    cache: HashMap<String, Arc<str>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn insert<P, C>(&mut self, path: P, contents: C) -> Option<Arc<str>>
    where
        P: AsRef<str>,
        C: AsRef<str>,
    {
        let arc_contents: Arc<str> = contents.as_ref().into();
        let normalized_path = Self::normalize_path(path);
        self.cache.insert(normalized_path, arc_contents)
    }

    pub fn remove<P>(&mut self, path: P) -> Option<Arc<str>>
    where
        P: AsRef<str>,
    {
        let normalized_path = Self::normalize_path(path);
        self.cache.remove(&normalized_path)
    }

    pub fn remove_directory<P>(&mut self, dir_path: P) -> bool
    where
        P: AsRef<str>,
    {
        let dir_path_str = dir_path.as_ref().trim();
        let normalized_path = Self::normalize_path(dir_path_str);

        if normalized_path.is_empty() {
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

    fn normalize_path<P: AsRef<str>>(path: P) -> String {
        path.as_ref().replace('\\', "/")
    }

    #[cfg(test)]
    fn paths(&self) -> Vec<String> {
        let mut keys = self.cache.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn creating_empty_cache_works() {
        // GIVEN
        // WHEN
        let cache = FileCache::new();

        // THEN
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn len_reflects_cache_size_correctly() {
        // GIVEN
        let mut cache = FileCache::new();
        assert_eq!(cache.len(), 0);

        // WHEN
        // THEN
        cache.insert("file1.txt", "content");
        assert_eq!(cache.len(), 1);

        cache.insert("file2.txt", "content");
        assert_eq!(cache.len(), 2);

        cache.remove("file1.txt");
        assert_eq!(cache.len(), 1);

        cache.remove("file2.txt");
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn inserting_new_file_works() {
        // GIVEN
        let mut cache = FileCache::new();

        // WHEN
        let result = cache.insert("file.txt", "content");

        // THEN
        assert!(result.is_none());
        assert_yaml_snapshot!(cache.paths(), @"- file.txt");
    }

    #[test]
    fn inserting_file_returns_previous_content_when_overwriting() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("file.txt", "old content");

        // WHEN
        let result = cache
            .insert("file.txt", "new content")
            .expect("insert should've returned previous content");

        // THEN
        assert_eq!(result.as_ref(), "old content");
        assert_yaml_snapshot!(cache.paths(), @"- file.txt");
    }

    #[test]
    fn inserting_path_with_backslashes_normalizes_to_forward_slashes() {
        // GIVEN
        let mut cache = FileCache::new();

        // WHEN
        cache.insert("src\\main.rs", "content");
        cache.insert("src\\lib.rs", "lib content");

        // THEN
        assert_yaml_snapshot!(cache.paths(), @r"
        - src/lib.rs
        - src/main.rs
        ");
    }

    #[test]
    fn removing_existing_file_returns_content() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("file.txt", "content");

        // WHEN
        let result = cache
            .remove("file.txt")
            .expect("remove should've returned content");

        // THEN
        assert_eq!(result.as_ref(), "content");
        assert_yaml_snapshot!(cache.paths(), @"[]");
    }

    #[test]
    fn removing_nonexistent_file_returns_none() {
        // GIVEN
        let mut cache = FileCache::new();

        // WHEN
        let result = cache.remove("nonexistent.txt");

        // THEN
        assert!(result.is_none());
    }

    #[test]
    fn removing_directory_removes_all_files_in_it() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("src/main.rs", "fn main() {}");
        cache.insert("src/lib.rs", "pub mod test;");
        cache.insert("src/utils/mod.rs", "pub fn helper() {}");
        cache.insert("tests/test.rs", "#[test] fn test() {}");

        // WHEN
        let result = cache.remove_directory("src/");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @"- tests/test.rs");
    }

    #[test]
    fn removing_directory_with_exact_prefix_match_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("src/main.rs", "content");
        cache.insert("src_backup/main.rs", "backup content");
        cache.insert("some_src/main.rs", "some file");
        cache.insert("project/src/main.rs", "some file");

        // WHEN
        let result = cache.remove_directory("src/");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - project/src/main.rs
        - some_src/main.rs
        - src_backup/main.rs
        ");
    }

    #[test]
    fn removing_directory_returns_false_when_no_files_removed() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("file.txt", "content");

        // WHEN
        let result = cache.remove_directory("nonexistent/");

        // THEN
        assert!(!result);
        assert_yaml_snapshot!(cache.paths(), @"- file.txt");
    }

    #[test]
    fn removing_directory_on_empty_cache_works() {
        // GIVEN
        let mut cache = FileCache::new();

        // WHEN
        let result = cache.remove_directory("any/");

        // THEN
        assert!(!result);
        assert_yaml_snapshot!(cache.paths(), @"[]");
    }

    #[test]
    fn removing_directory_with_nested_paths_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("project/src/main.rs", "main");
        cache.insert("project/src/lib.rs", "lib");
        cache.insert("project/tests/test.rs", "test");
        cache.insert("project/Cargo.toml", "toml");
        cache.insert("other/file.rs", "other");

        // WHEN
        let result = cache.remove_directory("project/");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @"- other/file.rs");
    }

    #[test]
    fn removing_a_nested_directory_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("project/src/main.rs", "main");
        cache.insert("project/src/lib.rs", "lib");
        cache.insert("project/tests/test.rs", "test");
        cache.insert("project/Cargo.toml", "toml");
        cache.insert("other/file.rs", "other");

        // WHEN
        let result = cache.remove_directory("project/src/");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - other/file.rs
        - project/Cargo.toml
        - project/tests/test.rs
        ");
    }

    #[test]
    fn removing_directory_with_root_path_doesnt_do_anything() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("file1.txt", "content1");
        cache.insert("dir/file2.txt", "content2");

        // WHEN
        // THEN
        let result = cache.remove_directory("");
        assert!(!result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - dir/file2.txt
        - file1.txt
        ");

        let result = cache.remove_directory("/");
        assert!(!result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - dir/file2.txt
        - file1.txt
        ");
    }

    #[test]
    fn removing_directory_is_case_sensitive() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("SRC/main.rs", "content");
        cache.insert("sRc/main.rs", "content");
        cache.insert("src/main.rs", "content");

        // WHEN
        let result = cache.remove_directory("src/");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - SRC/main.rs
        - sRc/main.rs
        ");
    }

    #[test]
    fn removing_directory_matches_path_exactly() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("app/src/main.rs", "main");
        cache.insert("app/src/lib.rs", "lib");
        cache.insert("app_test/src/test.rs", "test");
        cache.insert("myapp/src/other.rs", "other");
        cache.insert("project/app/src/other.rs", "other");

        // WHEN
        let result = cache.remove_directory("app/");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - app_test/src/test.rs
        - myapp/src/other.rs
        - project/app/src/other.rs
        ");
    }

    #[test]
    fn removing_directory_without_trailing_separator_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("src/main.rs", "fn main() {}");
        cache.insert("src/lib.rs", "pub mod test;");
        cache.insert("tests/test.rs", "#[test] fn test() {}");

        // WHEN
        let result = cache.remove_directory("src");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @"- tests/test.rs");
    }

    #[test]
    fn removing_directory_without_trailing_separator_matches_path_exactly() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("app/main.rs", "main");
        cache.insert("app/lib.rs", "lib");
        cache.insert("myapp/config.rs", "config");
        cache.insert("application/config.rs", "config");
        cache.insert("project/app/config.rs", "config");

        // WHEN
        let result = cache.remove_directory("app");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @r"
        - application/config.rs
        - myapp/config.rs
        - project/app/config.rs
        ");
    }

    #[test]
    fn removing_windows_path_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("src\\main.rs", "content");

        // WHEN
        let result = cache
            .remove("src\\main.rs")
            .expect("removing should have returned content");

        // THEN
        assert_eq!(result.as_ref(), "content");
        assert_yaml_snapshot!(cache.paths(), @"[]");
    }

    #[test]
    fn removing_windows_directory_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("src\\main.rs", "main");
        cache.insert("src\\lib.rs", "lib");
        cache.insert("tests\\test.rs", "test");

        // WHEN
        let result = cache.remove_directory("src\\");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @"- tests/test.rs");
    }

    #[test]
    fn removing_windows_directory_without_trailing_separator_works() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("src\\main.rs", "main");
        cache.insert("src\\lib.rs", "lib");
        cache.insert("src_backup\\main.rs", "backup");

        // WHEN
        let result = cache.remove_directory("src");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @"- src_backup/main.rs");
    }

    #[test]
    // This should never happen, but testing regardless
    fn mixed_path_separators_work_consistently() {
        // GIVEN
        let mut cache = FileCache::new();
        cache.insert("project\\src\\main.rs", "main");
        cache.insert("project/src/lib.rs", "lib");
        cache.insert("project\\tests\\test.rs", "test");

        // WHEN
        let result = cache.remove_directory("project/src");

        // THEN
        assert!(result);
        assert_yaml_snapshot!(cache.paths(), @"- project/tests/test.rs");
    }
}
