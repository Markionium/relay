use std::{
    fmt::{Display, Formatter, Result},
    path::MAIN_SEPARATOR,
};

use fnv::FnvBuildHasher;
use indexmap::IndexMap;
use intern::{
    string_key::{Intern, StringKey},
    Lookup,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::JsModuleFormat;

pub type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImportModulePath {
    // Original path which is not remapped by the import map
    OriginalPath(StringKey),
    // Meta path for haste modules
    HasteModule(StringKey),
    // Remapped path by the import map, pointing to a file
    MappedFile(StringKey),
    // Remapped path by the import map, pointing to a package (no file extension like .graphql)
    MappedPackage(StringKey),
}

impl ImportModulePath {
    pub fn new(key: StringKey, module_format: JsModuleFormat) -> Self {
        match module_format {
            JsModuleFormat::CommonJS => ImportModulePath::OriginalPath(key),
            JsModuleFormat::Haste => ImportModulePath::HasteModule(key),
        }
    }
}

impl Display for ImportModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.lookup())
    }
}

impl Lookup for ImportModulePath {
    fn lookup(self) -> &'static str {
        match self {
            ImportModulePath::OriginalPath(value) => value.lookup(),
            ImportModulePath::HasteModule(value) => value.lookup(),
            ImportModulePath::MappedFile(value) => value.lookup(),
            ImportModulePath::MappedPackage(value) => value.lookup(),
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

    pub fn resolve_path(&self, path: &str) -> Option<ImportModulePath> {
        // Normalize path separators as the import map uses `/` as the separator
        let path = normalize_path(path);
        if self.map.is_empty() {
            return None;
        }

        let matching_key = self
            .map
            .keys()
            .filter(|key| path.starts_with(key.as_str()))
            .max_by_key(|key| key.len());

        matching_key.and_then(|matching_key| self.get_replacement_value(&path, matching_key))
    }

    fn get_replacement_value(&self, path: &str, matching_key: &String) -> Option<ImportModulePath> {
        let rewrite_value = self.map.get(matching_key)?;
        if rewrite_value.ends_with("/") {
            let rewrite_value_with_trailing_slash = rewrite_value;
            let remaining_path = self.get_remaining_path(path, matching_key);

            Some(ImportModulePath::MappedFile(
                format!("{}{}", rewrite_value_with_trailing_slash, remaining_path).intern(),
            ))
        } else {
            Some(ImportModulePath::MappedPackage(rewrite_value.intern()))
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
        assert_eq!(
            map.resolve_path("foo"),
            Some(ImportModulePath::MappedPackage("bar".intern()))
        );
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
            Some(ImportModulePath::MappedPackage("@1js/my-package".intern()))
        );
    }

    #[test]
    fn test_resolves_package_sub_import_when_value_ends_with_slash() {
        let mut map = ImportMap::default();

        map.map
            .insert("my-package".to_string(), "@1js/my-package/".to_string());

        assert_eq!(
            map.resolve_path("my-package/src/index.ts"),
            Some(ImportModulePath::MappedFile(
                "@1js/my-package/src/index.ts".intern()
            ))
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
            Some(ImportModulePath::MappedFile(
                "@1js/my-package/lib/index.ts".intern()
            ))
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
            Some(ImportModulePath::MappedFile(
                "@1js/my-package/lib/index.ts".intern()
            ))
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
            Some(ImportModulePath::MappedFile(
                "@1js/my-resolvers/lib/__generated__/Query__viewData$normalization.graphql"
                    .intern()
            ))
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
            Some(ImportModulePath::MappedFile(
                "@1js/my-resolvers/lib/__generated__/Query__viewData$normalization.graphql"
                    .intern()
            ))
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
