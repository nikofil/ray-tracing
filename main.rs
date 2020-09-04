use std::fs::File;
use std::io::prelude::*;

extern crate tracer;
use tracer::Point;

fn write_to_file() -> std::io::Result<()> {
    let mut f = File::create("out.ppm")?;
    let width = 256;
    let height = 256;
    let head: String = format!("P3\n{} {}\n255\n", width, height);
    f.write_all(head.as_bytes())?;

    for (cnt, y) in (0..height).rev().enumerate() {
        eprint!("\rLine {} of {}", cnt+1, height);
        for x in 0..width {
            let r = (x as f64) / (width as f64 - 1.);
            let g = (y as f64) / (height as f64 - 1.);
            let b = 0.25f64;

            let px: String = format!("{} {} {}\n", (r*255.999) as u8, (g*255.999) as u8, (b*255.999) as u8);
            f.write_all(px.as_bytes())?;
        }
    }
    eprint!("\n");

    Ok(())
}

fn main() {
    write_to_file().unwrap();
    let mut x = Point::new(1.0,2.0,3.0);
    x += Point::new(1.1, 2.2, 3.3);
    println!("Hello {:?}", (x/2.0)*3.0);
}