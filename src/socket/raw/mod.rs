mod domain;
mod kind;

pub use domain::SocketDomain;
pub use kind::{SocketKind, SocketType};
use std::os::fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd};
use thiserror::Error;

/// Error used by [`RawSocket::new`].
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum RawSocketCreationError {
    #[doc = "Permission to create a socket of the specified type and/or protocol is denied."]
    #[error("permission to create a socket of the specified type and/or protocol is denied")]
    NoPermission,
    #[doc = "Unknown protocol, or invalid flags in kind."]
    #[error("unknown protocol, or invalid flags in kind")]
    UnknownProtocol,
    #[doc = "The per-process limit of open file descriptors has been reached."]
    #[error("the per-process limit of open file descriptors has been reached")]
    ProcessFdLimitReached,
    #[doc = "The system limit of open file descriptors has been reached."]
    #[error("the system limit of open file descriptors has been reached")]
    SystemFdLimitReached,
    #[doc = "Not enough system resources/memory to create socket."]
    #[error("not enough system resources/memory to create socket")]
    NoResources,
    #[doc = "Protocol is incompatible with the domain."]
    #[error("protocol is incompatible with the domain")]
    IncompatibleProtocol,
    /// General error that an implementation of the socket could return.
    #[error(transparent)]
    Other(#[from] std::io::Error),
}

/// Struct to handle a socket file descriptor.
///
/// Instead of using this struct, prefer using the socket abstractions provided in the crate.
#[derive(Debug)]
pub struct RawSocket {
    file_descriptor: OwnedFd,
}

impl RawSocket {
    /// Creates a new [`RawSocket`].
    ///
    /// This uses [`libc::socket`].
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
    /// - General error (returns [`RawSocketCreationError::IncompatibleProtocol`]).
    pub fn new(
        domain: SocketDomain,
        kind: SocketKind,
        protocol: i32,
    ) -> Result<Self, RawSocketCreationError> {
        use libc::socket;
        use std::io::Error;

        let raw_file_descriptor = unsafe { socket(domain as i32, kind.into(), protocol) };

        // Error in the socket call.
        if raw_file_descriptor < 0 {
            let err = Error::last_os_error();
            let raw_err = err.raw_os_error().unwrap();

            return match raw_err {
                libc::EACCES => Err(RawSocketCreationError::NoPermission),
                libc::EINVAL => Err(RawSocketCreationError::UnknownProtocol),
                libc::EMFILE => Err(RawSocketCreationError::ProcessFdLimitReached),
                libc::ENFILE => Err(RawSocketCreationError::SystemFdLimitReached),
                libc::ENOBUFS | libc::ENOMEM => Err(RawSocketCreationError::NoResources),
                libc::EPROTONOSUPPORT => Err(RawSocketCreationError::IncompatibleProtocol),
                _ => Err(RawSocketCreationError::Other(err)),
            };
        }

        let file_descriptor = unsafe { OwnedFd::from_raw_fd(raw_file_descriptor) };

        Ok(Self { file_descriptor })
    }
}

impl AsFd for RawSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.file_descriptor.as_fd()
    }
}
