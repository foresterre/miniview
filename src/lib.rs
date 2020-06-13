extern crate image as imagecrate; // There is also an image module in piston_window

use crate::config::Config;
use crate::errors::ImportError;
use crate::io::import_image_from_stdin_bytes_block;
use clap::crate_name;
use imagecrate::{DynamicImage, GenericImageView};
use piston_window::*;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub use crate::config::ConfigBuilder;
pub use crate::errors::MiniViewError;

pub mod config;
pub mod errors;
pub mod io;

pub type MVResult<T> = Result<T, MiniViewError>;

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
    fn open(&self) -> MVResult<DynamicImage> {
        match &self {
            Source::ByPath(path) => imagecrate::open(path.as_path())
                .map_err(|_| MiniViewError::FailedToImport(ImportError::OnPathNotFound)),
            Source::StdinBytes => import_image_from_stdin_bytes_block(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Close,
}

struct ImageWindow {
    window: PistonWindow,
}

impl ImageWindow {
    pub fn try_new(config: &Config, size: [u32; 2]) -> MVResult<ImageWindow> {
        let mut window: PistonWindow = WindowSettings::new(crate_name!(), size)
            .fullscreen(config.fullscreen())
            .resizable_when(|| {
                // if window resizing is not enabled, when setting fullscreen to true, the window won't go
                // into fullscreen mode
                config.fullscreen() || config.resizable_window()
            })
            .build()
            .map_err(|_| MiniViewError::UnableToCreateWindow)?;

        window.set_lazy(config.lazy_window());

        Ok(Self { window })
    }

    fn next(&mut self) -> Option<Event> {
        self.window.next()
    }

    fn draw_image<E: GenericEvent>(&mut self, event: &E, texture: &G2dTexture) {
        self.window.draw_2d(event, |c, g| {
            image(texture, c.transform, g);
        });
    }

    fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    fn device_factory(&self) -> GfxFactory {
        self.window.factory.clone()
    }
}

impl Debug for ImageWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageWindow").finish()
    }
}

pub struct MiniView {
    sender: mpsc::Sender<Action>,
    handle: thread::JoinHandle<Result<(), MiniViewError>>,
}

impl MiniView {
    pub fn show(config: Config) -> MVResult<Self> {
        let (sender, receiver) = mpsc::channel();

        let source = config.source();
        let img = source.open()?;

        let width = img.width();
        let height = img.height();
        let img = img.to_rgba();

        let handle = thread::spawn(move || {
            let mut window = ImageWindow::try_new(&config, [width, height])?;

            let texture =
                Texture::from_image(&mut window.device_factory(), &img, &TextureSettings::new())
                    .map_err(|_| MiniViewError::UnableToMapImage)?;

            loop {
                if let Ok(action) = receiver.try_recv() {
                    return match action {
                        Action::Close => {
                            window.close_window();
                            Ok(())
                        }
                    };
                }

                if let Some(event) = window.next() {
                    match event {
                        Event::Input(Input::Close(_)) => {
                            window.close_window();
                            return Ok(());
                        }
                        Event::Input(Input::Button(ButtonArgs { button, .. }))
                            if button == Button::Keyboard(Key::Escape) && config.exit_on_esc() =>
                        {
                            window.close_window();
                            return Ok(());
                        }
                        Event::Loop(Loop::AfterRender(_)) => continue,
                        _ => {}
                    }
                    window.draw_image(&event, &texture);
                }
            }
        });

        Ok(Self { sender, handle })
    }

    pub fn close(self) -> MVResult<()> {
        self.sender
            .send(Action::Close)
            .map_err(|_err| MiniViewError::SendStopError)?;

        self.handle
            .join()
            .map_err(|_err| MiniViewError::ViewThreadFailedToJoin)
            .and_then(|inner| inner)
    }

    pub fn wait_for_exit(self) -> MVResult<()> {
        self.handle
            .join()
            .map_err(|_err| MiniViewError::ViewThreadFailedToJoin)
            .and_then(|inner| inner)
    }
}
