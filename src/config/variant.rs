use std::ops::BitOr;
use syn::{Attribute, Error, Result};

#[derive(Default)]
pub struct VariantConfig {}

impl BitOr for VariantConfig {
    type Output = Result<VariantConfig>;

    fn bitor(self, _: Self) -> Self::Output {
        Ok(VariantConfig {})
    }
}

impl TryFrom<Attribute> for VariantConfig {
    type Error = Error;

    fn try_from(_: Attribute) -> Result<Self> {
        Ok(VariantConfig {})
    }
}

impl TryFrom<Vec<Attribute>> for VariantConfig {
    type Error = Error;

    fn try_from(attributes: Vec<Attribute>) -> Result<Self> {
        attributes
            .into_iter()
            .try_fold(VariantConfig::default(), |config, attribute| {
                config | VariantConfig::try_from(attribute)?
            })
    }
}
