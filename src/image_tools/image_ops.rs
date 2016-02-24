use std::io::Result;
use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher, SipHasher};


type Pixels = usize;
type FileName = String;
type FilePath = String;


#[derive(Clone, Eq, PartialEq, Hash)]
enum ResolutionUnits {
    PixelsPerInch,
    PixelsPerCentimeter,
}


#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    Horizonal,
    Vertical,
}


#[derive(Clone, Eq, PartialEq, Hash)]
enum ImageFileFormat {
    TIFF,
    PNG,
    JPEG,
    UNKNOWN,
}


#[derive(Clone, Eq, PartialEq, Hash)]
struct ImageDimensions {
    xPixels: Pixels,
    yPixels: Pixels,
}


#[derive(Clone, Eq, PartialEq, Hash)]
struct ImageResolution {
    amount: Pixels,
    units: ResolutionUnits,
}

impl ImageResolution {
    fn new(amount: usize, units: ResolutionUnits) -> ImageResolution {
        ImageResolution {
            amount: amount,
            units: units,
        }
    }
}

#[derive(Clone)]
enum PageOps {
    Identify(FilePath),
    Rescale(Pixels, Direction),
    ExpandLeftEdge(Pixels),
    ExpandRightEdge(Pixels),
    ExpandTopEdge(Pixels),
    ExpandBottomEdge(Pixels),
    TrimLeftEdge(Pixels),
    TrimRightEdge(Pixels),
    TrimTopEdge(Pixels),
    TrimBottomEdge(Pixels),
    SetResolution(ImageResolution),
}

trait ElementaryPageOperations {
    fn identify(path: &FilePath)               -> Result<String>;
    fn rescale(amount: Pixels, dir: Direction) -> Result<String>;
    fn expand_left_edge(amount: Pixels)        -> Result<String>;
    fn expand_right_edge(amount: Pixels)       -> Result<String>;
    fn expand_top_edge(amount: Pixels)         -> Result<String>;
    fn expand_bottom_edge(amount: Pixels)      -> Result<String>;
    fn trim_left_edge(amount: Pixels)          -> Result<String>;
    fn trim_right_edge(amount: Pixels)         -> Result<String>;
    fn trim_top_edge(amount: Pixels)           -> Result<String>;
    fn trim_bottom_edge(amount: Pixels)        -> Result<String>;
    fn set_resolution(res: ImageResolution)    -> Result<String>;
}

impl ElementaryPageOperations {

}


struct CompoundPageOperation {
    page_name: FileName,
    page_path: FilePath,
    ops: Vec<PageOps>,
}

impl CompoundPageOperation {
    fn new(page_name: FileName, page_path: FilePath, ops: &[PageOps]) -> CompoundPageOperation {
        unimplemented!();
    }

    fn run_operation(&self) -> Result<String> {
        unimplemented!();
    }
}


#[derive(Eq)]
struct Page {
    file_name: FileName,
    file_extension: ImageFileFormat,
    file_path: FilePath,
    dimensions: ImageDimensions,
    resolution: ImageResolution,
}

impl Page {
    fn new  ( 
                file_name: FileName, 
                file_extension: ImageFileFormat, 
                file_path: FilePath, 
                dimensions: ImageDimensions, 
                resolution: ImageResolution
            ) -> Page 
    {
        Page {
            file_name: file_name,
            file_extension: file_extension,
            file_path: file_path,
            dimensions: dimensions,
            resolution: resolution,
        }
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Page) -> bool {
            self.file_name == other.file_name
        &&  self.file_extension == other.file_extension
        &&  self.file_path == other.file_path
        &&  self.dimensions == other.dimensions
        &&  self.resolution == other.resolution
    }
}

impl Hash for Page {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher {

        self.file_name.hash(state);
        self.file_extension.hash(state);
        self.file_path.hash(state);
        self.dimensions.hash(state);
        self.resolution.hash(state);
    }
}


struct OperationSchedule {
    schedule: HashMap<Page, CompoundPageOperation>, 
}
/*
struct OperationResult {
    // Results from each page operation
}
*/
impl OperationSchedule {
    fn new() -> Self {
        OperationSchedule {
            schedule: HashMap::new(),
        }
    }
    /*
    fn run_schedule(schedule: OperationSchedule) -> OperationResult {
        unimplemented!();
    }
    */
}

