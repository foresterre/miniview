extern crate image as imagecrate; // There is also an image module in piston_window

use anyhow::Context;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};
use miniview::config::ConfigBuilder;
use miniview::errors::MiniViewError;
use miniview::io::read_path_from_stdin_block;
use miniview::Source;

const IMPORT_FROM_PATH_CLI: &str = "import_from_path";
const IMPORT_FROM_STDIN_BYTES: &str = "import_from_stdin_bytes";
const IMPORT_FROM_STDIN_PATH: &str = "import_from_stdin_path";
const POSITIONAL_FROM_PATH: &str = "positional_from_path";
const OPTION_FULLSCREEN: &str = "fullscreen";
const OPTION_WINDOW_RESIZE: &str = "window_resize";
const OPTION_CLOSE_AFTER: &str = "close_after_ms";

// Perhaps it will be better to use the lower level gfx tools instead of piston_window.
fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::NextLineHelp)
        .usage("miniview (<PATH> OR --from-path <PATH> OR --from-stdin-bytes OR --from-stdin-path) \
            [--fullscreen] \
            [--allow-window-resizing] \
            [--close-after <ms>]")
        .arg(
            Arg::with_name(IMPORT_FROM_PATH_CLI)
                .long("from-path")
                .short("p")
                .takes_value(true)
                .value_name("PATH")
                .help("Load an image from the given path and display it.")
                .conflicts_with_all(&[IMPORT_FROM_STDIN_BYTES, IMPORT_FROM_STDIN_PATH, POSITIONAL_FROM_PATH])
                .required_unless_one(&[IMPORT_FROM_STDIN_BYTES, IMPORT_FROM_STDIN_PATH, POSITIONAL_FROM_PATH]),
        )
        .arg(
            Arg::with_name("import_from_stdin_path")
                .long("from-stdin-path")
                .short("s")
                .help("Load an image from the path received by stdin and display it.")
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_BYTES, POSITIONAL_FROM_PATH])
                .required_unless_one(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_BYTES, POSITIONAL_FROM_PATH]),
        )
        .arg(
            Arg::with_name(IMPORT_FROM_STDIN_BYTES)
                .long("from-stdin-bytes")
                .short("b")
                .help("Load an image received by stdin (image as bytes), guess its format and display it.")
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH, POSITIONAL_FROM_PATH])
                .required_unless_one(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH, POSITIONAL_FROM_PATH]),
        )
        .arg(
            Arg::with_name(POSITIONAL_FROM_PATH)
                .help("Load an image from the given path and display it.")
                .index(1)
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH, IMPORT_FROM_STDIN_BYTES])
                .required_unless_one(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH, IMPORT_FROM_STDIN_BYTES]),
        )
        .arg(
            Arg::with_name(OPTION_FULLSCREEN)
                .help("Instruct the window to go into fullscreen mode")
                .long("fullscreen")
        )
        .arg(
            Arg::with_name(OPTION_WINDOW_RESIZE)
                .help("Allow window resizing (doesn't resize the image)")
                .long("allow-window-resizing")
        )
        .arg(
            Arg::with_name(OPTION_CLOSE_AFTER)
                .help("Close the window after n milliseconds; implies a non-lazy window")
                .long("close-after")
                .takes_value(true)
                .number_of_values(1)
                .validator(|f| f.parse::<u64>().map(|_| ()).map_err(|_| String::from("value should be a natural number")))
        )
}

fn determine_source(matches: &ArgMatches) -> Result<Source, MiniViewError> {
    match (
        matches.is_present(IMPORT_FROM_PATH_CLI),
        matches.is_present(IMPORT_FROM_STDIN_PATH),
        matches.is_present(IMPORT_FROM_STDIN_BYTES),
        matches.is_present(POSITIONAL_FROM_PATH),
    ) {
        (true, false, false, false) => {
            let path = matches
                .value_of(IMPORT_FROM_PATH_CLI)
                .ok_or_else(|| MiniViewError::EmptyInputPath)?;
            let path = Source::ByPath(path.into());

            Ok(path)
        }
        (false, true, false, false) => Ok(Source::ByPath(read_path_from_stdin_block()?.into())),
        (false, false, true, false) => Ok(Source::StdinBytes),
        (false, false, false, true) => {
            let path = matches
                .value_of(POSITIONAL_FROM_PATH)
                .ok_or_else(|| MiniViewError::EmptyInputPath)?;
            let path = Source::ByPath(path.into());

            Ok(path)
        }
        _ => Err(MiniViewError::CliUnableToDetermineInputMode),
    }
}

fn stop_after(start: std::time::Instant, ms: u64) -> bool {
    let time_passed = std::time::Instant::now().duration_since(start);

    time_passed >= std::time::Duration::from_millis(ms)
}

fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();
    let source = determine_source(&matches)?;

    let mut builder = ConfigBuilder::new(source)
        .set_fullscreen(matches.is_present(OPTION_FULLSCREEN))
        .allow_resizable_window(matches.is_present(OPTION_WINDOW_RESIZE));

    if let Some(ms) = matches.value_of(OPTION_CLOSE_AFTER) {
        let time = ms.parse::<u64>()?;
        let start = std::time::Instant::now();

        builder = builder.close_window_when(Some(Box::new(move || stop_after(start, time))));
        builder = builder.set_lazy_window(false);
    }

    let config = builder.build();

    miniview::show(&config).with_context(|| "miniview failed")
}
