use miniview::config::ConfigBuilder;
use std::time::Duration;

fn main() {
    let start = std::time::Instant::now();

    let timer = Box::new(move || {
        let time_passed = std::time::Instant::now().duration_since(start);
        time_passed >= Duration::new(1, 0)
    });

    let config =
        ConfigBuilder::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/plant.jpg"))
            .set_fullscreen(true)
            .set_lazy_window(false)
            .close_window_when(Some(timer))
            .build();

    miniview::show(&config).unwrap()
}
