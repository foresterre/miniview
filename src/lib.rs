extern crate image as imagecrate; // There is also an image module in piston_window

use crate::config::Config;
use crate::errors::{ImportError, MiniViewError};
use crate::io::import_image_from_stdin_bytes_block;
use clap::crate_name;
use imagecrate::{DynamicImage, GenericImageView};
use piston_window::*;
use std::path::PathBuf;

pub mod config;
pub mod errors;
pub mod io;

trait ResizableWhen {
    fn resizable_when<P: Fn() -> bool>(self, predicate: P) -> Self;
}

impl ResizableWhen for WindowSettings {
    fn resizable_when<P: Fn() -> bool>(self, predicate: P) -> Self {
        if predicate() {
            self.resizable(true)
        } else {
            self.resizable(false)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Source {
    ByPath(PathBuf),
    StdinBytes,
}

impl Source {
    fn open(&self) -> Result<DynamicImage, MiniViewError> {
        match &self {
            Source::ByPath(path) => imagecrate::open(path.as_path())
                .map_err(|_| MiniViewError::FailedToImport(ImportError::OnPathNotFound)),
            Source::StdinBytes => import_image_from_stdin_bytes_block(),
        }
    }
}

pub fn show(config: &Config) -> Result<(), MiniViewError> {
    let source = config.source();
    let img = source.open()?;

    let width = img.width();
    let height = img.height();
    let img = img.to_rgba();

    let mut window: PistonWindow = WindowSettings::new(crate_name!(), [width, height])
        .fullscreen(config.fullscreen())
        .exit_on_esc(true)
        .resizable_when(|| {
            // if window resizing is not enabled, when setting fullscreen to true, the window won't go
            // into fullscreen mode
            config.fullscreen() || config.resizable_window()
        })
        .build()
        .map_err(|_| MiniViewError::UnableToCreateWindow)?;

    let tex = Texture::from_image(&mut window.factory, &img, &TextureSettings::new())
        .map_err(|_| MiniViewError::UnableToMapImage)?;

    window.set_lazy(config.lazy_window());
    while let Some(event) = window.next() {
        if let Some(should_exit) = config.exit_when() {
            if should_exit() {
                window.set_should_close(true);
            }
        }

        window.draw_2d(&event, |c, g| {
            image(&tex, c.transform, g);
        });
    }

    Ok(())
}
