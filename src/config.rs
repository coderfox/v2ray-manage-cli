#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Log {
    #[serde(skip_serializing_if = "Option::is_none")] pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub loglevel: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dns {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Routing {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Inbound {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Outbound {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct InboundDetour {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OutboundDetour {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Transport {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")] pub log: Option<Log>,
    #[serde(skip_serializing_if = "Option::is_none")] pub dns: Option<Dns>,
    #[serde(skip_serializing_if = "Option::is_none")] pub routing: Option<Routing>,
    #[serde(skip_serializing_if = "Option::is_none")] pub inbound: Option<Inbound>,
    #[serde(skip_serializing_if = "Option::is_none")] pub outbound: Option<Outbound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inboundDetour")]
    pub inbound_detour: Option<InboundDetour>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outboundDetour")]
    pub outbound_detour: Option<OutboundDetour>,
    #[serde(skip_serializing_if = "Option::is_none")] pub transport: Option<Transport>,
}
