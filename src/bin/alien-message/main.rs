use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo_apps::color::Color;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const MESSAGES: &[&str] = &[
    // Arecibo message
    "\
    00000010101010000000000\
    00101000001010000000100\
    10001000100010010110010\
    10101010101010100100100\
    00000000000000000000000\
    00000000000011000000000\
    00000000001101000000000\
    00000000001101000000000\
    00000000010101000000000\
    00000000011111000000000\
    00000000000000000000000\
    11000011100011000011000\
    10000000000000110010000\
    11010001100011000011010\
    11111011111011111011111\
    00000000000000000000000\
    00010000000000000000010\
    00000000000000000000000\
    00001000000000000000001\
    11111000000000000011111\
    00000000000000000000000\
    11000011000011100011000\
    10000000100000000010000\
    11010000110001110011010\
    11111011111011111011111\
    00000000000000000000000\
    00010000001100000000010\
    00000000001100000000000\
    00001000001100000000001\
    11111000001100000011111\
    00000000001100000000000\
    00100000000100000000100\
    00010000001100000001000\
    00001100001100000010000\
    00000011000100001100000\
    00000000001100110000000\
    00000011000100001100000\
    00001100001100000010000\
    00010000001000000001000\
    00100000001100000000100\
    01000000001100000000100\
    01000000000100000001000\
    00100000001000000010000\
    00010000000000001100000\
    00001100000000110000000\
    00100011101011000000000\
    00100000001000000000000\
    00100000111110000000000\
    00100001011101001011011\
    00000010011100100111111\
    10111000011100000110111\
    00000000010100000111011\
    00100000010100000111111\
    00100000010100000110000\
    00100000110110000000000\
    00000000000000000000000\
    00111000001000000000000\
    00111010100010101010101\
    00111000000000101010100\
    00000000000000101000000\
    00000000111110000000000\
    00000011111111100000000\
    00001110000000111000000\
    00011000000000001100000\
    00110100000000010110000\
    01100110000000110011000\
    01000101000001010001000\
    01000100100010010001000\
    00000100010100010000000\
    00000100001000010000000\
    00000100000000010000000\
    00000001001010000000000\
    01111001111101001111000\
    ",
];

const COLORS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pixel {
    Background,
    Foreground,
}

#[derive(Clone, Debug)]
struct Frame {
    pixels: Array2d<i64, Pixel>,
}
impl Frame {
    pub fn new(bbox: BBox2d) -> Frame {
        let pixels = Array2d::with(bbox, |_| Pixel::Background);
        Frame { pixels }
    }
    pub fn bbox(&self) -> BBox2d {
        self.pixels.bbox()
    }
    pub fn set(&mut self, pos: Point2d, pixel: Pixel) {
        if self.bbox().contains(&pos) {
            self.pixels[pos] = pixel;
        }
    }
}

fn send<T: Write>(w: &mut T, frame: &Frame) -> Result<()> {
    for y in frame.bbox().y_range().rev() {
        for x in frame.bbox().x_range() {
            let pixel = frame.pixels[p2d(x, y)];
            let color = match pixel {
                Pixel::Background => Color::black(),
                Pixel::Foreground => Color::new(0x99, 0xcc, 0),
            };
            w.write_all(&color.rgb())?;
        }
    }
    Ok(w.flush()?)
}

const DEFAULT_ARG: usize = 0;

fn main() -> Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<usize>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let frame_size = x_size * y_size * COLORS;

    let x_size = i64::try_from(x_size)?;
    let y_size = i64::try_from(y_size)?;

    let bbox = bb2d(0..x_size as i64, 0..y_size as i64);
    let mut frame = Frame::new(bbox);

    let delay = Duration::from_millis(1000);

    let raw_message = *MESSAGES.get(arg).unwrap_or(&MESSAGES[0]);
    let msg = raw_message.chars().map(|c| c == '1').collect::<Vec<bool>>();
    let len = msg.len();
    eprintln!("Message has length {}", len);

    // Factor length
    let mut msg_size = None;
    for i in 2..len {
        if len % i == 0 {
            let x = i64::try_from(len / i).unwrap();
            let y = i64::try_from(i).unwrap();
            msg_size = Some(v2d(x, y));
            break;
        }
    }
    let msg_size = msg_size.ok_or("message length cannot be factorized")?;
    let x_msg_size = msg_size.x();
    let y_msg_size = msg_size.y();
    let msg_bbox = bb2d(0..x_msg_size, 0..y_msg_size);
    eprintln!("Message has format ({}, {})", x_msg_size, y_msg_size);

    // Start positions for rendering (lower left pixel).
    let x_margin = (x_size - x_msg_size).max(0) / 2;
    let y_margin = (y_size - y_msg_size).max(0) / 2;
    let x0 = bbox.x_start() + x_margin;
    let y0 = bbox.y_start() + y_margin;
    let p0 = p2d(x0, y0);

    for p_msg in msg_bbox {
        let v_msg = p_msg.to_vec();
        let x = v_msg.x();
        let y = v_msg.y();
        // TODO Automatically determine orientation
        // let i = usize::try_from(x + y * x_msg_size).unwrap();
        let i = usize::try_from(y + x * y_msg_size).unwrap();
        if msg[i] {
            frame.set(p0 + v_msg, Pixel::Foreground);
        }
    }

    loop {
        let mut buf = Vec::with_capacity(frame_size);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
