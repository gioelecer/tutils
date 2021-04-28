#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instances {
    pub instances: Vec<Instance>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    pub id: u64,
    pub name: String,
    pub dnsname: String,
    pub npm_url: String,
    pub external_rtmp: String,
    pub flv_path: String,
    pub token: String,
    pub status: u8,
    pub owner: String,
    pub owner_id: u8,
    pub output_path: String,
    pub title:String,
    pub description: String,
    pub cdn_path: String,
    pub backup_cdn_path: String,
    pub extended_data: String,
    pub latest_event: String,
    pub label: String,
    pub dedicated: String,
    pub thumbnail: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceRequest {
    pub api_key: String,
    pub client_id: String,
}

use quickxml_to_serde::xml_string_to_json;

pub fn _get_available_instances(api_key: &String, client_id: &String, api_url: &String) -> Result<Instances,String>{

    let request_data = json!({
        "api_key":api_key,
        "client_id":client_id
    });

    let mut result = false;
    let mut to_return = Ok(Instances{
        instances: vec![]
    });


    let _ = async_std::task::block_on(async {
        let mut r = surf::post(api_url)
            .body_json(&request_data)
            .unwrap()
            .await
            .unwrap();
            let t = r.body_string().await.unwrap();
            let s = r.status().to_string();
            
            if s == "200 OK"{
                let available_instances: Instances = serde_json::from_str(&t).unwrap();
                to_return = Ok(available_instances);
                result = true;
            }
    });

    if result {
        return to_return
    }else{
        Err("Unable to retrieve instances".to_string())
    }
    

}

pub fn _get_stats(instance_name: &String, base_url: &String) -> Result<super::structs::stats::Root,String>{

    let mut result = false;
    let mut to_return = Ok(super::structs::stats::Root{
        http_flv: None
    });

    let _ = async_std::task::block_on(async {
        let mut res = surf::get(format!("{}{}{}",base_url,instance_name,"/stats.xsl")).await.unwrap();
        let xml_contents = res.body_string().await.unwrap();
        
        if let Ok(xml_json) = xml_string_to_json(xml_contents, &quickxml_to_serde::Config::new_with_defaults()){
            let serde_xml: super::structs::stats::Root = serde_json::from_value(xml_json).unwrap();
            to_return = Ok(serde_xml);
            result = true;
        }
    });

    if result {
        return to_return
    }else{
        Err("Unable to retrieve stats".to_string())
    }
    

}

pub fn _get_ffprobe(instance: &Instance, secret: &String, base_url: &String) -> Result<super::structs::ffprobe::Root, String> {
    let probe_request = json!({
        "s":secret,
    });

    let mut to_return = Ok(super::structs::ffprobe::Root{
        result: None,
        code: 0
    });
    let mut result = false;

    let mut probe_result = "".to_string();
    let _ = async_std::task::block_on(async {
        let mut r = surf::post(format!(
            "{}{}{}",
            base_url, instance.name, "/v1/probe"
        ))
        .body_json(&probe_request)
        .unwrap()
        .await
        .unwrap();
        probe_result = r.body_string().await.unwrap();
        let s = r.status().to_string();
        if s == "200 OK"{
            let serde_ffprobe: super::structs::ffprobe::Root = serde_json::from_str(&probe_result).unwrap();
            to_return = Ok(serde_ffprobe);
            result = true;
        }
    });

    if result {
        return to_return
    }else{
        Err("Unable to retrieve ffprobe".to_string())
    }
}