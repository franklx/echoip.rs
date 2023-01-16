use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BBox {
    pub t: f64,
    pub r: f64,
    pub b: f64,
    pub l: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GeoInfo {
    pub country_name:   Option<String>,
    pub country_iso:    Option<String>,
    pub country_in_eu:  Option<bool>,
    pub region_name:    Option<String>,
    pub region_code:    Option<String>,
    pub city:           Option<String>,
    pub metro_code:     Option<u16>,
    pub postal_code:    Option<String>,
    pub latitude:       Option<f64>,
    pub longitude:      Option<f64>,
    pub timezone:       Option<String>,
    pub asn:            Option<String>,
    pub asn_org:        Option<String>,
    pub bbox:           Option<BBox>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInfo {
    pub hostname:           String,
    pub user_agent:         String,
    pub user_agent_comment: String,
    pub user_agent_raw:     String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub host:         String,
    pub ip:           String,
    pub decimal_ip:   String,
    pub geo_info:     Option<GeoInfo>,
    pub user_info:    Option<UserInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortLookup {
    pub ip:        String,
    pub port:      u16,
    pub reachable: bool,
}

pub struct IpResult {
    pub ip:      IpAddr,
    pub real_ip: String,
}
