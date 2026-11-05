#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketKind {
    sock_type: SocketType,
    non_blocking: bool,
    close_on_exec: bool,
}

impl SocketKind {
    pub fn new(sock_type: SocketType, non_blocking: bool, close_on_exec: bool) -> Self {
        Self { sock_type, non_blocking, close_on_exec }
    }
}

impl From<SocketKind> for i32 {
    fn from(value: SocketKind) -> Self {
        use libc::{SOCK_CLOEXEC, SOCK_NONBLOCK};

        let sock_type_raw: i32 = value.sock_type as i32;
        let non_blocking_raw: i32 = if value.non_blocking { SOCK_NONBLOCK } else { 0 };
        let close_on_exec_raw: i32 = if value.close_on_exec { SOCK_CLOEXEC } else { 0 };

        sock_type_raw | non_blocking_raw | close_on_exec_raw
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketType {
    Stream,
    Datagram,
    SequentialPacket,
    Raw,
    Rdm,
    Packet,
}
