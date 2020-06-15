//! In order to display the miniview window, a configuration which defines how the view should be
//! presented is required. This [`configuration`] may be constructed manually, or by using the
//! [`ConfigBuilder`] which leverages the builder pattern.
//!
//! [`configuration`]: struct.Config.html
//! [`ConfigBuilder`]: struct.ConfigBuilder.html
//! [`show`]: ../struct.MiniView.html#method.show

use crate::Source;
use std::fmt::{Debug, Formatter};

/// Configuration which can be [`provided`] to a miniview window controlling instance which enables
/// different program behaviours
///
/// [`provided`]: ../struct.MiniView.html#method.show
pub struct Config {
    source: Source,
    fullscreen: bool,
    resizable_window: bool,
    lazy_window: bool,
    window_name: &'static str,
}

impl Config {
    /// Source of the image
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Whether the window should open in fullscreen mode or not
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Whether the window should be resizable or not
    ///
    /// If [`fullscreen`] mode is active for the window, this setting is implied.
    ///
    /// [`fullscreen`]: struct.Config.html#method.fullscreen
    pub fn resizable_window(&self) -> bool {
        self.resizable_window
    }

    /// Whether the window continuously or lazily checks for input events
    ///
    /// Note: if this is set to true, the [`MiniView.close`] method will only close upon receiving
    /// some input event.
    pub fn lazy_window(&self) -> bool {
        self.lazy_window
    }

    pub fn window_name(&self) -> &str {
        self.window_name
    }
}

impl<'window_name> Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Config(source = {:?}, fullscreen = {:?}, resizable_window = {:?}, window_name = {:?}, ...)",
                             self.source, self.fullscreen, self.resizable_window, self.window_name))
    }
}

/// Builder to conveniently create a miniview [`configuration`]
///
/// [`configuration`]: struct.Config.html
#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Create a builder from the provided [`source`]
    ///
    /// [`source`]: ../enum.Source.html
    pub fn new(source: Source) -> Self {
        ConfigBuilder {
            config: Config {
                source,
                fullscreen: false,
                resizable_window: false,
                lazy_window: false,
                window_name: "miniview",
            },
        }
    }

    /// Creates a builder from the provided path
    ///
    /// Path should point to an image on the filesystem.
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self::new(Source::ByPath(path.as_ref().to_path_buf()))
    }

    /// Source of the image to be shown by the window. Can be a path or raw bytes imported from the
    /// stdin pipe
    pub fn source(mut self, value: Source) -> Self {
        self.config.source = value;
        self
    }

    /// Activates fullscreen mode
    pub fn set_fullscreen(mut self, value: bool) -> Self {
        self.config.fullscreen = value;
        self
    }

    /// Allow window resizing
    ///
    /// Note: Upon resizing of the window, the image itself is not resized
    /// Note: Fullscreen mode implies window resizing
    pub fn allow_resizable_window(mut self, value: bool) -> Self {
        self.config.resizable_window = value;
        self
    }

    /// Lazily update the window
    ///
    /// Warning: if the window uses lazy updates, events may not work as expected (input events are
    /// is required to poll successive window events in lazy window mode).
    pub fn set_lazy_window(mut self, value: bool) -> Self {
        self.config.lazy_window = value;
        self
    }

    /// Title of the window; useful when trying to capture the window from another program.
    pub fn window_name(mut self, value: &'static str) -> Self {
        self.config.window_name = value;
        self
    }

    /// Construct a configuration from the default and overridden configuration values.
    pub fn build(self) -> Config {
        self.config
    }
}
