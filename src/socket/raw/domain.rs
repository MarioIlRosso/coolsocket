/// Socket domains specify how and where a socket communicates.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketDomain {
    /// Useful for IPC on Unix systems.
    ///
    /// Equivalent to [`libc::AF_LOCAL`]/[`libc::AF_UNIX`].
    Local = libc::AF_LOCAL,
    /// Used for communicating with IPV4 addresses.
    ///
    /// Equivalent to [`libc::AF_INET`].
    Ipv4 = libc::AF_INET,
    /// The amateur radio AX.25 protocol.
    ///
    /// Equivalent to [`libc::AF_AX25`].
    Ax25 = libc::AF_AX25,
    /// IPX protocol.
    ///
    /// Equivalent to [`libc::AF_IPX`].
    Ipx = libc::AF_IPX,
    /// `AppleTalk` protocol.
    ///
    /// Equivalent to [`libc::AF_APPLETALK`].
    AppleTalk = libc::AF_APPLETALK,
    /// ITU-T X.25 / ISO/IEC 8208 protocol.
    ///
    /// Equivalent to [`libc::AF_X25`].
    X25 = libc::AF_X25,
    /// Used for communicating with IPV6 addresses.
    ///
    /// Equivalent to [`libc::AF_INET6`].
    Ipv6 = libc::AF_INET6,
    /// `DECet` protocol.
    ///
    /// Equivalent to [`libc::AF_DECnet`].
    DecNet = libc::AF_DECnet,
    /// Key management protocol, originally developed for usage with `IPsec`
    ///
    /// Equivalent to [`libc::AF_KEY`].
    Key = libc::AF_KEY,
    /// Kernel user interface device.
    ///
    /// Equivalent to [`libc::AF_NETLINK`].
    Netlink = libc::AF_NETLINK,
    /// Low level packet interface.
    ///
    /// Equivalent to [`libc::AF_PACKET`].
    Packet = libc::AF_PACKET,
    /// Reliable Datagram Sockets protocol.
    ///
    /// Equivalent to [`libc::AF_RDS`].
    Rds = libc::AF_RDS,
    /// Generic PPP transport layer, for setting up L2 tunnels (L2TP and `PPPoE`)
    ///
    /// Equivalent to [`libc::AF_PPPOX`].
    Pppox = libc::AF_PPPOX,
    /// Logical Link Control protocol.
    ///
    /// Equivalent to [`libc::AF_LLC`].
    Llc = libc::AF_LLC,
    /// InfiniBand native addressing.
    ///
    /// Equivalent to [`libc::AF_IB`].
    InfiniBand = libc::AF_IB,
    /// Multi Protocol Label Switching.
    ///
    /// Equivalent to [`libc::AF_MPLS`].
    Mpls = libc::AF_MPLS,
    /// Controller Area Network automotive bus protocol
    ///
    /// Equivalent to [`libc::AF_CAN`].
    CtrlAreaNetwork = libc::AF_CAN,
    /// Cluster domain sockets protocol.
    ///
    /// Equivalent to [`libc::AF_TIPC`].
    Tipc = libc::AF_TIPC,
    /// Bluetooth low level socket protocol.
    ///
    /// Equivalent to [`libc::AF_BLUETOOTH`].
    Bluetooth = libc::AF_BLUETOOTH,
    /// Interface to kernel crypto API.
    ///
    /// Equivalent to [`libc::AF_ALG`].
    Alg = libc::AF_ALG,
    /// VSOCK (originally "`VMWare` `VSockets`") protocol for hypervisor-guest.
    ///
    /// Equivalent to [`libc::AF_VSOCK`].
    Vsock = libc::AF_VSOCK,
    /// eXpress Data Path interface.
    ///
    /// Equivalent to [`libc::AF_XDP`].
    Xdp = libc::AF_XDP,
}
