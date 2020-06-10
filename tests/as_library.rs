use common::input;
use miniview::config::ConfigBuilder;
use std::time::Duration;

mod common;

#[test]
fn exit_after_1_second() {
    let start = std::time::Instant::now();

    let f = Box::new(move || {
        let time_passed = std::time::Instant::now().duration_since(start);
        time_passed >= Duration::new(1, 0)
    });

    let config = ConfigBuilder::from_path(input())
        .set_fullscreen(true)
        .set_lazy_window(false)
        .close_window_when(Some(f))
        .build();

    miniview::show(&config).unwrap()
}
