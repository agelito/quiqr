use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use image::Luma;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use qrcode::QrCode;
use std::io::{self, Read};
use structopt::StructOpt;

#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

const MAX_DIM_W: u32 = 360;
const MAX_DIM_H: u32 = 360;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "e")]
    input: Option<String>,
    #[structopt(short = "c", long = "clipboard")]
    clipboard: Option<Option<bool>>,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    debug!("Options: {:?}", opt);
    let encode_str = match (opt.input, opt.clipboard) {
        (Some(e), None) => Ok(String::from(&e)),
        (None, Some(_)) => read_clipboard(),
        (Some(_), Some(_)) => Err(anyhow!("Invalid input arguments.")),
        (None, None) => read_stdin(),
    }?;

    debug!("Generate QR for: {}", encode_str);
    let qr = QrCode::new(&encode_str)?;

    let image = qr
        .render::<Luma<u8>>()
        .max_dimensions(MAX_DIM_W, MAX_DIM_H)
        .build();

    let width = image.width();
    let height = image.height();

    debug!("QR Image: {}x{}", width, height);

    let mut buffer = Vec::<u32>::new();
    for pixel in image.pixels() {
        let lum = pixel[0];
        let bytes: [u8; 4] = [lum, lum, lum, 255];
        let color: u32 = u32::from_ne_bytes(bytes);

        buffer.push(color);
    }

    let window_options = WindowOptions {
        borderless: true,
        resize: false,
        scale: Scale::X1,
        scale_mode: ScaleMode::AspectRatioStretch,
        title: false,
        transparency: false,
        topmost: true,
    };

    debug!("Creating window with options: {:?}", window_options);

    let mut window = Window::new(
        &format!("QR - {}", encode_str),
        width as usize,
        height as usize,
        window_options,
    )?;

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    while window.is_open() {
        if window.is_key_down(Key::Escape) || window.is_key_down(Key::Q) {
            break;
        }

        window
            .update_with_buffer(buffer.as_slice(), width as usize, height as usize)
            .expect("Couldn't present image buffer.");
    }

    Ok(())
}

fn read_clipboard() -> Result<String> {
    let mut ctx: ClipboardContext =
        ClipboardProvider::new().map_err(|_| anyhow!("Couldn't get clipboard context"))?;
    ctx.get_contents()
        .map_err(|_| anyhow!("Couldn't get clipboard contents"))
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}
