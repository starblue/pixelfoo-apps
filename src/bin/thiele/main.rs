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
impl Square {
    fn color(&self) -> Color {
        match self {
            Square::Zero => Color::blue(),
            Square::Residue => Color::new(0xd2, 0xd4, 0xbc),
            Square::NonResidue => Color::black(),
        }
    }
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

fn send<T: Write>(
    w: &mut T,
    old_board: &Board,
    new_board: &Board,
    alpha: f64,
) -> std::io::Result<()> {
    for y in old_board.bbox().y_range() {
        for x in old_board.bbox().x_range() {
            let old_color = old_board.map[p2d(x, y)].color();
            let new_color = new_board.map[p2d(x, y)].color();
            let color = old_color.interpolate(new_color, alpha);

            w.write_all(&color.rgb())?;
        }
    }
    w.flush()
}

const DEFAULT_ARG: u64 = 10;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<i64>().unwrap();
    let y_size = args[2].parse::<i64>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<u64>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let min_size = x_size.min(y_size);

    // The range of the real and imaginary parts of a prime modulus.
    let prime_range = 0..=(min_size / 3).max(8);
    // The minimal norm of a prime modulus.
    let min_norm = (min_size / 10).pow(2).max(9);

    let mut rng = rng();
    let bbox = bb2d(0..x_size, 0..y_size);

    let frames_per_second = 25;
    let delay = Duration::from_millis(1000 / frames_per_second);
    let frame_seconds = if arg > 0 { arg } else { DEFAULT_ARG };
    let frame_time_count = frame_seconds * frames_per_second;

    let fade_time_count = (2 * frames_per_second).min(frame_time_count / 3);
    let fade_alpha_step = 1.0 / (fade_time_count as f64);

    let mut old_board = Board::with(bbox, |_p| Square::NonResidue);
    let mut new_board = Board::with(bbox, |_p| Square::NonResidue);

    let mut time_count = frame_time_count;
    loop {
        if time_count >= frame_time_count {
            time_count = 0;

            // Pick a random gaussian prime somewhat smaller than the screen size.
            let mut m;
            loop {
                let re = rng.random_range(prime_range.clone());
                let im = rng.random_range(prime_range.clone());
                m = Gaussian(re, im);

                if m.is_prime() && m.norm() >= min_norm {
                    // We found a suitable prime.
                    break;
                }
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

            old_board = new_board;
            new_board = Board::with(bbox, |p| {
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

        let alpha = ((time_count as f64) * fade_alpha_step).min(1.0);

        let mut buf = Vec::with_capacity((x_size * y_size * 3) as usize);
        send(&mut buf, &old_board, &new_board, alpha)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;

        sleep(delay);
        time_count += 1;
    }
}
