#![allow(dead_code)]
use std::io::Result as IoResult;
use std::io::Error  as IoError;
use super::imagemagick_commands;
use image_tools::image_ops::{ElementaryPageOperations, Pixels, Direction};
use image_tools::image_ops::{ImageResolution};


type ImageMagickArg = String;
type FileName = String;
type FilePath = String;

#[derive(Clone)]
enum ImageMagickOpType {
    Identify,
    IdentifyVerbose,
    Mogrify,
    Convert,
    NoOperation,
}

#[derive(Clone)]
struct ImageMagickArgs {
    file_path: FilePath,
    file_name: FileName,
    img_args:  Vec<ImageMagickArg>,  
}

impl ImageMagickArgs {
    fn new(file_path: FilePath, file_name: FileName, img_args: &Vec<ImageMagickArg>) -> ImageMagickArgs {
        ImageMagickArgs {
            file_path: file_path,
            file_name: file_name,
            img_args:  img_args.clone(),
        }
    }
}

#[derive(Clone)]
struct ElementaryImageMagickOperation {
    op:   ImageMagickOpType,
    args: ImageMagickArgs,
}

impl ElementaryImageMagickOperation {
    fn new(op: ImageMagickOpType, args: ImageMagickArgs) -> ElementaryImageMagickOperation {
        ElementaryImageMagickOperation {
            op:   op,
            args: args,
        }
    }

    fn arg(&mut self, arg: ImageMagickArg) {
        self.args.img_args.push(arg);
    }


    fn args(&mut self, args: &[ImageMagickArg]) {
        for arg in args {
            self.args.img_args.push(arg.clone());
        }
    }

    fn run_operation(&self) -> IoResult<String> {
        match self.op {
            ImageMagickOpType::Identify        => {
                imagemagick_commands::imagemagick_identify_default(&self.args.file_path)
            }
            ImageMagickOpType::IdentifyVerbose => {
                imagemagick_commands::imagemagick_identify_verbose(&self.args.file_path)
            }
            ImageMagickOpType::Convert         => {
                imagemagick_commands::imagemagick_convert(&self.args.file_path, &self.args.img_args)
            }
            ImageMagickOpType::Mogrify         => {
                imagemagick_commands::imagemagick_mogrify(&self.args.file_path, &self.args.img_args)
            }
            ImageMagickOpType::NoOperation     => {
                imagemagick_commands::imagemagick_no_operation()
            }
        }
    }
}

#[derive(Clone)]
struct ImageMagickOperation {
    ops: Vec<ElementaryImageMagickOperation>,
}

impl ImageMagickOperation {
    fn new() -> ImageMagickOperation {
        ImageMagickOperation {
            ops: Vec::new(),
        }
    }

    fn add_op(&mut self, op: ElementaryImageMagickOperation) {
        self.ops.push(op);
    }

    fn add_ops(&mut self, ops: &[ElementaryImageMagickOperation]) {
        for op in ops.iter() {
            self.ops.push(op.clone());
        }
    }
}

// This implementation will be the generator for the sequence of 
// ImageMagick commands to implement each operation.
impl ElementaryPageOperations for ImageMagickOperation {
    fn identify(path: FilePath)                -> IoResult<String> {
        unimplemented!();
    }

    fn rescale(amount: Pixels, dir: Direction) -> IoResult<String> {
        unimplemented!();
    }

    fn expand_left_edge(amount: Pixels)        -> IoResult<String> {
        unimplemented!();
    }

    fn expand_right_edge(amount: Pixels)       -> IoResult<String> {
        unimplemented!();
    }

    fn expand_top_edge(amount: Pixels)         -> IoResult<String> {
        unimplemented!();
    }

    fn expand_bottom_edge(amount: Pixels)      -> IoResult<String> {
        unimplemented!();
    }

    fn trim_left_edge(amount: Pixels)          -> IoResult<String> {
        unimplemented!();
    }

    fn trim_right_edge(amount: Pixels)         -> IoResult<String> {
        unimplemented!();
    }

    fn trim_top_edge(amount: Pixels)           -> IoResult<String> {
        unimplemented!();
    }

    fn trim_bottom_edge(amount: Pixels)        -> IoResult<String> {
        unimplemented!();
    }

    fn set_resolution(res: ImageResolution)    -> IoResult<String> {
        unimplemented!();
    } 
}


/*
fn make_operation<T>() -> T
    where T: ImageMagickOperation {

    unimplemented!();
}

fn run_operation<T>(op: &T) -> Result<String>
    where T: ImageMagickOperation {

    op.call()
}
*/