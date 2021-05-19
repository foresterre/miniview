[<img alt="github" src="https://img.shields.io/badge/github-foresterre/miniview-blue?labelColor=555555&logo=github" height="20">](https://github.com/foresterre/miniview)
[<img alt="crates.io" src="https://img.shields.io/crates/v/miniview.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/miniview)
[<img alt="ci" src="https://img.shields.io/github/workflow/status/foresterre/miniview/github_actions_ci/master" height="20">](https://github.com/foresterre/miniview/actions?query=workflow%3Agithub_actions_ci+branch%3Amaster+)
[<img alt="docs-rs" src="https://docs.rs/miniview/badge.svg" height="20">](https://docs.rs/miniview)

# miniview

MiniView is a bare-bones image viewer intended to be used during development and testing.
MiniView can be called as binary from the CLI, and used as a Rust library.

To see what it can do, and would fit your purpose, please see:
* Use from the [cli](https://github.com/foresterre/miniview#instructions-for-cli-use)
* Use as [library](https://github.com/foresterre/miniview#instructions-for-library-use)
    * Documentation: [docs.rs](https://docs.rs/miniview/)

MiniView is not intended to be used as your primary image viewer. For that purpose, I would recommend [Emulsion](https://github.com/ArturKovacs/emulsion).

# Install

### Binary installation

With [cargo](https://crates.io/crates/miniview) install: `cargo install --force miniview`

Pre-build binary: see [releases](https://github.com/foresterre/miniview/releases)

### Add library dependency

Run `cargo add miniview` with [cargo-edit](https://crates.io/crates/cargo-edit), or add `miniview` as a dependency to your `Cargo.toml`:

```toml
miniview = "*" # replace `*` with the latest version
```


# Usage

### Instructions for CLI use

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

### Instructions for library use

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

| Platform | Supported | Tested | Notes |
|----|----|----|-----|
| Linux | ✅ | ✅ ||
| MacOS |   |    | MacOS does not allow the creation of graphical windows off the main thread.  |
| Windows | ✅ | ✅  ||
| ... other [piston-window](https://github.com/PistonDevelopers/piston_window) + [glutin](https://github.com/rust-windowing/glutin) platforms | ✅ | | Assuming graphical windows can be created off the main thread.

### Configuration examples

When building MiniView, the piston-window backend can be used by compiling with:
```bash
cargo run --no-default-features --features backend_piston_window 
```

When using MiniView as a library, you can use:
```toml
[dependencies.miniview]
version = "*" # replace `*` with the latest version
default-features = false 
features = ["backend_piston_window"]
```

or 

```toml
[dependencies]
miniview = { version = "*", default-features = false, features = ["backend_piston_window"] }
```

NB: replace `*` in `version = "*"` with [any supported version](https://crates.io/crates/miniview/versions).

## backend: pixels

### Platform support


| Platform | Supported | Tested | Notes |
|----|----|----|-----|
| Linux | ✅ | ✅ ||
| Windows | ✅ | ✅  ||
| MacOS |   |    | MacOS does not allow the creation of graphical windows off the main thread.  |
| FreeBSD | ✅ |||
| DragonflyBSD | ✅ |||
| NetBSD | ✅ |||
| OpenBSD | ✅ |||
| ... other [pixels](https://github.com/parasyte/pixels) + [winit](https://github.com/rust-windowing/winit) platforms | | | Assuming graphical windows can be created off the main thread, support can be added. Please open an [issue](https://github.com/foresterre/miniview/issues).


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
version = "*" # replace `*` with the latest version
default-features = false 
features = ["backend_pixels"]
```

or

```toml
[dependencies]
miniview = { version = "*", default-features = false, features = ["backend_pixels"] }
```

NB: replace `*` in `version = "*"` with [any supported version](https://crates.io/crates/miniview/versions).

# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

🎸 🎺 🎻 🎷
