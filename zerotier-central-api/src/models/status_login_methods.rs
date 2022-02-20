/*
 * ZeroTier Central API
 *
 * ZeroTier Central Network Management Portal API.<p>All API requests must have an API token header specified in the <code>Authorization: Bearer xxxxx</code> format.  You can generate your API key by logging into <a href=\"https://my.zerotier.com\">ZeroTier Central</a> and creating a token on the Account page.</p><p>eg. <code>curl -X GET -H \"Authorization: bearer xxxxx\" https://my.zerotier.com/api/v1/network</code></p><p><h3>Rate Limiting</h3></p><p>The ZeroTier Central API implements rate limiting.  Paid users are limited to 100 requests per second.  Free users are limited to 20 requests per second.</p> <p> You can get the OpenAPI spec here as well: <code>https://docs.zerotier.com/openapi/centralv1.json</code></p>
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusLoginMethods {
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    #[serde(rename = "google", skip_serializing_if = "Option::is_none")]
    pub google: Option<bool>,
    #[serde(rename = "twitter", skip_serializing_if = "Option::is_none")]
    pub twitter: Option<bool>,
    #[serde(rename = "facebook", skip_serializing_if = "Option::is_none")]
    pub facebook: Option<bool>,
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: Option<bool>,
    #[serde(rename = "saml", skip_serializing_if = "Option::is_none")]
    pub saml: Option<bool>,
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc: Option<bool>,
}

impl StatusLoginMethods {
    pub fn new() -> StatusLoginMethods {
        StatusLoginMethods {
            local: None,
            google: None,
            twitter: None,
            facebook: None,
            github: None,
            saml: None,
            oidc: None,
        }
    }
}


