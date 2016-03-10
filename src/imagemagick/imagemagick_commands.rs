#![allow(dead_code)]
use std::process::{Command, Output};
use std::string::{String, ToString};
use std::ffi::OsString;
use std::io;
use std::fmt;


pub type FilePath = String;
type FileName = String;

enum ImageMagickCommand {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
    NoOperation,
}


impl fmt::Display for ImageMagickCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImageMagickCommand::Identify        => write!(f, "Identify"),
            ImageMagickCommand::IdentifyVerbose => write!(f, "IdentifyVerbose"),
            ImageMagickCommand::Mogrify         => write!(f, "Mogrify"),
            ImageMagickCommand::Convert         => write!(f, "Convert"),
            ImageMagickCommand::NoOperation     => write!(f, "NoOperation"),
        }
    }
}

fn imagemagick_command(command_name: ImageMagickCommand, 
                       file_path: &FilePath, 
                       args: &[String]) 
    -> io::Result<String> {

    let mut command_args: Vec<OsString> = Vec::new();
    for arg in args {
        command_args.push(OsString::from(arg));
    }

    let output: io::Result<Output> = Command::new(command_name.to_string())
                                            .args(command_args.as_ref())
                                            .arg(file_path)
                                            .output();
    
    let result: io::Result<String> = match output {
            Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
            Err(e)     => Err(e),
        };

    result
}

#[inline]
fn imagemagick_identify(file_path: &FilePath, args: &[String]) -> io::Result<String> {
    imagemagick_command(ImageMagickCommand::Identify, file_path, args)
}


pub fn imagemagick_identify_default(file_path: &FilePath) -> io::Result<String> {
    imagemagick_identify(file_path, &[])
}


pub fn imagemagick_identify_verbose(file_path: &FilePath) -> io::Result<String> {
    let args = ["-verbose".to_string()];

    imagemagick_identify(file_path, &args)
}


pub fn imagemagick_mogrify(file_path: &FilePath, args: &[String]) -> io::Result<String> {
    imagemagick_command(ImageMagickCommand::Mogrify, file_path, args)
}


pub fn imagemagick_convert(file_path: &FilePath, args: &[String]) -> io::Result<String> {
    imagemagick_command(ImageMagickCommand::Convert, file_path, args)
}

pub fn imagemagick_no_operation() -> io::Result<String> {
    Ok(String::from("No Operation"))
}
