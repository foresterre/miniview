extern crate image as imagecrate; // There is also an image module in piston_window

use std::error::Error;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};
use imagecrate::DynamicImage;
use imagecrate::GenericImageView;
use piston_window::*;

use crate::errors::{ImportError, MiniViewError};
use crate::io::{import_image_from_stdin_bytes_block, read_path_from_stdin_block};

mod errors;
mod io;

const IMPORT_FROM_PATH_CLI: &str = "import_from_path";
const IMPORT_FROM_STDIN_BYTES: &str = "import_from_stdin_bytes";
const IMPORT_FROM_STDIN_PATH: &str = "import_from_stdin_path";

// Perhaps it will be better to use the lower level gfx tools instead of piston_window.
fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::NextLineHelp)
        .usage("miniview [--from-path <PATH> OR --from-stdin-bytes OR --from-stdin-path]")
        .arg(
            Arg::with_name(IMPORT_FROM_PATH_CLI)
                .long("from-path")
                .conflicts_with_all(&[IMPORT_FROM_STDIN_BYTES, IMPORT_FROM_STDIN_PATH])
                .takes_value(true)
                .value_name("PATH")
                .help("Load and an image from the given path and display it.")
                .required_unless_one(&[IMPORT_FROM_STDIN_BYTES, IMPORT_FROM_STDIN_PATH]),
        )
        .arg(
            Arg::with_name("import_from_stdin_path")
                .long("from-stdin-path")
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_BYTES])
                .help("Load and an image from the path received by stdin and display it.")
                .required_unless_one(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_BYTES]),
        )
        .arg(
            Arg::with_name(IMPORT_FROM_STDIN_BYTES)
                .long("from-stdin-bytes")
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH])
                .help("Load and an image received by stdin (image as bytes), guess its format and display it.")
                .required_unless_one(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH]),
        )
}

enum ImportFromSource {
    ByPath(String),
    StdinBytes,
}

impl ImportFromSource {
    fn try_new(matches: &ArgMatches) -> Result<Self, MiniViewError> {
        match (
            matches.is_present(IMPORT_FROM_PATH_CLI),
            matches.is_present(IMPORT_FROM_STDIN_PATH),
            matches.is_present(IMPORT_FROM_STDIN_BYTES),
        ) {
            (true, false, false) => {
                let path = matches
                    .value_of(IMPORT_FROM_PATH_CLI)
                    .ok_or_else(|| MiniViewError::EmptyInputPath)?;
                let path = ImportFromSource::ByPath(path.to_string());

                Ok(path)
            }
            (false, true, false) => Ok(ImportFromSource::ByPath(read_path_from_stdin_block()?)),
            (false, false, true) => Ok(ImportFromSource::StdinBytes),
            _ => Err(MiniViewError::CliUnableToDetermineInputMode),
        }
    }

    fn open(&self) -> Result<DynamicImage, MiniViewError> {
        match &self {
            ImportFromSource::ByPath(path) => imagecrate::open(&path)
                .map_err(|_| MiniViewError::FailedToImport(ImportError::OnPathNotFound)),
            ImportFromSource::StdinBytes => import_image_from_stdin_bytes_block(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app().get_matches();

    let source = ImportFromSource::try_new(&app)?;
    let img = source.open()?;

    let width = img.width();
    let height = img.height();
    let img = img.to_rgba();

    let mut window: PistonWindow = WindowSettings::new(crate_name!(), [width, height])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .map_err(|_| MiniViewError::UnableToCreateWindow)?;

    let tex = Texture::from_image(&mut window.factory, &img, &TextureSettings::new())
        .map_err(|_| MiniViewError::UnableToMapImage)?;

    window.set_lazy(true);
    while let Some(frame) = window.next() {
        window.draw_2d(&frame, |c, g| {
            image(&tex, c.transform, g);
        });
    }

    Ok(())
}
