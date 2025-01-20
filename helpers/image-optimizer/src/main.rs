use anyhow::{Context, Result as AnyResult, anyhow};
use clap::Parser;
use glob::glob;
use image::DynamicImage;
use image::GenericImageView;
use image::{ImageReader, imageops::FilterType};
use ravif::RGBA8 as Rgba;
use ravif::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use webp::Encoder as WebpEncoder;

#[derive(Debug, Clone, Default)]
struct Params {
    path: PathBuf,
    should_recreate: bool,
    should_resize: bool,
}

#[derive(Parser, Debug)]
#[command(about = "Convert images to AVIF and WebP")]
struct Cli {
    #[arg(short = 'r', long = "recreate")]
    recreate: bool,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("{:?}", args);
    // Path to the input JPG image
    let mut input_paths: Vec<Params> = glob("content/**/*.png")?
        .filter_map(Result::ok)
        .map(|path| Params {
            path,
            should_recreate: args.recreate,
            ..Default::default()
        })
        .collect();
    let theme_image_paths: Vec<Params> = glob("themes/**/*.jpg")?
        .filter_map(Result::ok)
        .map(|path| Params {
            path,
            should_recreate: args.recreate,
            should_resize: true,
        })
        .collect();
    let otherext_image_paths: Vec<Params> = glob("content/**/*.webp")?
        .filter_map(Result::ok)
        .map(|path| Params {
            path,
            should_recreate: args.recreate,
            ..Default::default()
        })
        .collect();
    input_paths.extend(theme_image_paths);
    input_paths.extend(otherext_image_paths);
    println!("{:?}", input_paths);
    input_paths.into_par_iter().map(handle).collect::<Vec<_>>();
    Ok(())
}

fn convert_to_webp(img: &DynamicImage, output_path: &str) -> AnyResult<()> {
    let encoder = WebpEncoder::from_image(img).unwrap();
    let webp_data = encoder.encode(75.0); // Quality 75
    let mut file = File::create(output_path)?;
    file.write_all(&webp_data)?;
    println!("Saved WebP to {}", output_path);
    Ok(())
}

fn resize(img: &DynamicImage, max_width: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let max_height = (width as f32 * (max_width as f32 / height as f32)).round() as u32;
    img.resize(max_width, max_height, FilterType::Lanczos3)
}

fn convert_to_avif(img: &DynamicImage, output_path: &str) -> AnyResult<()> {
    let (width, height) = img.dimensions();

    let rgba = img.to_rgba8();
    let encoded_avif = Encoder::new()
        .with_quality(50.0)
        .with_alpha_quality(50.0)
        .with_speed(10)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedClean)
        .with_num_threads(Some(4));
    let avif_pixels = rgba
        .pixels()
        .map(|p| Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect::<Vec<Rgba>>();

    let EncodedImage {
        avif_file,
        color_byte_size,
        alpha_byte_size,
        ..
    } = encoded_avif
        .encode_rgba(Img::new(
            &avif_pixels,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        ))
        .unwrap();
    let mut file = File::create(output_path)?;
    file.write_all(&avif_file)?;
    println!("Saved AVIF to {}", output_path);
    Ok(())
}

fn handle(params: Params) -> AnyResult<()> {
    println!("Handling path: {:?}", params.path);
    let file_stem = params
        .path
        .file_stem()
        .ok_or("Failed to get file stem")
        .unwrap()
        .to_str()
        .ok_or("Failed to convert OsStr to &str")
        .unwrap();

    let parent_dir = params
        .path
        .parent()
        .ok_or("Failed to get parent directory")
        .unwrap();

    let avif_file_path = parent_dir.join(format!("{}.avif", file_stem).as_str()); // Convert to .avif as an example
    let webp_file_path = parent_dir.join(format!("{}.webp", file_stem).as_str()); // Convert to .webp as an example

    println!("AVIF path: {:?}", avif_file_path);
    println!("WebP path: {:?}", webp_file_path);
    // println!("Backup path: {:?}", backup_path);
    let mut img = ImageReader::open(params.path)?.decode()?;
    if params.should_resize {
        img = resize(&img, 650);
    }
    if params.should_recreate {
        let _ = convert_to_avif(&img, avif_file_path.to_str().unwrap_or_default());
        let _ = convert_to_webp(&img, webp_file_path.to_str().unwrap_or_default());
    }
    if !avif_file_path.exists() {
        // Convert and save as AVIF
        let _ = convert_to_avif(&img, avif_file_path.to_str().unwrap_or_default());
    }
    if !webp_file_path.exists() {
        // Convert and save as WebP
        let _ = convert_to_webp(&img, webp_file_path.to_str().unwrap_or_default());
    }

    println!("Conversion completed!");
    Ok(())
}
