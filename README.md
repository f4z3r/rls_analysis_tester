# rls-analysis tester
Tests the use of rls-analysis to test for unsafe code.

## Usage
### Building save-analysis files
Run
```
./build_data.sh
```
from within the `test_data/` directory. This builds the JSON files containing the build data for all test programs.

> To add a program, copy the source into `test_data/` and add the command `build <program> <program>/save-analysis` to `build_data.sh`. Then create an analysis in `src/analyses.rs`.

### Running sample analyses
First make sure that the save-analysis data has been generated. Otherwise this will not work. Then run
```
cargo run <test> <test> ...
```
in the root directory. <test> correspond to the test program names found in `test_data/`. Note that the only two supported so far are `hello` and `ogl`, both simply taking a function definition, listing its properties and printing all references to it.

#### Adding your own analyses
If adding a new test program, please use `print_title()` to print the analysis title and the other `pretty_print_...()` functions to make output to command line readable (imported from `resources.rs`).

## Notes
Running a query on a larger code base (such as `ogl`, containing the OpenGL backend as compiled libraries)can take some time.

![Time taken for OGL analysis to complete: 55s](assets/time.ogl.png "OGL analysis timer")
