use std::io::{Result, Error};

type Arguments = Vec<String>;

enum ImageMagickOps {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
}

trait ImageMagickOperation {
    fn new()                -> Self;
    fn get_op_type(&self)   -> ImageMagickOps;
    fn get_arguments(&self) -> Arguments;
    fn call(&self)          -> Result<String>;
}

struct Identify {
    file_path: FilePath,
    file_name: FileName,
    args: Arguments,
    action: Fn<>(),    
}

impl ImageMagickOperation for Identify {
    fn new() -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Identify
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<> {
        unimplemented!();
    }
}


struct IdentifyVerbose {
    file_path: FilePath,
    file_name: FileName,
    args: Arguments,
    actions: Fn<>(),
}

impl ImageMagickOperation for IdentifyVerbose {
    fn new() -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::IdentifyVerbose
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<> {
        unimplemented!();
    }
}


struct Mogrify {
    file_path: FilePath,
    file_name: FileName,
    args: Arguments,
    action: Fn<>()
}

impl ImageMagickOperation for Mogrify {
    fn new() -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Mogrify
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<> {
        unimplemented!();
    }
}


struct Convert {
    file_path: FilePath,
    file_name: FileName,
    args: Arguments,
    action: Fn<>(),
}

impl ImageMagickOperation for Convert {
    fn new() -> Self {
        unimplemented!();
    }

    fn get_op_type(&self) -> ImageMagickOps {
        ImageMagickOps::Convert
    }

    fn get_arguments(&self) -> Arguments {
        unimplemented!();
    }

    fn call(&self) -> Result<> {
        unimplemented!();
    }
}


fn make_operation() -> ImageMagickOperation {
    unimplemented!();
}

fn run_operaton(op: ImageMagickOperation) -> Result<String>
    unimplemented!();

