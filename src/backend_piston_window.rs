use crate::config::Config;
use crate::{Action, MVResult, MiniView, MiniViewError, ResizableWhen};
use clap::crate_name;
use piston_window::{
    Button, ButtonArgs, Event, EventLoop, G2dTexture, G2dTextureContext, GenericEvent, Input, Key,
    Loop, PistonWindow, Texture, TextureSettings, Window, WindowSettings,
};
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;
use std::thread;

pub(crate) struct ImageWindow {
    window: PistonWindow,
}

impl ImageWindow {
    pub fn try_new(config: &Config, size: [u32; 2]) -> MVResult<ImageWindow> {
        let mut window: PistonWindow = WindowSettings::new(crate_name!(), size)
            .fullscreen(config.fullscreen())
            .exit_on_esc(true)
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
        self.window.draw_2d(event, |c, g, _device| {
            piston_window::image(texture, c.transform, g);
        });
    }

    fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    fn create_texture_context(&mut self) -> G2dTextureContext {
        self.window.create_texture_context()
    }
}

impl Debug for ImageWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageWindow").finish()
    }
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

pub(crate) fn show(config: Config) -> MVResult<MiniView> {
    let (sender, receiver) = mpsc::channel();

    let source = config.source();
    let img = source.open()?;

    let width = img.width();
    let height = img.height();
    let img = img.to_rgba8();

    let handle = thread::spawn(move || {
        let mut window = ImageWindow::try_new(&config, [width, height])?;

        let texture = Texture::from_image(
            &mut window.create_texture_context(),
            &img,
            &TextureSettings::new(),
        )
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
                    Event::Input(Input::Close(_), _) => {
                        window.close_window();
                        return Ok(());
                    }
                    Event::Input(Input::Button(ButtonArgs { button, .. }), _)
                        if button == Button::Keyboard(Key::Escape) =>
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

    Ok(MiniView { sender, handle })
}
