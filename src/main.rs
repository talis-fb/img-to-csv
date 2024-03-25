use clap::{Parser, Subcommand};
use image::{GenericImageView, Pixel};
use std::error::Error;
use std::path::PathBuf;

use std::io::stdout;

#[derive(Parser)]
#[command(name = "catimg", author, version, about, long_about = None)] // Get info in Cargo.toml
pub struct Args {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    ToImage {
        file: PathBuf,

        #[arg(short, long)]
        output_file: PathBuf,
    },
    ToCsv {
        file: PathBuf,
    } 
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Args::parse();

    match matches.command {
        SubCommands::ToCsv { file } => {
            image_to_csv(file)?;
        }
        SubCommands::ToImage { file, output_file } => {
            csv_to_image(file, output_file)?;
        }
    }

    Ok(())
}

fn csv_to_image(file: PathBuf, output_file: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_path(file)
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

    let mut img = image::RgbImage::new(width + 1, height + 1);
    for pixel in pixels {
        let ((x, y), red, green, blue) = pixel;
        img.put_pixel(y, x, image::Rgb([red, green, blue]));
    }

    img.save(output_file).unwrap();

    Ok(())
}

fn image_to_csv(file: PathBuf) -> Result<(), Box<dyn Error>> {
    let img = image::open(file).unwrap();
    let (width, height) = img.dimensions();

    struct Pixel {
        red: u8,
        green: u8,
        blue: u8,
        position: (u32, u32),
    }

    let mut values: Vec<Pixel> = Vec::default();

    for row in 0..height {
        for col in 0..width {
            let pixel = img.get_pixel(col, row);
            let red = pixel[0];
            let green = pixel[1];
            let blue = pixel[2];
            values.push(Pixel {
                red,
                green,
                blue,
                position: (row, col),
            });
        }
    }

    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(b' ')
        .from_writer(stdout());

    for v in values {
        csv_writer
            .write_record(&[
                format!("{}:{}", v.position.0, v.position.1),
                v.red.to_string(),
                v.green.to_string(),
                v.blue.to_string(),
            ])
            .unwrap();
    }

    Ok(())
}
