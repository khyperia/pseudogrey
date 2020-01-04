use png::{BitDepth, ColorType, Encoder};
use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

fn write_image(
    image: &[u8],
    width: usize,
    height: usize,
    w: impl Write,
) -> Result<(), Box<dyn Error>> {
    let mut encoder = Encoder::new(w, width as u32, height as u32);
    encoder.set_color(ColorType::RGB);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image)?;
    Ok(())
}

fn gen_image() -> (Vec<u8>, usize, usize) {
    let width = 256 * 3 - 2;
    let height = 256;
    let mut result = vec![0; width * height * 3];
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3;
            let base = (x / 3) as u8;
            if y < height / 2 {
                let pseudoadd = match x % 3 {
                    0 => (0, 0, 0),
                    1 => match y % 3 {
                        0 => (1, 0, 0),
                        1 => (0, 1, 0),
                        2 => (0, 0, 1),
                        _ => panic!(),
                    },
                    2 => match y % 3 {
                        0 => (0, 1, 1),
                        1 => (1, 0, 1),
                        2 => (1, 1, 0),
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                result[idx] = base + pseudoadd.0;
                result[idx + 1] = base + pseudoadd.1;
                result[idx + 2] = base + pseudoadd.2;
            } else {
                result[idx] = base;
                result[idx + 1] = base;
                result[idx + 2] = base;
            }
        }
    }
    (result, width, height)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (image, width, height) = gen_image();
    let file = File::create("output.png")?;
    let w = &mut BufWriter::new(file);
    write_image(&image, width, height, w)?;
    Ok(())
}
