use libc;

/// The type of socket.
///
/// This is an abstraction over [`libc::socket`]'s type parameter.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketKind {
    /// The type of socket.
    ///
    /// This controls the semantics of communication of the socket.
    pub sock_type: SocketType,
    /// Controls if the socket is non blocking.
    ///
    /// Equivalent to using [`libc::SOCK_NONBLOCK`].
    pub non_blocking: bool,
    /// Controls if the socket will be closed when the process uses any function from the `exec`
    /// family to spawn a new process.
    ///
    /// While this operation can be done using [`libc::fcntl`], this is useful in multithreaded
    /// programs to atomically set this option to the socket.
    ///
    /// Equivalent to using [`libc::SOCK_CLOEXEC`]
    pub close_on_exec: bool,
}

impl SocketKind {
    /// Creates a new socket type given a [`SocketType`], and other parameters
    pub const fn new(sock_type: SocketType, non_blocking: bool, close_on_exec: bool) -> Self {
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

/// The semantics of a socket.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketType {
    /// Makes the socket use a TCP connection, providing an ordered, reliable stream.
    ///
    /// Equivalent to [`libc::SOCK_STREAM`]
    Stream = libc::SOCK_STREAM,
    /// Makes the socket use a UDP connection, providing unreliable datagram messaging.
    ///
    /// Equivalent to [`libc::SOCK_DGRAM`]
    Datagram = libc::SOCK_DGRAM,
    /// Makes the socket use a TCP connection, providing an ordered, reliable stream.
    ///
    /// This is similiar to [`SocketType::Stream`], but makes sure the order of packets sent is the
    /// same for both sender and receiver.
    ///
    /// Equivalent to [`libc::SOCK_SEQPACKET`]
    SequentialPacket = libc::SOCK_SEQPACKET,
    /// Makes the socket use raw network protocols.
    ///
    /// Equivalent to [`libc::SOCK_RAW`]
    Raw = libc::SOCK_RAW,
    /// Makes the socket use reliable, unordered, datagram messaging.
    ///
    /// Equivalent to [`libc::SOCK_RDM`]
    Rdm = libc::SOCK_RDM,
    /// Makes the socket use packets.
    ///
    /// This shouldn't be used, instead, [`crate::socket::raw::SocketDomain::Packet`]
    ///
    /// Equivalent to [`libc::SOCK_PACKET`]
    #[allow(deprecated)]
    Packet = libc::SOCK_PACKET,
}
