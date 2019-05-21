use std::io::{stdin, Read};

use crate::errors::{ImportError, MiniViewError};

pub(crate) fn import_image_from_stdin_bytes_block() -> Result<image::DynamicImage, MiniViewError> {
    let mut buffer = Vec::new();

    stdin()
        .lock()
        .read_to_end(&mut buffer)
        .map_err(|_| MiniViewError::FailedToImport(ImportError::OnStdinUnableToRead))?;

    if buffer.is_empty() {
        return Err(MiniViewError::FailedToImport(
            ImportError::OnStdinBytesStreamWasEmpty,
        ));
    }

    imagecrate::load_from_memory(&buffer).map_err(|_| {
        MiniViewError::FailedToImport(ImportError::OnStdinBytesUnableToGuessOrLoadFormat)
    })
}

pub(crate) fn read_path_from_stdin_block() -> Result<String, MiniViewError> {
    let mut path = String::new();

    stdin()
        .lock()
        .read_to_string(&mut path)
        .map_err(|_| MiniViewError::FailedToImport(ImportError::OnStdinUnableToRead))?;

    if path.is_empty() {
        return Err(MiniViewError::FailedToImport(
            ImportError::OnStdinPathWasEmpty,
        ));
    }

    Ok(path.trim().to_string())
}
