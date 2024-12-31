use std::collections::HashMap;
use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo_apps::color::Color;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const COLORS: usize = 3;

#[derive(Clone, Debug)]
struct Frame {
    pixels: Array2d<i64, Color>,
}
impl Frame {
    pub fn new(bbox: BBox2d, background: Color) -> Frame {
        let pixels = Array2d::with(bbox, |_| background);
        Frame { pixels }
    }
    pub fn bbox(&self) -> BBox2d {
        self.pixels.bbox()
    }
    pub fn set(&mut self, pos: Point2d, pixel: Color) {
        if self.bbox().contains(&pos) {
            self.pixels[pos] = pixel;
        }
    }
    fn set_rectangle(&mut self, bbox: BBox2d, color: Color) {
        for p in bbox {
            self.set(p, color);
        }
    }
}

fn send<T: Write>(w: &mut T, frame: &Frame) -> Result<()> {
    for y in frame.bbox().y_range().rev() {
        for x in frame.bbox().x_range() {
            let color = frame.pixels[p2d(x, y)];
            w.write_all(&color.rgb())?;
        }
    }
    Ok(w.flush()?)
}

const DEFAULT_ARG: usize = 0;

#[derive(Clone, Copy, Debug)]
enum Ring {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Gray,
    White,
    Silver,
    Gold,
}
impl Ring {
    #[allow(unused)]
    fn value(&self) -> i64 {
        match self {
            Ring::Black => 0,
            Ring::Brown => 1,
            Ring::Red => 2,
            Ring::Orange => 3,
            Ring::Yellow => 4,
            Ring::Green => 5,
            Ring::Blue => 6,
            Ring::Violet => 7,
            Ring::Gray => 8,
            Ring::White => 9,
            Ring::Silver => -2,
            Ring::Gold => -1,
        }
    }
    fn color(&self) -> Color {
        match self {
            Ring::Black => Color::new(0, 0, 0),
            Ring::Brown => Color::new(165, 82, 42),
            Ring::Red => Color::new(255, 0, 0),
            Ring::Orange => Color::new(255, 165, 0),
            Ring::Yellow => Color::new(255, 255, 0),
            Ring::Green => Color::new(0, 255, 0),
            Ring::Blue => Color::new(0, 0, 255),
            Ring::Violet => Color::new(238, 130, 238),
            Ring::Gray => Color::new(190, 190, 190),
            Ring::White => Color::new(255, 255, 255),
            Ring::Silver => Color::new(220, 220, 230),
            Ring::Gold => Color::new(255, 215, 0),
        }
    }
}

#[derive(Clone, Debug)]
struct Resistor {
    rings: Vec<Ring>,
}
impl Resistor {
    fn new(rings: Vec<Ring>) -> Resistor {
        Resistor { rings }
    }
    fn rings(&self) -> &[Ring] {
        &self.rings[..]
    }
}

const E12: &[[Ring; 2]; 12] = &[
    [Ring::Brown, Ring::Black],
    [Ring::Brown, Ring::Red],
    [Ring::Brown, Ring::Green],
    [Ring::Brown, Ring::Gray],
    [Ring::Red, Ring::Red],
    [Ring::Red, Ring::Violet],
    [Ring::Orange, Ring::Orange],
    [Ring::Orange, Ring::White],
    [Ring::Yellow, Ring::Violet],
    [Ring::Green, Ring::Blue],
    [Ring::Blue, Ring::Gray],
    [Ring::Gray, Ring::Red],
];

fn random_element<'a, T, R: Rng>(elements: &'a [T], rng: &mut R) -> &'a T {
    let i = rng.gen_range(0..elements.len());
    &elements[i]
}

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

    let mut rng = thread_rng();

    let frame_size = x_size * y_size * COLORS;

    let x_size = i64::try_from(x_size)?;
    let y_size = i64::try_from(y_size)?;

    let bbox = bb2d(0..x_size as i64, 0..y_size as i64);
    let mut frame = Frame::new(bbox, Color::black());

    let metal_body_color = Color::new(0, 192, 192);
    let carbon_body_color = Color::new(242, 200, 150);
    let wire_color = Color::new(180, 180, 190);

    let delay = Duration::from_millis(1000);

    let y_mid = (bbox.y_min() + bbox.y_max()) / 2;

    let y_wire_min = y_mid;
    let y_wire_max = y_mid + 1;
    let wire = bb2d(bbox.x_range(), y_wire_min..=y_wire_max);

    let x_body_size = 56;
    let y_body_size = 12;
    let x_ring_offset = 8;
    let x_ring_extra_offset = 4;
    let x_ring_size = 4;
    let x_ring_space = 4;
    let x_ring_extra_space = 4;

    let x_body_min = (x_size - x_body_size) / 2;
    let y_body_min = (y_size - y_body_size) / 2;
    let x_body_end = x_body_min + x_body_size;
    let y_body_end = y_body_min + y_body_size;
    let body = bb2d(x_body_min..x_body_end, y_body_min..y_body_end);
    let mut body_color = metal_body_color;

    let mut rings_variants = HashMap::new();
    for n in 4..=5 {
        let mut rings = Vec::new();
        for i in 0..n {
            let x_ring_min = x_body_min
                + x_ring_offset
                + if n == 4 { x_ring_extra_offset } else { 0 }
                + i * (x_ring_size + x_ring_space)
                + if i == n - 1 { x_ring_extra_space } else { 0 };
            let x_ring_end = x_ring_min + x_ring_size;
            let ring = bb2d(x_ring_min..x_ring_end, y_body_min..y_body_end);

            rings.push(ring);
        }
        rings_variants.insert(usize::try_from(n)?, rings);
    }

    let p_pruefungsfragen = 0.1;
    let pruefungsfragen = vec![
        Resistor::new(vec![Ring::Brown, Ring::Red, Ring::Red, Ring::Gold]), // NC103
        Resistor::new(vec![Ring::Red, Ring::Violet, Ring::Red, Ring::Gold]), // NC104
        Resistor::new(vec![Ring::Yellow, Ring::Violet, Ring::Red, Ring::Silver]), // NC105
        Resistor::new(vec![Ring::Red, Ring::Violet, Ring::Orange, Ring::Gold]), // NC106
        Resistor::new(vec![Ring::Yellow, Ring::Violet, Ring::Orange, Ring::Silver]), // NC107
        Resistor::new(vec![Ring::Green, Ring::Blue, Ring::Red, Ring::Silver]), // EC112, EC113
    ];

    let t_solve = 10;
    let t_end = 15;

    let mut t = 0;
    let mut r = Resistor::new(vec![Ring::Brown, Ring::Black, Ring::Red, Ring::Gold]);
    loop {
        if t == 0 {
            // Pick a new resistor.
            if rng.gen::<f64>() < p_pruefungsfragen {
                r = random_element(&pruefungsfragen, &mut rng).clone();
                body_color = carbon_body_color;
            } else {
                let is_five_ring = rng.gen::<f64>() < 0.5;

                body_color = if is_five_ring {
                    // A turquise blue metal film resistor.
                    metal_body_color
                } else {
                    // A brownish beige carbon resistor.
                    carbon_body_color
                };

                let mut rings = random_element(E12, &mut rng).to_vec();
                if is_five_ring {
                    // Add a black ring for a five-ring resistor.
                    rings.push(Ring::Black);
                }

                let r3s = [
                    Ring::Black,
                    Ring::Brown,
                    Ring::Red,
                    Ring::Orange,
                    Ring::Yellow,
                ];
                rings.push(*random_element(&r3s, &mut rng));

                let r4s = if is_five_ring {
                    [Ring::Brown, Ring::Red]
                } else {
                    [Ring::Gold, Ring::Silver]
                };
                rings.push(*random_element(&r4s, &mut rng));

                r = Resistor::new(rings);
            }
        }

        // Display the resistor.
        frame.set_rectangle(wire, wire_color);
        frame.set_rectangle(body, body_color);
        let ring_bboxes = &rings_variants[&r.rings().len()];
        for (&ring_bbox, ring) in ring_bboxes.iter().zip(r.rings().iter()) {
            frame.set_rectangle(ring_bbox, ring.color());
        }

        if t >= t_solve {
            // TODO Display the resistor value as text.
        }

        let mut buf = Vec::with_capacity(frame_size);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);

        t += 1;
        if t >= t_end {
            t = 0;
        }
    }
}
