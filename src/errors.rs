use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiniViewError {
    #[error("No path to an image was provided.")]
    EmptyInputPath,

    #[error(transparent)]
    FailedToImport(#[from] ImportError),

    #[error("Unable to determine input mode.")]
    CliUnableToDetermineInputMode,

    #[error("Unable to create a window to display the image.")]
    UnableToCreateWindow,

    #[error("Unable to map the image to a texture.")]
    UnableToMapImage,
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Provided path to image could not be found or loaded.")]
    OnPathNotFound,

    #[error("Unable to read from the stdin.")]
    OnStdinUnableToRead,

    #[error("Given path to image was empty.")]
    OnStdinPathWasEmpty,

    #[error("Stdin was empty.")]
    OnStdinBytesStreamWasEmpty,

    #[error("The input received from stdin could not be loaded.")]
    OnStdinBytesUnableToGuessOrLoadFormat,
}
