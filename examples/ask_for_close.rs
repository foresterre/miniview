use miniview::{ConfigBuilder, MiniView};
use std::io::stdin;

fn main() {
    let config =
        ConfigBuilder::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/plant.jpg"))
            .exit_on_esc(false)
            .build();

    // This works because the object gets destroyed at the end of the scope!
    // If that weren't the case, we could close our view with `_view.close()`
    let _view = MiniView::show(config).expect("unable to create miniview");

    println!("Type 'close' to close the window!");
    let mut buffer = String::new();
    while let Ok(_) = stdin().read_line(&mut buffer) {
        if buffer.trim() == "close" {
            break;
        } else {
            buffer.clear();
        }
    }
}
