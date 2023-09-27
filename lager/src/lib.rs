use std::net::TcpStream;

mod threads;
mod request;

pub trait NetworkProtocol {
    fn parse(stream: &mut TcpStream) -> Self where Self: Sized;

    fn send(self, message: Self) where Self: Sized;
}

type Protocol = Box<dyn NetworkProtocol>;

/// The main struct that contains the entire server
pub struct Server<F: FnMut(&mut Protocol)> {
    protocol: Protocol,
    handler: F,
}

impl<F: FnMut(&mut Protocol)> Server<F> {

}