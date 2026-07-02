use roead::byml::Byml;

use uk_util::uk_error::{self, UKError};

pub trait FromByml
where
    Self: Sized,
{
    fn from_byml(byml: &Byml) -> uk_error::Result<Self>;
}

impl<T, E> FromByml for T
where
    T: for<'a> TryFrom<&'a Byml, Error = E>,
    UKError: std::convert::From<E>,
{
    #[inline(always)]
    fn from_byml(byml: &Byml) -> uk_error::Result<Self> {
        Ok(Self::try_from(byml)?)
    }
}
