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
use fltk::app::handle;
use fltk::button::Button;
use fltk::draw::font;
use fltk::enums::{Cursor, Event, Font, LabelType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{InputType, IntInput};
use fltk::text::TextDisplay;
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, window::Window, *};
use fltk_theme::{SchemeType, WidgetScheme};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::ffi::{c_int, c_long, OsStr};
use std::mem::transmute;
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
use winapi::ctypes::c_void;
use winapi::shared::minwindef::LPARAM;
// use tray_icon::{icon::Icon, menu::Menu, TrayIconBuilder};
use std::ffi::CString;
use std::os::raw::c_char;
use winapi::shared::windef::{HICON, HWND};
use winapi::um::winuser::{
    DestroyIcon, LoadImageA, MessageBoxW, SendMessageA, UpdateWindow, ICON_BIG, ICON_SMALL,
    IMAGE_ICON, LR_LOADFROMFILE, MB_OKCANCEL, WM_SETICON,
};
use winreg::{enums::*, RegKey};

use crate::util::{mb_Info, showWindow};
extern crate libloading;

// mod util;
// use util::*;

static MAIN_HWND: Mutex<i128> = Mutex::new(0);

struct MainTheme {
    /**主背景颜色 */
    background: Color,
    /**次背景*/
    backgroundMain: Color,
    /**顶部文字和logo */
    logo: Color,
    /**卡片文本成功 */
    cardSuccessText: Color,
    /**卡片文本失败 */
    cardFailureText: Color,
    /**卡片文本 */
    cardText: Color,
    /**卡片描边 */
    cardStroke: Color,
    /**分割线 */
    cuttingLine: Color,
    /** 底部三个按钮的颜色*/
    botBtnColor: Color,
    /** 底部三个按钮的图标颜色*/
    botBtnIconColor: Color,
    // null
    not: Color,
}
#[link(name = "Dll3")]
extern "C" {
    fn initializeHandle(a: c_long) -> c_void;
    fn set_win_icon(a: c_long) -> c_void;
    fn lockSystemInteraction(a: bool) -> bool;
    fn open_tray_win(a: c_long) -> c_void;
    fn block_input(a: bool) -> bool;

}

// 主窗口开始
pub fn mianWindow(show: bool) {
    // "./img/Clip_20230513_211004.png"
    let mut mainTheme: MainTheme = getMainTheme();
    app::set_background_color(24, 24, 24);
    // app::set_fonts("name");
    app::set_frame_shadow_width(0);
    app::set_frame_color(mainTheme.not);
    app::set_background2_color(17, 17, 17);
    app::set_foreground_color(17, 17, 17);
    app::set_selection_color(17, 17, 17);
    app::set_frame_type(FrameType::NoBox);
    app::set_inactive_color(24, 24, 24);

    let appMain = app::App::default();
    let mut appMainWin = Window::new(0, 0, 600, 469, "HM神秘口袋");
    setWinBackground_image(&mut appMainWin);
    set_mian_top_title(&mut appMainWin);
    app::set_selection_color(0, 0, 0);
    app::set_scheme(app::AppScheme::Base);
    // 主界面的窗口 2  悬浮在主窗口1上面
    let mut appRootView = window::Window::new(0, 0, 600, 469, "mian");

    // let mut group = Group::new(0, 0, 400, 300, "");
    // group.set_color(Color::from_u32(0));
    // group.set_frame(FrameType::NoBox);
    // group.set_down_frame(FrameType::NoBox);
    // group.set_selection_color(FrameType::NoBox);

    setWinBackground_forRoot_image(&mut appRootView);
    setinteractiveFunctionMainButton(&mut appRootView);
    set_mian_bot_btn(&mut appRootView);
    set_mian_top_title(&mut appRootView);
    set_mian_state_btn(&mut appRootView);
    appRootView.end();
    appMainWin.show();
    appRootView.show();
    appMainWin.end();
    // AddTray();

    // appMainWin.visible_focus(true);

    setMianHWND(&mut appMainWin);
    unsafe { initializeHandle(getHWND().try_into().unwrap()) };
    setWinIcon(&mut appMainWin);

    AddTray();

    appMainWin.handle({
        let mut x = 0;
        let mut y = 0;
        move |w, ev| match ev {
            enums::Event::Show => {
                appRootView.set_visible_focus();
                true
            }
            enums::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                true
            }
            _ => false,
        }
    });

    appMain.run().unwrap();
}

// 调用c++插件添加托盘
fn AddTray() {
    // thread::spawn(move || {
    //     unsafe { open_tray() };
    // });
    // unsafe { open_tray() };
    unsafe { open_tray_win(getHWND().try_into().unwrap()) };
}

// 设置窗口句柄到全局中方便调用
fn setMianHWND(appMainWin: &mut window::DoubleWindow) {
    let hwnd = appMainWin.raw_handle() as i128;
    let mut _HWND: std::sync::MutexGuard<i128> = MAIN_HWND.lock().unwrap();
    *_HWND = hwnd;
    // print!("{}", _HWND);
    drop(_HWND);
}

// 获取主窗口句柄
fn getHWND() -> i128 {
    let mut hwnd_co: i128 = 0;
    let mut _HWND: std::sync::MutexGuard<i128> = MAIN_HWND.lock().unwrap();
    hwnd_co = *_HWND;
    drop(_HWND);
    return hwnd_co;
}

// 设置一个无边框可拖拽的窗口
fn setWinNotFrame(appMainWin: &mut window::DoubleWindow) {
    appMainWin.set_border(false);
    // 窗体可以拖拽
    appMainWin.handle({
        let mut x = 0;
        let mut y = 0;
        move |w, ev| match ev {
            enums::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                true
            }
            enums::Event::Drag => {
                w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                true
            }
            _ => false,
        }
    });
}

// 设置背景为图片(底窗)
fn setWinBackground_image(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::load(
        "D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\Clip_20230522_165921.png",
    )
    // image::PngImage::from_data(include_bytes!("./img/mian/background.png"))
    .expect("set main setWinBackground_image error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 设置背景为图片（主视图）
fn setWinBackground_forRoot_image(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = 
    // image::PngImage::load(    "D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\Clip_20230522_165921.png",)
    image::PngImage::from_data(include_bytes!("./img/mian/backgroundRoot_.png"))
    .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 设置窗口图标（从二进制加载）
fn setWinIcon(appMainWin: &mut window::DoubleWindow) {
    // let icon_data = include_bytes!("./icon/ICON1.ico");
    //  设置窗口图标
    // let ICON1 = image::IcoImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\icon\\ICON1.ico")
    // let ICON1 = image::IcoImage::from_data(include_bytes!("D:\\source\\rust\\hmPocketSecrets\\src\\icon\\ICON1.ico"))
    //   let ICON1 = image::IcoImage::from_data(icon_data)
    // .expect("set main icon error");
    // appMainWin.set_icon(Some(ICON1.clone()));
    // 直接c++ 那边设置 单独一个图标放文件不合适  也没办法编译进二进制
    unsafe { set_win_icon(getHWND().try_into().unwrap()) };
}

// 设置最大的状态按钮
fn setinteractiveFunctionMainButton(appMainWin: &mut window::DoubleWindow) {
    let mut is_show_btnType: i32 = 0;
    let is_open: i32 = 0;
    let is_stop: i32 = 1;
    let is_off: i32 = 2;

    let background_image_open =
        image::PngImage::from_data(include_bytes!("./img/mian/state/open.png"))
            .expect("set main icon error");
    let background_image_stop =
        image::PngImage::from_data(include_bytes!("./img/mian/state/stop.png"))
            .expect("set main icon error");
    let background_image_off =
        image::PngImage::from_data(include_bytes!("./img/mian/state/off.png"))
            .expect("set main icon error");

    // 服务正在启用中的按钮
    let mut img_frame_open = Frame::default().with_size(184, 184).center_of(appMainWin);
    img_frame_open.set_color(Color::from_u32(0));
    img_frame_open.set_frame(FrameType::NoBox);
    img_frame_open.set_image(Some(background_image_open));
    img_frame_open.set_pos(62, 142);
    img_frame_open.set_id("open");
    img_frame_open.hide();

    // 服务被关闭的按钮
    let mut img_frame_off = Frame::default().with_size(184, 184).center_of(appMainWin);
    img_frame_off.set_color(Color::from_u32(0));
    img_frame_off.set_frame(FrameType::NoBox);
    img_frame_off.set_image(Some(background_image_off));
    img_frame_off.set_pos(62, 142);
    img_frame_off.set_id("off");
    img_frame_off.hide();

    // 服务暂停的按钮
    let mut img_frame_stop = Frame::default().with_size(184, 184).center_of(appMainWin);
    img_frame_stop.set_color(Color::from_u32(0));
    img_frame_stop.set_frame(FrameType::NoBox);
    img_frame_stop.set_image(Some(background_image_stop));
    img_frame_stop.set_pos(62, 142);
    img_frame_stop.set_id("stop");
    img_frame_stop.hide();

    // 初始化按钮显示的类型
    if is_show_btnType == is_open {
        img_frame_open.show();
    } else if is_show_btnType == is_off {
        img_frame_off.show();
    } else if is_show_btnType == is_stop {
        img_frame_stop.show();
    }

    // 一个不可见的按钮 用来响应点击事件
    let mut btn_frame = Button::new(62, 142, 184, 184, "");
    btn_frame.set_color(Color::from_u32(0));
    btn_frame.set_frame(FrameType::NoBox);
    btn_frame.set_down_frame(FrameType::NoBox);
    btn_frame.set_selection_color(Color::from_u32(0));
    btn_frame.clear_visible_focus();
    // btn_frame.set_pos(62, 142);
    btn_frame.set_callback(move |btn_frame| {
        if is_show_btnType != is_open {
            // unsafe { lockSystemInteraction(true) };
            img_frame_open.clone().hide();
        } else {
            // unsafe { lockSystemInteraction(false) };
            img_frame_open.clone().show();
        }

        if is_show_btnType != is_off {
            img_frame_off.clone().hide();
        } else {
            img_frame_off.clone().show();
        }

        if is_show_btnType != is_stop {
            img_frame_stop.clone().hide();
        } else {
            img_frame_stop.clone().show();
        }

        if is_show_btnType == is_open {
            is_show_btnType = is_stop;
        } else if is_show_btnType == is_stop {
            is_show_btnType = is_open;
        }
    });
    // btn_frame.s();
    btn_frame.show();
    btn_frame.top_window();
}

// 隐藏主窗口
fn hide_mian_window() {
    let mut hwnd: std::sync::MutexGuard<i128> = MAIN_HWND.lock().unwrap();
    showWindow(*hwnd, true);
    drop(hwnd);
}

// 设置顶部的软件名称 和logo
fn set_mian_top_title(appMainWin: &mut window::DoubleWindow) {
    let MainTheme: MainTheme = getMainTheme();

    // 标题栏
    let mut buf: text::TextBuffer = text::TextBuffer::default();
    buf.set_text(format!("{}", "HM神秘口袋 (开源免费程序)").as_str());
    let mut txt: text::TextEditor = text::TextEditor::default()
        .with_size(350, 20)
        .center_of_parent();
    txt.set_buffer(buf);
    txt.show_cursor(false);
    txt.set_text_color(MainTheme.logo);
    txt.set_text_size(12);
    txt.set_label_type(LabelType::None);
    txt.set_color(MainTheme.background);
    txt.clear_visible_focus();
    txt.set_pos(56, 24);
    txt.set_frame(FrameType::FlatBox);
    txt.deactivate();
    txt.set_text_color(MainTheme.cardText);
    txt.set_label_color(MainTheme.cardText);

    // 关闭按钮;
    let mut btn_frame = Button::new(552, 28, 20, 20, "");
    btn_frame.set_color(Color::from_u32(0));
    btn_frame.set_frame(FrameType::NoBox);
    btn_frame.set_down_frame(FrameType::NoBox);
    btn_frame.set_selection_color(Color::from_u32(0));
    btn_frame.clear_visible_focus();
    btn_frame.set_callback(move |btn_frame: &mut Button| {
        hide_mian_window();
    });

    let logo = image::PngImage::from_data(include_bytes!("./img/mian/logo/title-logo.png"))
        .expect("set main icon error");
    let mut img_frame_logo = Frame::default().with_size(18, 19).center_of(appMainWin);
    img_frame_logo.set_color(Color::from_u32(0));
    img_frame_logo.set_frame(FrameType::NoBox);
    img_frame_logo.set_image(Some(logo));
    img_frame_logo.set_pos(35, 24);
    // return btn_frame;
}

// 设置底部的三大功能按钮
fn set_mian_bot_btn(appMainWin: &mut window::DoubleWindow) {
    let about_img = image::PngImage::from_data(include_bytes!("./img/mian/bot/about-btn.png"))
        .expect("set main icon error");
    let quit_img = image::PngImage::from_data(include_bytes!("./img/mian/bot/fn-btn.png"))
        .expect("set main icon error");
    let setup_img = image::PngImage::from_data(include_bytes!("./img/mian/bot/set-btn.png"))
        .expect("set main icon error");

    // 关于
    let mut img_frame_about = Frame::default().with_size(151, 57).center_of(appMainWin);
    img_frame_about.set_color(Color::from_u32(0));
    img_frame_about.set_frame(FrameType::NoBox);
    img_frame_about.set_image(Some(about_img));
    img_frame_about.set_pos(225, 389);
    img_frame_about.set_id("about");

    // 功能面板
    let mut img_frame_function = Frame::default().with_size(151, 57).center_of(appMainWin);
    img_frame_function.set_color(Color::from_u32(0));
    img_frame_function.set_frame(FrameType::NoBox);
    img_frame_function.set_image(Some(quit_img));
    img_frame_function.set_pos(405, 389);
    img_frame_function.set_id("function");

    // 设置
    let mut img_frame_setup = Frame::default().with_size(151, 57).center_of(appMainWin);
    img_frame_setup.set_color(Color::from_u32(0));
    img_frame_setup.set_frame(FrameType::NoBox);
    img_frame_setup.set_image(Some(setup_img));
    img_frame_setup.set_pos(45, 389);
    img_frame_setup.set_id("setup");

    let mut setup: Button = Button::new(45, 389, 151, 57, "");
    hide_btn_color(setup.clone());
    let mut about: Button = Button::new(225, 389, 151, 57, "");
    hide_btn_color(about.clone());
    let mut func: Button = Button::new(405, 389, 151, 57, "");
    hide_btn_color(func.clone());
    func.set_callback(|btn| {
        mb_Info("title", "info");
    });
    setup.set_callback(|btn| {
        mb_Info("setup", "setup");
    });
    about.set_callback(|btn| {
        mb_Info("about", "about");
    });
}

// 移除并禁用按钮背景和文本 以达到图片也能实现按钮的功能
fn hide_btn_color(mut btn_frame: Button) {
    btn_frame.set_color(Color::from_u32(0));
    btn_frame.set_frame(FrameType::NoBox);
    btn_frame.set_down_frame(FrameType::NoBox);
    // btn_frame
    // btn_frame.down_frame(FrameType::NoBox);
    // btn_frame.set_down_frame(FrameType::NoBox);
    btn_frame.set_selection_color(Color::from_u32(0));
    // btn_frame.set_;
    btn_frame.clear_visible_focus();
}

// 处理文本添加时候风格的宏
macro_rules! setTheStyleToInterface {
    ($b:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};

    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr,$fsize:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size($fsize);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::NoBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
}

// 设置状态卡片(两个)的主要视图(非动态)
fn set_mian_state_btn(appMainWin: &mut window::DoubleWindow) {
    let MainTheme: MainTheme = getMainTheme();

    // DesktopLock
    let mut bftext_DesktopLock: text::TextBuffer = text::TextBuffer::default();
    bftext_DesktopLock.append("DesktopLock");
    let mut text_DesktopLock: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_DesktopLock.set_buffer(bftext_DesktopLock);

    setTheStyleToInterface!(
        text_DesktopLock,
        /* x */ 320,
        /* y */ 115,
        /* w */ 80,
        /* h */ 20,
        12
    );

    // 键鼠锁定
    let mut bftext_DesktopLock_cn: text::TextBuffer = text::TextBuffer::default();
    bftext_DesktopLock_cn.append("键鼠锁定");

    let mut text_DesktopLock_cn: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_DesktopLock_cn.set_buffer(bftext_DesktopLock_cn);
    setTheStyleToInterface!(
        text_DesktopLock_cn,
        /* x */ 320,
        /* y */ 133,
        /* w */ 80,
        /* h */ 25,
        14
    );
    text_DesktopLock_cn.set_label_color(MainTheme.cardText);

    // 今日已护航
    let mut bftext_info_title: text::TextBuffer = text::TextBuffer::default();
    bftext_info_title.append("今日已护航");

    let mut text_info_title: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_info_title.set_buffer(bftext_info_title);
    setTheStyleToInterface!(
        text_info_title,
        /* x */ 483,
        /* y */ 125,
        /* w */ 66,
        /* h */ 20,
        12
    );
    text_info_title.set_label_color(MainTheme.cardText);

    // PrivacyLlst
    let mut bftext_PrivacyLlst: text::TextBuffer = text::TextBuffer::default();
    bftext_PrivacyLlst.append("PrivacyLlst");
    let mut text_PrivacyLlst: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_PrivacyLlst.set_buffer(bftext_PrivacyLlst);
    setTheStyleToInterface!(
        text_PrivacyLlst,
        /* x */ 320,
        /* y */ 287,
        /* w */ 80,
        /* h */ 20,
        12
    );

    // 受保护的程序列表
    let mut bftext_PrivacyLlst_cn: text::TextBuffer = text::TextBuffer::default();
    bftext_PrivacyLlst_cn.append("受保护的程序列表");

    let mut text_PrivacyLlst_cn: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_PrivacyLlst_cn.set_buffer(bftext_PrivacyLlst_cn);
    setTheStyleToInterface!(
        text_PrivacyLlst_cn,
        /* x */ 320,
        /* y */ 305,
        /* w */ 120,
        /* h */ 28,
        12
    );
    text_PrivacyLlst_cn.set_label_color(MainTheme.cardText);

    // 今日已护航
    let mut bftext_PrivacyLlst_info_title: text::TextBuffer = text::TextBuffer::default();
    bftext_PrivacyLlst_info_title.append("匹配的进程");

    let mut text_PrivacyLlst_info_title: text::TextEditor =
        text::TextEditor::default().center_of_parent();
    text_PrivacyLlst_info_title.set_buffer(bftext_PrivacyLlst_info_title);
    setTheStyleToInterface!(
        text_PrivacyLlst_info_title,
        /* x */ 483,
        /* y */ 296,
        /* w */ 66,
        /* h */ 20,
        12
    );
    text_PrivacyLlst_info_title.set_label_color(MainTheme.cardText);

    let pimg_logo_key =
        image::PngImage::from_data(include_bytes!("./img/mian/state/key.png"))
            .expect("set main icon error");
    let mut logo_key = Frame::default().with_size(9, 9).center_of(appMainWin);
    logo_key.set_color(Color::from_u32(0));
    logo_key.set_frame(FrameType::NoBox);
    logo_key.set_image(Some(pimg_logo_key));
    logo_key.set_pos(328, 222);

    let pimg_logo_setApps =
        image::PngImage::from_data(include_bytes!("./img/mian/state/set_apps.png"))
            .expect("set main icon error");
    let mut logo_setApps = Frame::default().with_size(11, 11).center_of(appMainWin);
    logo_setApps.set_color(Color::from_u32(0));
    logo_setApps.set_frame(FrameType::NoBox);
    logo_setApps.set_image(Some(pimg_logo_setApps));
    logo_setApps.set_pos(328, 340);

    // 管理设备
    let mut setDeviceList: Button = Button::new(448, 220, 50, 11, "管理设备");
    hide_btn_color(setDeviceList.clone());
    setDeviceList.set_label_color(MainTheme.logo);  
    setDeviceList.set_label_size(12);
    
    // 设置应用列表
    let mut setAppList: Button = Button::new(485, 339, 50, 11, "设置内容");
    hide_btn_color(setAppList.clone());
    setAppList.set_label_color(MainTheme.logo);
    setAppList.set_label_size(12);
    
    set_mian_state_dynamic_infor(&mut appMainWin.clone());
    // FrameType::NoBox
}

// 设置状态卡片(两个)的主要视图(动态)
fn set_mian_state_dynamic_infor(appMainWin: &mut window::DoubleWindow) {
    let MainTheme: MainTheme = getMainTheme();

    // 文本 今日已经护航次数
    let mut bftext_quen: text::TextBuffer = text::TextBuffer::default();
    bftext_quen.append("00");

    let mut text_quen: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_quen.set_buffer(bftext_quen);
    setTheStyleToInterface!(text_quen, /* x */ 450, /* y */ 120, /* w */ 34, /* h */ 40, 24);
    text_quen.set_label_color(MainTheme.cardText);

    // 文本 今日已经护航次数
    let mut bftext_processQuen: text::TextBuffer = text::TextBuffer::default();
    bftext_processQuen.append("00");

    let mut text_processQuen: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_processQuen.set_buffer(bftext_processQuen);
    setTheStyleToInterface!(
        text_processQuen,
        /* x */ 450,
        /* y */ 290,
        /* w */ 34,
        /* h */ 40,
        24
    );

       // 文本 N个设备匹配
       let mut bftext_processQuen: text::TextBuffer = text::TextBuffer::default();
       bftext_processQuen.append("0个设备匹配");
   
       let mut text_processQuen: text::TextEditor = text::TextEditor::default().center_of_parent();
       text_processQuen.set_buffer(bftext_processQuen);
       setTheStyleToInterface!(
           text_processQuen,
           /* x */ 342,
           /* y */ 219,
           /* w */ 90,
           /* h */ 18,
           11
       );

    // 文本 已设置 N 个应用
    let mut bftext_processQuen: text::TextBuffer = text::TextBuffer::default();
    bftext_processQuen.append("已设置 0 个应用");

    let mut text_processQuen: text::TextEditor = text::TextEditor::default().center_of_parent();
    text_processQuen.set_buffer(bftext_processQuen);
    setTheStyleToInterface!(
        text_processQuen,
        /* x */ 342,
        /* y */ 337,
        /* w */ 110,
        /* h */ 18,
        11
    );

    let mut has_Lock_state = false;

    // 已经启用/ 已关闭
    let mut bftext_Lock_state: text::TextBuffer = text::TextBuffer::default();

    let mut text_Lock_state: text::TextEditor = text::TextEditor::default().center_of_parent();
    
    if(has_Lock_state){
        bftext_Lock_state.remove(0, bftext_Lock_state.length());
        bftext_Lock_state.append("已启用");
    }else{
        bftext_Lock_state.remove(0, bftext_Lock_state.length());
        bftext_Lock_state.append("已关闭");
    }

    text_Lock_state.set_buffer(bftext_Lock_state);
    setTheStyleToInterface!(text_Lock_state, /* x */ 350, /* y */ 175, /* w */ 80, /* h */ 25, 13);
    text_Lock_state.set_label_color(MainTheme.cardText);


    // 已经启用/ 已关闭
    let mut bftext_KeyDevice_state: text::TextBuffer = text::TextBuffer::default();

    let mut text_Device_state: text::TextEditor = text::TextEditor::default().center_of_parent();
    
    if(has_Lock_state){
        bftext_KeyDevice_state.remove(0, bftext_KeyDevice_state.length());
        bftext_KeyDevice_state.append("可解锁设备已连接");
    }else{
        bftext_KeyDevice_state.remove(0, bftext_KeyDevice_state.length());
        bftext_KeyDevice_state.append("可解锁设备已弹出");
    }

    text_Device_state.set_buffer(bftext_KeyDevice_state);
    setTheStyleToInterface!(text_Device_state, /* x */ 453, /* y */ 175, /* w */ 115, /* h */ 25, 11);
    text_Device_state.set_label_color(MainTheme.cardText);

    let pimg_lock_state_success_con: image::PngImage = image::PngImage::from_data(include_bytes!("./img/mian/state/off_state.png"))
        .expect("set main icon error");
    let mut lock_state_success_con = Frame::default().with_size(21, 20).center_of(appMainWin);
    lock_state_success_con.set_color(Color::from_u32(0));
    lock_state_success_con.set_frame(FrameType::NoBox);
    lock_state_success_con.set_image(Some(pimg_lock_state_success_con));
    lock_state_success_con.set_pos(429, 174);


    let pimg_lock_state_failure_con = image::PngImage::from_data(include_bytes!("./img/mian/state/launch.png"))
    .expect("set main icon error");
    let mut lock_state_failure_con = Frame::default().with_size(21, 20).center_of(appMainWin);
    lock_state_failure_con.set_color(Color::from_u32(0));
    lock_state_failure_con.set_frame(FrameType::NoBox);
    lock_state_failure_con.set_image(Some(pimg_lock_state_failure_con));
    lock_state_failure_con.set_pos(325, 174);


}

// 统一在这里定义主题颜色
fn getMainTheme() -> MainTheme {
    let mut mainTheme: MainTheme = MainTheme {
        /**主背景颜色 */
        background: Color::rgb_color(24, 24, 24),
        /**次背景*/
        backgroundMain: Color::rgb_color(17, 17, 17),
        /**顶部文字和logo */
        logo: Color::rgb_color(122, 120, 120),
        /**卡片文本成功 */
        cardSuccessText: Color::rgb_color(99, 138, 99),
        /**卡片文本失败 */
        cardFailureText: Color::rgb_color(189, 79, 79),
        /**卡片文本 */
        cardText: Color::rgb_color(255, 255, 255),
        /**卡片描边 */
        cardStroke: Color::rgb_color(46, 46, 46),
        /**分割线 */
        cuttingLine: Color::rgb_color(38, 38, 38),
        /** 底部三个按钮的颜色*/
        botBtnColor: Color::rgb_color(0, 0, 0),
        /** 底部三个按钮的图标颜色*/
        botBtnIconColor: Color::rgb_color(125, 125, 125),
        not: Color::from_u32(0),
    };
    return mainTheme;
}
