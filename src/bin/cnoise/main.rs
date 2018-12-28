use std::env::args;
use std::io::stdout;
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

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    eprintln!("screen size {}x{}", x_size, y_size);

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let c0 = Color::new(0, 160, 95);
    let c1 = Color::new(0, 138, 170);

    let mut rng = thread_rng();

    let mut frame = repeat(repeat(Color::black()).take(x_size).collect::<Vec<_>>())
        .take(y_size)
        .collect::<Vec<_>>();
    loop {
        let x = rng.gen_range(0, x_size);
        let y = rng.gen_range(0, y_size);
        let c = c0.interpolate(c1, rng.gen::<f64>().powf(2.0));
        frame[y][x] = c;

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}
