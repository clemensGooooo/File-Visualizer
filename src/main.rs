// About, this program is designed to help you getting a rougth understanding of a binary, and what data you can expect.
// You can get a graphical understanding of the entropy of the data.


use imageproc::image::{Rgb, RgbImage};
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::process::exit;


fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> Rgb<u8> {
    let chroma = value * saturation;
    let h_prime = hue / 60.0;
    let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs());

    let (r, g, b) = if (0.0..=1.0).contains(&h_prime) {
        (chroma, x, 0.0)
    } else if (1.0..=2.0).contains(&h_prime) {
        (x, chroma, 0.0)
    } else if (2.0..=3.0).contains(&h_prime) {
        (0.0, chroma, x)
    } else if (3.0..=4.0).contains(&h_prime) {
        (0.0, x, chroma)
    } else if (4.0..=5.0).contains(&h_prime) {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    let m = value - chroma;
    Rgb([
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ])
}

fn gen_image(image_data: Vec<u8>,width: u32, height: u32) -> Vec<u32> {
    
    let mut img: imageproc::image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(width as u32, height as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index: usize = (y * width as u32 + x) as usize;
        let color_val: f64;
        if let Some(&value) = image_data.get(index) {
            color_val = value as f64;
        } else {
            color_val = 0 as f64;
        }
        let saturation: f64 = 1.0;
        let value: f64 = 1.0;

        let color = hsv_to_rgb(color_val, saturation, value);
        *pixel = color;
    }

    let image: Vec<u32> = img
        .pixels()
        .map(|p: &Rgb<u8>| {
            let r: u32 = p[0] as u32;
            let g: u32 = p[1] as u32;
            let b: u32 = p[2] as u32;
            (r << 16) | (g << 8) | b
        })
        .collect();

    return image;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_name: String;
    if args.len() > 1 {
        file_name = args[1].clone();
    } else {
        println!("Please add the file you want to visualize as an argument.\n Example: ./executable <file> <output_image>");
        exit(0);
    }

    let mut file: File = File::open(file_name)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    let width: i64 = (buffer.len() as f64).sqrt() as i64 + 1;

    let height: i64 = (buffer.len() as i64) / width;

    let image = gen_image(buffer, width as u32, height as u32);

    let mut window = Window::new(
        "Binary image viewer",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if window.is_key_down(Key::Space) {
            println!("Hi");
        }
        window
            .update_with_buffer(&image, width as usize, height as usize)
            .unwrap();
    }
    Ok(())
}