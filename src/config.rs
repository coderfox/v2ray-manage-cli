#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Log {
    pub access: Option<String>,
    pub error: Option<String>,
    pub loglevel: Option<String>,
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
    pub log: Option<Log>,
    pub dns: Option<Dns>,
    pub routing: Option<Routing>,
    pub inbound: Option<Inbound>,
    pub outbound: Option<Outbound>,
    #[serde(rename = "inboundDetour")] pub inbound_detour: Option<InboundDetour>,
    #[serde(rename = "outboundDetour")] pub outbound_detour: Option<OutboundDetour>,
    pub transport: Option<Transport>,
}
