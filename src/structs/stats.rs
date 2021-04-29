#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "http-flv")]
    pub http_flv: Option<HttpFlv>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpFlv {
    #[serde(rename = "nginx_version")]
    pub nginx_version: String,
    #[serde(rename = "nginx_http_flv_version")]
    pub nginx_http_flv_version: String,
    pub compiler: String,
    pub built: String,
    pub pid: u64,
    pub uptime: u64,
    pub naccepted: u64,
    #[serde(rename = "bw_in")]
    pub bw_in: u64,
    #[serde(rename = "bytes_in")]
    pub bytes_in: u64,
    #[serde(rename = "bw_out")]
    pub bw_out: u64,
    #[serde(rename = "bytes_out")]
    pub bytes_out: u64,
    pub server: Server,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub port: u64,
    #[serde(rename = "server_index")]
    pub server_index: u64,
    pub application: Vec<Application>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub live: Live,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub stream: Option<Stream>,
    pub nclients: u64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub name: String,
    pub time: u64,
    #[serde(rename = "bw_in")]
    pub bw_in: u64,
    #[serde(rename = "bytes_in")]
    pub bytes_in: u64,
    #[serde(rename = "bw_out")]
    pub bw_out: u64,
    #[serde(rename = "bytes_out")]
    pub bytes_out: u64,
    #[serde(rename = "bw_audio")]
    pub bw_audio: u64,
    #[serde(rename = "bw_video")]
    pub bw_video: u64,
    pub client: ::serde_json::Value,
    pub meta: Meta,
    pub nclients: u64,
    pub publishing: Publishing,
    pub active: Active,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub video: Video,
    pub audio: Audio,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub width: u64,
    pub height: u64,
    #[serde(rename = "frame_rate")]
    pub frame_rate: f64,
    pub codec: String,
    pub profile: Option<String>,
    pub level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub codec: String,
    pub profile: Option<String>,
    pub channels: u64,
    #[serde(rename = "sample_rate")]
    pub sample_rate: u64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publishing {
    #[serde(rename = "-self-closing")]
    pub self_closing: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Active {
    #[serde(rename = "-self-closing")]
    pub self_closing: Option<String>,
}
