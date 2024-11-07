use std::env::args;
use std::fs;
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

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const COLORS: usize = 4;
const WHITE_VALUE: u8 = 128;

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
            let white = match pixel {
                Pixel::Background => WHITE_VALUE,
                Pixel::Foreground => 0,
            };
            let rgbw: &[u8] = &[0, 0, 0, white];
            w.write_all(rgbw)?;
        }
    }
    Ok(w.flush()?)
}

fn encode_digit(d: u32) -> (i64, bool) {
    if d <= 5 {
        (i64::from(d), false)
    } else {
        (i64::from(d) - 5, true)
    }
}

fn render_zero(frame: &mut Frame, pos: &mut Point2d) {
    for v in &[
        v2d(1, 0),
        v2d(2, 0),
        v2d(3, 0),
        v2d(1, 4),
        v2d(2, 4),
        v2d(3, 4),
        v2d(0, 1),
        v2d(0, 2),
        v2d(0, 3),
        v2d(4, 1),
        v2d(4, 2),
        v2d(4, 3),
    ] {
        frame.set(*pos + v, Pixel::Foreground);
    }
    *pos += v2d(5, 0);
}

const STROKE_LEN: i64 = 11;
const STROKE_MID: i64 = 5;
const HEIGHT: i64 = 12;
const MIN_LINE_SEP: i64 = 1;
const MIN_MARGIN: i64 = 1;

fn render_vertical(frame: &mut Frame, pos: &mut Point2d, digit: u32) {
    if digit == 0 {
        render_zero(frame, pos);
    } else {
        let (ones, five) = encode_digit(digit);
        for i in 0..ones {
            for j in 0..STROKE_LEN {
                frame.set(*pos + v2d(2 * i, j), Pixel::Foreground);
            }
        }
        if five {
            for j in 0..STROKE_LEN {
                frame.set(*pos + v2d(ones - 1 - STROKE_MID + j, 11), Pixel::Foreground);
            }
        }
        *pos += v2d(2 * ones - 1, 0);
    }
}

fn render_horizontal(frame: &mut Frame, pos: &mut Point2d, digit: u32) {
    if digit == 0 {
        render_zero(frame, pos);
    } else {
        let (ones, five) = encode_digit(digit);
        for i in 0..ones {
            for j in 0..STROKE_LEN {
                frame.set(*pos + v2d(j, 2 * i), Pixel::Foreground);
            }
        }
        if five {
            for j in (2 * ones - 1)..STROKE_LEN {
                frame.set(*pos + v2d(STROKE_MID, j), Pixel::Foreground);
            }
        }
        *pos += v2d(STROKE_LEN, 0);
    }
}

fn render(frame: &mut Frame, pos: &mut Point2d, n: usize) {
    let s = n.to_string();
    let mut vertical = s.len() % 2 != 0;
    for c in s.chars() {
        let digit = c.to_digit(10).unwrap();
        if vertical {
            render_vertical(frame, pos, digit);
        } else {
            render_horizontal(frame, pos, digit);
        }
        vertical = !vertical;
    }
}

const DEFAULT_ARG: u64 = 2;

const STATE_FILENAME: &str = ".primes.state";

fn write_state(p: usize) {
    let _ = fs::write(STATE_FILENAME, p.to_string());
}

fn read_state() -> Result<usize> {
    let s = fs::read_to_string(STATE_FILENAME)?;
    Ok(s.parse::<usize>()?)
}

fn main() -> Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<u64>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let frame_size = x_size * y_size * COLORS;

    let x_size = i64::try_from(x_size)?;
    let y_size = i64::try_from(y_size)?;

    let last_prime = read_state().unwrap_or(1);
    eprintln!("starting after {}", last_prime);
    let mut primes_iter = primal::Primes::all().skip_while(|&p| p < last_prime);

    let bbox = bb2d(0..x_size as i64, 0..y_size as i64);

    let mut frame = Frame::new(bbox);

    let t_frame = (1000 / arg).max(50); // s
    let delay = Duration::from_millis(t_frame);

    // Start positions for rendering (lower left pixel).
    let lines = (y_size - 2 * MIN_MARGIN + MIN_LINE_SEP) / (HEIGHT + MIN_LINE_SEP);
    let line_sep = if lines >= 2 {
        (y_size - 2 * MIN_MARGIN - lines * HEIGHT) / (lines - 1)
    } else {
        MIN_LINE_SEP
    };
    let y0 = MIN_MARGIN + (y_size - 2 * MIN_MARGIN - lines * HEIGHT - (lines - 1) * line_sep) / 2;
    let mut start_positions = (0..lines)
        .map(|i| p2d(bbox.x_end(), y0 + i * (line_sep + HEIGHT)))
        .collect::<Vec<_>>();

    let lines = usize::try_from(lines)?;

    let mut visible_primes = (0..lines).map(|_| Vec::new()).collect::<Vec<_>>();

    let space = v2d(2, 0);
    loop {
        let mut new_frame = Frame::new(bbox);
        let mut new_visible_primes = (0..lines).map(|_| Vec::new()).collect::<Vec<_>>();

        for i in 0..lines {
            let mut pos = start_positions[i];

            // Render the primes that are already visible.
            for &p in &visible_primes[i] {
                render(&mut new_frame, &mut pos, p);
                if pos.x() + STROKE_MID < frame.bbox().x_start() {
                    // The prime is not visible any more,
                    // not even some overhang from a five bar.
                    // Omit it from the visible primes and move the start position
                    // to the start of the next number.
                    start_positions[i] = pos + space;
                } else {
                    // Keep the prime for displaying it the next time.
                    new_visible_primes[i].push(p);
                }

                // Render a space between numbers.
                pos += space;
            }

            // Fill up the visible primes when necessary.
            while pos.x() < frame.bbox().x_end() + STROKE_MID {
                let p = primes_iter.next().unwrap();
                write_state(p);
                eprintln!("{}", p);

                // Keep the prime for displaying it the next time.
                new_visible_primes[i].push(p);

                render(&mut new_frame, &mut pos, p);
            }

            // Scroll one pixel to the left.
            start_positions[i] -= v2d(1, 0);
        }

        frame = new_frame;
        visible_primes = new_visible_primes;

        let mut buf = Vec::with_capacity(frame_size);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
