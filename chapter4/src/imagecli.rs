#[allow(dead_code)]
mod imagix;
use ::imagix::error::ImagixError;
use ::imagix::resize::{process_resize_request, Mode, SizeOption};
use ::imagix::stats::get_stats;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt,Debug)]
#[structopt(name = "resize",about ="This tool resizes or gets info about images.")]
enum Commandline {
    #[structopt(about = "Resize the image to a specified size")]
    Resize {
        #[structopt(long, possible_values = &["small","medium","large"],case_insensitive = true)]
        size: String,

        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },

    #[structopt(about = "Get the dimensions of the image")]
    Info {
        #[structopt(parse(from_os_str))]
        file:PathBuf,
    },
}

fn resize_image(file: &PathBuf,size: &str) {
    println!("Resizing image: {:?} to size: {}",file,size);
}

fn get_image_info(file: &PathBuf) {
    match file.metadata() {
        Ok(metadata) => {
            let file_size = metadata.len();
            println!("The size of the image {:?} is {} bytes",file,file_size);
        }
        Err(e) => println!("Failed to get metadata for {:?}:{}",file,e),
    }
}

fn main() {
    let args = Commandline::from_args();

    match args {
        Commandline::Resize { size, file } => {
            resize_image(&file, &size);
        }
        Commandline::Info { file } => {
            get_image_info(&file);
        }
        
    }
}