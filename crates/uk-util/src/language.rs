use std::{fmt, path::Path, str::FromStr};
use join_str::jstr;
use lighter::lighter;

use uk_localization::LocLang;

use crate::uk_error::UKError;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    #[default]
    USen,
    EUen,
    USfr,
    USes,
    EUde,
    EUes,
    EUfr,
    EUit,
    EUnl,
    EUru,
    CNzh,
    JPja,
    KRko,
    TWzh,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl From<Language> for LocLang {
    fn from(value: Language) -> Self {
        match value {
            Language::CNzh | Language::TWzh => Self::SimpleChinese,
            Language::EUde => Self::German,
            Language::EUen | Language::USen => Self::English,
            Language::EUes | Language::USes => Self::Spanish,
            Language::EUfr | Language::USfr => Self::French,
            Language::EUit => Self::Italian,
            Language::EUnl => Self::Dutch,
            Language::EUru => Self::Russian,
            Language::JPja => Self::Japanese,
            Language::KRko => Self::Korean,
        }
    }
}

impl Language {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        [
            Self::USen,
            Self::EUen,
            Self::USfr,
            Self::USes,
            Self::EUde,
            Self::EUes,
            Self::EUfr,
            Self::EUit,
            Self::EUnl,
            Self::EUru,
            Self::CNzh,
            Self::JPja,
            Self::KRko,
            Self::TWzh,
        ]
            .iter()
    }

    #[inline(always)]
    pub fn to_str(self) -> &'static str {
        self.into()
    }

    #[inline(always)]
    pub fn short(&self) -> &'static str {
        &self.to_str()[2..4]
    }

    pub fn nearest<'l>(&self, langs: &'l [Self]) -> &'l Self {
        langs
            .iter()
            .find(|lang| *lang == self)
            .or_else(|| langs.iter().find(|lang| lang.short() == self.short()))
            .or_else(|| langs.iter().find(|lang| lang.short() == "en"))
            .or_else(|| langs.first())
            .unwrap_or(&Language::USen)
    }

    #[inline(always)]
    pub fn from_path(path: &Path) -> Option<Self> {
        path.file_stem()
            .and_then(|n| n.to_str())
            .filter(|n| n.len() >= 4)
            .and_then(|n| Self::from_str(&n[n.len() - 4..]).ok())
    }

    #[inline(always)]
    pub fn from_message_path(path: &Path) -> Option<Self> {
        path.file_stem()
            .and_then(|n| n.to_str())
            .filter(|n| n.len() >= 4)
            .and_then(|n| Self::from_str(&n[n.len() - 12..n.len() - 8]).ok())
    }

    #[inline]
    pub fn bootup_path(&self) -> smartstring::alias::String {
        let mut string = smartstring::alias::String::from("Pack/Bootup_");
        string.push_str(self.to_str());
        string.push_str(".pack");
        string
    }

    #[inline]
    pub fn message_path(&self) -> smartstring::alias::String {
        let mut string = smartstring::alias::String::from("Message/Msg_");
        string.push_str(self.to_str());
        string.push_str(".product.ssarc");
        string
    }
}

impl FromStr for Language {
    type Err = UKError;

    #[allow(clippy::needless_borrow)]
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lighter! {
            match s {
                "USen" => Ok(Language::USen),
                "EUen" => Ok(Language::EUen),
                "USfr" => Ok(Language::USfr),
                "USes" => Ok(Language::USes),
                "EUde" => Ok(Language::EUde),
                "EUes" => Ok(Language::EUes),
                "EUfr" => Ok(Language::EUfr),
                "EUit" => Ok(Language::EUit),
                "EUnl" => Ok(Language::EUnl),
                "EUru" => Ok(Language::EUru),
                "CNzh" => Ok(Language::CNzh),
                "JPja" => Ok(Language::JPja),
                "KRko" => Ok(Language::KRko),
                "TWzh" => Ok(Language::TWzh),
                _ => Err(UKError::OtherD(jstr!("Invalid language: {s}"))),
            }
        }
    }
}

impl From<Language> for &str {
    fn from(lang: Language) -> Self {
        match lang {
            Language::USen => "USen",
            Language::EUen => "EUen",
            Language::USfr => "USfr",
            Language::USes => "USes",
            Language::EUde => "EUde",
            Language::EUes => "EUes",
            Language::EUfr => "EUfr",
            Language::EUit => "EUit",
            Language::EUnl => "EUnl",
            Language::EUru => "EUru",
            Language::CNzh => "CNzh",
            Language::JPja => "JPja",
            Language::KRko => "KRko",
            Language::TWzh => "TWzh",
        }
    }
}