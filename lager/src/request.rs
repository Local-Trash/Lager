use std::net::IpAddr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct TcpRequest {
    ip: IpAddr,
    message: Vec<String>
}