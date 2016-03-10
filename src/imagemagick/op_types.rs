use std::fmt;
use util::shell;

#[derive(Clone, Debug)]
pub enum ImageMagickOpType {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
    NoOperation,
}


impl shell::AsShellCommand for ImageMagickOpType {
    fn as_shell_command(&self) -> String {
        match *self {
            ImageMagickOpType::Identify        => String::from("identify"),
            ImageMagickOpType::IdentifyVerbose => String::from("identify"),
            ImageMagickOpType::Mogrify         => String::from("mogrify"),
            ImageMagickOpType::Convert         => String::from("convert"),
            ImageMagickOpType::NoOperation     => String::from("echo"),
        }
    }
}

impl fmt::Display for ImageMagickOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImageMagickOpType::Identify        => write!(f, "Identify"),
            ImageMagickOpType::IdentifyVerbose => write!(f, "IdentifyVerbose"),
            ImageMagickOpType::Mogrify         => write!(f, "Mogrify"),
            ImageMagickOpType::Convert         => write!(f, "Convert"),
            ImageMagickOpType::NoOperation     => write!(f, "NoOperation"),
        }
    }
}
