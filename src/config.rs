use crate::Source;
use std::fmt::{Debug, Formatter};

pub struct Config {
    source: Source,
    fullscreen: bool,
    resizable_window: bool,
    lazy_window: bool,
    window_name: &'static str,
    exit_on_esc: bool,
}

impl Config {
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn resizable_window(&self) -> bool {
        self.resizable_window
    }

    pub fn lazy_window(&self) -> bool {
        self.lazy_window
    }

    pub fn window_name(&self) -> &str {
        self.window_name
    }

    pub fn exit_on_esc(&self) -> bool {
        self.exit_on_esc
    }
}

impl<'window_name> Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Config(source = {:?}, fullscreen = {:?}, resizable_window = {:?}, window_name = {:?}, ...)",
                             self.source, self.fullscreen, self.resizable_window, self.window_name))
    }
}

#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new(source: Source) -> Self {
        ConfigBuilder {
            config: Config {
                source,
                fullscreen: false,
                resizable_window: false,
                lazy_window: false,
                window_name: "miniview",
                exit_on_esc: true,
            },
        }
    }

    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self::new(Source::ByPath(path.as_ref().to_path_buf()))
    }

    /// Where to load the image from
    pub fn source(mut self, value: Source) -> Self {
        self.config.source = value;
        self
    }

    /// Activates fullscreen
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
    /// Warning: if the window uses lazy updates, events may not work as expected (user interaction
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

    /// Whether to signal the window to close upon pressing the escape (ESC) key
    pub fn exit_on_esc(mut self, value: bool) -> Self {
        self.config.exit_on_esc = value;
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}
