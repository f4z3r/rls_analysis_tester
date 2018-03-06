
use rls_analysis::AnalysisLoader;       // Required trait
use rls_analysis::{AnalysisHost, Span, Def};
use std::path::{Path, PathBuf};

/// Prints the title of the analysis underlined.
pub fn print_title(title: &str) {
    println!("\n\n{}\n{:=<2$}", title, "", title.len());
}


/// Pretty prints the spans given in the argument.
///
/// # Arguments
/// - `spans` is a `Vec<rls_analysis::Span>` containing the spans to be printed.
pub fn pretty_print_spans(spans: &Vec<Span>) {
    println!("\nReferences:");
    for span in spans {
        println!("SPAN");
        println!("{:<20}{:?}", "file:", span.file);
        println!("{:<20}{:?}\n{:<20}{:?}\n{:<20}{:?}\n{:<20}{:?}",
                 "range:  row_start: ",
                 span.range.row_start,
                 "        row_end:",
                 span.range.row_end,
                 "        col_start:",
                 span.range.col_start,
                 "        col_end:",
                 span.range.col_end);
        println!("");
    }
}

/// Pretty prints the definitions given in the argument.
///
/// # Arguments
/// - `defs` is a `Vec<rls_analysis::Def>` containing the definitions to be printed.
pub fn pretty_print_defs(defs: &Vec<Def>) {
    for def in defs {
        println!("Found definition with name: {}", def.name);
        println!("{:<20}{:?}", "Kind:", def.kind);
        println!("{:<20}{:?}\n{:<20}{:?}\n{:<20}{:?}\n{:<20}{:?}",
                 "range:  row_start: ",
                 def.span.range.row_start,
                 "        row_end:",
                 def.span.range.row_end,
                 "        col_start:",
                 def.span.range.col_start,
                 "        col_end:",
                 def.span.range.col_end);
        println!("{:<20}{:?}", "Qualname:", def.qualname);
        println!("{:<20}{:?}", "Distro crate:", def.distro_crate);
        println!("{:<20}{:?}", "Parent:", def.parent);
        println!("{:<20}{:?}", "Value:", def.value);
        println!("{:<20}{:?}", "Docs:", def.docs);
        println!("");
    }
}


// ####################################################################################
// Do not modify

#[derive(Clone)]
pub struct TestAnalysisLoader {
    path: PathBuf,
}

impl TestAnalysisLoader {
    pub fn new(path: PathBuf) -> TestAnalysisLoader {
        TestAnalysisLoader { path }
    }
}

impl AnalysisLoader for TestAnalysisLoader {
    fn needs_hard_reload(&self, _path_prefix: &Path) -> bool {
        true
    }

    fn fresh_host(&self) -> AnalysisHost<Self> {
        AnalysisHost::new_with_loader(self.clone())
    }

    fn set_path_prefix(&mut self, _path_prefix: &Path) {}

    fn abs_path_prefix(&self) -> Option<PathBuf> {
        panic!();
    }

    fn search_directories(&self) -> Vec<PathBuf> { vec![self.path.clone()] }
}
