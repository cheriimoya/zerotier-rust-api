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
pub struct NetworkAllOf1 {
    /// Let ZeroTier modify the system's DNS settings.
    #[serde(rename = "allowDNS", skip_serializing_if = "Option::is_none")]
    pub allow_dns: Option<bool>,
    /// Let ZeroTier modify the system's default route.
    #[serde(rename = "allowDefault", skip_serializing_if = "Option::is_none")]
    pub allow_default: Option<bool>,
    /// Let ZeroTier manage IP addresses and Route assignments that aren't in private ranges (rfc1918).
    #[serde(rename = "allowGlobal", skip_serializing_if = "Option::is_none")]
    pub allow_global: Option<bool>,
    /// Let ZeroTier to manage IP addresses and Route assignments.
    #[serde(rename = "allowManaged", skip_serializing_if = "Option::is_none")]
    pub allow_managed: Option<bool>,
    #[serde(rename = "assignedAddresses", skip_serializing_if = "Option::is_none")]
    pub assigned_addresses: Option<Vec<String>>,
    #[serde(rename = "bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<bool>,
    #[serde(rename = "broadcastEnabled", skip_serializing_if = "Option::is_none")]
    pub broadcast_enabled: Option<bool>,
    #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<Box<crate::models::NetworkAllOf1Dns>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// MAC address for this network's interface.
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename = "mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "multicastSubscriptions", skip_serializing_if = "Option::is_none")]
    pub multicast_subscriptions: Option<Vec<crate::models::NetworkAllOf1MulticastSubscriptions>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "netconfRevision", skip_serializing_if = "Option::is_none")]
    pub netconf_revision: Option<i32>,
    #[serde(rename = "portDeviceName", skip_serializing_if = "Option::is_none")]
    pub port_device_name: Option<String>,
    #[serde(rename = "portError", skip_serializing_if = "Option::is_none")]
    pub port_error: Option<i32>,
    #[serde(rename = "routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<serde_json::Value>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl NetworkAllOf1 {
    pub fn new() -> NetworkAllOf1 {
        NetworkAllOf1 {
            allow_dns: None,
            allow_default: None,
            allow_global: None,
            allow_managed: None,
            assigned_addresses: None,
            bridge: None,
            broadcast_enabled: None,
            dns: None,
            id: None,
            mac: None,
            mtu: None,
            multicast_subscriptions: None,
            name: None,
            netconf_revision: None,
            port_device_name: None,
            port_error: None,
            routes: None,
            status: None,
            _type: None,
        }
    }
}


