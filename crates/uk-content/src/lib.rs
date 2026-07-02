#![allow(clippy::derive_partial_eq_without_eq, unstable_name_collisions)]
#![deny(clippy::unwrap_used)]
use std::path::Path;

use smartstring::alias::String;

use uk_util::endianness::Endian;

pub mod actor;
pub mod chemical;
pub mod constants;
pub mod cooking;
pub mod data;
pub mod demo;
pub mod eco;
pub mod event;
pub mod font;
pub mod layout;
pub mod map;
pub mod message;
pub mod quest;
pub mod resource;
pub mod sound;
pub mod tips;
pub mod util;
pub mod worldmgr;

pub type Assets = util::DeleteMap<String, Vec<u8>>;

pub const fn platform_content(endian: Endian) -> &'static str {
    match endian {
        Endian::Little => "01007EF00011E000/romfs",
        Endian::Big => "content",
    }
}

pub const fn platform_aoc(endian: Endian) -> &'static str {
    match endian {
        Endian::Little => "01007EF00011F001/romfs",
        Endian::Big => "aoc/0010",
    }
}

pub const fn platform_prefixes(endian: Endian) -> (&'static str, &'static str) {
    match endian {
        Endian::Little => ("01007EF00011E000/romfs", "01007EF00011F001/romfs"),
        Endian::Big => ("content", "aoc/0010"),
    }
}

pub fn canonicalize(path: impl AsRef<Path>) -> String {
    fn canonicalize(path: &Path) -> String {
        let path = path.to_str().unwrap_or("INVALID_FILENAME");
        let mut canon = path.replace('\\', "/");
        for (k, v) in [
            ("Content/", ""),
            ("content/", ""),
            ("atmosphere/titles/", ""),
            ("atmosphere/contents/", ""),
            ("01007EF00011E000/romfs/", ""),
            ("01007ef00011e000/romfs/", ""),
            ("01007EF00011E001/romfs", "Aoc/0010"),
            ("01007EF00011E002/romfs", "Aoc/0010"),
            ("01007EF00011F001/romfs", "Aoc/0010"),
            ("01007EF00011F002/romfs", "Aoc/0010"),
            ("01007ef00011e001/romfs", "Aoc/0010"),
            ("01007ef00011e002/romfs", "Aoc/0010"),
            ("01007ef00011f001/romfs", "Aoc/0010"),
            ("01007ef00011f002/romfs", "Aoc/0010"),
            ("romfs/", ""),
            ("aoc/content", "Aoc"),
            ("aoc", "Aoc"),
        ]
        .into_iter()
        {
            if canon.starts_with(k) {
                canon = [v, canon.trim_start_matches(k)].concat();
            }
        }
        canon.replace(".s", ".").into()
    }
    canonicalize(path.as_ref())
}

pub mod prelude {
    pub(crate) use smartstring::alias::String;
    pub type Endian = uk_util::endianness::Endian;
    pub type String32 = roead::types::FixedSafeString<32>;
    pub type String64 = roead::types::FixedSafeString<64>;
    pub type String256 = roead::types::FixedSafeString<256>;

    pub trait Mergeable {
        #[must_use]
        fn diff(&self, other: &Self) -> Self;
        #[must_use]
        fn merge(&self, diff: &Self) -> Self;
    }

    macro_rules! impl_simple_aamp {
        ($type:ty, $field:tt) => {
            impl Mergeable for $type {
                fn diff(&self, other: &Self) -> Self {
                    Self(ParameterIO {
                        param_root: crate::util::diff_plist(
                            &self.$field.param_root,
                            &other.$field.param_root,
                        ),
                        version:    self.$field.version,
                        data_type:  self.$field.data_type.clone(),
                    })
                }

                fn merge(&self, diff: &Self) -> Self {
                    Self(ParameterIO {
                        data_type:  self.$field.data_type.clone(),
                        version:    self.$field.version,
                        param_root: crate::util::merge_plist(
                            &self.$field.param_root,
                            &diff.$field.param_root,
                        ),
                    })
                }
            }
        };
    }

    impl Mergeable for roead::aamp::ParameterIO {
        fn diff(&self, other: &Self) -> Self {
            Self {
                data_type:  self.data_type.clone(),
                version:    self.version,
                param_root: crate::util::diff_plist(&self.param_root, &other.param_root),
            }
        }

        fn merge(&self, diff: &Self) -> Self {
            Self {
                data_type:  self.data_type.clone(),
                version:    self.version,
                param_root: crate::util::merge_plist(&self.param_root, &diff.param_root),
            }
        }
    }

    pub(crate) use impl_simple_aamp;

    macro_rules! impl_simple_byml {
        ($type:ty, $field:tt) => {
            impl Mergeable for $type {
                fn diff(&self, other: &Self) -> Self {
                    crate::util::diff_byml_shallow(&self.$field, &other.$field).into()
                }

                fn merge(&self, diff: &Self) -> Self {
                    crate::util::merge_byml_shallow(&self.$field, &diff.$field).into()
                }
            }
        };
    }

    impl Mergeable for roead::byml::Byml {
        fn diff(&self, other: &Self) -> Self {
            crate::util::diff_byml_shallow(self, other)
        }

        fn merge(&self, diff: &Self) -> Self {
            crate::util::merge_byml_shallow(self, diff)
        }
    }

    pub(crate) use impl_simple_byml;

    pub trait Resource where Self: Sized,
    {
        fn from_binary(data: impl AsRef<[u8]>) -> uk_util::uk_error::Result<Self>;
        fn into_binary(self, endian: Endian) -> Vec<u8>;
        fn path_matches(path: impl AsRef<std::path::Path>) -> bool;
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
pub(crate) mod tests {
    use join_str::jstr;

    use crate::canonicalize;

    pub fn test_base_actorpack(name: &str) -> roead::sarc::Sarc<'static> {
        roead::sarc::Sarc::new(
            roead::yaz0::decompress(
                std::fs::read(jstr!("test/Actor/Pack/{name}.sbactorpack")).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    pub fn test_mod_actorpack(name: &str) -> roead::sarc::Sarc<'static> {
        roead::sarc::Sarc::new(
            roead::yaz0::decompress(
                std::fs::read(jstr!("test/Actor/Pack/{name}_Mod.sbactorpack")).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn canon_names() {
        assert_eq!(
            &canonicalize("content\\Actor\\Pack\\Enemy_Lizal_Senior.sbactorpack"),
            "Actor/Pack/Enemy_Lizal_Senior.bactorpack"
        );
        assert_eq!(
            &canonicalize("aoc/0010/Map/MainField/A-1/A-1_Dynamic.smubin"),
            "Aoc/0010/Map/MainField/A-1/A-1_Dynamic.mubin"
        );
        assert_eq!(
            &canonicalize(
                "atmosphere/contents/01007EF00011E000/romfs/Actor/ActorInfo.product.sbyml"
            ),
            "Actor/ActorInfo.product.byml"
        );
        assert_eq!(
            &canonicalize("atmosphere/contents/01007EF00011F001/romfs/Pack/AocMainField.pack"),
            "Aoc/0010/Pack/AocMainField.pack"
        );
        assert_eq!(
            &canonicalize("Hellow/Sweetie.tardis"),
            "Hellow/Sweetie.tardis"
        );
        assert_eq!(
            &canonicalize("Event/EventInfo.product.sbyml"),
            "Event/EventInfo.product.byml"
        )
    }
}
