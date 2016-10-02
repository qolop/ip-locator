extern crate hyper;
use std::env;
use std::net::IpAddr;
use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

struct IPAddrInfo {
    region: String,
    country: String,
    city: String,
    region_code: String,
    country_code: String,
    zip_code: String,
    organization: String,
    latitude: String,
    longitude: String,
    isp: String,
    time_zone: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let ip = &args[1];
    if is_valid_ip(&ip) {
        let ip_info = request_ip_info(&ip).unwrap();
        println!("The IP {} is located at {}, {}, {}.",
                 ip,
                 ip_info.city,
                 ip_info.region,
                 ip_info.country);
    } else {
        println!("IP address is not valid!");
    }
}

fn is_valid_ip(addr: &String) -> bool {
    addr.parse::<IpAddr>().is_ok()
}

fn request_ip_info(addr: &String) -> Option<IPAddrInfo> {
    let client = Client::new();
    let mut response = client.get(&format!("http://ip-api.com/line/{}", addr))
        .header(Connection::close())
        .send()
        .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let p: Vec<&str> = body.trim().split("\n").collect();
    return match p[0] {
        "success" => {
            Some(IPAddrInfo {
                country: p[1].to_string(),
                country_code: p[2].to_string(),
                region_code: p[3].to_string(),
                region: p[4].to_string(),
                city: p[5].to_string(),
                zip_code: p[6].to_string(),
                latitude: p[7].to_string(),
                longitude: p[8].to_string(),
                time_zone: p[9].to_string(),
                isp: p[10].to_string(),
                organization: p[11].to_string(),
            })
        }
        "fail" => {
            println!("IP could not be found by the API.");
            None
        }
        _ => {
            None
        }
    };
}
