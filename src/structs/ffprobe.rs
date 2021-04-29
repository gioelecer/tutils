#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub result: Option<Result>,
    pub code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub index: i64,
    #[serde(rename = "codec_name")]
    pub codec_name: String,
    #[serde(rename = "codec_long_name")]
    pub codec_long_name: String,
    pub profile: String,
    #[serde(rename = "codec_type")]
    pub codec_type: String,
    #[serde(rename = "codec_time_base")]
    pub codec_time_base: String,
    #[serde(rename = "codec_tag_string")]
    pub codec_tag_string: String,
    #[serde(rename = "codec_tag")]
    pub codec_tag: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(rename = "coded_width")]
    pub coded_width: Option<i64>,
    #[serde(rename = "coded_height")]
    pub coded_height: Option<i64>,
    #[serde(rename = "has_b_frames")]
    pub has_b_frames: Option<i64>,
    #[serde(rename = "pix_fmt")]
    pub pix_fmt: Option<String>,
    pub level: Option<i64>,
    #[serde(rename = "color_range")]
    pub color_range: Option<String>,
    #[serde(rename = "color_space")]
    pub color_space: Option<String>,
    #[serde(rename = "color_transfer")]
    pub color_transfer: Option<String>,
    #[serde(rename = "color_primaries")]
    pub color_primaries: Option<String>,
    #[serde(rename = "chroma_location")]
    pub chroma_location: Option<String>,
    #[serde(rename = "field_order")]
    pub field_order: Option<String>,
    pub refs: Option<i64>,
    #[serde(rename = "is_avc")]
    pub is_avc: Option<String>,
    #[serde(rename = "nal_length_size")]
    pub nal_length_size: Option<String>,
    #[serde(rename = "r_frame_rate")]
    pub r_frame_rate: String,
    #[serde(rename = "avg_frame_rate")]
    pub avg_frame_rate: String,
    #[serde(rename = "time_base")]
    pub time_base: String,
    #[serde(rename = "start_pts")]
    pub start_pts: i64,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "bit_rate")]
    pub bit_rate: String,
    #[serde(rename = "bits_per_raw_sample")]
    pub bits_per_raw_sample: Option<String>,
    pub disposition: Disposition,
    #[serde(rename = "sample_fmt")]
    pub sample_fmt: Option<String>,
    #[serde(rename = "sample_rate")]
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    #[serde(rename = "channel_layout")]
    pub channel_layout: Option<String>,
    #[serde(rename = "bits_per_sample")]
    pub bits_per_sample: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disposition {
    pub default: i64,
    pub dub: i64,
    pub original: i64,
    pub comment: i64,
    pub lyrics: i64,
    pub karaoke: i64,
    pub forced: i64,
    #[serde(rename = "hearing_impaired")]
    pub hearing_impaired: i64,
    #[serde(rename = "visual_impaired")]
    pub visual_impaired: i64,
    #[serde(rename = "clean_effects")]
    pub clean_effects: i64,
    #[serde(rename = "attached_pic")]
    pub attached_pic: i64,
    #[serde(rename = "timed_thumbnails")]
    pub timed_thumbnails: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub filename: String,
    #[serde(rename = "nb_streams")]
    pub nb_streams: i64,
    #[serde(rename = "nb_programs")]
    pub nb_programs: i64,
    #[serde(rename = "format_name")]
    pub format_name: String,
    #[serde(rename = "start_time")]
    pub start_time: String,
    pub duration: String,
    #[serde(rename = "probe_score")]
    pub probe_score: i64,
    pub tags: Tags,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    #[serde(rename = "Server")]
    pub server: String,
    pub display_width: String,
    pub display_height: String,
    pub fps: String,
    pub profile: String,
    pub level: String,
}
