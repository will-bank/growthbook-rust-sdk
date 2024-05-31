use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub struct GrowthbookGatewayMock;

impl GrowthbookGatewayMock {
    #[allow(dead_code)]
    pub async fn simple_flag(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": flag_enabled
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn simple_rule_condition(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "any-id": "018fcf11-bb67-7789-8d10-fcbb7de4ff7b"
                            },
                            "force": flag_enabled
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn multiple_rule_condition(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "any-id": "018fcf11-bb67-7789-8d10-fcbb7de4ff7b",
                                "any-key": "018fcf64-1827-709a-a8ae-7d206aafb5e2"
                            },
                            "force": flag_enabled
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn rollout_rule_condition(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        coverage: f32,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "force": flag_enabled,
                            "coverage": coverage
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn rollout_rule_condition_by_attribute(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        coverage: f32,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "force": flag_enabled,
                            "coverage": coverage,
                            "hashAttribute": "any-id"
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn gte_rule(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        version: &str,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "version": {
                                    "$gte": version
                                }
                            },
                            "force": flag_enabled,
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn gt_rule(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        version: &str,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "version": {
                                    "$gt": version
                                }
                            },
                            "force": flag_enabled,
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn lt_rule(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        version: &str,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "version": {
                                    "$lt": version
                                }
                            },
                            "force": flag_enabled,
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn lte_rule(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        version: &str,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "version": {
                                    "$lte": version
                                }
                            },
                            "force": flag_enabled,
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn elem_match_eq(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "any-data": {
                                    "$elemMatch": {
                                        "$eq": "3"
                                    }
                                }
                            },
                            "force": flag_enabled,
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn not_elem_match_eq(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "condition": {
                                "any-data": {
                                    "$not": {
                                        "$elemMatch": {
                                            "$eq": "3"
                                        }
                                    }
                                }
                            },
                            "force": flag_enabled,
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }

    #[allow(dead_code)]
    pub async fn experiment_rule_condition(
        mock_server: &MockServer,
        sdk: Uuid,
        flag_enabled: bool,
        coverage: f32,
        status_code: StatusCode,
    ) {
        let body = json!({
            "status": 200,
            "features": {
                "flag": {
                    "defaultValue": !flag_enabled,
                    "rules": [
                        {
                            "coverage": coverage,
                            "hashAttribute": "any-id",
                            "seed": "any-seed",
                            "hashVersion": 2,
                            "variations": [
                                false,
                                true,
                                true
                            ],
                            "weights": [
                                0.3334,
                                0.3333,
                                0.3333
                            ],
                            "key": "any-experiment-key",
                            "meta": [
                                {
                                    "key": "0"
                                },
                                {
                                    "key": "1"
                                },
                                {
                                    "key": "2"
                                }
                            ],
                            "phase": "1"
                        }
                    ]
                }
            },
            "dateUpdated": "2024-05-29T18:43:22.153Z"
        });
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk}")))
            .and(header(CONTENT_TYPE, "application/json"))
            .respond_with(
                ResponseTemplate::new(status_code.as_u16()).set_body_string(body.to_string()),
            )
            .mount(mock_server)
            .await;
    }
}
