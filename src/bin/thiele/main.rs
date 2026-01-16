use std::collections::HashSet;
use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::rng;
use rand::seq::IndexedRandom;
use rand::Rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo_apps::color::Color;

mod gaussian_integers;
use gaussian_integers::Gaussian;

mod gaussian_modulo;
use gaussian_modulo::mod_from;
use gaussian_modulo::mod_square;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Zero,
    Residue,
    NonResidue,
}

#[derive(Clone, Debug)]
struct Board {
    map: Array2d<i64, Square>,
}
impl Board {
    pub fn with(bbox: BBox2d, f: impl FnMut(Point2d) -> Square) -> Board {
        let map = Array2d::with(bbox, f);
        Board { map }
    }
    pub fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for y in board.bbox().y_range() {
        for x in board.bbox().x_range() {
            let square = board.map[p2d(x, y)];
            let c = match square {
                Square::Zero => Color::blue(),
                Square::Residue => Color::new(0xd2, 0xd4, 0xbc),
                Square::NonResidue => Color::black(),
            };
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

const DEFAULT_ARG: i64 = 60;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<i64>().unwrap();
    let y_size = args[2].parse::<i64>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<i64>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = rng();
    let bbox = bb2d(0..x_size, 0..y_size);

    let delay = Duration::from_millis(200);
    let frame_seconds = if arg > 0 { arg } else { DEFAULT_ARG };
    let max_time_count = frame_seconds * 5;

    let mut board = Board::with(bbox, |_p| Square::NonResidue);

    let mut time_count = 0;
    loop {
        if time_count <= 0 {
            time_count = max_time_count;

            // Pick a random gaussian prime somewhat smaller than the screen size.
            let mut m = Gaussian::ZERO;
            while !m.is_prime() || m.norm() < 16 {
                let min_size = x_size.min(y_size);
                let range = 0..=(min_size / 3).max(8);
                let re = rng.random_range(range.clone());
                let im = rng.random_range(range);
                m = Gaussian(re, im);
            }

            let mut reps = HashSet::new();
            let limit = m.re().max(m.im());
            for re in -limit..=limit {
                for im in -limit..=limit {
                    let a = mod_from(m, Gaussian(re, im));
                    reps.insert(a);
                }
            }
            let reps = reps.into_iter().collect::<Vec<_>>();

            let mut residues = HashSet::new();
            for &r in &reps {
                let a = mod_square(m, r);
                residues.insert(a);
            }

            // Pick a random offset of the board smaller than the prime.
            let origin = reps.choose(&mut rng).unwrap();

            eprintln!("chose prime modulus {m:?}, offset {origin:?}");

            board = Board::with(bbox, |p| {
                let re = p.x();
                let im = p.y();
                let a = mod_from(m, origin + Gaussian(re, im));
                if a.is_zero() {
                    Square::Zero
                } else if residues.contains(&a) {
                    Square::Residue
                } else {
                    Square::NonResidue
                }
            });
        }

        let mut buf = Vec::with_capacity((x_size * y_size * 3) as usize);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;

        sleep(delay);
        time_count -= 1;
    }
}
