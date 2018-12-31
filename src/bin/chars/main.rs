use std::env::args;
use std::fs::File;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use pixelfoo::color::Color;

type Frame = Vec<Vec<Color>>;

fn send<T: Write>(w: &mut T, f: &Frame) -> std::io::Result<()> {
    for l in f {
        for c in l {
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

fn paint_char(frame: &mut Frame, x: usize, y: usize, bitmap: &[u8], fg: Color, bg: Color) {
    for i in 0..8 {
        let b = bitmap[i];
        for j in 0..8 {
            frame[y * 8 + 7 - i][x * 8 + j] = {
                if b & (1 << j) != 0 {
                    fg
                } else {
                    bg
                }
            };
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    eprintln!("screen size {}x{}", x_size, y_size);

    let mut f = File::open("apps/pet-2.bin")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let mut rng = thread_rng();

    let colors = vec![
        Color::black(),
        Color::red(),
        Color::green(),
        Color::blue(),
        Color::yellow(),
        Color::cyan(),
        Color::magenta(),
        Color::white(),
    ];
    let mut frame = repeat(repeat(Color::black()).take(x_size).collect::<Vec<_>>())
        .take(y_size)
        .collect::<Vec<_>>();
    for x in 0..(x_size / 8) {
        for y in 0..(y_size / 8) {
            let fg = colors[rng.gen_range(0, colors.len())];
            let bg = colors[rng.gen_range(0, colors.len())];
            let start_index = rng.gen_range(0, buffer.len() / 8) * 8;
            let bitmap = &buffer[start_index..(start_index + 8)];
            paint_char(&mut frame, x, y, bitmap, fg, bg);
        }
    }
    loop {
        if rng.gen::<f64>() < 0.02 {
            let x = rng.gen_range(0, x_size / 8);
            let y = rng.gen_range(0, y_size / 8);
            let fg = colors[rng.gen_range(0, colors.len())];
            let bg = colors[rng.gen_range(0, colors.len())];
            let start_index = rng.gen_range(0, buffer.len() / 8) * 8;
            let bitmap = &buffer[start_index..(start_index + 8)];
            paint_char(&mut frame, x, y, bitmap, fg, bg);
        }

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
