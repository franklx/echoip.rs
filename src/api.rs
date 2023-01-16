use std::{
    net::{AddrParseError, IpAddr, SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

use actix_web::{dev, middleware::ErrorHandlerResponse, web, HttpRequest, HttpResponse, Responder, Result};
use dns_lookup::lookup_addr;
use log::{debug, warn};

use crate::{
    error::EchoIpError,
    geoip_lookup,
    model::{Index, IpResult, PortLookup, UserInfo},
    templates,
};

fn extract_ip(req: &HttpRequest) -> std::result::Result<IpResult, AddrParseError> {
    let conn_info = req.connection_info();

    if let Some(real_ip) = conn_info.realip_remote_addr().map(clean_ip) {
        real_ip.parse::<IpAddr>().map(|ip| IpResult { ip, real_ip: real_ip.to_owned() })
    } else {
        panic!("Cannot determine IP Address!");
    }
}

fn ip_to_decimal(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ip4) => u32::from(ip4).to_string(),
        IpAddr::V6(ip6) => u128::from(ip6).to_string(),
    }
}

/***
* Possible IPs that we've seen so far:
* [::1]:51224
* 127.0.0.1:47324
* IPv4/v6 without trailing ports
 */
fn clean_ip(ip: &str) -> &str {
    debug!("Removing trailing colon and square brackets from IP.");
    let ip = &ip[ip.find('[').unwrap_or(0)..ip.rfind([']', ':']).unwrap_or(ip.len())];
    debug!("Cleaned IP: {}", ip);
    ip
}

fn get_user_info(req: &HttpRequest, ip: &IpAddr) -> UserInfo {
    let user_agent_raw: String = req.headers().get("User-Agent").unwrap().to_str().unwrap().to_string();
    debug!("Raw user agent [ {} ] for {}", user_agent_raw, ip.to_string());

    let mut user_agent = user_agent_raw.clone();
    let mut user_agent_comment = String::new();
    if user_agent_raw.contains(' ') {
        let ua_split: Vec<&str> = user_agent_raw.splitn(2, ' ').collect();
        user_agent = ua_split[0].to_string();
        user_agent_comment = ua_split[1].to_string();
    }

    UserInfo { hostname: lookup_addr(ip).unwrap(), user_agent, user_agent_comment, user_agent_raw }
}

fn generate_response(req: HttpRequest) -> Index {
    if let Ok(IpResult { ip, real_ip }) = extract_ip(&req) {
        debug!("Converted IP {} properly, getting GeoIP info.", real_ip);

        let geo_info = geoip_lookup::GeoipLookup::new().lookup_geo_for_ip(ip).ok();

        if geo_info.is_some() {
            debug!("Collected GeoIP info for {real_ip}.");
        } else {
            warn!("Could not retrieve GeoIP info for {real_ip}.")
        };

        debug!("Getting user data for {real_ip}.");
        let user_info = Some(get_user_info(&req, &ip));

        Index {
            host: String::from(req.connection_info().host()),
            ip: ip.to_string(),
            decimal_ip: ip_to_decimal(ip),
            geo_info,
            user_info,
        }
    } else {
        let ip: IpAddr = IpAddr::from_str("127.0.0.1").unwrap();
        Index {
            host: String::from(req.connection_info().host()),
            ip: ip.to_string(),
            decimal_ip: ip_to_decimal(ip),
            geo_info: None,
            user_info: None,
        }
    }
}

pub(crate) async fn html_response(http_request: HttpRequest) -> Result<HttpResponse, EchoIpError> {
    let data = generate_response(http_request);
    let json = serde_json::to_string(&data).unwrap();
    let body = templates::index_html(data, json);

    debug!("Returning response to browser.");
    Ok(HttpResponse::Ok().body(body))
}

pub(crate) async fn plain_response(http_request: HttpRequest) -> HttpResponse {
    let real_ip = clean_ip(http_request.connection_info().realip_remote_addr().unwrap()).to_owned();
    debug!("IP from the client: {}", real_ip);
    HttpResponse::Ok().content_type("text/plain").body(real_ip)
}

pub(crate) async fn json_response(http_request: HttpRequest) -> HttpResponse {
    let data = generate_response(http_request);

    debug!("Sending JSON response.");
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&data).unwrap())
}

pub async fn port_lookup(req: HttpRequest, path: web::Path<u16>) -> HttpResponse {
    let port = path.into_inner();
    let ip = extract_ip(&req).unwrap();

    let stream = TcpStream::connect_timeout(&SocketAddr::new(ip.ip, port), Duration::new(5, 0));
    let reachable = stream.is_ok();

    let result = PortLookup { ip: ip.real_ip, port, reachable };

    debug!("Sending port lookup response.");
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&result).unwrap())
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = templates::err500_html()
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
