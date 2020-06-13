use common::input;
use miniview::config::ConfigBuilder;
use miniview::MiniView;
use std::time::Duration;

mod common;

#[test]
fn exit_after_100ms() {
    let config = ConfigBuilder::from_path(input())
        .set_fullscreen(true)
        .set_lazy_window(false)
        .build();

    let controls = MiniView::show(config).expect("unable to create miniview");
    std::thread::sleep(Duration::from_millis(100));
    assert!(controls.close().is_ok());
}
