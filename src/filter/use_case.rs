use serde_json::Value;
use crate::extensions::{FindGrowthBookAttribute, JsonHelper};
use crate::hash::{HashCode, HashCodeVersion};
use crate::model_public::GrowthBookAttribute;
use crate::range::model::Range;

pub struct Filter;

impl Filter {
    pub fn is_filtered_out(
        filters: &Value,
        hash_attribute: &String,
        option_user_attributes: &Option<Vec<GrowthBookAttribute>>,
    ) -> bool {
        for filter in filters.force_array() {
            if let Some(user_attributes) = &option_user_attributes {
                if let Some(user_value) = user_attributes.find_value(hash_attribute) {
                    if let Some(user_weight) = HashCode::hash_code(
                        &user_value.to_string(),
                        &filter.get_string("seed"),
                        HashCodeVersion::from(filter.get("hashVersion").unwrap_or(&Value::from(2)).as_i64().expect("Failed to convert to i64")),
                    ) {
                        for array in filter.get_array("ranges") {
                            let range = Range {
                                start: array[0].force_f32(),
                                end: array[1].force_f32(),
                            };
                            if range.in_range(&user_weight) {
                                return false;
                            }
                        }
                    }
                } else {
                    return true;
                }
            } else {
                return false;
            }
        }
        return true;
    }
}