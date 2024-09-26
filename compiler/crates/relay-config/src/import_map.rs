use std::path::MAIN_SEPARATOR;

use fnv::FnvBuildHasher;
use indexmap::IndexMap;
use intern::string_key::StringKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Debug, Clone)]
pub enum ImportDeclarationKind {
    AbsoluteSource(StringKey),
    RelativeSource(StringKey),
}

impl ImportDeclarationKind {
    pub fn key(&self) -> StringKey {
        match self {
            ImportDeclarationKind::AbsoluteSource(source) => source.clone(),
            ImportDeclarationKind::RelativeSource(source) => source.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
pub struct ImportMap {
    map: FnvIndexMap<String, String>,
}

impl ImportMap {
    pub fn new(map: Option<FnvIndexMap<String, String>>) -> Self {
        match map {
            Some(map) => Self {
                map: map
                    .iter()
                    // Remove trailing slashes from keys
                    .map(|(k, v)| (remove_trailing_slashes_from_key(k), v.clone()))
                    .collect::<FnvIndexMap<String, String>>(),
            },
            None => Self {
                map: FnvIndexMap::default(),
            },
        }
    }

    pub fn resolve_path(&self, path: &str) -> Option<String> {
        // Normalize path separators as the import map uses `/` as the separator
        let path = normalize_path(path);
        if self.map.is_empty() {
            return None;
        }

        // If this is called with a remapped path (e.g. `@1js/my-package`), we should return it as is
        if self.is_remapped_path(&path) {
            return Some(path.to_string());
        }

        let matching_key = self
            .map
            .keys()
            .filter(|key| path.starts_with(key.as_str()))
            .max_by_key(|key| key.len());

        matching_key.and_then(|matching_key| self.get_replacement_value(&path, matching_key))
    }

    fn is_remapped_path(&self, path: &str) -> bool {
        self.map.values().any(|value| path.starts_with(value))
    }

    fn get_replacement_value(&self, path: &str, matching_key: &String) -> Option<String> {
        let rewrite_value = self.map.get(matching_key)?;
        if rewrite_value.ends_with("/") {
            let rewrite_value_with_trailing_slash = rewrite_value;
            let remaining_path = self.get_remaining_path(path, matching_key);

            Some(format!(
                "{}{}",
                rewrite_value_with_trailing_slash, remaining_path
            ))
        } else {
            Some(rewrite_value.to_string())
        }
    }

    fn get_remaining_path(&self, path: &str, matching_key: &String) -> String {
        path.trim_start_matches(matching_key)
            .trim_start_matches("/")
            .to_string()
    }
}

fn normalize_path(path: &str) -> String {
    path.to_string().replace(MAIN_SEPARATOR, "/")
}

fn remove_trailing_slashes_from_key(key: &str) -> String {
    key.trim_end_matches("/").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_import_map() {
        let map = ImportMap::default();
        assert_eq!(map.map.len(), 0);
    }

    #[test]
    fn test_resolve_path() {
        let mut map = ImportMap::default();
        map.map.insert("foo".to_string(), "bar".to_string());
        assert_eq!(map.resolve_path("foo"), Some("bar".to_string()));
    }

    #[test]
    fn test_resolve_path_not_found() {
        let map = ImportMap::default();
        assert_eq!(map.resolve_path("foo"), None);
    }

    #[test]
    fn test_resolves_package() {
        let mut map = ImportMap::default();

        map.map
            .insert("my-package".to_string(), "@1js/my-package".to_string());

        assert_eq!(
            map.resolve_path("my-package/src/index.ts"),
            Some("@1js/my-package".to_string())
        );
    }

    #[test]
    fn test_resolves_package_sub_import_when_value_ends_with_slash() {
        let mut map = ImportMap::default();

        map.map
            .insert("my-package".to_string(), "@1js/my-package/".to_string());

        assert_eq!(
            map.resolve_path("my-package/src/index.ts"),
            Some("@1js/my-package/src/index.ts".to_string())
        );
    }

    #[test]
    fn test_resolves_package_sub_import_multiple_folders() {
        let mut map = ImportMap::default();

        map.map.insert(
            "my-package/src".to_string(),
            "@1js/my-package/lib/".to_string(),
        );

        assert_eq!(
            map.resolve_path("my-package/src/index.ts"),
            Some("@1js/my-package/lib/index.ts".to_string())
        );
    }

    #[test]
    fn test_resolves_the_most_specific_path_to_use() {
        let mut map = ImportMap::default();

        map.map
            .insert("my-package".to_string(), "@1js/my-package".to_string());

        map.map.insert(
            "my-package/src".to_string(),
            "@1js/my-package/lib/".to_string(),
        );

        assert_eq!(
            map.resolve_path("my-package/src/index.ts"),
            Some("@1js/my-package/lib/index.ts".to_string())
        );
    }

    #[test]
    fn test_correctly_rewrites_the_subpath() {
        let mut map = ImportMap::default();

        map.map.insert(
            "my-resolvers/src".to_string(),
            "@1js/my-resolvers/lib/".to_string(),
        );

        assert_eq!(
            map.resolve_path(
                "my-resolvers/src/__generated__/Query__viewData$normalization.graphql"
            ),
            Some(
                "@1js/my-resolvers/lib/__generated__/Query__viewData$normalization.graphql"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_correctly_rewrites_the_subpath_with_trailing_slash() {
        let mut map = ImportMap::default();

        map.map.insert(
            "my-resolvers/src/__generated__/".to_string(),
            "@1js/my-resolvers/lib/__generated__/".to_string(),
        );

        assert_eq!(
            map.resolve_path(
                "my-resolvers/src/__generated__/Query__viewData$normalization.graphql"
            ),
            Some(
                "@1js/my-resolvers/lib/__generated__/Query__viewData$normalization.graphql"
                    .to_string()
            )
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_rewrites_windows_style_paths() {
        let mut map = ImportMap::default();

        map.map
            .insert("my-package".to_string(), "@1js/my-package".to_string());

        map.map.insert(
            "my-package/src".to_string(),
            "@1js/my-package/lib/".to_string(),
        );

        assert_eq!(
            map.resolve_path("my-package\\src\\index.ts"),
            Some("@1js/my-package/lib/index.ts".to_string())
        );
    }
}
