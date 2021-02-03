[<img alt="github" src="https://img.shields.io/badge/github-foresterre/miniview-blue?labelColor=555555&logo=github" height="20">](https://github.com/foresterre/miniview)
[<img alt="crates.io" src="https://img.shields.io/crates/v/miniview.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/miniview)
[<img alt="ci" src="https://img.shields.io/github/workflow/status/foresterre/miniview/github_actions_ci/master" height="20">](https://github.com/foresterre/miniview/actions?query=workflow%3Agithub_actions_ci+branch%3Amaster+)
[<img alt="docs-rs" src="https://docs.rs/miniview/badge.svg" height="20">](https://docs.rs/miniview)

# miniview
_'mini' as in, it can't do much =)_

Display an image within a (graphical) window.

* Use from the [cli](https://github.com/foresterre/miniview#usage-binary)
* Use as [library](https://github.com/foresterre/miniview#usage-example-library)
    * Documentation: [docs.rs](https://docs.rs/miniview/)


# Install

With [cargo](https://crates.io/crates/miniview) install: `cargo install --force miniview`

Pre build binary: see [releases](https://github.com/foresterre/miniview/releases)


# Usage (binary)

| Usage | Linux example | Windows example (cmd.exe) |
|----------------------------------------|------------------------------------------------|------------------------------------------------|
| `miniview <PATH_TO_IMAGE>` | `miniview image.png` | `miniview image.png` |
| `miniview --from-path <PATH_TO_IMAGE>` | `miniview --from-path image.png` | `miniview --from-path  image.png` |
| `miniview --from-stdin-path` | `echo image.png \| miniview --from-stdin-path`  | `echo image.png \| miniview --from-stdin-path` |
| `miniview --from-stdin-bytes` | `cat image.png \| miniview --from-stdin-bytes` | `type image.png \| miniview --from-stdin-bytes` |

<br>

**Flags &amp; Options**

| Flag/Option | Description | 
| ---    | ---         |
| `--fullscreen` | Set the window to fullscreen |
| `--allow-window-resizing` | Allow the window to resize (doesn't resize the image!) |
| `--close-after <n>` | Close the window after `n` milliseconds |

<br>

**Keyboard shortcuts**

Press `ESC` to exit the image window.

# Usage example (library)

```rust
use miniview::config::ConfigBuilder;
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
```

# Backends

MiniView supports two backends: piston-window and pixels. You can switch between backends on compile time. This requires
setting Cargo [features](https://doc.rust-lang.org/cargo/reference/features.html). The piston-window backend can be
enabled using the `backend_piston_window` feature, and the pixels backend can be enabled using the `backend_pixels` feature.

The default backend is **pixels**. This backend will be used if no-default-features is not specified. Note that the default backend
is not available on MacOS.

The next sections provide examples, on how to enable each backend. Only one backend should be enabled at a time.

## backend: piston-window

### Platform support

Supported platforms:
* any platform supported by [piston-window](https://github.com/PistonDevelopers/piston_window) with Glutin, including:
  * Linux
  * MacOS
  * Windows

### Configuration examples

When building MiniView, the piston-window backend can be used by compiling with:
```bash
cargo run --no-default-features --features backend_piston_window 
```

When using MiniView as a library, you can use:
```toml
[dependencies.miniview]
version = "*" # select the latest version here
default-features = false 
features = ["backend_piston_window"]
```

or 

```toml
[dependencies]
miniview = { version = "*", default-features = false, features = ["backend_piston_window"] }
```

## backend: pixels

### Platform support

Supported platforms:
  * Linux
  * Dragonfly
  * FreeBSD
  * NetBSD
  * OpenBSD
  * Windows

Note: MacOS is not yet supported for this backend.

### Configuration examples

When building MiniView, the pixels backend can be used by compiling with:
```bash
cargo run --no-default-features --features backend_pixels
```

When using MiniView as a library, you can use:
```toml
[dependencies.miniview]
version = "*" # select the latest version here
default-features = false 
features = ["backend_pixels"]
```

or

```toml
[dependencies]
miniview = { version = "*", default-features = false, features = ["backend_pixels"] }
```

# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

🎸 🎺 🎻 🎷
