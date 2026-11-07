use crate::socket::raw::{RawSocket, RawSocketCreationError};

/// Arp socket, to work with the ARP protocol.
///
/// This struct will automatically manage the socket's file descriptor.
#[derive(Debug)]
pub struct ArpSocket {
    raw_socket: RawSocket,
}

impl ArpSocket {
    /// Creates a new [`ArpSocket`], provided other parameters.
    ///
    /// Note that this operation may commonly fail, as creating an ARP socket requires elevated
    /// privileges.
    ///
    /// # Errors
    ///
    /// If any of these conditions happen, the function will return an error.
    ///
    /// - Permission to create socket was denied (returns [`RawSocketCreationError::NoPermission`]).
    /// - Unavailable protocol (returns [`RawSocketCreationError::UnknownProtocol`]).
    /// - If the process open fd limit is reached (returns [`RawSocketCreationError::ProcessFdLimitReached`]).
    /// - If the system open fd limit is reached (returns [`RawSocketCreationError::SystemFdLimitReached`]).
    /// - The system doesn't have enough resources to create the socket (returns [`RawSocketCreationError::NoResources`]).
    /// - protocol and domain arguments are incompatible (returns [`RawSocketCreationError::IncompatibleProtocol`]).
    /// - General error (returns [`RawSocketCreationError::Other`]).
    pub fn try_new(
        non_blocking: bool,
        close_on_exec: bool,
    ) -> Result<Self, RawSocketCreationError> {
        use crate::socket::raw::{SocketDomain, SocketKind, SocketType};

        let socket_kind = SocketKind::new(SocketType::Raw, non_blocking, close_on_exec);
        let socket =
            RawSocket::try_new(SocketDomain::Packet, socket_kind, libc::ETH_P_ARP.to_be())?;

        let arp_socket = ArpSocket { raw_socket: socket };

        Ok(arp_socket)
    }

    /// Transforms this [`ArpSocket`] into a [`RawSocket`].
    ///
    /// Note that transforming a [`RawSocket`] back into an [`ArpSocket`] is an unsafe operation.
    pub fn into_raw_socket(self) -> RawSocket {
        self.into()
    }

    /// Transforms this [`RawSocket`] into an [`ArpSocket`].
    ///
    /// # Safety
    ///
    /// This function expects the [`RawSocket`] to be a valid ARP socket, and not just any socket.
    /// If the passed socket is not an ARP socket, the program may not behave as expected.
    pub unsafe fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self { raw_socket }
    }
}

impl From<ArpSocket> for RawSocket {
    /// Transforms this [`ArpSocket`] into a [`RawSocket`].
    ///
    /// Note that transforming a [`RawSocket`] back into an [`ArpSocket`] is an unsafe operation.
    fn from(value: ArpSocket) -> Self {
        value.raw_socket
    }
}
