use std::io::{Result, Error};

type Arguments = Vec<String>;
type FileName = String;
type FilePath = String;

enum ImageMagickOps {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
}

trait ImageMagickOperation {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self;
    fn get_op_type(&self)   -> ImageMagickOps;
    fn get_arguments(&self) -> Arguments;
    fn call(&self)          -> Result<String>;
}

/*
struct ImageMagickArgs {

}
*/

struct Identify {
    file_path: FilePath,
    file_name: FileName,
    args:      Arguments,  
}

impl ImageMagickOperation for Identify {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Identify
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<String> {
        unimplemented!();
    }
}


struct IdentifyVerbose {
    file_path: FilePath,
    file_name: FileName,
    args:      Arguments,
}

impl ImageMagickOperation for IdentifyVerbose {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::IdentifyVerbose
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<String> {
        unimplemented!();
    }
}


struct Mogrify {
    file_path: FilePath,
    file_name: FileName,
    args:      Arguments,
}

impl ImageMagickOperation for Mogrify {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Mogrify
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<String> {
        unimplemented!();
    }
}


struct Convert {
    file_path: FilePath,
    file_name: FileName,
    args: Arguments,
}

impl ImageMagickOperation for Convert {
    fn new(file_path: FilePath, file_name: FileName, args: Arguments) -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Convert
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<String> {
        unimplemented!();
    }
}

/*
fn make_operation() -> ImageMagickOperation {
    unimplemented!();
}

fn run_operaton(op: ImageMagickOperation) -> Result<String> {
    unimplemented!();
}
*/
