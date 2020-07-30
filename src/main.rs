use structopt::StructOpt;
use std::io::{self, Read};
use std::convert::TryInto;
use qrcode::QrCode;
use image::Rgba;
use minifb::{Window, WindowOptions, Key};

const WIDTH: u32 = 260;
const HEIGHT: u32 = 260;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "e")]
    input: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let encode_str = match opt.input {
        Some(e) => String::from(&e),
        None => read_stdin(),
    };

    let qr = QrCode::new(&encode_str)
        .expect("Couldn't create QR code from input.");

    let image = qr.render::<Rgba<u8>>()
        .max_dimensions(WIDTH, HEIGHT)
        .build();

    image.save("out.png").expect("Couldn't save image.");

    let raw_image_bytes = image.into_raw();

    let mut buffer = Vec::<u32>::new();

    let mut index = 0;
    while index < raw_image_bytes.len() {

        let bytes = &raw_image_bytes[index..index + 4];
        let bytes_array: [u8; 4] = 
            bytes.try_into().expect("Couldn't convert byte array");
        let color: u32 = u32::from_ne_bytes(bytes_array);

        buffer.push(color);

        index += 4;
    }

    assert_eq!((WIDTH * HEIGHT * 4) as usize, raw_image_bytes.len());
    assert_eq!((WIDTH * HEIGHT) as usize, buffer.len());

    let mut window = Window::new(&format!("QR - {}", encode_str), WIDTH as usize, HEIGHT as usize, WindowOptions::default())
        .expect("Couldn't create window.");

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    while window.is_open() {

        if window.is_key_down(Key::Escape) || window.is_key_down(Key::Q) {
            break;
        }

        window.update_with_buffer(buffer.as_slice(), WIDTH as usize, HEIGHT as usize)
            .expect("Couldn't present image buffer.");
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();

    // NOTE: If couldn't read from stdin it's fine. Just return
    // empty string in that case.
    io::stdin().read_to_string(&mut buffer).ok();

    buffer
}
