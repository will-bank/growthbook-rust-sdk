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

Check the [client](./examples/client/src/main.rs) folder for a complete example using the SDK.