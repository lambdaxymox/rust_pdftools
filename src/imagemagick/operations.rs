#![allow(dead_code)]
use std::io;
use std::convert::AsRef;
use super::imagemagick_commands;
use image_tools::image_ops::{ElementaryPageOperations, Pixels, Direction};
use image_tools::image_ops::ImageResolution;
use image_tools::image_ops::RunOperation;
use image_tools::image_ops::OperationResults;
use image_tools::image_ops::{FileName, FilePath};
use util::shell;


type ImageMagickArg = String;


#[derive(Clone, Debug)]
enum ImageMagickOpType {
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ElementaryImageMagickOperation {
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

    fn run_operation(&self) -> io::Result<String> {
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

#[derive(Clone, Debug)]
pub struct ImageMagickOperation {
    ops: Vec<ElementaryImageMagickOperation>,
}

impl ImageMagickOperation {
    fn new() -> ImageMagickOperation {
        ImageMagickOperation {
            ops: Vec::new(),
        }
    }

    fn add_op(&mut self, op: ElementaryImageMagickOperation)  {
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

    fn identify(file_name: FileName, file_path: FilePath) -> ImageMagickOperation {
        // Identify presesntly generates an identify command without the -verbose flag.
        // This may change in the future where we use verbose as the default.
        let args = ImageMagickArgs::new(file_name, file_path, &Vec::new());
        let elem_op = ElementaryImageMagickOperation::new(ImageMagickOpType::Identify, args);
        let mut op = ImageMagickOperation::new();
        op.add_op(elem_op);

        op

    }

    fn rescale(amount: Pixels, dir: Direction) -> ImageMagickOperation {
        unimplemented!();
    }

    fn expand_left_edge(amount: Pixels)        -> ImageMagickOperation {
        unimplemented!();
    }

    fn expand_right_edge(amount: Pixels)       -> ImageMagickOperation {
        unimplemented!();
    }

    fn expand_top_edge(amount: Pixels)         -> ImageMagickOperation {
        unimplemented!();
    }

    fn expand_bottom_edge(amount: Pixels)      -> ImageMagickOperation {
        unimplemented!();
    }

    fn trim_left_edge(amount: Pixels)          -> ImageMagickOperation {
        unimplemented!();
    }

    fn trim_right_edge(amount: Pixels)         -> ImageMagickOperation {
        unimplemented!();
    }

    fn trim_top_edge(amount: Pixels)           -> ImageMagickOperation {
        unimplemented!();
    }

    fn trim_bottom_edge(amount: Pixels)        -> ImageMagickOperation {
        unimplemented!();
    }

    fn set_resolution(res: ImageResolution)    -> ImageMagickOperation {
        unimplemented!();
    }

    fn no_operation()                          -> ImageMagickOperation {
        let mut op = ImageMagickOperation::new();
        let args = ImageMagickArgs::new(String::from(""), String::from(""), &Vec::new());
        let elem_op = ElementaryImageMagickOperation::new(ImageMagickOpType::NoOperation, args);
        op.add_op(elem_op);

        op
    }
}


impl AsRef<[ElementaryImageMagickOperation]> for ImageMagickOperation {
    fn as_ref(&self) -> &[ElementaryImageMagickOperation] {
        self.ops.as_ref()
    }
}
 

impl RunOperation for ImageMagickOperation {
    fn run_operation(op: ImageMagickOperation) -> OperationResults {
        let mut results = OperationResults::new();

        for action in op.ops {
            let mut result = Vec::new();
            result.push(action.run_operation());
            results.append(&mut OperationResults::from(&mut result));
        }

        results
    }
}
