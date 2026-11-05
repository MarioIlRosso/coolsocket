use std::os::fd::OwnedFd;

#[derive(Debug)]
pub struct ArpSocket {
    file_descriptor: OwnedFd,
}
