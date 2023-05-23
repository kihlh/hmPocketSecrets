#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    unused_mut,
    unused_must_use,
    unused_assignments,
    non_snake_case
)]

use std::path::{Path, PathBuf};

mod util;
use util::*;

mod store;
use store::*;

mod main_view;
use main_view::*;


fn main() {
    let mut argv = getArgs();
    let current_exe_path: &Path = Path::new(argv[0].as_str());
    let mut as_exe_cwd: PathBuf = current_exe_path.join("..");
    let mut is_show_Main = true;
    for value  in argv {
        if(str_eq_ostr(value, "--auto_start")){
            is_show_Main= false;
        }
    }

    mianWindow(is_show_Main);
}
