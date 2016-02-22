use std::process::{Command, Output};
use std::string::{String, ToString};
use std::ffi::OsString;
use std::path::Path;
use std::io::{Result, Error};
use std::result;


pub type FilePath = String;
type FileName = String;

enum ImageMagickCommand {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
}

impl ImageMagickCommand {

}

impl ToString for ImageMagickCommand {
    fn to_string(&self) -> String {
        match *self {
            ImageMagickCommand::Identify        => "identify".to_string(),
            ImageMagickCommand::IdentifyVerbose => "identify".to_string(),
            ImageMagickCommand::Mogrify         => "mogrify".to_string(),
            ImageMagickCommand::Convert         => "convert".to_string(),
        }
    }
}

fn imagemagick_command(command_name: ImageMagickCommand, 
                       file_path: &FilePath, 
                       args: &[String]) 
    -> Result<String> {

    let mut command_args: Vec<OsString> = Vec::new();
    for arg in args {
        command_args.push(OsString::from(arg));
    }

    let output: Result<Output> = Command::new(command_name.to_string())
                                        .args(command_args.as_ref())
                                        .arg(file_path)
                                        .output();
    
    let result: Result<String> = match output {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(e)     => Err(e),
    };
    
    result
}

fn imagemagick_identify(file_path: &FilePath, args: &[String]) -> Result<String> {
    imagemagick_command(ImageMagickCommand::Identify, file_path, args)
}


pub fn imagemagick_identify_default(file_path: &FilePath) -> Result<String> {
    imagemagick_identify(file_path, &[])
}


fn imagemagick_identify_verbose(file_path: &FilePath) -> Result<String> {
    let args = ["-verbose".to_string()];

    imagemagick_identify(file_path, &args)
}


fn imagemagick_mogrify(file_path: &FilePath, args: &[String]) -> Result<String> {
    imagemagick_command(ImageMagickCommand::Mogrify, file_path, args)
}


fn imagemagick_convert(file_path: &FilePath, args: &[String]) -> Result<String> {
    imagemagick_command(ImageMagickCommand::Convert, file_path, args)
}
