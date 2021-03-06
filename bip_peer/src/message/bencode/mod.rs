use std::io;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str;

use bip_bencode::{BencodeConvertError, BConvert, BRefAccess, BDictAccess};
use bip_util::convert;

use message::bits_extension::ExtendedType;

pub const CONVERT: IoErrorBencodeConvert = IoErrorBencodeConvert;

pub struct IoErrorBencodeConvert;

impl BConvert for IoErrorBencodeConvert {
    type Error = io::Error;

    fn handle_error(&self, error: BencodeConvertError) -> Self::Error {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    }
}

// ----------------------------------------------------------------------------//

pub const ID_MAP_KEY:              &'static [u8] = b"m";
pub const CLIENT_ID_KEY:           &'static [u8] = b"v";
pub const CLIENT_TCP_PORT_KEY:     &'static [u8] = b"p";
pub const OUR_IP_KEY:              &'static [u8] = b"yourip";
pub const CLIENT_IPV6_ADDR_KEY:    &'static [u8] = b"ipv6";
pub const CLIENT_IPV4_ADDR_KEY:    &'static [u8] = b"ipv4";
pub const CLIENT_MAX_REQUESTS_KEY: &'static [u8] = b"reqq";
pub const METADATA_SIZE_KEY:       &'static [u8] = b"metadata_size";

pub fn parse_id_map<'a, B>(root: &BDictAccess<'a, B>) -> HashMap<ExtendedType, i64>
    where B: BRefAccess<'a> {
    let mut id_map = HashMap::new();
    
    if let Ok(ben_id_map) = CONVERT.lookup_and_convert_dict(root, ID_MAP_KEY) {
        for (id, ben_value) in ben_id_map.to_list() {
            match (str::from_utf8(id), CONVERT.convert_int(ben_value, id)) {
                (Ok(str_id), Ok(value)) => { id_map.insert(ExtendedType::from_id(str_id), value); },
                _                       => ()
            }
        }
    }

    id_map
}

pub fn parse_client_id<'a, B>(root: &BDictAccess<'a, B>) -> Option<String>
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_str(root, CLIENT_ID_KEY)
        .map(|id| id.to_string())
        .ok()
}

pub fn parse_client_tcp_port<'a, B>(root: &BDictAccess<'a, B>) -> Option<u16>
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_int(root, CLIENT_TCP_PORT_KEY)
        .ok()
        .and_then(|port| {
            if port as u16 as i64 == port {
                Some(port as u16)
            } else {
                None
            }
        })
}

pub fn parse_our_ip<'a, B>(root: &BDictAccess<'a, B>) -> Option<IpAddr>
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_bytes(root, OUR_IP_KEY)
        .ok()
        .and_then(|ip_bytes| {
            if ip_bytes.len() == 4 {
                Some(IpAddr::V4(parse_ipv4_addr(ip_bytes)))
            } else if ip_bytes.len() == 16 {
                Some(IpAddr::V6(parse_ipv6_addr(ip_bytes)))
            } else {
                None
            }
        })
}

pub fn parse_client_ipv6_addr<'a, B>(root: &BDictAccess<'a, B>) -> Option<Ipv6Addr>
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_bytes(root, CLIENT_IPV6_ADDR_KEY)
        .ok()
        .and_then(|ipv6_bytes| {
            if ipv6_bytes.len() == 16  {
                Some(parse_ipv6_addr(ipv6_bytes))
            } else {
                None
            }
        })
}

pub fn parse_client_ipv4_addr<'a, B>(root: &BDictAccess<'a, B>) -> Option<Ipv4Addr> 
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_bytes(root, CLIENT_IPV4_ADDR_KEY)
        .ok()
        .and_then(|ipv4_bytes| {
            if ipv4_bytes.len() == 4 {
                Some(parse_ipv4_addr(ipv4_bytes))
            } else {
                None
            }
        })
}

pub fn parse_client_max_requests<'a, B>(root: &BDictAccess<'a, B>) -> Option<i64>
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_int(root, CLIENT_MAX_REQUESTS_KEY)
        .ok()
}

pub fn parse_metadata_size<'a, B>(root: &BDictAccess<'a, B>) -> Option<i64>
    where B: BRefAccess<'a> {
    CONVERT.lookup_and_convert_int(root, METADATA_SIZE_KEY)
        .ok()
}

fn parse_ipv4_addr(ipv4_bytes: &[u8]) -> Ipv4Addr {
    convert::bytes_be_to_ipv4([ipv4_bytes[0], ipv4_bytes[1], ipv4_bytes[2], ipv4_bytes[3]])
}

fn parse_ipv6_addr(ipv6_bytes: &[u8]) -> Ipv6Addr {
    convert::bytes_be_to_ipv6([ipv6_bytes[0], ipv6_bytes[1], ipv6_bytes[2], ipv6_bytes[3],
                               ipv6_bytes[4], ipv6_bytes[5], ipv6_bytes[6], ipv6_bytes[7],
                               ipv6_bytes[8], ipv6_bytes[9], ipv6_bytes[10], ipv6_bytes[11],
                               ipv6_bytes[12], ipv6_bytes[13], ipv6_bytes[14], ipv6_bytes[15]])
}