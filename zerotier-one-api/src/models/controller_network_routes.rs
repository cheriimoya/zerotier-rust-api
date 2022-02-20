/*
 * ZeroTierOne Service API
 *
 * <p> This API controls the ZeroTier service that runs in the background on your computer. This is how zerotier-cli, and the macOS and Windows apps control the service. </p> <p> API requests must be authenticated via an authentication token. ZeroTier One saves this token in the authtoken.secret file in its working directory. This token may be supplied via the X-ZT1-Auth HTTP request header. </p> <p> For example: <code>curl -H \"X-ZT1-Auth: $TOKEN\" http://localhost:9993/status</code> </p> <p> The token can be found in: <ul> <li>Mac :: /Library/Application Support/ZeroTier/One</li> <li>Windows :: \\ProgramData\\ZeroTier\\One</li> <li>Linux :: /var/lib/zerotier-one</li> </ul> </p> <p> You can get the OpenAPI spec here as well: <code>https://docs.zerotier.com/openapi/servicev1.json</code></p>
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ControllerNetworkRoutes {
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "via", skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

impl ControllerNetworkRoutes {
    pub fn new() -> ControllerNetworkRoutes {
        ControllerNetworkRoutes {
            target: None,
            via: None,
        }
    }
}


