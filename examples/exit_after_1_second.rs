use miniview::{ConfigBuilder, MiniView};
use std::time::Duration;

fn main() {
    let config =
        ConfigBuilder::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/plant.jpg"))
            .set_fullscreen(true)
            .set_lazy_window(false)
            .build();

    let controls = MiniView::show(config).expect("unable to create miniview");

    // do some important other work!
    std::thread::sleep(Duration::from_millis(1000));

    let closed = controls.close();
    assert!(closed.is_ok());
}
