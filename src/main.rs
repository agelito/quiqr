// #![windows_subsystem = "windows"]

use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use image::Luma;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use qrcode::QrCode;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

const MAX_DIM_W: u32 = 360;
const MAX_DIM_H: u32 = 360;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "e", long = "encode", help = "Display QR code from text.")]
    input: Option<String>,
    #[structopt(
        short = "c",
        long = "clipboard",
        help = "Display QR code from clipboard contents."
    )]
    clipboard: bool,
    #[structopt(
        short = "s",
        long = "stdin",
        help = "Display QR code from stdin contents."
    )]
    stdin: bool,
    #[structopt(
        short = "w",
        long = "write-file",
        help = "Write resulting QR code to image file.",
        parse(from_os_str)
    )]
    save_image: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    debug!("options: {:?}", opt);
    let encode_str = match (opt.input, opt.clipboard, opt.stdin) {
        (Some(e), false, false) => Ok(String::from(&e)),
        (None, true, false) => read_clipboard(),
        (None, false, true) => read_stdin(),
        _ => Err(anyhow!(
            "Please select one of `encode`, `clipboard`, or `stdin` options."
        )),
    }?;

    debug!("encode data: {}", encode_str);
    let qr = QrCode::new(&encode_str)?;

    let image = qr
        .render::<Luma<u8>>()
        .max_dimensions(MAX_DIM_W, MAX_DIM_H)
        .build();

    let width = image.width();
    let height = image.height();

    debug!("QR image dimensions: {}x{}", width, height);

    if let Some(save_path) = opt.save_image {
        image.save(save_path)?;
    }

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

    debug!("window options: {:?}", window_options);

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
