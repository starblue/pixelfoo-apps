use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
struct Color(u8, u8, u8);

#[allow(unused)]
impl Color {
    fn black() -> Color {
        Color(0, 0, 0)
    }
    fn white() -> Color {
        Color(255, 255, 255)
    }
    fn gray50() -> Color {
        Color(128, 128, 128)
    }
    fn red() -> Color {
        Color(255, 0, 0)
    }
    fn green() -> Color {
        Color(0, 255, 0)
    }
    fn blue() -> Color {
        Color(0, 0, 255)
    }
    fn yellow() -> Color {
        Color(255, 255, 0)
    }
    fn cyan() -> Color {
        Color(0, 255, 255)
    }
    fn magenta() -> Color {
        Color(255, 0, 255)
    }
    fn brown() -> Color {
        Color(165, 42, 42)
    }
    fn complement(&self) -> Color {
        Color(255 - self.0, 255 - self.1, 255 - self.2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Unknown,
    Corridor,
    Wall,
    Start,
    Finish,
}

struct Board(Vec<Vec<Square>>);

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for line in &board.0 {
        for square in line {
            let Color(r, g, b) = match square {
                Square::Unknown => Color::black(),
                Square::Corridor => Color::yellow(),
                Square::Wall => Color::brown(),
                Square::Start => Color::red(),
                Square::Finish => Color::green(),
            };
            let buf = &[r, g, b];
            w.write_all(buf)?;
        }
    }
    w.flush()
}

impl Board {
    fn new(x_size: i32, y_size: i32, x_maze_size: i32, y_maze_size: i32) -> Board {
        Board(
            (0..y_size)
                .map(move |y| {
                    (0..x_size)
                        .map(|x| {
                            if x < x_maze_size && y < y_maze_size {
                                if x == 0
                                    || x == x_maze_size - 1
                                    || y == 0
                                    || y == y_maze_size - 1
                                    || (x % 2 == 0 && y % 2 == 0)
                                {
                                    Square::Wall
                                } else {
                                    Square::Unknown
                                }
                            } else {
                                Square::Unknown
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
    fn get(&self, pos: (i32, i32)) -> Square {
        let (x, y) = pos;
        self.0[y as usize][x as usize]
    }
    fn set(&mut self, pos: (i32, i32), sq: Square) {
        let (x, y) = pos;
        self.0[y as usize][x as usize] = sq;
    }
}

struct Move {
    from: (i32, i32),
    dir: (i32, i32),
    prio: i32,
}

fn add_move(open: &mut Vec<Move>, arg: isize, from: (i32, i32), dir: (i32, i32)) {
    open.push(Move { from, dir, prio: 0 });
    open.sort_unstable_by(|Move { prio: pa, .. }, Move { prio: pb, .. }| pa.cmp(pb));
}

const DEFAULT_ARG: isize = 16;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = args[3].parse::<isize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);
    let mut t_wait = 0.0; // s

    let x_size = x_size as i32;
    let y_size = y_size as i32;
    let x_maze_size = (x_size - 1) / 2 * 2 + 1;
    let y_maze_size = (y_size - 1) / 2 * 2 + 1;
    let x_maze_mid = (x_maze_size - 1) / 4 * 2 + 1;
    let y_maze_mid = (y_maze_size - 1) / 4 * 2 + 1;

    let mut board = Board(Vec::new());
    let mut open = Vec::new();
    loop {
        if open.is_empty() {
            if t_wait > 0.0 {
                t_wait -= t_frame;
            } else {
                t_wait = 60.0; // s
                board = Board::new(x_size, y_size, x_maze_size, y_maze_size);

                // start in the middle
                let mid = (x_maze_mid, y_maze_mid);
                board.set(mid, Square::Corridor);
                add_move(&mut open, arg, mid, (1, 0));
                add_move(&mut open, arg, mid, (0, 1));
                add_move(&mut open, arg, mid, (-1, 0));
                add_move(&mut open, arg, mid, (0, -1));
            }
        }

        // draw maze
        let work = arg.max(1);
        let mut count = 0;
        while !open.is_empty() && count < work {
            let mut start = 0;
            let mut limit = 0;
            let mut prio = open[limit].prio;
            loop {
                limit += 1;
                if limit >= open.len() {
                    break;
                }
                if open[limit].prio > prio {
                    if rng.gen::<f64>() >= 0.4 {
                        break;
                    }
                    start = limit;
                    prio = open[limit].prio;
                }
            }
            let r = rng.gen_range(start, limit);
            let Move {
                from: (x0, y0),
                dir: (dx, dy),
                ..
            } = open.remove(r);

            let (x1, y1) = (x0 + dx, y0 + dy);
            let (x2, y2) = (x1 + dx, y1 + dy);
            if board.get((x1, y1)) == Square::Unknown {
                board.set((x1, y1), Square::Corridor);
                board.set((x2, y2), Square::Corridor);

                for (dx, dy) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let (x3, y3) = (x2 + dx, y2 + dy);
                    let (x4, y4) = (x3 + dx, y3 + dy);
                    if board.get((x3, y3)) == Square::Unknown {
                        if board.get((x4, y4)) == Square::Unknown {
                            add_move(&mut open, arg, (x2, y2), (dx, dy));
                        } else {
                            board.set((x3, y3), Square::Wall);
                        }
                    }
                }

                count += 1;
            }

            if open.is_empty() {
                board.set((1, 1), Square::Start);
                board.set((x_maze_size - 2, y_maze_size - 2), Square::Finish);
            }
        }

        let mut buf = Vec::with_capacity((x_size * y_size * 3) as usize);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
