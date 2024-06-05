use crate::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};

pub trait FindGrowthBookAttribute {
    fn find_value(&self, attribute_key: &str) -> Option<GrowthBookAttributeValue>;
}

impl FindGrowthBookAttribute for Vec<GrowthBookAttribute> {
    fn find_value(&self, attribute_key: &str) -> Option<GrowthBookAttributeValue> {
        look_for_attribute(0, attribute_key, self).map(|it| it.value)
    }
}

impl FindGrowthBookAttribute for &[GrowthBookAttribute] {
    fn find_value(&self, attribute_key: &str) -> Option<GrowthBookAttributeValue> {
        look_for_attribute(0, attribute_key, self).map(|it| it.value)
    }
}

fn look_for_attribute(
    split_index: usize,
    attribute_key: &str,
    user_attributes: &[GrowthBookAttribute],
) -> Option<GrowthBookAttribute> {
    let key_part = attribute_key.split('.').collect::<Vec<&str>>()[split_index];
    let option_attribute = user_attributes.iter().find(|item| item.key == key_part);
    if let Some(found_attribute) = option_attribute {
        match found_attribute.value.clone() {
            GrowthBookAttributeValue::Object(it) => {
                look_for_attribute(split_index + 1, attribute_key, &it)
            }
            GrowthBookAttributeValue::Empty => None,
            _ => Some(found_attribute.clone()),
        }
    } else {
        None
    }
}
