![github_actions_ci](https://img.shields.io/badge/github-foresterre/miniview-blue?labelColor=555555&logo=github)
![github_actions_ci](https://img.shields.io/crates/v/miniview.svg?color=fc8d62&logo=rust)
![github_actions_ci](https://img.shields.io/github/workflow/status/foresterre/miniview/github_actions_ci/master)

# miniview
_'mini' as in, it can't do much =)_

Display an image within a (graphical) window.


# Install

With [cargo](https://crates.io/crates/miniview) install: `cargo install --force miniview`

Pre build binary: see [releases](https://github.com/foresterre/miniview/releases)


# Usage

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

<br>

**Keyboard shortcuts**

Press `ESC` to exit the image window.

# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

🎸 🎺 🎻 🎷
