use crate::config::Config;
use crate::{MVResult, MiniView, MiniViewError};
use clap::crate_name;
use imagecrate::GenericImageView;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;
use std::thread;

struct ImageWindow {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl ImageWindow {
    pub fn try_new(config: &Config, size: [u32; 2]) -> MVResult<ImageWindow> {
        let size = winit::dpi::PhysicalSize::new(size[0] as f64, size[1] as f64);
        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_title(crate_name!())
            .with_min_inner_size(size)
            .build(&event_loop)
            .map_err(|_| MiniViewError::UnableToCreateWindow)?;

        Ok(ImageWindow { window, event_loop })
    }
}

impl Debug for ImageWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageWindow").finish()
    }
}

pub(crate) fn show(config: Config) -> MVResult<MiniView> {
    unimplemented!()
}

pub(crate) fn close(mini_view: MiniView) -> MVResult<()> {
    unimplemented!()
}
pub(crate) fn wait_for_exit(mini_view: MiniView) -> MVResult<()> {
    unimplemented!()
}
