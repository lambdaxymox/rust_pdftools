use std::result::Result;

type Pixels = usize;

enum ResolutionUnits {
    PixelsPerInch,
    PixelsPerCentimeter,
}

enum Direction {
    Horizonal,
    Vertical,
}

struct Resolution {
    amount: Pixels,
    units: ResolutionUnits,
}

impl Resolution {
    fn new(amount: usize, units: ResolutionUnits) -> Resolution {
        Resolution {
            amount: amount,
            units: units,
        }
    }
}

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
    SetResolution(Resolution),
}

trait ElementaryPageOperations {
    fn identify(&FilePath)                     -> Result<String>;
    fn rescale(amount: pixels, dir: Direction) -> Result<String>;
    fn expand_left_edge(amount: Pixels)        -> Result<String>;
    fn expand_right_edge(amount: Pixels)       -> Result<String>;
    fn expand_top_edge(amount: Pixels)         -> Result<String>;
    fn expand_bottom_edge(amount: Pixels)      -> Result<String>;
    fn trim_left_edge(amount: Pixels)          -> Result<String>;
    fn trim_right_edge(amount: Pixels)         -> Result<String>;
    fn trim_top_edge(amount: Pixels)           -> Result<String>;
    fn trim_bottom_edge(amount: Pixels)        -> Result<String>;
    fn set_resolution(res: Resolution)         -> Result<String>;
}

struct CompoundPageOperation {
    page_name: FileName,
    page_path: FilePath,
    ops: Vec<PageOps>,
}

impl CompoundPageOperation {

}

struct Page {

    // Some information about the page goes here.
}

impl Page {

}

struct Book {
    // Collection of pages.
    pages: Vec<Page>,
}

impl Book {

}

struct OperationSchedule {
    // Some kind of map between Pages and CompundPageOperations.
}

impl OperationSchedule {

}