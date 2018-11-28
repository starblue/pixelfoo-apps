use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
struct Color(u8, u8, u8);

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
    fn complement(&self) -> Color {
        Color(255 - self.0, 255 - self.1, 255 - self.2)
    }
}

fn interpolate_u8(b0: u8, b1: u8, a: f64) -> u8 {
    let b0 = b0 as f64;
    let b1 = b1 as f64;
    ((1.0 - a) * b0 + a * b1) as u8
}

fn interpolate(c0: Color, c1: Color, a: f64) -> Color {
    Color(
        interpolate_u8(c0.0, c1.0, a),
        interpolate_u8(c0.1, c1.1, a),
        interpolate_u8(c0.2, c1.2, a),
    )
}

type Frame = Vec<Vec<Color>>;

fn send<T: Write>(w: &mut T, f: &Frame) -> std::io::Result<()> {
    for l in f {
        for c in l {
            let Color(r, g, b) = c;
            let buf = &[*r, *g, *b];
            w.write_all(buf)?;
        }
    }
    w.flush()
}

const DEFAULT_LOOP_TIME: usize = 120;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let loop_time = args[3].parse::<usize>().unwrap_or(DEFAULT_LOOP_TIME);
    eprintln!(
        "screen size {}x{}, loop time {:?}s",
        x_size, y_size, loop_time
    );

    let mut border = 0;
    while (x_size - 2 * border) * (y_size - 2 * border) > x_size * y_size / 2 {
        border += 1;
    }

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let mut previous_color = Color::blue();
    let colors = vec![
        Color::red(),
        Color::yellow(),
        Color::green(),
        Color::cyan(),
        Color::blue(),
        Color::magenta(),
    ];
    let mut color_index = 0;
    let mut next_color = colors[color_index];

    // time to interpolate from one color to the next
    let t_interpolate = (loop_time as f64) / (colors.len() as f64); // s
    let delta_a = t_frame / t_interpolate;
    let mut a = 0.0;

    eprint!("delta_a {}", delta_a);
    loop {
        let c0 = interpolate(previous_color, next_color, a);
        let c1 = c0.complement();
        let mut frame = repeat(repeat(c0).take(x_size).collect::<Vec<_>>())
            .take(y_size)
            .collect::<Vec<_>>();
        for x in border..(x_size - border) {
            for y in border..(y_size - border) {
                frame[y][x] = c1
            }
        }

        a += delta_a;
        if a >= 1.0 {
            a = 0.0;

            color_index += 1;
            if color_index >= colors.len() {
                color_index = 0;
            }

            previous_color = next_color;
            next_color = colors[color_index];
        }
        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
