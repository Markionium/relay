use std::borrow::Cow;

use fnv::FnvBuildHasher;
use indexmap::IndexMap;

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

#[derive(Debug, Default, Clone)]
pub struct ReverseImportMap {
    pub map: FnvIndexMap<String, String>,
}

impl ReverseImportMap {
    pub fn new() -> Self {
        Self {
            map: FnvIndexMap::default(),
        }
    }

    pub fn resolve_path(&self, path: String) -> Cow<'static, str> {
        let relative_path = strip_relative_part(&path);
        let parts: Vec<&str> = relative_path.split('/').collect();

        let matching_key = self
            .map
            .keys()
            .filter(|key| relative_path.starts_with(key.as_str()))
            .max_by_key(|key| key.len());

        let resolved_path: String = matching_key
            .and_then(|matching_key| {
                let matching_key_parts: Vec<&str> = matching_key.split('/').collect();
                let rest_parts = parts
                    .iter()
                    .skip(matching_key_parts.len())
                    .cloned()
                    .collect::<Vec<&str>>()
                    .join("/");

                self.map.get(matching_key).map(|f| {
                    if rest_parts.is_empty() {
                        f.to_owned()
                    } else {
                        format!("{}/{}", f, rest_parts)
                    }
                })
            })
            .unwrap_or(path);

        resolved_path.into()
    }
}

impl From<Option<FnvIndexMap<String, String>>> for ReverseImportMap {
    fn from(map: Option<FnvIndexMap<String, String>>) -> Self {
        match map {
            Some(map) => Self { map },
            None => Self::new(),
        }
    }
}

fn strip_relative_part(path: &str) -> String {
    path.split('/')
        .filter(|part| part.to_string() != "..".to_string() && part.to_string() != ".".to_string())
        .collect::<Vec<&str>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_import_map() {
        let map = ReverseImportMap::new();
        assert_eq!(map.map.len(), 0);
    }

    #[test]
    fn test_resolve_path() {
        let mut map = ReverseImportMap::new();
        map.map.insert("foo".to_string(), "bar".to_string());
        assert_eq!(map.resolve_path("foo".to_string()), "bar".to_string());
    }

    #[test]
    fn test_resolve_path_not_found() {
        let map = ReverseImportMap::new();
        assert_eq!(map.resolve_path("foo".to_string()), "foo".to_string());
    }

    #[test]
    fn test_resolves_package() {
        let mut map = ReverseImportMap::new();

        map.map
            .insert("my-package".to_string(), "@1js/my-package".to_string());

        assert_eq!(
            map.resolve_path("../../../my-package".to_string()),
            "@1js/my-package".to_string()
        );
    }

    #[test]
    fn test_resolves_package_sub_import() {
        let mut map = ReverseImportMap::new();

        map.map
            .insert("my-package".to_string(), "@1js/my-package".to_string());

        assert_eq!(
            map.resolve_path("../../../my-package/src/index.ts".to_string()),
            "@1js/my-package/src/index.ts".to_string()
        );
    }

    #[test]
    fn test_resolves_package_sub_import_multiple_folders() {
        let mut map = ReverseImportMap::new();

        map.map.insert(
            "my-package/src".to_string(),
            "@1js/my-package/lib".to_string(),
        );

        assert_eq!(
            map.resolve_path("../../../my-package/src/index.ts".to_string()),
            "@1js/my-package/lib/index.ts".to_string()
        );
    }

    #[test]
    fn test_resolves_the_most_specific_path_to_use() {
        let mut map = ReverseImportMap::new();

        map.map
            .insert("my-package".to_string(), "@1js/my-package".to_string());

        map.map.insert(
            "my-package/src".to_string(),
            "@1js/my-package/lib".to_string(),
        );

        assert_eq!(
            map.resolve_path("../../../my-package/src/index.ts".to_string()),
            "@1js/my-package/lib/index.ts".to_string()
        );
    }
}
