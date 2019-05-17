extern crate image as imgcrate; // There is also an image crate in piston_window

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use piston_window::*;
use imgcrate::DynamicImage;
use crate::imgcrate::GenericImageView;

const SUPPORTED_IMAGE_FORMATS: &'static [&'static str] = &[
    "bmp",
    "gif",
    "jpeg",
    "png",
    "pbm",
    "pgm",
    "ppm",
    "pam",
    "tiff",
    "webp-luma-lossy",
];

const IMPORT_FROM_PATH_CLI: &str = "import_from_path";
const IMPORT_FROM_STDIN_BYTES: &str = "import_from_stdin_bytes";
const IMPORT_FROM_STDIN_PATH: &str = "import_from_stdin_path";

// Something seems to hog a ton of memory (perhaps leak).
// Perhaps it will be better to use the lower level gfx tools instead of piston_window.

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name(IMPORT_FROM_PATH_CLI)
                .long("from-path")
                .conflicts_with_all(&[IMPORT_FROM_STDIN_BYTES, IMPORT_FROM_STDIN_PATH])
                .takes_value(true)
                .value_name("PATH TO IMAGE")
                .help("<>")
                .required(false),
        )
        .arg(
            Arg::with_name(IMPORT_FROM_STDIN_BYTES)
                .long("from-stdin-bytes")
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_PATH])
                .takes_value(true)
                .value_name("TYPE")
                .possible_values(SUPPORTED_IMAGE_FORMATS)
                .help("<>")
                .required(false),
        )
        .arg(
            Arg::with_name("import_from_stdin_path")
                .long("from-stdin-path")
                .conflicts_with_all(&[IMPORT_FROM_PATH_CLI, IMPORT_FROM_STDIN_BYTES])
                .help("<>")
                .required(false),
        )
}

enum ImportFromSource {
    CliOptPath(String),
    StdinBytes(imgcrate::ImageFormat),
    StdinPath(String),
}

impl ImportFromSource {
    fn new(matches: &ArgMatches) -> Self {
        match (matches.is_present(IMPORT_FROM_PATH_CLI),
               matches.is_present(IMPORT_FROM_STDIN_BYTES),
               matches.is_present(IMPORT_FROM_STDIN_PATH)) {
            (true, _, _) => ImportFromSource::CliOptPath(matches.value_of(IMPORT_FROM_PATH_CLI).unwrap().to_string()),
            (_, true, _) => panic!("Unsupported"),
            (_, _, true) => ImportFromSource::CliOptPath(matches.value_of(IMPORT_FROM_STDIN_PATH).unwrap().to_string()),
            _ => panic!("Unable to determine input mode."),
        }
    }

    fn open(&self) -> DynamicImage {
        match &self {
            ImportFromSource::CliOptPath(path) => imgcrate::open(&path).expect(&format!("Unable to find image path: {}", path)),
            _ => panic!("Unsupported mode."),
        }
    }
}

fn main() {
    let app = app().get_matches();

    let source = ImportFromSource::new(&app);
    let img = source.open();

    let width = img.width();
    let height = img.height();

    let img = img.to_rgba();

    let mut window: PistonWindow = WindowSettings::new(crate_name!(), [width, height])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .expect("Unable to create a PistonWindow.");

    let mut tex = Texture::from_image(
        &mut window.factory,
        &img,
        &TextureSettings::new(),
    ).expect("Unable to create an image mapped texture.");


    window.set_lazy(true);
    while let Some(e) = window.next() {
        tex.update(&mut window.encoder, &img).unwrap();

        window.draw_2d(&e, |c, g| {
            clear([0.9, 0.9, 0.9, 1.0], g);
            image(
                &tex,
                c.transform,
                g,
            );
        });
    }
}
