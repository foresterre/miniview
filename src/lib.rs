//! MiniView is a small program which allows to show a single image in a graphical window.
//! It supports both windowed mode and fullscreen mode and can be useful for debugging or testing
//! programs dealing with images.
//!
//! MiniView can be used both as a binary executable (usually `miniview` or `miniview.exe`),
//! or as a library.
//!
//! If you want to use a `miniview` binary, provide the `--help` flag for information on its usage
//! and options. In addition, you could take a look at the [`readme`].
//!
//! For library usage you may want to start by looking at the [`MiniView.show`] method and
//! [`ConfigBuilder`] struct, to respectively create a `MiniView` window controlling instance and
//! conveniently create a configuration which is required for `MiniView.show`.
//!
//! Feel free to post questions, issues, suggestions and feedback at the [`issue tracker`].
//!
//! Example usage:
//!
//! ```rust
//! use miniview::{ConfigBuilder, MiniView};
//! use std::time::Duration;
//!
//! let config = ConfigBuilder::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/plant.jpg"))
//!         .set_fullscreen(true)
//!         .build();
//!
//! let controls = MiniView::show(config).expect("unable to create miniview");
//!
//! // do some important other work!
//! std::thread::sleep(Duration::from_millis(1000));
//!
//! let closed = controls.close();
//! assert!(closed.is_ok());
//! ```
//!
//! [`issue tracker`]: https://github.com/foresterre/miniview/issues
//! [`readme`]: https://github.com/foresterre/miniview/blob/master/README.md
//! [`MiniView.show`]: struct.MiniView.html#method.show
//! [`ConfigBuilder`]: config/struct.ConfigBuilder.html

extern crate image as imagecrate; // There is also an image module in piston_window

use crate::config::Config;
use crate::errors::ImportError;
use crate::io::import_image_from_stdin_bytes_block;
use imagecrate::DynamicImage;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub use crate::config::ConfigBuilder;
pub use crate::errors::MiniViewError;

#[cfg(feature = "backend_piston_window")]
pub(crate) mod backend_piston_window;
#[cfg(feature = "backend_pixels")]
pub(crate) mod backend_pixels;

pub mod config;
pub mod errors;
pub mod io;

/// A convenience type alias which represents a regular [`Result`] where the error type is
/// represented by the [`MiniViewError`], which is the top-level error type for this crate.
///
/// [`Result`]: https://doc.rust-lang.org/stable/core/result/enum.Result.html
/// [`MiniViewError`]: errors/enum.MiniViewError.html
pub type MVResult<T> = Result<T, MiniViewError>;

trait ResizableWhen {
    fn resizable_when<P: Fn() -> bool>(self, predicate: P) -> Self;
}

trait FullscreenWhen {
    fn fullscreen_when<P: Fn() -> bool>(self, predicate: P) -> Self;
}

/// The source of an image which will be shown by the view
#[derive(Debug, Clone)]
pub enum Source {
    /// A path which points at an image file, e.g. `/home/myuser/image.png` or
    /// `C:/Users/MyUser/image.png`.
    ByPath(PathBuf),

    /// A raw (as in an image formatted using a supported encoding as byte stream) image piped or
    /// otherwise provided to the stdin
    StdinBytes,
}

impl Source {
    /// Load the image to memory
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

/// Provides the controls to show and consecutively close a `miniview` window
///
/// For more, see [`show`].
///
/// [`show`]: struct.MiniView.html#method.show
pub struct MiniView {
    sender: mpsc::Sender<Action>,
    handle: thread::JoinHandle<Result<(), MiniViewError>>,
}

impl MiniView {
    /// Create the controls to a new `miniview` window
    ///
    /// This will spawn a thread which will manage and create a graphical window. The
    /// [`MiniView`] struct on the main thread can be used to control the window.
    ///
    /// The window can be closed explicitly by calling [`close`] or we can wait until the user will
    /// close the window manually by using [`wait_for_exit`] instead.
    ///
    /// When a [`MiniView`] instance goes out of scope and is dropped, the thread managing the
    /// graphical image view window will also die.
    ///
    /// [`MiniView`]: struct.MiniView.html
    /// [`close`]: struct.MiniView.html#method.close
    /// [`wait_for_exit`]: struct.MiniView.html#method.wait_for_exit
    pub fn show(config: Config) -> MVResult<Self> {
        #[cfg(feature = "backend_piston_window")]
        {
            backend_piston_window::show(config)
        }

        #[cfg(feature = "backend_pixels")]
        {
            backend_pixels::show(config)
        }
    }

    /// Sends a 'close window' event to the thread managing the graphical window and waits for the
    /// thread to return
    ///
    /// Compared to [`wait_for_exit`] this method will explicitly attempt to close the window,
    /// and should close almost instantly. This method blocks the until the window has been closed,
    /// and the thread has been returned, _or_ an error has been returned instead.
    ///
    /// [`wait_for_exit`]: struct.MiniView.html#method.wait_for_exit
    pub fn close(self) -> MVResult<()> {
        close(self)
    }

    /// Waits until the thread managing the graphical window returns
    ///
    /// Compared to [`close`] which attempts to instantaneously close the window regardless of user
    /// input, this method will block and wait for the user to close the window.
    ///
    /// [`close`]: struct.MiniView.html#method.close
    pub fn wait_for_exit(self) -> MVResult<()> {
        wait_for_exit(self)
    }
}

pub(crate) fn close(mini_view: MiniView) -> MVResult<()> {
    mini_view
        .sender
        .send(Action::Close)
        .map_err(|_err| MiniViewError::SendStopError)?;

    mini_view
        .handle
        .join()
        .map_err(|_err| MiniViewError::ViewThreadFailedToJoin)
        .and_then(|inner| inner)
}

pub(crate) fn wait_for_exit(mini_view: MiniView) -> MVResult<()> {
    mini_view
        .handle
        .join()
        .map_err(|_err| MiniViewError::ViewThreadFailedToJoin)
        .and_then(|inner| inner)
}
