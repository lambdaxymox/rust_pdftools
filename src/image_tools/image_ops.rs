#![allow(dead_code)]
use std::io::Result as IoResult;
use std::result::Result;
use std::iter::{Iterator, IntoIterator};
use std::collections::HashMap;
use std::collections::hash_map;
use std::vec::Vec;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::convert::From;


pub type Pixels = usize;
pub type FileName = String;
pub type FilePath = String;


#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ResolutionUnits {
    PixelsPerInch,
    PixelsPerCentimeter,
}


#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    Vertical,
}


#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ImageFileFormat {
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
pub struct ImageResolution {
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
pub enum PageOps {
    Identify(FileName, FilePath),
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

pub trait ElementaryPageOperations {
    fn identify(file_name: FileName, path: FilePath) -> Self;
    fn rescale(amount: Pixels, dir: Direction)       -> Self;
    fn expand_left_edge(amount: Pixels)              -> Self;
    fn expand_right_edge(amount: Pixels)             -> Self;
    fn expand_top_edge(amount: Pixels)               -> Self;
    fn expand_bottom_edge(amount: Pixels)            -> Self;
    fn trim_left_edge(amount: Pixels)                -> Self;
    fn trim_right_edge(amount: Pixels)               -> Self;
    fn trim_top_edge(amount: Pixels)                 -> Self;
    fn trim_bottom_edge(amount: Pixels)              -> Self;
    fn set_resolution(res: ImageResolution)          -> Self;
}

pub trait RunOperation {
    fn run_operation(op: Self) -> OperationResults;
}

trait CompileOperation<OpType, Op> where Op: ElementaryPageOperations {
    fn compile_operation(op: OpType) -> Op;
}

impl<Op> CompileOperation<PageOps, Op> for Op where Op: ElementaryPageOperations {
    fn compile_operation(op: PageOps) -> Op {
        match op {
            PageOps::Identify(file, path)     => Op::identify(file, path),
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

    fn run_operation<Op>(&self) -> OperationResults
        where Op: ElementaryPageOperations + RunOperation + CompileOperation<PageOps, Op> {

        match self.ops {   
            None      => {
                OperationResults::from(Ok(String::from("No Operation")))
            }
            Some(ref vec) => {
                if self.is_noop() {
                    // Should not happen.
                    unreachable!();
                    //return Ok(String::from("No Operation"));
                }

                let mut result = Ok(String::from(""));
                for op in vec.iter() {
                    let op_results = Op::run_operation(Op::compile_operation(op.clone()));
                    for res in op_results.results {    
                        match res {
                            Ok(s) => {
                                continue;
                            }
                            Err(e) => {
                                // Fail fast if there is an error.
                                result = Err(e);
                                break;
                            }
                        }
                    }
                }

                OperationResults::from(result)
            }
        }
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
    fn hash<H>(&self, state: &mut H) where H: Hasher {

        self.file_name.hash(state);
        self.file_extension.hash(state);
        self.file_path.hash(state);
        self.dimensions.hash(state);
        self.resolution.hash(state);
    }
}


pub type OperationResult = IoResult<String>;


pub struct OperationResults {
    results: Vec<OperationResult>,
}

impl OperationResults {
    pub fn new() -> OperationResults {
        OperationResults {
            results: Vec::new(),
        }
    }

    pub fn push(&mut self, result: OperationResult) {
        self.results.push(result);
    }

    pub fn append(&mut self, other: &mut OperationResults) {
        self.results.append(&mut other.results);
    }

}

impl From<Vec<OperationResult>> for OperationResults {
    fn from(vec: Vec<OperationResult>) -> OperationResults {
        unimplemented!();
    }
}

/// Destructive conversion from a mutable vector of OperationResult
/// to simplify the process of returning results from running operations.
impl<'a> From<&'a mut Vec<OperationResult>> for OperationResults {
    fn from(vec: &mut Vec<OperationResult>) -> OperationResults {
        let mut results = Vec::new();
        results.append(vec);

        OperationResults {
            results: results,
        }
    }
}

/// Generates an OperationResults struct from a single OperationResult.
/// For compatibility between operations that may return multiple results
/// and ones that may return only one result.
impl From<OperationResult> for OperationResults {
    fn from(op_res: OperationResult) -> OperationResults {
        let mut results = Vec::new();
        results.push(op_res);

        OperationResults {
            results: results
        }
    }
}


struct OperationPlan {
    schedule: HashMap<Page, CompoundPageOperation>, 
}


#[derive(Clone, Eq, PartialEq)]
enum OperationScheduleError {
    LengthMismatch,
}


impl OperationPlan {
    fn new() -> OperationPlan {
        OperationPlan {
            schedule: HashMap::new(),
        }
    }
    
    fn add_operation(&mut self, page: Page, op: CompoundPageOperation) {
        self.schedule.insert(page, op);
    }

    fn build_schedule(pages: &[Page], ops: &[CompoundPageOperation]) -> Result<Self, OperationScheduleError> {
        if pages.len() == ops.len() {

            let mut schedule = OperationPlan::new();

            for page_number in 0..pages.len() {
                schedule.add_operation(pages[page_number].clone(), ops[page_number].clone());
            }

            Ok(schedule)
        
        } else {
            Err(OperationScheduleError::LengthMismatch)
        }


    }

    fn iter(&self) -> OpPlanIter {
        OpPlanIter {
            inner: self.schedule.iter()
        }
    }

    fn run_operation<Op>(&self) -> OperationResults 
        where Op: ElementaryPageOperations + RunOperation + CompileOperation<PageOps, Op> {

        let mut results = OperationResults::new();

        for (page, op) in self {
            
            match op.ops {
                None => {
                    continue;
                } 
                Some(ref operations) => {
                    for elem_op in operations {
                        let compiled_op = Op::compile_operation(elem_op.clone());
                        let mut result = Op::run_operation(compiled_op);
                        results.append(&mut result);
                    }
                }
            }

        }

        results
    }
}

/// Iterator implementation for OperationPlan.
struct OpPlanIter<'a> {
    inner:  hash_map::Iter<'a, Page, CompoundPageOperation>
}

impl<'a> Iterator for OpPlanIter<'a> {
    type Item = (&'a Page, &'a CompoundPageOperation);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

}

/// IntoIterator implementation for OperationPlan.
impl<'a> IntoIterator for &'a OperationPlan {
    type Item = (&'a Page, &'a CompoundPageOperation);
    type IntoIter = OpPlanIter<'a>;

    fn into_iter(self) -> OpPlanIter<'a> {
        self.iter()
    }
}

struct OpPlanIntoIter {
    inner: hash_map::IntoIter<Page, CompoundPageOperation>,
}

impl IntoIterator for OperationPlan {
    type Item = (Page, CompoundPageOperation);
    type IntoIter = OpPlanIntoIter;

    fn into_iter(self) -> OpPlanIntoIter {
        OpPlanIntoIter {
            inner: self.schedule.into_iter()
        }
    }
}

impl Iterator for OpPlanIntoIter {
    type Item = (Page, CompoundPageOperation);

    fn next(&mut self) -> Option<(Page, CompoundPageOperation)> {
        self.inner.next()
    }
}


struct OperationPlanResult {
    results: HashMap<Page, OperationResults>,
}

impl OperationPlanResult {
    fn new() -> OperationPlanResult {
        OperationPlanResult {
            results: HashMap::new(),
        }
    }

    fn insert(&mut self, page: Page, res: OperationResults) {
        self.results.insert(page, res);
    }

    fn iter(&self) -> OpPlanResultIter {
        OpPlanResultIter {
            inner: self.results.iter()
        }
    }
}

struct OpPlanResultIter<'a> {
    inner: hash_map::Iter<'a, Page, OperationResults>,
}

impl<'a> Iterator for OpPlanResultIter<'a> {
    type Item = (&'a Page, &'a OperationResults);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// IntoIterator implementation for OpPlanResultIter.
impl<'a> IntoIterator for &'a OperationPlanResult {
    type Item = (&'a Page, &'a OperationResults);
    type IntoIter = OpPlanResultIter<'a>;

    fn into_iter(self) -> OpPlanResultIter<'a> {
        self.iter()
    }
}

struct OpPlanResultIntoIter {
    inner: hash_map::IntoIter<Page, OperationResults>,
}

impl IntoIterator for OperationPlanResult {
    type Item = (Page, OperationResults);
    type IntoIter = OpPlanResultIntoIter;

    fn into_iter(self) -> OpPlanResultIntoIter {
        OpPlanResultIntoIter {
            inner: self.results.into_iter()
        }
    }
}

impl Iterator for OpPlanResultIntoIter {
    type Item = (Page, OperationResults);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
