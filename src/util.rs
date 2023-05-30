use fltk::input::{InputType, IntInput};
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, *};
// use fltk_theme::{SchemeType, WidgetScheme};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};
use winapi::um::winuser::{MessageBoxW,ShowWindow, MB_ICONERROR, MB_OK, MB_OKCANCEL,SW_SHOW,SW_HIDE};
use winapi::shared::windef::HWND;
use winreg::{enums::*, RegKey};
use std::mem::transmute;

// use crate::{CONFIG, Config};

/**
 * 确认窗口 MB_OKCANCEL
 */
pub fn mb_confirm(title: &str, info: &str) -> bool {
    let lpText: Vec<u16> = OsStr::new(info)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let lpCaption: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let result =
        unsafe { MessageBoxW(null_mut(), lpText.as_ptr(), lpCaption.as_ptr(), MB_OKCANCEL) };
    if (result == 1) {
        return true;
    }

    return false;
}

/**
 * 确认窗口 MB_OK
 */
pub fn mb_Info(title: &str, info: &str) {
    let lpText: Vec<u16> = OsStr::new(info)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let lpCaption: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let result = unsafe { MessageBoxW(null_mut(), lpText.as_ptr(), lpCaption.as_ptr(), MB_OK) };
}

/**
 * 确认窗口 MB_ERR
 */
pub fn mb_Error(title: &str, info: &str) {
    let lpText: Vec<u16> = OsStr::new(info)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let lpCaption: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let result = unsafe {
        MessageBoxW(
            null_mut(),
            lpText.as_ptr(),
            lpCaption.as_ptr(),
            MB_ICONERROR,
        )
    };
}

/**
 * 文本是否相等
 */
pub fn str_eq_str(str: String, eq_str: String) -> bool {
    if (str.len() != eq_str.len()) {
        return false;
    };

    // 转为二进制再判断
    let str_buf = str.as_bytes();
    let eq_str_buf = eq_str.as_bytes();
    return str_buf.eq(eq_str_buf);
}

/**
 * 文本是否相等
 */
pub fn str_eq_ostr(str: String, eq_str: &str) -> bool {
    return str_eq_str(str, String::from(eq_str));
}

// json 数据的文本是否重合
pub fn json_eq_str(value: &serde_json::Value, key: &str, eq_str: &str) -> bool {
    return value[key].is_string() && value[key] == eq_str;
}

// JSON 的数字数据是否重合
pub fn json_eq_num(value: &serde_json::Value, key: &str, eq_i32: i32) -> bool {
    return value[key].is_number() && value[key] == eq_i32;
}

/**
 * 计算哈希值
 */
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

struct AppEnv {
    args: Vec<String>,
    exe: String,
    cwd: String,
}
/**
 * 获取命令行
 */
pub fn getArgs() -> Vec<String> {
    // 获取命令行内容并转为数组
    let args: std::env::Args = std::env::args();
    let mut argv_list: Vec<String> = Vec::new();
    std::env::args().for_each(|value: String| {
        argv_list.push(value);
    });
    return argv_list;
}

pub fn showWindow (hwnd:i128 , hide:bool)->bool{
    unsafe { ShowWindow(hwnd as HWND,SW_HIDE); }
    return true;
}


macro_rules! Sleep {
    ($b:expr) => {{
        std::thread::sleep(std::time::Duration::from_millis($b));
    }};
}

pub fn Sleep(ms:u64){
    Sleep!(ms);
}
