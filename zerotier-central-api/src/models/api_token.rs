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
pub struct ApiToken {
    /// user specified token name
    #[serde(rename = "tokenName", skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
    /// API Token.  Minimum 32 characters. This token is encrypted in the database and can not be retrieved once set
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl ApiToken {
    pub fn new() -> ApiToken {
        ApiToken {
            token_name: None,
            token: None,
        }
    }
}


