use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;

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

const DEFAULT_ARG: usize = 20;
fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = args[3].parse::<usize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    // Background color.
    let c0 = Color::new(0, 0, 80);
    let [r0, g0, b0] = c0.rgb();

    let c1 = Color::new(0, 255, 80);

    let mut frame = repeat(repeat(c0).take(x_size).collect::<Vec<_>>())
        .take(y_size)
        .collect::<Vec<_>>();

    let x0 = (x_size as f64) / 2.0;
    let y0 = (y_size as f64) / 2.0;

    // x in [-20.0,20.0], z in [0.0,50.0]

    let xc = 0.0;
    let zc = 25.0;
    let x_scale = x0 / 20.0;
    let z_scale = y0 / 25.0;

    let dt = 0.0025;
    let a = 10.0;
    let b = 27.0;
    let c = 8.0 / 3.0;

    let mut x = 1.0;
    let mut y = 0.0;
    let mut z = zc;

    loop {
        // Decay towards the background color.
        for row in &mut frame {
            for pixel in row {
                let [mut r, mut g, mut b] = pixel.rgb();
                if r > r0 {
                    r -= 1;
                }
                if g > g0 {
                    g -= 1;
                }
                if b > b0 {
                    b -= 1;
                }
                *pixel = Color::new(r, g, b);
            }
        }

        for _ in 0..arg {
            x += dt * a * (y - x);
            y += dt * (x * (b - z) - y);
            z += dt * (x * y - c * z);

            let xf = (x_scale * (x - xc) + x0).round();
            let yf = (z_scale * (z - zc) + y0).round();
            if xf >= 0.0 && xf < x_size as f64 && yf >= 0.0 && yf < y_size as f64 {
                frame[yf as usize][xf as usize] = c1;
            }
        }

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
