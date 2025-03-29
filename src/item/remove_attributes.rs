use syn::{Attribute, Item, Meta};

pub fn remove_attributes(item: &mut Item) {
    match item {
        Item::Enum(item) => {
            item.attrs.retain(retain_attribute);

            for variant in &mut item.variants {
                variant.attrs.retain(retain_attribute);

                for field in &mut variant.fields {
                    field.attrs.retain(retain_attribute);
                }
            }
        }
        Item::Struct(item) => {
            item.attrs.retain(retain_attribute);

            for field in &mut item.fields {
                field.attrs.retain(retain_attribute);
            }
        }
        _ => (),
    }
}

fn retain_attribute(attribute: &Attribute) -> bool {
    retain_meta(&attribute.meta)
}

fn retain_meta(meta: &Meta) -> bool {
    if meta.path().leading_colon.is_some() {
        return true;
    }

    let Some(first) = meta.path().segments.first() else {
        return true;
    };

    // FIXME
    if first.ident != "debug" && first.ident != "hash" {
        return true;
    }

    false
}
