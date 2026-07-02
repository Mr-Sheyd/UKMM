use thiserror::Error;
use crate::context::ContextData;

#[derive(Debug, Error)]
pub enum UKError {
    #[error("Parameter file missing key: {0}")]
    MissingAampKey(&'static str, Box::<Option<ContextData>>),
    #[error("Parameter file missing key: {0}")]
    MissingAampKeyD(std::string::String),
    #[error("BYML file missing key: {0}")]
    MissingBymlKey(&'static str),
    #[error("BYML file missing key: {0}")]
    MissingBymlKeyD(std::string::String),
    #[error("Wrong type for BYML value: found {0}, expected {1}")]
    WrongBymlType(std::string::String, &'static str),
    #[error("{0} missing from SARC")]
    MissingSarcFile(&'static str),
    #[error("{0} missing from SARC")]
    MissingSarcFileD(std::string::String),
    #[error("Invalid weather value: {0}")]
    InvalidWeatherOrTime(std::string::String),
    #[error("Missing resource at {0}")]
    MissingResource(std::string::String),
    #[error("{0}")]
    Other(&'static str),
    #[error("{0}")]
    OtherD(std::string::String),
    #[error(transparent)]
    _Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    RoeadError(#[from] roead::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error("Invalid BYML data for field {0}: {1:#?}")]
    InvalidByml(String, roead::byml::Byml),
    #[error("Invalid parameter data for field {0}: {1:#?}")]
    InvalidParameter(String, roead::aamp::Parameter),
}

impl UKError {
    pub fn context_data(&self) -> Option<ContextData> {
        match self {
            Self::MissingAampKey(_, data) => *data.clone(),
            Self::InvalidByml(_, data) => Some(ContextData::Byml(data.clone())),
            Self::InvalidParameter(_, data) => Some(ContextData::Parameter(data.clone())),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, UKError>;