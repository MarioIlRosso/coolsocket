#![allow(missing_docs)]

use coolsocket::socket::raw::{RawSocket, SocketDomain, SocketKind, SocketType};

#[test]
fn create_socket() {
    let sock_kind = SocketKind::new(SocketType::Stream, false, false);
    RawSocket::try_new(SocketDomain::Ipv4, sock_kind, 0).unwrap();
}

#[test]
#[should_panic]
fn create_socket_panic() {
    let sock_kind = SocketKind::new(SocketType::Stream, false, false);
    // Creating a TCP socket with UDP protocol, should panic.
    RawSocket::try_new(SocketDomain::Ipv4, sock_kind, libc::IPPROTO_UDP).unwrap();
}
