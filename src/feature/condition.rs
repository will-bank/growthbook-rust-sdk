use regex::Regex;

use crate::extensions::FindGrowthBookAttribute;
use crate::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};

pub trait ConditionEnabledCheck {
    fn is_on(&self, user_attributes: &[GrowthBookAttribute]) -> bool;
}

impl ConditionEnabledCheck for Vec<GrowthBookAttribute> {
    fn is_on(&self, user_attributes: &[GrowthBookAttribute]) -> bool {
        self.iter()
            .all(|feature_attribute| is_on(None, feature_attribute, user_attributes, false))
    }
}

fn is_on(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
) -> bool {
    match feature_attribute.key.as_str() {
        "$not" => not_condition(parent_attribute, feature_attribute, user_attributes),
        "$ne" => ne_condition(parent_attribute, feature_attribute, user_attributes),
        "$and" => and_condition(parent_attribute, feature_attribute, user_attributes),
        "$nor" => nor_condition(parent_attribute, feature_attribute, user_attributes),
        "$or" => or_condition(parent_attribute, feature_attribute, user_attributes),
        "$in" => in_condition(parent_attribute, feature_attribute, user_attributes),
        "$nin" => nin_condition(parent_attribute, feature_attribute, user_attributes),
        "$gt" => gt_condition(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
        ),
        "$gte" => gte_condition(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
        ),
        "$lt" => lt_condition(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
        ),
        "$lte" => lte_condition(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
        ),
        "$eq" => eq_condition(parent_attribute, feature_attribute, user_attributes),
        "$exists" => exists_condition(parent_attribute, feature_attribute, user_attributes),
        "$regex" => regex_condition(parent_attribute, feature_attribute, user_attributes),
        "$type" => type_condition(parent_attribute, feature_attribute, user_attributes),
        "$size" => size_condition(parent_attribute, feature_attribute, user_attributes),
        "$all" => all_condition(parent_attribute, feature_attribute, user_attributes),
        "$vgt" => vgt_condition(parent_attribute, feature_attribute, user_attributes),
        "$vgte" => vgte_condition(parent_attribute, feature_attribute, user_attributes),
        "$vlt" => vlt_condition(parent_attribute, feature_attribute, user_attributes),
        "$vlte" => vlte_condition(parent_attribute, feature_attribute, user_attributes),
        "$veq" => veq_condition(parent_attribute, feature_attribute, user_attributes),
        "$vne" => vne_condition(parent_attribute, feature_attribute, user_attributes),
        "$elemMatch" => elem_match_condition(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
        ),
        _ => match &feature_attribute.value {
            GrowthBookAttributeValue::String(_) => {
                if feature_attribute.key.starts_with('$') {
                    println!("is unknown operator={:?}", &feature_attribute.key);
                    false
                } else {
                    println!("is string={:?}", &feature_attribute.key);
                    eq_condition(parent_attribute, feature_attribute, user_attributes)
                }
            }
            GrowthBookAttributeValue::Array(feature_values) => {
                let a = if let Some(GrowthBookAttributeValue::Array(user_values)) =
                    user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
                {
                    if feature_values.len() == user_values.len() {
                        feature_values
                            .iter()
                            .enumerate()
                            .all(|(index, value)| value == &user_values[index])
                    } else {
                        false
                    }
                } else {
                    false
                };
                println!("is array={:?}", &feature_attribute.key);
                a
            }
            GrowthBookAttributeValue::Object(it) => {
                println!("is object={:?}", &feature_attribute.key);
                if it.is_empty() {
                    user_attributes
                        .find_value(&parent_attribute.unwrap_or(feature_attribute).key)
                        .is_none()
                } else {
                    it.iter().all(|next| {
                        let parent = feature_attribute.aggregate_key(parent_attribute);
                        is_on(Some(&parent), next, user_attributes, false)
                    })
                }
            }
            GrowthBookAttributeValue::Empty => {
                println!("empty value={:?}", &feature_attribute.key);
                if let Some(it) =
                    user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
                {
                    it == GrowthBookAttributeValue::Empty
                } else {
                    true
                }
            }
            it => {
                if let Some(user_value) =
                    user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
                {
                    it == &user_value
                } else {
                    false
                }
            }
        },
    }
}

impl GrowthBookAttribute {
    fn aggregate_key(&self, parent_attribute: Option<&GrowthBookAttribute>) -> Self {
        let key = parent_attribute
            .map(|parent| format!("{}.{}", parent.key, self.key))
            .unwrap_or(self.key.clone());
        GrowthBookAttribute {
            key,
            value: self.value.clone(),
        }
    }
}

fn in_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let Some(user_value) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        match &feature_attribute.value {
            GrowthBookAttributeValue::Array(feature_array) => {
                feature_array.iter().any(|feature_item| match &user_value {
                    GrowthBookAttributeValue::Array(user_array) => user_array
                        .iter()
                        .any(|user_item| feature_item.to_string() == user_item.to_string()),
                    GrowthBookAttributeValue::Empty => false,
                    it => feature_item.to_string() == it.to_string(),
                })
            }
            _ => false,
        }
    } else {
        false
    };
    println!("in_condition={a}");
    a
}

fn nin_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let Some(user_value) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        match &feature_attribute.value {
            GrowthBookAttributeValue::Array(feature_array) => {
                feature_array.iter().all(|feature_item| !match &user_value {
                    GrowthBookAttributeValue::Array(user_array) => user_array
                        .iter()
                        .any(|user_item| feature_item.to_string() == user_item.to_string()),
                    GrowthBookAttributeValue::Empty => false,
                    it => feature_item.to_string() == it.to_string(),
                })
            }
            _ => false,
        }
    } else {
        false
    };
    println!("nin_condition={a}");
    a
}

fn or_condition(
    _parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Array(it) => {
            if it.is_empty() {
                true
            } else {
                it.iter().any(|next_value| match next_value {
                    GrowthBookAttributeValue::Object(feature_value) => feature_value
                        .iter()
                        .all(|next_attribute| is_on(None, next_attribute, user_attributes, false)),
                    _ => false,
                })
            }
        }
        GrowthBookAttributeValue::Empty => true,
        _ => false,
    };
    println!("or_condition={a}");
    a
}

fn not_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Object(it) => it
            .iter()
            .all(|next| !is_on(parent_attribute, next, user_attributes, false)),
        _ => false,
    };
    println!("not_condition={a}");
    a
}

fn and_condition(
    _parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Array(it) => it.iter().all(|next_value| match next_value {
            GrowthBookAttributeValue::Object(feature_value) => feature_value
                .iter()
                .all(|next_attribute| is_on(None, next_attribute, user_attributes, false)),
            _ => false,
        }),
        _ => false,
    };
    println!("and_condition={a}");
    a
}

fn nor_condition(
    _parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Array(it) => it.iter().all(|next_value| match next_value {
            GrowthBookAttributeValue::Object(feature_value) => !feature_value
                .iter()
                .all(|next_attribute| is_on(None, next_attribute, user_attributes, false)),
            _ => false,
        }),
        _ => false,
    };
    println!("nor_condition={a}");
    a
}

fn ne_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let Some(user_value) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        !match &user_value {
            GrowthBookAttributeValue::Array(it) => {
                it.iter().any(|item| item == &feature_attribute.value)
            }
            GrowthBookAttributeValue::Empty => true,
            it => it == &feature_attribute.value,
        }
    } else {
        true
    };
    println!("ne_condition={a}");
    a
}

fn gt_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
) -> bool {
    let a = if feature_attribute.value.is_number() {
        number_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
            |feature_number, user_number| user_number.gt(feature_number),
        )
    } else {
        string_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            |feature_string, user_string| user_string.gt(feature_string),
        )
    };
    println!("gt_condition={a}");
    a
}

fn gte_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
) -> bool {
    let a = if feature_attribute.value.is_number() {
        number_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
            |feature_number, user_number| user_number.ge(feature_number),
        )
    } else {
        string_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            |feature_string, user_string| user_string.ge(feature_string),
        )
    };
    println!("gte_condition={a}");
    a
}

fn lt_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
) -> bool {
    let a = if feature_attribute.value.is_number() {
        number_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
            |feature_number, user_number| user_number.lt(feature_number),
        )
    } else {
        string_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            |feature_string, user_string| user_string.lt(feature_string),
        )
    };
    println!("lt_condition={a}");
    a
}

fn lte_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
) -> bool {
    let a = if feature_attribute.value.is_number() {
        number_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            array_size,
            |feature_number, user_number| user_number.le(feature_number),
        )
    } else {
        string_condition_evaluate(
            parent_attribute,
            feature_attribute,
            user_attributes,
            |feature_string, user_string| user_string.le(feature_string),
        )
    };
    println!("lte_condition={a}");
    a
}

fn number_condition_evaluate(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
    condition: fn(&f64, &f64) -> bool,
) -> bool {
    let a = if let Some(user_value) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        let feature_number = if let GrowthBookAttributeValue::Int(it) = feature_attribute.value {
            it as f64
        } else if let GrowthBookAttributeValue::Float(it) = feature_attribute.value {
            it
        } else if let GrowthBookAttributeValue::String(string_number) = &feature_attribute.value {
            if let Ok(it) = string_number.replace('.', "").parse::<f64>() {
                it
            } else {
                return false;
            }
        } else {
            return false;
        };

        let user_numbers = if let GrowthBookAttributeValue::Int(it) = user_value {
            vec![it as f64]
        } else if let GrowthBookAttributeValue::Float(it) = user_value {
            vec![it]
        } else if let GrowthBookAttributeValue::Array(it) = user_value {
            if array_size {
                vec![it.len() as f64]
            } else {
                it.iter()
                    .filter(|item| item.is_number())
                    .map(|item| item.as_f64().expect("Failed to convert to f64"))
                    .collect()
            }
        } else if let GrowthBookAttributeValue::String(string_number) = &user_value {
            if let Ok(it) = string_number.replace('.', "").parse::<f64>() {
                vec![it]
            } else {
                return false;
            }
        } else {
            return false;
        };

        user_numbers
            .iter()
            .any(|number| condition(&feature_number, number))
    } else {
        true
    };
    println!("number_condition_evaluate={a}");
    a
}

fn string_condition_evaluate(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    condition: fn(&str, &str) -> bool,
) -> bool {
    let a = if let Some(user_value) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        let feature_value = feature_attribute.value.to_string();
        match user_value {
            GrowthBookAttributeValue::Array(it) => it
                .iter()
                .any(|item| condition(&feature_value, &item.to_string())),
            it => condition(&feature_value, &it.to_string()),
        }
    } else {
        true
    };
    println!("string_condition_evaluate={a}");
    a
}

fn eq_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let Some(user_value) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        match &user_value {
            GrowthBookAttributeValue::Array(it) => {
                it.iter().any(|item| item == &feature_attribute.value)
            }
            GrowthBookAttributeValue::Empty => false,
            it => it.to_string() == feature_attribute.value.to_string(),
        }
    } else {
        false
    };
    println!("eq_condition={a}");
    a
}

fn exists_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let GrowthBookAttributeValue::Bool(it) = feature_attribute.value {
        if user_attributes
            .find_value(&parent_attribute.unwrap_or(feature_attribute).key)
            .is_some()
        {
            it
        } else {
            !it
        }
    } else {
        true
    };
    println!("exists_condition={a}");
    a
}

fn regex_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let GrowthBookAttributeValue::String(feature_value) = &feature_attribute.value {
        if let Ok(regex) = Regex::new(feature_value) {
            if let Some(user_value) =
                user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
            {
                match &user_value {
                    GrowthBookAttributeValue::Array(it) => {
                        it.iter().any(|item| regex.is_match(&item.to_string()))
                    }
                    it => regex.is_match(&it.to_string()),
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        true
    };
    println!("regex_condition={a}");
    a
}

fn type_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = if let GrowthBookAttributeValue::String(feature_type) = &feature_attribute.value {
        if let Some(user_value) =
            user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
        {
            match user_value {
                GrowthBookAttributeValue::String(_) => feature_type == "string",
                GrowthBookAttributeValue::Int(_) => feature_type == "number",
                GrowthBookAttributeValue::Float(_) => feature_type == "number",
                GrowthBookAttributeValue::Bool(_) => feature_type == "boolean",
                GrowthBookAttributeValue::Array(_) => feature_type == "array",
                GrowthBookAttributeValue::Object(it) => {
                    if it.is_empty() {
                        feature_type == "null"
                    } else {
                        feature_type == "object"
                    }
                }
                GrowthBookAttributeValue::Empty => feature_type == "null",
            }
        } else {
            feature_type == "null"
        }
    } else {
        false
    };
    println!("type_condition={a}");
    a
}

fn size_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Int(feature_value) => {
            if let Some(GrowthBookAttributeValue::Array(user_value)) =
                user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
            {
                feature_value == &(user_value.len() as i64)
            } else {
                false
            }
        }
        GrowthBookAttributeValue::Object(feature_value) => feature_value
            .iter()
            .all(|next| is_on(parent_attribute, next, user_attributes, true)),
        _ => false,
    };
    println!("size_condition={a}");
    a
}

fn all_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Array(feature_values) => {
            if let Some(GrowthBookAttributeValue::Array(user_values)) =
                user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
            {
                feature_values.iter().all(|feature_item| {
                    user_values
                        .iter()
                        .any(|user_item| feature_item == user_item)
                })
            } else {
                false
            }
        }
        _ => false,
    };
    println!("size_condition={a}");
    a
}

fn vgt_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = version_condition_evaluate(
        parent_attribute,
        feature_attribute,
        user_attributes,
        |feature_version, user_version| user_version.gt(feature_version),
    );
    println!("vgt_condition={a}");
    a
}

fn vgte_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = version_condition_evaluate(
        parent_attribute,
        feature_attribute,
        user_attributes,
        |feature_version, user_version| user_version.ge(feature_version),
    );
    println!("vgte_condition={a}");
    a
}

fn vlt_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = version_condition_evaluate(
        parent_attribute,
        feature_attribute,
        user_attributes,
        |feature_version, user_version| user_version.lt(feature_version),
    );
    println!("vlt_condition={a}");
    a
}

fn vlte_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = version_condition_evaluate(
        parent_attribute,
        feature_attribute,
        user_attributes,
        |feature_version, user_version| user_version.le(feature_version),
    );
    println!("vlte_condition={a}");
    a
}

fn veq_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = version_condition_evaluate(
        parent_attribute,
        feature_attribute,
        user_attributes,
        |feature_version, user_version| user_version.eq(feature_version),
    );
    println!("veq_condition={a}");
    a
}

fn vne_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
) -> bool {
    let a = version_condition_evaluate(
        parent_attribute,
        feature_attribute,
        user_attributes,
        |feature_version, user_version| user_version.ne(feature_version),
    );
    println!("vne_condition={a}");
    a
}

fn version_condition_evaluate(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    condition: fn(&str, &str) -> bool,
) -> bool {
    let a = if let Some(GrowthBookAttributeValue::String(user_version)) =
        user_attributes.find_value(&parent_attribute.unwrap_or(feature_attribute).key)
    {
        let feature_version = feature_attribute.value.to_string();
        println!("{}", &normalize_version(&feature_version));
        println!("{}", &normalize_version(&user_version));
        condition(
            &normalize_version(&feature_version),
            &normalize_version(&user_version),
        )
    } else {
        true
    };
    println!("version_condition_evaluate={a}");
    a
}

fn normalize_version(version: &str) -> String {
    if let Ok(regex1) = Regex::new("(^v|\\+.*$)") {
        if let Ok(regex2) = Regex::new("[-.]") {
            if let Ok(regex3) = Regex::new("^\\d+") {
                let string = regex1.replace_all(version, "").to_string();
                let mut split = regex2
                    .split(&string)
                    .filter(|item| !item.is_empty())
                    .collect::<Vec<&str>>();
                if split.len() == 3 {
                    split.push("~");
                }
                split
                    .iter()
                    .map(|part| {
                        if regex3.is_match(part) {
                            format!("{:0>5}", part)
                        } else {
                            part.to_string()
                        }
                    })
                    .filter(|part| !part.is_empty())
                    .reduce(|a, b| format!("{a}-{b}"))
                    .unwrap_or(version.to_string())
            } else {
                version.to_string()
            }
        } else {
            version.to_string()
        }
    } else {
        version.to_string()
    }
}

fn elem_match_condition(
    parent_attribute: Option<&GrowthBookAttribute>,
    feature_attribute: &GrowthBookAttribute,
    user_attributes: &[GrowthBookAttribute],
    array_size: bool,
) -> bool {
    let a = match &feature_attribute.value {
        GrowthBookAttributeValue::Object(it) => it.iter().any(|condition_attribute| {
            is_on(
                parent_attribute,
                condition_attribute,
                user_attributes,
                array_size,
            )
        }),
        _ => false,
    };
    println!("elem_match_condition={a}");
    a
}

use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[cfg(test)]
mod test {
    use crate::feature::condition::{
        Cases, ConditionEnabledCheck, EvalCondition, EvalConditionValue,
    };
    use crate::model_public::GrowthBookAttribute;

    #[tokio::test]
    async fn evaluate_conditions() -> Result<(), Box<dyn std::error::Error>> {
        let cases = Cases::new();

        for value in cases.eval_condition {
            let eval_condition = value_to_eval_condition(value);
            println!("--------------------");
            println!("eval_condition={}", eval_condition.name);
            let vec_condition = &GrowthBookAttribute::from(eval_condition.condition)
                .expect("Failed to create attributes");
            let vec_attributes = GrowthBookAttribute::from(eval_condition.attribute)
                .expect("Failed to create attributes");
            println!("conditions={:?}", vec_condition);
            println!("attributes={:?}", vec_attributes);
            println!();
            let enabled = vec_condition.is_on(&vec_attributes);
            println!("--------------------");
            if enabled != eval_condition.result {
                panic!("EvalCondition failed: {}", eval_condition.name)
            }
        }

        Ok(())
    }

    fn value_to_eval_condition(value: EvalConditionValue) -> EvalCondition {
        match value {
            EvalConditionValue::Condition(condition) => EvalCondition::new(condition),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Cases {
    #[allow(dead_code)]
    eval_condition: Vec<EvalConditionValue>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum EvalConditionValue {
    Condition(Value),
}

#[allow(dead_code)]
pub struct EvalCondition {
    name: String,
    condition: Value,
    attribute: Value,
    result: bool,
}

impl EvalCondition {
    #[allow(dead_code)]
    fn new(value: Value) -> Self {
        let array = value.as_array().expect("Failed to convert to array");
        Self {
            name: array[0]
                .as_str()
                .expect("Failed to convert do str")
                .to_string(),
            condition: array[1].clone(),
            attribute: array[2].clone(),
            result: array[3].as_bool().expect("Failed to convert to bool"),
        }
    }
}

impl Cases {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let contents = fs::read_to_string("./tests/all_cases.json")
            .expect("Should have been able to read the file");

        serde_json::from_str(&contents).expect("Failed to create cases")
    }
}
