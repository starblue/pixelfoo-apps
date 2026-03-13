use std::collections::HashMap;
use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo_apps::color::Color;

mod expression;
use expression::Env;
use expression::RandomExpressionBuilder;
use expression::Variable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Common,
    Uncommon,
}
impl Square {
    fn color(&self) -> Color {
        match self {
            Square::Common => Color::new(0xd2, 0xd4, 0xbc),
            Square::Uncommon => Color::black(),
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

    let mut rng = rng();
    let bbox = bb2d(0..x_size, 0..y_size);

    let min_percent = 30;
    let max_percent = 70;

    let frames_per_second = 25;
    let delay = Duration::from_millis(1000 / frames_per_second);
    let frame_seconds = if arg > 0 { arg } else { DEFAULT_ARG };
    let frame_time_count = frame_seconds * frames_per_second;

    let fade_time_count = (2 * frames_per_second).min(frame_time_count / 3);
    let fade_alpha_step = 1.0 / (fade_time_count as f64);

    let mut old_board = Board::with(bbox, |_p| Square::Uncommon);
    let mut new_board = Board::with(bbox, |_p| Square::Uncommon);

    let mut time_count = frame_time_count;
    loop {
        if time_count >= frame_time_count {
            time_count = 0;

            // Pick a random expression.
            loop {
                let expression = RandomExpressionBuilder::build(&mut rng);
                let values = Array2d::with(bbox, |p| {
                    let x = p.x();
                    let y = p.y();
                    let env = Env { x, y };
                    expression.eval(&env)
                });
                let mut histogram = HashMap::new();
                for p in bbox {
                    let entry = histogram.entry(values[p]).or_insert(0);
                    *entry += 1;
                }

                // Find the most common value and its count.
                let mut max_count = 0;
                let mut max_count_value = 0;
                for (value, count) in histogram {
                    if count > max_count {
                        max_count = count;
                        max_count_value = value;
                    }
                }

                let percent = 100 * max_count / bbox.area();

                if expression.contains_var(Variable::X)
                    && expression.contains_var(Variable::Y)
                    && (min_percent..=max_percent).contains(&percent)
                {
                    eprintln!("chose expression {expression}");

                    old_board = new_board;
                    new_board = Board::with(bbox, |p| {
                        // Use the `onebit` scheme for now.
                        if values[p] == max_count_value {
                            Square::Common
                        } else {
                            Square::Uncommon
                        }
                    });
                    break;
                }
            }
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
