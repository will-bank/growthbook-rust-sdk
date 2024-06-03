# GrowthBook Rust SDK

[![Crates.io](https://img.shields.io/crates/l/datadog-tracing)](LICENSE)

Non-official GrowthBook SDK for Rust services.

This crate provide an easy way to retrieve a feature value using SDK-KEY.
___

## How to use

Initializing SDK

```rust
let gb_url = "HTTP_OR_HTTPS_URL";
let sdk_key = "SDK_KEY";
let gb = GrowthBookClient::new(gb_url, sdk_key, None, None)?;

```

# Configuration

The lib is configurable via environment variables as following:

| env var                | required | description                                                                    |
|------------------------|----------|--------------------------------------------------------------------------------|
| GB_HTTP_CLIENT_TIMEOUT | false    | Timeout from gb client to wait a response from gb server. Default value is 10s |
| GB_UPDATE_INTERVAL     | false    | Interval to fetch features data from gb server. Default value is 60s           |
| GB_URL                 | false    | URL from gb server                                                             |
| GB_SDK_KEY             | false    | SDK key to get features from gb server                                         |


# Examples

- Check the [client][examples/client/src/main.rs] folder for a complete example using the SDK.

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