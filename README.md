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

## Checking if a Feature is Enabled or Disabled

With GrowthBook, you can determine if a feature is on or off by checking the feature's value using different data types. A feature is considered off (i.e., is_on is false) under the following conditions, based on the data type:

### String Feature

If the feature value (regardless if it is default_value, force, or derived from an experiment) is an **empty string** or explicitly **OFF**, then is_on will be false.

```rust
let gb_url = "HTTP_OR_HTTPS_URL";
let sdk_key = "SDK_KEY";
let gb = GrowthBookClient::new(gb_url, sdk_key, None, None)?;

let my_gb_feature = gb.feature_result(
    "my_feature_string".to_string(),
    Some(gb_attributes.clone()) // Optional: use attributes or set to None
);

```

### Numeric Feature

If the feature value is **0** (regardless if it is default_value, force, or derived from an experiment), then is_on will be false.

```rust
let gb_url = "HTTP_OR_HTTPS_URL";
let sdk_key = "SDK_KEY";
let gb = GrowthBookClient::new(gb_url, sdk_key, None, None)?;

let my_gb_feature = gb.feature_result(
    "my_feature_numeric".to_string(),
    Some(gb_attributes.clone()) // Optional: use attributes or set to None
);

```

### Object (HashMap) Feature

If the feature value is an **empty object** (regardless if it is default_value, force, or derived from an experiment), then is_on will be false.

```rust
let gb_url = "HTTP_OR_HTTPS_URL";
let sdk_key = "SDK_KEY";
let gb = GrowthBookClient::new(gb_url, sdk_key, None, None)?;

let my_gb_feature = gb.feature_result(
    "my_feature_as_object".to_string(),
    Some(gb_attributes.clone()) // Optional: use attributes or set to None
);

```

### Array Feature

If the feature value is an **empty array** (regardless if it is default_value, force, or derived from an experiment), then is_on will be false.

```rust
let gb_url = "HTTP_OR_HTTPS_URL";
let sdk_key = "SDK_KEY";
let gb = GrowthBookClient::new(gb_url, sdk_key, None, None)?;

let my_gb_feature = gb.feature_result(
    "my_feature_as_array".to_string(),
    Some(gb_attributes.clone()) // Optional: use attributes or set to None
);

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

Check the [client](./examples/client/src/main.rs) folder for a complete example using the SDK.