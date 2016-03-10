#![allow(dead_code)]
use std::process::{Command, Output};
use std::string::{String, ToString};
use std::ffi::OsString;
use std::io;
use image_tools::image_ops::{FileName, FilePath};
use super::op_types::{ImageMagickOpType};
use util::shell::AsShellCommand;


fn imagemagick_command(command_name: ImageMagickOpType, 
                       file_path: &FilePath, 
                       args: &[String]) 
    -> io::Result<String> {

    let mut command_args: Vec<OsString> = Vec::new();
    for arg in args {
        command_args.push(OsString::from(arg));
    }

    let output: io::Result<Output> = Command::new(command_name.as_shell_command())
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
    imagemagick_command(ImageMagickOpType::Identify, file_path, args)
}


pub fn imagemagick_identify_default(file_path: &FilePath) -> io::Result<String> {
    imagemagick_identify(file_path, &[])
}


pub fn imagemagick_identify_verbose(file_path: &FilePath) -> io::Result<String> {
    let args = ["-verbose".to_string()];

    imagemagick_identify(file_path, &args)
}


pub fn imagemagick_mogrify(file_path: &FilePath, args: &[String]) -> io::Result<String> {
    imagemagick_command(ImageMagickOpType::Mogrify, file_path, args)
}


pub fn imagemagick_convert(file_path: &FilePath, args: &[String]) -> io::Result<String> {
    imagemagick_command(ImageMagickOpType::Convert, file_path, args)
}

pub fn imagemagick_no_operation() -> io::Result<String> {
    Ok(String::from("No Operation"))
}
