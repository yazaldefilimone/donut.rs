use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 22;
const BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const CHARACTERS: &str = ".,-~:;=!*#$@";

type Buffer = [u8; BUFFER_SIZE];

fn rotate(mul: i32, shift: i32, x: &mut i32, y: &mut i32, temp: &mut i32) {
  *temp = *x;
  *x -= (mul * *y) >> shift;
  *y += mul.wrapping_mul(*temp) >> shift;
  *temp = (3145728 - (*x * *x + *y * *y)) >> 11;
  *x = (*x * *temp) >> 10;
  *y = (*y * *temp) >> 10;
}

fn render_frame(s_a: &mut i32, c_a: &mut i32, s_b: &mut i32, c_b: &mut i32) {
  let mut text_buffer: Buffer = [b' '; BUFFER_SIZE];
  let mut z_buffer: Buffer = [127; BUFFER_SIZE];

  let mut s_j = 0;
  let mut c_j = 1024;
  let mut temp = 0;

  for _j in 0..90 {
    let mut s_i = 0;
    let mut c_i = 1024;

    for _i in 0..324 {
      let r1 = 1;
      let r2 = 2048;
      let k2 = 5120 * 1024;

      let x0 = r1 * c_j + r2;
      let x1 = (c_i * x0) >> 10;
      let x2 = (*c_a * s_j) >> 10;
      let x3 = (s_i * x0) >> 10;
      let x4 = r1 * x2 - ((*s_a * x3) >> 10);
      let x5 = (*s_a * s_j) >> 10;
      let x6 = k2 + r1 * 1024 * x5 + *c_a * x3;
      let x7 = (c_j * s_i) >> 10;

      let x = 40 + 30 * (*c_b * x1 - *s_b * x4) / x6;
      let y = 12 + 15 * (*c_b * x4 + *s_b * x1) / x6;

      let n = (((-*c_a * x7
        - *c_b * (-((*s_a * x7) >> 10) + x2)
        - c_i * ((c_j * *s_b) >> 10))
        >> 10)
        - x5)
        >> 7;

      let o = x + SCREEN_WIDTH as i32 * y;
      let zz = (x6 - k2) >> 15;

      if y >= 0
        && y < SCREEN_HEIGHT as i32
        && x >= 0
        && x < SCREEN_WIDTH as i32
        && zz < z_buffer[o as usize] as i32
      {
        z_buffer[o as usize] = zz as u8;
        text_buffer[o as usize] =
          CHARACTERS.chars().nth(n.max(0) as usize).unwrap_or(' ') as u8;
      }

      rotate(5, 8, &mut c_i, &mut s_i, &mut temp);
    }
    rotate(9, 7, &mut c_j, &mut s_j, &mut temp);
  }

  let mut stdout = io::stdout();
  for (i, &ch) in text_buffer.iter().enumerate() {
    if i % SCREEN_WIDTH == 0 {
      writeln!(stdout).unwrap();
    }
    stdout.write_all(&[ch]).unwrap();
  }
  stdout.flush().unwrap();

  rotate(5, 7, c_a, s_a, &mut temp);
  rotate(5, 8, c_b, s_b, &mut temp);
}

fn main() {
  let mut s_a = 1024;
  let mut c_a = 0;
  let mut s_b = 1024;
  let mut c_b = 0;

  loop {
    render_frame(&mut s_a, &mut c_a, &mut s_b, &mut c_b);
    thread::sleep(Duration::from_millis(15));
    print!("\x1b[23A"); // Move cursor up to redraw
  }
}
