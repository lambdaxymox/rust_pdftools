use std::io::{Result, Error};
use super::imagemagick_commands;


type Arguments = Vec<String>;
type FileName = String;
type FilePath = String;

enum ImageMagickOps {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
}

struct ImageMagickArgs {
    file_path: FilePath,
    file_name: FileName,
    img_args:  Arguments,  
}

trait ImageMagickOperation {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self;
    fn get_op_type(&self)   -> ImageMagickOps;
    fn get_arguments(&self) -> &ImageMagickArgs;
    fn call(&self)          -> Result<String>;
}

struct Identify {
    args: ImageMagickArgs,
}

impl ImageMagickOperation for Identify {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        Identify {
            args: ImageMagickArgs {
                file_path: file_path,
                file_name: file_name,
                img_args:  args,
            }
        }
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Identify
    }

    fn get_arguments(&self) -> &ImageMagickArgs {
        &self.args
    }

    fn call(&self) -> Result<String> {
        imagemagick_commands::imagemagick_identify_default(&self.args.file_path)
    }
}


struct IdentifyVerbose {
    args: ImageMagickArgs,
}

impl ImageMagickOperation for IdentifyVerbose {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        IdentifyVerbose {
            args: ImageMagickArgs {
                file_path: file_path,
                file_name: file_name,
                img_args:  args,
            }
        }
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::IdentifyVerbose
    }

    fn get_arguments(&self) -> &ImageMagickArgs {
        &self.args
    }

    fn call(&self) -> Result<String> {
        imagemagick_commands::imagemagick_identify_verbose(&self.args.file_path)
    }
}


struct Mogrify {
    args: ImageMagickArgs,
}

impl ImageMagickOperation for Mogrify {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        Mogrify {
            args: ImageMagickArgs {
                file_path: file_path,
                file_name: file_name,
                img_args:  args,
            }
        }
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Mogrify
    }

    fn get_arguments(&self) -> &ImageMagickArgs {
        &self.args
    }

    fn call(&self) -> Result<String> {
        unimplemented!();
    }
}


struct Convert {
    args: ImageMagickArgs,
}

impl ImageMagickOperation for Convert {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        Convert {
            args: ImageMagickArgs {
                file_path: file_path,
                file_name: file_name,
                img_args:  args,
            }
        }
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Convert
    }

    fn get_arguments(&self) -> &ImageMagickArgs {
        &self.args
    }

    fn call(&self) -> Result<String> {
        unimplemented!()
    }
}


fn make_operation<T>() -> T
    where T: ImageMagickOperation {

    unimplemented!();
}

fn run_operation<T>(op: &T) -> Result<String>
    where T: ImageMagickOperation {

    op.call()
}
