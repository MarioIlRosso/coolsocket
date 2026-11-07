#![allow(missing_docs)]

use coolsocket::socket::{
    ArpSocket,
    raw::{RawSocket, SocketDomain, SocketKind, SocketType},
};

#[test]
fn create_socket() {
    let sock_kind = SocketKind::new(SocketType::Stream, false, false);
    RawSocket::try_new(SocketDomain::Ipv4, sock_kind, 0).expect("Failed to create socket!");
}

#[test]
#[should_panic(expected = "Failed to create socket")]
fn create_socket_panic() {
    let sock_kind = SocketKind::new(SocketType::Stream, false, false);
    // Creating a TCP socket with UDP protocol, should panic.
    RawSocket::try_new(SocketDomain::Ipv4, sock_kind, libc::IPPROTO_UDP)
        .expect("Failed to create socket");
}

#[test]
#[ignore = "requires sudo privileges"]
fn create_arp_socket() {
    ArpSocket::try_new(false, false).expect("Failed to create socket");
}
