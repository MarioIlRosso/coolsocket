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
    #[error("permission to create a socket of the specified type and/or protocol is denied")]
    NoPermission,
    #[error("unknown protocol, or invalid flags in kind")]
    UnknownProtocol,
    #[error("the per-process limit of open file descriptors has been reached")]
    ProcessFdLimitReached,
    #[error("the system limit of open file descriptors has been reached")]
    SystemFdLimitReached,
    #[error("not enough system resources/memory to create socket")]
    NoResources,
    #[error("protocol is incompatible with the domain")]
    IncompatibleProtocol,
    #[error("{0}")]
    Other(String),
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
    /// # Safety
    /// The `domain`, `kind`, and `protocol` argument must all be compatible with each other.
    pub unsafe fn new(
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
                libc::ENOBUFS | libc::ENOMEM => Err(RawSocketCreationError::SystemFdLimitReached),
                libc::EPROTONOSUPPORT => Err(RawSocketCreationError::IncompatibleProtocol),
                _ => Err(RawSocketCreationError::Other(err.to_string())),
            };
        }

        let file_descriptor = unsafe { OwnedFd::from_raw_fd(raw_file_descriptor) };

        Ok(Self { file_descriptor })
    }

    /// Returns the file descriptor of the [`RawSocket`].
    pub fn fd(&self) -> BorrowedFd<'_> {
        self.file_descriptor.as_fd()
    }
}
