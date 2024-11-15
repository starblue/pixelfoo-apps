use std::cmp;
use std::collections::BinaryHeap;
use std::env::args;
use std::fs;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo_apps::color::Color;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

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
                Pixel::Foreground => Color::new(0xcc, 0x99, 0),
            };
            w.write_all(&color.rgb())?;
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
const HEIGHT: i64 = 11;
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
                frame.set(
                    *pos + v2d(ones - 1 - STROKE_MID + j, HEIGHT - 1),
                    Pixel::Foreground,
                );
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct State {
    prime: usize,
    xs: Vec<i64>,
}
impl State {
    fn init(len: usize, x: i64) -> State {
        State {
            prime: 2,
            xs: vec![x; len],
        }
    }
    fn load(lines: usize) -> Result<State> {
        let s = fs::read_to_string(STATE_FILENAME)?;
        let state: State = serde_json::from_str(&s)?;
        if state.xs.len() != lines {
            return Err("number of lines don't match".into());
        }
        Ok(state)
    }
    fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(self)?;
        Ok(fs::write(STATE_FILENAME, serialized)?)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinePos {
    /// Index of the line.
    index: usize,
    /// Rendering position in this line.
    pos: Point2d,
}
impl PartialOrd for LinePos {
    fn partial_cmp(&self, other: &LinePos) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LinePos {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other
            .pos
            .x()
            .cmp(&self.pos.x())
            .then(self.index.cmp(&other.index))
    }
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

    let bbox = bb2d(0..x_size as i64, 0..y_size as i64);

    let mut frame = Frame::new(bbox);

    let t_frame = (1000 / arg).max(50); // s
    let delay = Duration::from_millis(t_frame);

    // Start positions for rendering (lower left pixel).
    let lines = (y_size - 2 * MIN_MARGIN + MIN_LINE_SEP) / (HEIGHT + MIN_LINE_SEP);
    let lines = lines.max(1);
    let line_sep = if lines >= 2 {
        (y_size - 2 * MIN_MARGIN - lines * HEIGHT) / lines
    } else {
        MIN_LINE_SEP
    };
    let line_sep = line_sep.max(MIN_LINE_SEP);
    let x0 = bbox.x_start() + MIN_MARGIN;
    let y0 = MIN_MARGIN + (y_size - 2 * MIN_MARGIN - lines * HEIGHT - (lines - 1) * line_sep) / 2;
    let line_count = usize::try_from(lines)?;

    let start_state = State::load(line_count).unwrap_or_else(|_| State::init(line_count, x0));
    eprintln!("starting at {}", start_state.prime);

    let mut primes_iter = primal::Primes::all().skip_while(|&p| p < start_state.prime);

    let mut start_positions = start_state
        .xs
        .iter()
        .enumerate()
        .map(|(i, &x)| p2d(x, y0 + (i as i64) * (line_sep + HEIGHT)))
        .collect::<Vec<_>>();

    let mut visible_primes = Vec::new();

    let space = v2d(2, 0);
    loop {
        let mut new_frame = Frame::new(bbox);
        let mut new_visible_primes = Vec::new();

        // Fill empty space from the left to the right over all lines.
        let mut positions = start_positions
            .iter()
            .enumerate()
            .map(|(i, &pos)| LinePos { index: i, pos })
            .collect::<BinaryHeap<_>>();
        // Render the primes that are already visible.
        for p in visible_primes {
            let line_pos = positions.pop().unwrap();
            let i = line_pos.index;
            let mut pos = line_pos.pos;

            render(&mut new_frame, &mut pos, p);
            if new_visible_primes.is_empty() && pos.x() + STROKE_MID < frame.bbox().x_start() {
                // The prime is not visible any more,
                // not even some overhang from a five bar.
                // Omit it from the visible primes and move the start position
                // to the start of the next number.
                // Only do this if no previous prime was kept.
                start_positions[i] = pos + space;
            } else {
                // Keep the prime for displaying it the next time.
                new_visible_primes.push(p);
            }

            // Render a space between numbers.
            pos += space;

            // Save the new position for later handling.
            positions.push(LinePos { index: i, pos });
        }
        while let Some(line_pos) = positions.pop() {
            let i = line_pos.index;
            let mut pos = line_pos.pos;

            // Fill up the visible primes when necessary.
            if pos.x() < frame.bbox().x_end() + STROKE_MID {
                let p = primes_iter.next().unwrap();
                eprintln!("{} {}", i, p);

                // Keep the prime for displaying it the next time.
                new_visible_primes.push(p);

                render(&mut new_frame, &mut pos, p);

                // Render a space between numbers.
                pos += space;

                positions.push(LinePos { index: i, pos });
            }
        }

        // Scroll one pixel to the left.
        for pos in &mut start_positions {
            *pos -= v2d(1, 0);
        }

        frame = new_frame;
        visible_primes = new_visible_primes;

        // Save the state
        let state = State {
            prime: visible_primes[0],
            xs: start_positions.iter().map(|p| p.x()).collect::<Vec<_>>(),
        };
        let _ = state.save();

        let mut buf = Vec::with_capacity(frame_size);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
