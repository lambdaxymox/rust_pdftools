use std::io::Result as IoResult;
use std::result::Result;
use std::iter::{Iterator, IntoIterator};
use std::collections::HashMap;
use std::collections::hash_map;
use std::vec::Vec;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};


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
    Horizontal,
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
    x_pixels: Pixels,
    y_pixels: Pixels,
}

impl ImageDimensions {
    fn new(x: Pixels, y: Pixels) -> ImageDimensions {
        ImageDimensions {
            x_pixels: x,
            y_pixels: y,
        }
    }
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
    fn identify(path: FilePath)                -> IoResult<String>;
    fn rescale(amount: Pixels, dir: Direction) -> IoResult<String>;
    fn expand_left_edge(amount: Pixels)        -> IoResult<String>;
    fn expand_right_edge(amount: Pixels)       -> IoResult<String>;
    fn expand_top_edge(amount: Pixels)         -> IoResult<String>;
    fn expand_bottom_edge(amount: Pixels)      -> IoResult<String>;
    fn trim_left_edge(amount: Pixels)          -> IoResult<String>;
    fn trim_right_edge(amount: Pixels)         -> IoResult<String>;
    fn trim_top_edge(amount: Pixels)           -> IoResult<String>;
    fn trim_bottom_edge(amount: Pixels)        -> IoResult<String>;
    fn set_resolution(res: ImageResolution)    -> IoResult<String>;
}

trait RunOperation<OpTrait, OtherOp> {
    fn run_operation(op: OtherOp) -> IoResult<String>;
}

impl<Op> RunOperation<Op, PageOps> for Op where Op: ElementaryPageOperations {
    fn run_operation(page_op: PageOps) -> IoResult<String> {
        match page_op {
            PageOps::Identify(path)           => Op::identify(path),
            PageOps::Rescale(amount, dir)     => Op::rescale(amount, dir),
            PageOps::ExpandLeftEdge(amount)   => Op::expand_left_edge(amount),
            PageOps::ExpandRightEdge(amount)  => Op::expand_right_edge(amount),
            PageOps::ExpandTopEdge(amount)    => Op::expand_top_edge(amount),
            PageOps::ExpandBottomEdge(amount) => Op::expand_bottom_edge(amount),
            PageOps::TrimLeftEdge(amount)     => Op::trim_left_edge(amount),
            PageOps::TrimRightEdge(amount)    => Op::trim_right_edge(amount),
            PageOps::TrimTopEdge(amount)      => Op::trim_top_edge(amount),
            PageOps::TrimBottomEdge(amount)   => Op::trim_bottom_edge(amount),
            PageOps::SetResolution(res)       => Op::set_resolution(res),
        }
    }
}


#[derive(Clone)]
struct CompoundPageOperation {
    page_name: FileName,
    page_path: FilePath,
    ops: Option<Vec<PageOps>>,
}

impl CompoundPageOperation {
    fn new(page_name: FileName, page_path: FilePath, ops: &[PageOps]) -> CompoundPageOperation {
        
        if ops.is_empty() {
            return CompoundPageOperation {
                page_name: page_name,
                page_path: page_path,
                ops: None,
            };
        }

        let mut vec = Vec::new();
        for op in ops.iter() {
            vec.push(op.clone());
        }

        CompoundPageOperation {
            page_name: page_name,
            page_path: page_path,
            ops: Some(vec),
        }
    }

    fn make_noop() -> CompoundPageOperation {
        CompoundPageOperation {
            page_name: String::from(""),
            page_path: String::from(""),
            ops: None,
        }
    }

    fn is_noop(&self) -> bool {
        match self.ops {
            Some(ref vec) => vec.is_empty(),
            None          => true,
        }
    }

    fn run_operation<Op>(&self) -> IoResult<String>
        where Op: ElementaryPageOperations {


        match self.ops {
            
            None      => {
                Ok(String::from("No Operation"))
            }
            Some(ref vec) => {
                if self.is_noop() {
                    // Should not happen.
                    unreachable!();
                    //return Ok(String::from("No Operation"));
                }

                let mut result = Ok(String::from(""));
                for op in vec.iter() {
                    let res = Op::run_operation(op.clone());
                    match res {
                        Ok(s) => {
                            continue;
                        }
                        Err(e) => {
                            result = Err(e);
                            break;
                        }
                    }
                }
                result
            }
        }
    }

}


#[derive(Clone)]
enum CompoundPageOps {
    AnOp(CompoundPageOperation),
    NoOp(CompoundPageOperation),
}

impl CompoundPageOps {
    fn new(page_name: FileName, page_path: FilePath, ops: &[PageOps]) -> CompoundPageOps {
        if ops.is_empty() {
            return CompoundPageOps::NoOp(CompoundPageOperation::new(page_name, page_path, ops));
        }

        CompoundPageOps::AnOp(CompoundPageOperation::new(page_name, page_path, ops))
    }
}


#[derive(Clone, Eq)]
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
            self.file_name      == other.file_name
        &&  self.file_extension == other.file_extension
        &&  self.file_path      == other.file_path
        &&  self.dimensions     == other.dimensions
        &&  self.resolution     == other.resolution
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


struct OperationResults {
    results: Vec<IoResult<String>>,
}

impl OperationResults {
    fn new(vec: Vec<IoResult<String>>) -> Self {
        OperationResults {
            results: vec,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum OperationScheduleError {
    LengthMismatch,
}

impl OperationSchedule {
    fn new() -> Self {
        OperationSchedule {
            schedule: HashMap::new(),
        }
    }
    
    fn add_operation(&mut self, page: Page, op: CompoundPageOperation) {
        self.schedule.insert(page, op);
    }

    fn build_schedule(pages: &[Page], ops: &[CompoundPageOperation]) -> Result<Self, OperationScheduleError> {
        if pages.len() == ops.len() {

            let mut schedule = OperationSchedule::new();

            for page_number in 0..pages.len() {
                schedule.add_operation(pages[page_number].clone(), ops[page_number].clone());
            }

            Ok(schedule)
        
        } else {
            Err(OperationScheduleError::LengthMismatch)
        }


    }

    fn iter(&self) -> OpSchedIter {
        OpSchedIter {
            inner: self.schedule.iter()
        }
    }

    fn run_operation<Op>(&self) -> OperationResults 
        where Op: ElementaryPageOperations {

        let mut results = Vec::new();

        for (page, op) in self {
            let result = op.run_operation::<Op>();
            results.push(result);
        }

        OperationResults::new(results)
    }

}

struct OpSchedIter<'a> {
    inner:  hash_map::Iter<'a, Page, CompoundPageOperation>
}

impl<'a> Iterator for OpSchedIter<'a> {
    type Item = (&'a Page, &'a CompoundPageOperation);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

}

impl<'a> IntoIterator for &'a OperationSchedule {
    type Item = (&'a Page, &'a CompoundPageOperation);
    type IntoIter = OpSchedIter<'a>;

    fn into_iter(self) -> OpSchedIter<'a> {
        self.iter()
    }
}

struct OpSchedIntoIter {
    inner: hash_map::IntoIter<Page, CompoundPageOperation>,
}

impl IntoIterator for OperationSchedule {
    type Item = (Page, CompoundPageOperation);
    type IntoIter = OpSchedIntoIter;

    fn into_iter(self) -> OpSchedIntoIter {
        OpSchedIntoIter {
            inner: self.schedule.into_iter()
        }
    }
}

impl Iterator for OpSchedIntoIter {
    type Item = (Page, CompoundPageOperation);

    fn next(&mut self) -> Option<(Page, CompoundPageOperation)> {
        self.inner.next()
    }
}
