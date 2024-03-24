use clap::Parser;
use image::{GenericImageView, Pixel};
use std::error::Error;
use std::path::PathBuf;

use std::io::stdout;

#[derive(Parser, Debug)]
#[command(name = "catimg", author, version, about, long_about = None)] // Get info in Cargo.toml
pub struct Args {
    #[command()]
    pub file: Option<PathBuf>,
}

// TODO: Make the widht be first
// = width x height

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Args::parse();
    let path = matches.file.unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_path(path)
        .unwrap();

    let mut pixels: Vec<((u32, u32), u8, u8, u8)> = Vec::new();

    for result in rdr.deserialize() {
        let record: (String, u8, u8, u8) = result?;

        let (x, y) = record.0.split_once(':').unwrap();
        let height = x.parse::<u32>().unwrap();
        let widht = y.parse::<u32>().unwrap();

        pixels.push(((height,widht), record.1, record.2, record.3));
    }

    let height: u32 = *pixels.iter().map(|((x,_), _, _, _)| x).max().unwrap();
    let width: u32 = *pixels.iter().map(|((_,y), _, _, _)| y).max().unwrap();

    println!("input {}x{}", width, height);

    let mut img = image::RgbImage::new(width, height);
    for row in 0..height {
        for col in 0..width {
            // let pixel = pixels[((row * width) + col) as usize];
            let pixel = pixels.get(((row * width) + col) as usize);
            // println!("{}x{} - {} = {:?}", row, col, ((row * width) + col), pixel);
            // if pixel.is_none()  {
            // }

            let ((_, _), red, green, blue) = pixel.unwrap().clone();
            img.put_pixel(col, row, image::Rgb([red, green, blue]));
        }
    }

    img.save("output_do_meu_cli.png").unwrap();

    Ok(())
}

// fn main() -> Result<(), ()> {
//     let matches = Args::parse();
//     let path = matches.file.unwrap();
//
//     let img = image::open(path).unwrap();
//     let (width, height) = img.dimensions();
//
//     struct Pixel {
//         red: u8,
//         green: u8,
//         blue: u8,
//         position: (u32, u32),
//     }
//
//     let mut values: Vec<Pixel> = Vec::default();
//
//     for row in 0..height {
//         for col in 0..width {
//             let pixel = img.get_pixel(col, row);
//             let red = pixel[0];
//             let green = pixel[1];
//             let blue = pixel[2];
//             values.push(Pixel {
//                 red,
//                 green,
//                 blue,
//                 position: (row, col),
//             });
//         }
//     }
//
//     let mut csv_writer = csv::WriterBuilder::new()
//         .delimiter(b' ')
//         .from_writer(stdout());
//
//     for v in values {
//         csv_writer
//             .write_record(&[
//                 format!("{}:{}", v.position.0, v.position.1),
//                 v.red.to_string(),
//                 v.green.to_string(),
//                 v.blue.to_string(),
//             ])
//             .unwrap();
//     }
//
//     Ok(())
// }
