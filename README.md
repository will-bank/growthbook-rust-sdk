# GrowthBook Rust SDK

[![Crates.io](https://img.shields.io/crates/l/datadog-tracing)](LICENSE)

Non-official GrowthBook SDK for Rust services.

This crate provide an easy way to retrieve a feature value using SDK-KEY.
___

## How to use

Initializing SDK

```rust
let gb_url = "HTTP_OR_HTTPS_URL";
let timeout_in_millis = 500;
let gb = Growthbook::new(gb_url, timeout_in_millis)?;

let sdk_key = "SDK_KEY"; // will be used later
```

### Boolean feature
```rust
pub struct BooleanFlag {
    pub enabled: bool,
    pub experiment_key: Option<String>,
}
```

Retrieving flag value

```rust
let feature_name = "name";
let default_feature_value = false;
let attributes: HashMap<String, Vec<String>> = HashMap::new();

let flag: BooleanFlag = gb.is_on(
    sdk_key,
    feature_name, 
    default_feature_value,
    Some(attributes),
).await?;
```

### String feature

```rust
pub struct StringFlag {
    pub value: String,
    pub experiment_key: Option<String>,
}
```

Retrieving flag value

```rust
let feature_name = "name";
let default_feature_value = "default-string";
let attributes: HashMap<String, Vec<String>> = HashMap::new();

let flag: StringFlag = gb.get_string_value(
    sdk_key,
    feature_name,
    default_feature_value, 
    Some(attributes),
).await?;
```

### Struct feature

```rust
pub struct ObjectFlag {
    value: Value,
    pub experiment_key: Option<String>,
}
```

Retrieving flag value

```rust
let feature_name = "name";
let default_feature_value = json!({
    "any_attribute": "any_value",
});
let attributes: HashMap<String, Vec<String>> = HashMap::new();

let flag: StructFlag = gb.get_object_value(
    sdk_key,
    feature_name,
    default_feature_value, 
    Some(attributes),
).await?;
```

Converting to generic struct

```rust
#[derive(serde::Deserialize)]
pub struct Any {
    any_attribute: String,
}

let any: Any = flag.value()?;
```
___