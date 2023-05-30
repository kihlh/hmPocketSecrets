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

// #![windows_subsystem = "windows"]
use fltk::{app, frame};
use std::ffi::CString;
use std::ffi::{c_int, c_long, OsStr};
use std::mem::transmute;
use std::os::raw::c_char;
use std::os::windows::ffi::OsStrExt;
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process, thread,
};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::LPARAM;
mod libNcPlus;
mod util;
mod DeviceTube;
use util::*;

mod store;
use store::*;

mod main_view;
use main_view::*;
#[link(name = "hmpPlus")]
extern "C" {
    /**初始化句柄值 并传递到cpp处理 */
    fn initializeHandle(a: c_long) -> c_void;
    fn set_win_icon(a: c_long) -> c_void;
    fn lockSystemInteraction(a: bool) -> bool;
    fn open_tray_win(a: c_long) -> c_void;
    fn block_input(a: bool) -> bool;
    fn setShowWinTray(a: bool) -> bool;
    fn setShowWindow(a: bool) -> bool;
    fn getShowWinTray() -> bool;
    fn getShowWindow() -> bool;
    fn hasExit() -> bool;
    fn hasOpenWinTray() -> bool;
}

fn main() {
    let mut argv = getArgs();
    let current_exe_path: &Path = Path::new(argv[0].as_str());
    let mut as_exe_cwd: PathBuf = current_exe_path.join("..");
    let mut is_show_Main = true;
    let mut is_show_tray = true;
    // 只打开托盘面板
    let mut is_open_tray = false;

    for value in argv {
        if (str_eq_ostr(value.clone(), "--auto_start")) {
            is_show_Main = false;
        }

        if (str_eq_ostr(value.clone(), "--open_tray")) {
            is_open_tray = false;
            is_show_Main = false;
        }
        if (str_eq_ostr(value.clone(), "--hide_tray")) {
            is_show_tray = false;
        }
        if (str_eq_ostr(value.clone(), "--hide_window")) {
            is_show_tray = false;
            is_show_Main = false;
        }
        if (str_eq_ostr(value.clone(), "--start_mode")) {
            // is_show_tray= false;
            is_show_Main = false;
        }
    }

    // 创建窗口
    let appMain = app::App::default();
    setMainTheme();
    // openTrayWin();

    unsafe { setShowWinTray(is_show_tray) };
    unsafe { setShowWindow(is_show_Main) };

    if (!is_show_Main) {
        if (is_show_tray && !unsafe { hasOpenWinTray() }) {
            unsafe { open_tray_win(0) };
        }
        loop {
            std::thread::sleep(std::time::Duration::from_millis(300));
            if (unsafe { hasExit() }) {
                process::exit(50);
            }
            if (unsafe { getShowWindow() }) {
                mianWindow(is_show_Main);
                break;
            }
        }
    } else {
        mianWindow(is_show_Main);
    }

    DeviceTube::DeviceTubeMain();
    appMain.run().unwrap();
}
