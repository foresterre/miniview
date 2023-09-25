use crate::config::Config;
use crate::{Action, FullscreenWhen, MVResult, MiniView, MiniViewError};
use clap::crate_name;
use imagecrate::EncodableLayout;
use pixels::{Pixels, SurfaceTexture};
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;
use std::thread;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use winit::platform::unix::EventLoopBuilderExtUnix;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Fullscreen, WindowBuilder};
use winit_input_helper::WinitInputHelper;

struct ImageWindow {
    window: winit::window::Window,
}

impl ImageWindow {
    pub fn try_new(
        config: &Config,
        size: [u32; 2],
        event_loop: &EventLoop<()>,
    ) -> MVResult<ImageWindow> {
        let size = winit::dpi::PhysicalSize::new(size[0] as f64, size[1] as f64);

        let window = WindowBuilder::new()
            .with_title(crate_name!())
            .with_inner_size(size)
            .fullscreen_when(|| config.fullscreen())
            .with_resizable(config.resizable_window())
            .build(event_loop)
            .map_err(|_| MiniViewError::UnableToCreateWindow)?;

        Ok(ImageWindow { window })
    }
}

impl Debug for ImageWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageWindow").finish()
    }
}

impl FullscreenWhen for WindowBuilder {
    fn fullscreen_when<P: Fn() -> bool>(self, predicate: P) -> Self {
        let fullscreen = if predicate() {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };

        self.with_fullscreen(fullscreen)
    }
}

pub(crate) fn show(config: Config) -> MVResult<MiniView> {
    let (sender, receiver) = mpsc::channel();

    let source = config.source();
    let img = source.open()?;

    let width = img.width();
    let height = img.height();
    let img = img.to_rgba8();

    let handle = thread::spawn(move || {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "windows"
        ))]
        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

        // FIXME: this will probably crash, since we explicitly start the event loop off thread.
        //   As a result, macos is not supported for now
        #[cfg(target_os = "macos")]
        let event_loop = EventLoop::new();

        let mut input = WinitInputHelper::new();
        let image_window = ImageWindow::try_new(&config, [width, height], &event_loop)?;

        let mut pixels = {
            let window_size = image_window.window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &image_window.window);
            Pixels::new(width, height, surface_texture)
                .map_err(|_| MiniViewError::UnableToMapImage)?
        };

        event_loop.run(move |event, _target, control_flow| {
            // Pause event loop to save cpu time and power
            if config.lazy_window() {
                *control_flow = ControlFlow::Wait;
            }

            // Exit when receiving the Close action
            if let Ok(action) = receiver.try_recv() {
                match action {
                    Action::Close => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                };
            }

            if input.update(&event) {
                // Exit when either pressing escape or the close button
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() || input.destroyed() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize
                if let Some(size) = input.window_resized() {
                    let _ = pixels.resize_surface(size.width, size.height);
                }

                // Redraw on change
                image_window.window.request_redraw();
            }

            // Redraw the image, if requested
            if let Event::RedrawRequested(_id) = event {
                let frame = pixels.frame_mut();
                frame.copy_from_slice(img.as_bytes());

                let _ = pixels.render();
            }
        });

        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(MiniView { sender, handle })
}
