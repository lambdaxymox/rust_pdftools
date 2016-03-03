#![allow(dead_code)]
use std::io::Result as IoResult;
use std::result::Result;
use std::iter::{Iterator, IntoIterator};
use std::collections::HashMap;
use std::collections::hash_map;
use std::vec::Vec;
use std::vec;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::convert::From;
use std::slice;


pub type Pixels = usize;
pub type FileName = String;
pub type FilePath = String;


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ResolutionUnits {
    PixelsPerInch,
    PixelsPerCentimeter,
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ImageFileFormat {
    TIFF,
    PNG,
    JPEG,
    UNKNOWN,
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
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


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PageOps {
    NoOperation,
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
    fn no_operation()                                -> Self;
}

pub trait RunOperation {
    fn run_operation(op: Self) -> OperationResults;
}

trait CompileOperation<OpType, Op> {
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
            PageOps::NoOperation              => Op::no_operation(),
        }
    }
}


#[derive(Clone, Debug)]
struct CompoundPageOperation<Op> {
    page_name: FileName,
    page_path: FilePath,
    ops: Vec<Op>,
}

impl<Op> CompoundPageOperation<Op> where Op: Clone {

    fn new(page_name: FileName, page_path: FilePath, ops: &[Op]) -> CompoundPageOperation<Op> {
        let mut vec = Vec::new();
        for op in ops.iter() {
            vec.push(op.clone());
        }

        CompoundPageOperation {
            page_name: page_name,
            page_path: page_path,
            ops: vec,
        }
    }

    fn make_no_op(page_name: FileName, page_path: FilePath) -> CompoundPageOperation<Op> {
        CompoundPageOperation {
            page_name: page_name,
            page_path: page_path,
            ops: Vec::new(),
        }
    }


    fn is_no_op(&self) -> bool {
        self.ops.is_empty()
    }

    fn iter(&self) -> CPOIter<Op> {
        CPOIter {
            inner: self.ops.iter()
        }
    }

}

impl CompoundPageOperation<PageOps> {
    fn is_no_op(&self) -> bool {
        for op in self.ops.iter() {
            if *op == PageOps::NoOperation {
                return true;
            }
        }

        false
    }
}


/// Implementation of CompileOperation for compiling between CompoundPageOperations.
/// TODO: Cloning feels unnecessary here. CompileOperation may be revised to pass append
///       nonmutable pointer instead.
impl<Op, OtherOp> CompileOperation<CompoundPageOperation<Op>, CompoundPageOperation<OtherOp>>
    for CompoundPageOperation<Op> 
        where Op: Clone + CompileOperation<Op, OtherOp>,
              OtherOp: Clone
{
    fn compile_operation(old_ops: CompoundPageOperation<Op>) -> CompoundPageOperation<OtherOp> {
        let mut new_ops = Vec::new();

        for old_op in old_ops.clone() {
            let new_op = Op::compile_operation(old_op);
            new_ops.push(new_op);
        }

        CompoundPageOperation::new(old_ops.page_name.clone(), old_ops.page_path.clone(), new_ops.as_ref())
    }
}

impl<Op> RunOperation for CompoundPageOperation<Op>
    where Op: RunOperation {

    fn run_operation(op: CompoundPageOperation<Op>) -> OperationResults {
        let mut final_results = OperationResults::new();

        for elem_op in op {
            let mut results = Op::run_operation(elem_op);
            final_results.append(&mut results);
        }  

        final_results
    }
}


/// Iterator interface for a CompoundPageOperation.
struct CPOIter<'a, Op> where Op: 'a {
    inner: slice::Iter<'a, Op>,
}

impl<'a, Op> Iterator for CPOIter<'a, Op> {
    type Item = &'a Op;

    fn next(&mut self) -> Option<&'a Op> {
        self.inner.next()
    }
}

struct CPOIntoIter<Op> {
    inner: vec::IntoIter<Op>,
}

impl<Op> IntoIterator for CompoundPageOperation<Op> {
    type Item = Op;
    type IntoIter = CPOIntoIter<Op>;

    fn into_iter(self) -> CPOIntoIter<Op> {
        CPOIntoIter {
            inner: self.ops.into_iter(),
        }
    }
}

impl<Op> Iterator for CPOIntoIter<Op> {
    type Item = Op;

    fn next(&mut self) -> Option<Op> {
        self.inner.next()
    }
}

impl<'a, Op> IntoIterator for &'a CompoundPageOperation<Op> where Op: Clone {
    type Item = &'a Op;
    type IntoIter = CPOIter<'a, Op>;

    fn into_iter(self) -> CPOIter<'a, Op> {
        self.iter()
    }

}

impl<Op> AsRef<[Op]> for CompoundPageOperation<Op> {
    fn as_ref(&self) -> &[Op] {
        self.ops.as_ref()
    }
}


#[derive(Clone, Eq, Debug)]
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

#[derive(Clone, Eq, PartialEq, Debug)]
enum OperationStatus {
    NotExecuted,    // Not (yet) run
    Completed,      // Ran to completion with not errors reported
    Failed,         // Completed with errors.
    Aborted,        // Operation aborted.
}

#[derive(Debug)]
pub struct OperationResults {
    status: OperationStatus,
    results: Vec<OperationResult>,
}

impl OperationResults {
    pub fn new() -> OperationResults {
        OperationResults {
            status:  OperationStatus::NotExecuted,
            results: Vec::new(),
        }
    }

    pub fn push(&mut self, result: OperationResult) {
        if result.is_err() {
            self.status = OperationStatus::Failed;
        }

        if self.status == OperationStatus::NotExecuted {
            self.status = OperationStatus::Completed;
        }

        self.results.push(result);
    }

    pub fn append(&mut self, other: &mut OperationResults) {
        
        let mut status = OperationStatus::NotExecuted;

        for res in other.results.iter() {
            if res.is_err() {
                status = OperationStatus::Failed;
                break;
            }
        }

        if other.is_empty() {
            status = OperationStatus::Completed;
        }

        self.results.append(&mut other.results);
        self.status = status;
    }

    pub fn is_failed(&self) -> bool {
        self.status == OperationStatus::Failed
    }

    pub fn is_aborted(&self) -> bool {
        self.status == OperationStatus::Aborted
    }

    pub fn is_empty(&self) -> bool {
        self.results.len() == 0
    }

}


/// Destructive conversion from a mutable vector of OperationResult
/// to simplify the process of returning results from running operations.
impl<'a> From<&'a mut Vec<OperationResult>> for OperationResults {
    fn from(vec: &mut Vec<OperationResult>) -> OperationResults {
        let mut results = Vec::new();
        results.append(vec);

        let mut status = OperationStatus::Completed;

        for result in results.iter() {
            if result.is_err() {
                status = OperationStatus::Failed;
                break;
            }
        } 

        OperationResults {
            status: status,
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

        let status = if op_res.is_err() {
                        OperationStatus::Failed
                    } else {
                        OperationStatus::Completed
                    };

        results.push(op_res);

        OperationResults {
            status: status,
            results: results,
        }
    }
}


impl AsRef<[OperationResult]> for OperationResults {
    fn as_ref(&self) -> &[OperationResult] {
        self.results.as_ref()
    }
}


#[derive(Clone, Debug)]
struct OperationPlan<Op> {
    plan: HashMap<Page, CompoundPageOperation<Op>>, 
}


#[derive(Clone, Eq, PartialEq, Debug)]
enum OperationPlanError {
    LengthMismatch,
}


impl<Op> OperationPlan<Op> where Op: Clone {
    fn new() -> OperationPlan<Op> {
        OperationPlan {
            plan: HashMap::new(),
        }
    }
    
    fn insert(&mut self, page: Page, op: CompoundPageOperation<Op>) {
        self.plan.insert(page, op);
    }

    fn build_schedule(pages: &[Page], ops: &[CompoundPageOperation<Op>]) -> Result<Self, OperationPlanError> {
        if pages.len() == ops.len() {

            let mut plan = OperationPlan::new();

            for page_number in 0..pages.len() {
                plan.insert(pages[page_number].clone(), ops[page_number].clone());
            }

            Ok(plan)
        
        } else {
            Err(OperationPlanError::LengthMismatch)
        }


    }

    fn iter(&self) -> OpPlanIter<Op> {
        OpPlanIter {
            inner: self.plan.iter()
        }
    }

}


impl<Op, OtherOp> CompileOperation<OperationPlan<Op>, OperationPlan<OtherOp>> 
    for OperationPlan<Op>
        where Op: CompileOperation<Op, OtherOp> + Clone,
              OtherOp: Clone
{
    fn compile_operation(old_plan: OperationPlan<Op>) -> OperationPlan<OtherOp> {
        let mut new_plan = OperationPlan::new();

        for (page, old_op) in old_plan.clone() {
            let new_op = CompoundPageOperation::<Op>::compile_operation(old_op);
            new_plan.insert(page.clone(), new_op);
        }

        new_plan
    }
}


/// Iterator implementation for OperationPlan.
struct OpPlanIter<'a, Op: 'a> {
    inner:  hash_map::Iter<'a, Page, CompoundPageOperation<Op>>
}

impl<'a, Op> Iterator for OpPlanIter<'a, Op> {
    type Item = (&'a Page, &'a CompoundPageOperation<Op>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

}

/// IntoIterator implementation for OperationPlan.
impl<'a, Op> IntoIterator for &'a OperationPlan<Op> where Op: Clone {
    type Item = (&'a Page, &'a CompoundPageOperation<Op>);
    type IntoIter = OpPlanIter<'a, Op>;

    fn into_iter(self) -> OpPlanIter<'a, Op> {
        self.iter()
    }
}

struct OpPlanIntoIter<Op> {
    inner: hash_map::IntoIter<Page, CompoundPageOperation<Op>>,
}

impl<Op> IntoIterator for OperationPlan<Op> {
    type Item = (Page, CompoundPageOperation<Op>);
    type IntoIter = OpPlanIntoIter<Op>;

    fn into_iter(self) -> OpPlanIntoIter<Op> {
        OpPlanIntoIter {
            inner: self.plan.into_iter()
        }
    }
}

impl<Op> Iterator for OpPlanIntoIter<Op> {
    type Item = (Page, CompoundPageOperation<Op>);

    fn next(&mut self) -> Option<(Page, CompoundPageOperation<Op>)> {
        self.inner.next()
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
enum OperationPlanStatus {
    NotCompleted,
    Completed,
    Failed,
    Aborted,
}


#[derive(Debug)]
struct OperationPlanResult {
    status: OperationPlanStatus,
    results: HashMap<Page, OperationResults>,
}

impl OperationPlanResult {
    fn new() -> OperationPlanResult {
        OperationPlanResult {
            status: OperationPlanStatus::NotCompleted,
            results: HashMap::new(),
        }
    }

    fn insert(&mut self, page: Page, res: OperationResults) {
        let status = res.status.clone();

        self.status = match self.status {
            OperationPlanStatus::Failed      =>     OperationPlanStatus::Failed,
            OperationPlanStatus::NotCompleted => {
                match status {
                    OperationStatus::NotExecuted => OperationPlanStatus::NotCompleted,
                    OperationStatus::Completed   => OperationPlanStatus::Completed,
                    OperationStatus::Failed      => OperationPlanStatus::Failed,
                    OperationStatus::Aborted     => OperationPlanStatus::Aborted,
                }
            }
            OperationPlanStatus::Completed   => {
                match status {
                    OperationStatus::NotExecuted => OperationPlanStatus::NotCompleted, // FIXME: Change this case.
                    OperationStatus::Completed   => OperationPlanStatus::Completed,
                    OperationStatus::Failed      => OperationPlanStatus::Failed,
                    OperationStatus::Aborted     => OperationPlanStatus::Failed,
                }
            }
            OperationPlanStatus::Aborted     => {
                match status {
                    OperationStatus::Aborted     => OperationPlanStatus::Aborted,
                    _                            => OperationPlanStatus::Failed,
                }
            }
        };

        self.results.insert(page, res);
    }

    fn iter(&self) -> OpPlanResultIter {
        OpPlanResultIter {
            inner: self.results.iter()
        }
    }

    fn plan_status(&self) -> OperationPlanStatus {
        self.status.clone()
    }

    fn is_not_completed(&self) -> bool {
        self.status == OperationPlanStatus::NotCompleted
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


trait ExecutePlan<OpType> where OpType: RunOperation {
    type ExecutionResult;

    fn execute_plan(&self)                               -> Self::ExecutionResult;
    fn abort_plan(&self, result: &mut Self::ExecutionResult) -> Self::ExecutionResult;
}


impl<Op> ExecutePlan<Op> for OperationPlan<Op> 
    where Op: RunOperation + Clone
{
    type ExecutionResult = OperationPlanResult;

    /// Execute plan does not short circuit by default.
    /// TODO: Incorporate Execution strategies into executing plans.
    fn execute_plan(&self) -> OperationPlanResult {
        let mut report = OperationPlanResult::new();

        for (page, op) in self {
            let result = CompoundPageOperation::<Op>::run_operation(op.clone());
            report.insert(page.clone(), result);
        }

        report
    }

    /// 
    fn abort_plan(&self, result: &mut OperationPlanResult) -> OperationPlanResult {
        unimplemented!();
    }
}
