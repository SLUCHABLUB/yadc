use std::ops::BitOr;
use syn::{Attribute, Error, Result};

#[derive(Default)]
pub struct TypeConfig {}

impl BitOr for TypeConfig {
    type Output = Result<TypeConfig>;

    fn bitor(self, _: Self) -> Self::Output {
        Ok(TypeConfig {})
    }
}

impl TryFrom<Attribute> for TypeConfig {
    type Error = Error;

    fn try_from(_: Attribute) -> Result<Self> {
        Ok(TypeConfig {})
    }
}

impl TryFrom<Vec<Attribute>> for TypeConfig {
    type Error = Error;

    fn try_from(attributes: Vec<Attribute>) -> Result<Self> {
        attributes
            .into_iter()
            .try_fold(TypeConfig::default(), |config, attribute| {
                config | TypeConfig::try_from(attribute)?
            })
    }
}
