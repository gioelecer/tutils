#![allow(non_snake_case)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;
#[macro_use]
extern crate serde_json;

use serde::{Deserialize, Serialize};

pub mod restreamer;
pub mod structs;

pub fn get_available_instances(api_key: &String, client_id: &String, api_url: &String) -> Result<restreamer::Instances,String> {
    restreamer::_get_available_instances(api_key, client_id, api_url)
}

pub fn get_stats(instance_name: &String, base_url: &String) -> Result<structs::stats::Root,String> {
    restreamer::_get_stats(instance_name, base_url)
}

pub fn get_ffprobe(instance: &restreamer::Instance, secret: &String, base_url: &String) -> Result<structs::ffprobe::Root,String> {
    restreamer::_get_ffprobe(instance,secret,base_url)
}