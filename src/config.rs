use crate::Source;
use std::fmt::{Debug, Formatter};

pub struct Config {
    source: Source,
    fullscreen: bool,
    resizable_window: bool,
    lazy_window: bool,
    exit_when: Option<Box<dyn Fn() -> bool>>,
    window_name: &'static str,
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

    pub fn exit_when(&self) -> Option<&dyn Fn() -> bool> {
        self.exit_when.as_ref().map(|inner| inner.as_ref())
    }
}

impl<'a> Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Config(source = {:?}, fullscreen = {:?}, resizable_window = {:?}, window_name = {:?}, ...)",
                             self.source, self.fullscreen, self.resizable_window, self.window_name))
    }
}

#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl<'a> ConfigBuilder {
    pub fn new(source: Source) -> Self {
        ConfigBuilder {
            config: Config {
                source,
                fullscreen: false,
                resizable_window: false,
                lazy_window: true,
                window_name: "miniview",
                exit_when: None,
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

    /// Close the window in certain cases.
    ///
    /// Warning: if the window uses lazy updates, this may not work as intended (as user interaction
    /// is required to poll successive window events).
    pub fn close_window_when(mut self, value: Option<Box<dyn Fn() -> bool>>) -> Self {
        self.config.exit_when = value;
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}
