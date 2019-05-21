use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum MiniViewError {
    EmptyInputPath,
    FailedToImport(ImportError),
    CliUnableToDetermineInputMode,
    UnableToCreateWindow,
    UnableToMapImage,
}

#[derive(Debug)]
pub enum ImportError {
    OnPathNotFound,
    OnStdinUnableToRead,
    OnStdinPathWasEmpty,
    OnStdinBytesStreamWasEmpty,
    OnStdinBytesUnableToGuessOrLoadFormat,
}

impl Display for MiniViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for MiniViewError {
    fn description(&self) -> &str {
        match *self {
            MiniViewError::EmptyInputPath => "No path to an image was provided.",
            MiniViewError::FailedToImport(ImportError::OnPathNotFound) => {
                "Provided path to image could not be found or loaded."
            }
            MiniViewError::FailedToImport(ImportError::OnStdinUnableToRead) => {
                "Unable to read from the stdin."
            }
            MiniViewError::FailedToImport(ImportError::OnStdinPathWasEmpty) => {
                "Given path to image was empty."
            }
            MiniViewError::FailedToImport(ImportError::OnStdinBytesStreamWasEmpty) => {
                "Stdin was empty."
            }
            MiniViewError::FailedToImport(ImportError::OnStdinBytesUnableToGuessOrLoadFormat) => {
                "The input received from stdin could not be loaded."
            }
            MiniViewError::CliUnableToDetermineInputMode => "Unable to determine input mode.",
            MiniViewError::UnableToCreateWindow => {
                "Unable to create a window to display the image."
            }
            MiniViewError::UnableToMapImage => "Unable to map the image to a texture.",
        }
    }
}
