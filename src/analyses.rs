use rls_analysis::AnalysisHost;
use std::path::Path;

use resources::{TestAnalysisLoader, print_title, pretty_print_spans, pretty_print_defs};


/// Analyse hello program (simple `hello world` program using a variable and a function).
///
/// # TLDR
/// Prints definition and the spans of all references to "print_hello" function defined
/// in the program.
///
/// # Precise Analysis Description
/// ...TODO...
pub fn check_hello() {
    print_title("ANALYSING `HELLO`");
    // Load analysis host
    let host = AnalysisHost::new_with_loader(TestAnalysisLoader::new(
        Path::new("test_data/hello/save-analysis").to_owned(),
    ));
    host.reload(
        Path::new("test_data/hello"),
        Path::new("test_data/hello"),
    ).unwrap();

    // Find all definitions whose name starts with "print_hello"
    let defs = host.matching_defs("print_hello").unwrap();
    // Should only find a single definition
    assert_eq!(defs.len(), 1);

    // Print the definitions properties
    pretty_print_defs(&defs);

    // Get all `id`s which have "print_hello" in their name.
    let ids = host.search_for_id("print_hello").unwrap();
    // Should only have a single function having a name starting with "print_hello"
    assert_eq!(ids.len(), 1);

    // Get all references to the id
    let res = host.find_all_refs_by_id(ids[0]).unwrap();
    // There are two reference, the definition and the call
    assert_eq!(res.len(), 2);

    // Print the spans from the references.
    pretty_print_spans(&res);
}



/// Analyse an OpenGL program using unsafe code (more complex).
///
/// # TLDR
/// Prints definition and the spans of all references to "process_events" function defined
/// in the program.
///
/// # Precise Analysis Description
/// ...TODO...
pub fn check_ogl() {
    print_title("ANALYSING `OGL`");
    // Load analysis host
    let host = AnalysisHost::new_with_loader(TestAnalysisLoader::new(
        Path::new("test_data/ogl/save-analysis").to_owned(),
    ));
    host.reload(
        Path::new("test_data/ogl"),
        Path::new("test_data/ogl"),
    ).unwrap();

    // Find all definitions whose name starts with "process_events"
    let defs = host.matching_defs("process_events").unwrap();
    // Should only find a single definition
    assert_eq!(defs.len(), 1);

    // Print the definitions properties
    pretty_print_defs(&defs);

    // Get all `id`s which have "process_events" in their name.
    let ids = host.search_for_id("process_events").unwrap();
    assert_eq!(ids.len(), 1);

    // Get all references to the id
    let res = host.find_all_refs_by_id(ids[0]).unwrap();
    // There are two reference, the definition and the call
    assert_eq!(res.len(), 2);
    // Print the spans from the references.
    pretty_print_spans(&res);
}
