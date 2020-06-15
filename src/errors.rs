//! Errors which signal faulty behaviour.
//!
//! MiniView attempts to always return an error in case of faulty behaviour instead of crashing
//! by virtue of panicking.

use thiserror::Error;

/// The top-level error type
#[derive(Error, Debug)]
pub enum MiniViewError {
    /// Returned in case an empty input path is given
    #[error("No path to an image was provided.")]
    EmptyInputPath,

    /// Returned in case an image can not be loaded from a [`source`].
    ///
    /// [`source`]: ../enum.Source.html
    #[error(transparent)]
    FailedToImport(#[from] ImportError),

    /// Returned if the input mode could not be defined
    #[error("Unable to determine input mode.")]
    CliUnableToDetermineInputMode,

    /// Returned if inter-thread communication trough a multi-producer single-consumer channel
    /// failed
    #[error("Unable to signal window to stop showing")]
    SendStopError,

    /// Created when it was not possible to create a graphical window
    #[error("Unable to create a window to display the image.")]
    UnableToCreateWindow,

    /// Returned if an image could not be mapped to the texture which is shown by the image view
    /// in the window
    #[error("Unable to map the image to a texture.")]
    UnableToMapImage,

    /// Returned when a thread does not exit properly
    #[error("View thread exited improperly")]
    ViewThreadFailedToJoin,
}

/// Errors related to loading an image from a path or stream
#[derive(Error, Debug)]
pub enum ImportError {
    /// Returned when a path can't be found on the filesystem
    #[error("Provided path to image could not be found or loaded.")]
    OnPathNotFound,

    /// Returned when the program is unable to read from the stdin
    #[error("Unable to read from the stdin.")]
    OnStdinUnableToRead,

    /// Returned when the program expects a qualified path from the stdin, but the given path is empty  
    #[error("Given path to image was empty.")]
    OnStdinPathWasEmpty,

    /// Returned when the program expects an ordered stream of bytes (the formatted image) from the
    /// stdin, but the stdin did not receive any input
    #[error("Stdin was empty.")]
    OnStdinBytesStreamWasEmpty,

    /// Returned when the program expects an ordered stream of bytes (the formatted image) from the
    /// stdin, but the given bytes could not be decoded
    #[error("The input received from stdin could not be loaded.")]
    OnStdinBytesUnableToGuessOrLoadFormat,
}
