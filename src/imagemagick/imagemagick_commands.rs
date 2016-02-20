use std::process::{Command, Output};
use std::string::String;
use std::ffi::OsString;
use std::path::Path;
use std::io::{Result, Error};


pub type FilePath = String;
type FileName = String;


enum ImageMagickCommand {
    Identify(FilePath, FileName),
    IdentifyVerbose(FilePath, FileName),
}


fn imagemagick_identify(file_path: &FilePath, args: &[String]) -> Result<String> {
    
    let mut command_args: Vec<OsString> = Vec::new();
    for arg in args {
        command_args.push(OsString::from(arg));
    }
    
    let output: Result<Output> = Command::new("identify")
                                        .args(command_args.as_ref())
                                        .arg(file_path)
                                        .output();
    
    let result: Result<String> = match output {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(e)     => Err(e),
    };
    
    result
}


pub fn imagemagick_identify_default(file_path: &FilePath) -> Result<String> {
    imagemagick_identify(file_path, &[])
}


fn imagemagick_identify_verbose(file_path: &FilePath) -> Result<String> {
    let args = ["-verbose".to_string()];

    imagemagick_identify(file_path, &args)
}


#[allow(unused_variables)]
fn run_command(command: ImageMagickCommand) -> Result<String> {
    match command {
        ImageMagickCommand::Identify(file_path, file_name)        => {
            imagemagick_identify_default(&file_path)
        }
        ImageMagickCommand::IdentifyVerbose(file_path, file_name) => { 
            imagemagick_identify_verbose(&file_path)
        }
    }
}
