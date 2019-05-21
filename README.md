[![Build Status](https://travis-ci.org/foresterre/miniview.svg?branch=master)](https://travis-ci.org/foresterre/miniview)

# miniview
_'mini' as in, it can't do much =)_

Display an image within a (graphical) window.


# Install

With [cargo](https://crates.io/crates/miniview) install: `cargo install --force miniview`

Pre build binary: see [releases](https://github.com/foresterre/miniview/releases)


# Usage

| Usage | Linux example | Windows example (cmd.exe) |
|----------------------------------------|------------------------------------------------|------------------------------------------------|
| `miniview --from-path <PATH_TO_IMAGE>` | `miniview --from-path image.png` | `miniview --from-path  image.png` |
| `miniview --from-stdin-path` | `echo image.png \| miniview --from-stdin-path`  | `echo image.png \| miniview --from-stdin-path` |
| `miniview --from-stdin-bytes` | `cat image.png \| miniview --from-stdin-bytes` | `type image.png \| miniview --from-stdin-bytes` |

Press `ESC` to exit the image window.

# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

:guitar: :trumpet: :violin: :saxophone:
