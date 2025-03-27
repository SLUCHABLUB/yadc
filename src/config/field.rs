use std::ops::BitOr;
use syn::{Attribute, Error, Result};

#[derive(Clone, Default)]
pub struct FieldConfig {}

impl BitOr for FieldConfig {
    type Output = Result<FieldConfig>;

    fn bitor(self, _: Self) -> Self::Output {
        Ok(FieldConfig {})
    }
}

impl TryFrom<Attribute> for FieldConfig {
    type Error = Error;

    fn try_from(_: Attribute) -> Result<Self> {
        Ok(FieldConfig {})
    }
}

impl TryFrom<Vec<Attribute>> for FieldConfig {
    type Error = Error;

    fn try_from(attributes: Vec<Attribute>) -> Result<Self> {
        attributes
            .into_iter()
            .try_fold(FieldConfig::default(), |config, attribute| {
                config | FieldConfig::try_from(attribute)?
            })
    }
}
