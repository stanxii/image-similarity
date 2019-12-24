extern crate clap;
extern crate opencv;
mod image_similarity;

use clap::{Arg, App, SubCommand};
use image_similarity::{similarity, similarity_directory, similarity_file_directory};
use opencv::core::Mat;
use opencv::imgcodecs::imread;

/// Compute image similarity with image a and image b
fn compare_pair(image_a: &str, image_b: &str) {
    let img_a: Mat = imread(image_a, 0).expect("Image A is not a valid file");
    let img_b: Mat = imread(image_b, 0).expect("Image B is not a valid file");
    match similarity(&img_a, &img_b, 64, 16) {
        Ok(sim) => println!("{}", sim),
        Err(e) => println!("[ERROR] {}", e),
    }
}

/// Compute image similarity of all image pairs with allowed extensions in given directory
fn compare_directory(directory: &str, allowed_ext: &Vec<&str>) {
    match similarity_directory(directory, allowed_ext) {
        Some(similarity) => {
            similarity.iter().for_each(|result| {
                println!("{} \"{}\" \"{}\"", result.0, result.1, result.2);
            });
        },
        None => ()
    };
}

/// Compute similarities of given image with all images that ends in allowed extensions in given directory
fn compare_match(image: &str, directory: &str, allowed_ext: &Vec<&str>) {
    let image = imread(image, 0).unwrap();
    match similarity_file_directory(&image, directory, allowed_ext) {
        Ok(similarity) => match similarity {
            None => (),
            Some(similarity) => {
                similarity.iter().for_each(|result| {
                    println!("{} \"{}\"", result.0, result.1);
                });
            }
        },
        Err(e) => println!("[ERROR] {}", e),
    }
}

fn main() {
    let matches = App::new("Image Similarity")
        .version("0.1.0")
        .author("Cocoa <0xbbc@0xbbc.com>")
        .about("Compute image similarity")
        .subcommand(SubCommand::with_name("pair")
            .about("Compute image similarity with image a and image b")
            .version("0.1.0")
            .arg(Arg::with_name("imagea")
                .short("a")
                .long("imagea")
                .help("Image A")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("imageb")
                .short("b")
                .long("imageb")
                .help("Image B")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("directory")
            .about("Compute image similarity of all image pairs with allowed extensions in given directory")
            .version("0.1.0")
            .arg(Arg::with_name("directory")
                .short("d")
                .long("directory")
                .help("Directory")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("extension")
                .short("e")
                .long("ext")
                .help("Allowed extensions, defaults are \"png,jpg,jpeg\"")
                .takes_value(true)
                .required(false)))
        .subcommand(SubCommand::with_name("match")
            .about("Compute similarities of given image with all images that ends in allowed extensions in given directory")
            .version("0.1.0")
            .arg(Arg::with_name("directory")
                .short("d")
                .long("directory")
                .help("Directory")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("image")
                .short("i")
                .long("image")
                .help("Image")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("extension")
                .short("e")
                .long("ext")
                .help("Allowed extensions, defaults are \"png,jpg,jpeg\"")
                .takes_value(true)
                .required(false)))
        .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("pair") {
        let image_a = matches.value_of("imagea").unwrap();
        let image_b = matches.value_of("imageb").unwrap();
        compare_pair(image_a, image_b);
    } else if let Some(matches) = matches.subcommand_matches("directory") {
        let directory = matches.value_of("directory").unwrap();
        let exts = match matches.value_of("extension") {
            Some(extension) => get_extension(extension),
            None => vec!["png", "jpg", "jpeg"],
        };
        compare_directory(directory, &exts);
    } else if let Some(matches) = matches.subcommand_matches("match") {
        let image = matches.value_of("image").unwrap();
        
        let directory = matches.value_of("directory").unwrap();
        let exts = match matches.value_of("extension") {
            Some(extension) => get_extension(extension),
            None => vec!["png", "jpg", "jpeg"],
        };
        compare_match(image, directory, &exts);
    }
}

fn get_extension<'a>(extension_str: &'a str) -> Vec<&'a str> {
    let exts: Vec<&str> = extension_str.split(',').collect();
    match exts.len() {
        0 => vec!["png", "jpg", "jpeg"],
        _ => exts,
    }
}
