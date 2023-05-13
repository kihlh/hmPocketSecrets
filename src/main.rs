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
#![windows_subsystem = "windows"]
use fltk::app::handle;
use fltk::button::Button;
use fltk::enums::{Cursor, Event, LabelType};
use fltk::frame::Frame;
use fltk::input::{InputType, IntInput};
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, window::Window, *};
use fltk_theme::{SchemeType, WidgetScheme};
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
use winapi::um::winuser::{MessageBoxW, MB_OKCANCEL};
use winreg::{enums::*, RegKey};

mod util;
use util::*;

const CONFIGKEY: &str = "G3Lt2Tb7NTnY0Up5mMmmMrhnNDnk718liVbPEN4LMMMrjOCaOGtaHsZaZfuRyqUE";
const CONFIGPATH: &str = "config.hmcg";
const HWND: &i64 = &0;

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

fn main() {
    let mut argv = getArgs();
    let current_exe_path: &Path = Path::new(argv[0].as_str());
    let mut as_exe_cwd: PathBuf = current_exe_path.join("..");
    mianWindow();
}

fn mianWindow() {
    // "./img/Clip_20230513_211004.png"
    let mut mainTheme: MainTheme = getMainTheme();

    let appMain = app::App::default();
    let mut appMainWin = Window::new(2400, 100, 500, 390, "HM神秘口袋");
    // 允许调整窗口大小
    appMainWin.make_resizable(true);

    // setWinNotFrame(&mut appMainWin);
    let mut background_frame = setWinBackground_image(&mut appMainWin);
    setWinIcon(&mut appMainWin);
    setinteractiveFunctionMainButton(&mut appMainWin);
    set_mian_bot_btn(&mut appMainWin);
    set_mian_top_title(&mut appMainWin);
    set_mian_state_btn(&mut appMainWin);
    appMainWin.end();
    appMainWin.show();
    appMainWin.visible_focus(true);
    appMain.run().unwrap();
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

// 设置背景为图片
fn setWinBackground_image(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\background.png")
            .expect("set main icon error");
    let mut frame = Frame::default().with_size(500, 390).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

fn setWinIcon(appMainWin: &mut window::DoubleWindow) {
    //  设置窗口图标
    let ICON1 = image::IcoImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\icon\\ICON1.ico")
        .expect("set main icon error");
    appMainWin.set_icon(Some(ICON1.clone()));
}

fn setinteractiveFunctionMainButton(appMainWin: &mut window::DoubleWindow) {
    let mut is_show_btnType: i32 = 0;
    let is_open: i32 = 0;
    let is_stop: i32 = 1;
    let is_off: i32 = 2;

    let background_image_open =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\open.png")
            .expect("set main icon error");
    let background_image_stop =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\stop.png")
            .expect("set main icon error");
    let background_image_off =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\off.png")
            .expect("set main icon error");

    // 服务正在启用中的按钮
    let mut img_frame_open = Frame::default().with_size(153, 153).center_of(appMainWin);
    img_frame_open.set_color(Color::from_u32(0));
    img_frame_open.set_frame(FrameType::NoBox);
    img_frame_open.set_image(Some(background_image_open));
    img_frame_open.set_pos(52, 119);
    img_frame_open.set_id("open");
    img_frame_open.hide();

    // 服务被关闭的按钮
    let mut img_frame_off = Frame::default().with_size(153, 153).center_of(appMainWin);
    img_frame_off.set_color(Color::from_u32(0));
    img_frame_off.set_frame(FrameType::NoBox);
    img_frame_off.set_image(Some(background_image_off));
    img_frame_off.set_pos(52, 119);
    img_frame_off.set_id("off");
    img_frame_off.hide();

    // 服务暂停的按钮
    let mut img_frame_stop = Frame::default().with_size(153, 153).center_of(appMainWin);
    img_frame_stop.set_color(Color::from_u32(0));
    img_frame_stop.set_frame(FrameType::NoBox);
    img_frame_stop.set_image(Some(background_image_stop));
    img_frame_stop.set_pos(52, 119);
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
    let mut btn_frame = Button::new(52, 119, 153, 153, "");
    btn_frame.set_color(Color::from_u32(0));
    btn_frame.set_frame(FrameType::NoBox);
    btn_frame.set_down_frame(FrameType::NoBox);
    btn_frame.set_selection_color(Color::from_u32(0));
    btn_frame.clear_visible_focus();
    btn_frame.set_pos(52, 119);

    btn_frame.set_callback(move |btn_frame| {
        if is_show_btnType != is_open {
            img_frame_open.clone().hide();
        } else {
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
}

fn hide_mian_window() {}

fn set_mian_top_title(appMainWin: &mut window::DoubleWindow) {
    let MainTheme: MainTheme = getMainTheme();

    // 标题栏
    let mut buf: text::TextBuffer = text::TextBuffer::default();
    buf.set_text("HM神秘口袋 (免费开源程序)");
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
    txt.set_pos(56, 18);
    txt.set_frame(FrameType::FlatBox);

    // 关闭按钮;
    let mut btn_frame = Button::new(455, 18, 20, 20, "");
    btn_frame.set_color(Color::from_u32(0));
    btn_frame.set_frame(FrameType::NoBox);
    btn_frame.set_down_frame(FrameType::NoBox);
    btn_frame.set_selection_color(Color::from_u32(0));
    btn_frame.clear_visible_focus();
    // btn_frame.set_callback(move |btn_frame: &mut Button| {
    //     hide_mian_window();
    // });
    // return btn_frame;
}

fn set_mian_bot_btn(appMainWin: &mut window::DoubleWindow) {
    let about_img =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\btn\\about.png")
            .expect("set main icon error");
    let quit_img =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\btn\\quit.png")
            .expect("set main icon error");
    let setup_img =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\btn\\setup.png")
            .expect("set main icon error");

    // 关于
    let mut img_frame_about = Frame::default().with_size(126, 0).center_of(appMainWin);
    img_frame_about.set_color(Color::from_u32(0));
    img_frame_about.set_frame(FrameType::NoBox);
    img_frame_about.set_image(Some(about_img));
    img_frame_about.set_pos(42 + 150, 348);
    img_frame_about.set_id("about");

    // 退出
    let mut img_frame_quit = Frame::default().with_size(126, 0).center_of(appMainWin);
    img_frame_quit.set_color(Color::from_u32(0));
    img_frame_quit.set_frame(FrameType::NoBox);
    img_frame_quit.set_image(Some(quit_img));
    img_frame_quit.set_pos(42 + 150 + 150, 348);
    img_frame_quit.set_id("quit");

    // 设置
    let mut img_frame_setup = Frame::default().with_size(126, 0).center_of(appMainWin);
    img_frame_setup.set_color(Color::from_u32(0));
    img_frame_setup.set_frame(FrameType::NoBox);
    img_frame_setup.set_image(Some(setup_img));
    img_frame_setup.set_pos(42, 348);
    img_frame_setup.set_id("setup");

    let mut setup: Button = Button::new(42, 325, 127, 45, "");
    hide_btn_color(setup);
    let mut about: Button = Button::new(42 + 150, 325, 127, 45, "");
    hide_btn_color(about);
    let mut quit: Button = Button::new(42 + 150 + 150, 325, 127, 45, "");
    hide_btn_color(quit);
}

fn hide_btn_color(mut btn_frame: Button) {
    btn_frame.set_color(Color::from_u32(0));
    btn_frame.set_frame(FrameType::NoBox);
    btn_frame.set_down_frame(FrameType::NoBox);
    btn_frame.set_selection_color(Color::from_u32(0));
    btn_frame.clear_visible_focus();
}
fn set_mian_state_btn(appMainWin: &mut window::DoubleWindow) {
    let MainTheme: MainTheme = getMainTheme();

    let state_btn =
        image::PngImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\img\\mian\\btn\\state.png")
            .expect("set main icon error");
    let mut img_frame_state = Frame::default().with_size(197, 0).center_of(appMainWin);
    img_frame_state.set_color(MainTheme.not);
    img_frame_state.set_frame(FrameType::NoBox);
    img_frame_state.set_image(Some(state_btn));
    img_frame_state.set_pos(240, 190);
    img_frame_state.set_id("state");

    // 钉钉机器人配置

    let mut buf: text::TextBuffer = text::TextBuffer::default();
    buf.append("远控：是");
    buf.append("\n");
    buf.append("类型：钉钉");
    
    let mut txt: text::TextEditor = text::TextEditor::default()
        .with_size(70, 35)
        .center_of_parent();
    txt.set_buffer(buf);
    txt.show_cursor(false);
    txt.set_text_color(MainTheme.botBtnIconColor);
    txt.set_text_size(11);
    txt.set_label_type(LabelType::None);
    txt.set_color(MainTheme.backgroundMain);
    txt.clear_visible_focus();
    txt.set_pos(250, 180);
    txt.set_frame(FrameType::FlatBox);
    txt.show_cursor(false);
    // USB密匙预览

    let mut buf2: text::TextBuffer = text::TextBuffer::default();
    buf2.append("USB：3");
    buf2.append("\n");
    buf2.append("解锁：666");

    let mut txt2: text::TextEditor = text::TextEditor::default()
        .with_size(70, 35)
        .center_of_parent();
    txt2.set_buffer(buf2);
    txt2.show_cursor(false);
    txt2.set_text_color(MainTheme.botBtnIconColor);
    txt2.set_text_size(11);
    txt2.set_label_type(LabelType::None);
    txt2.set_color(MainTheme.backgroundMain);
    txt2.clear_visible_focus();
    txt2.set_pos(250+113, 180);
    txt2.set_frame(FrameType::FlatBox);
    txt2.show_cursor(false);

    let mut buf4: text::TextBuffer = text::TextBuffer::default();
    buf4.append("密匙已连接");
    
    let mut txt4: text::TextEditor = text::TextEditor::default()
        .with_size(70, 20)
        .center_of_parent();
    txt4.set_buffer(buf4);
    txt4.show_cursor(false);
    txt4.set_text_color(MainTheme.cardSuccessText);
    txt4.set_text_size(11);
    txt4.set_label_type(LabelType::None);
    txt4.set_color(MainTheme.backgroundMain);
    txt4.clear_visible_focus();
    txt4.set_pos(250+113, 215);
    txt4.set_frame(FrameType::FlatBox);
    txt4.show_cursor(false);

    let mut buf5: text::TextBuffer = text::TextBuffer::default();
    buf5.append("机器人异常");
    
    let mut txt5: text::TextEditor = text::TextEditor::default()
        .with_size(70, 20)
        .center_of_parent();
    txt5.set_buffer(buf5);
    txt5.show_cursor(false);
    txt5.set_text_color(MainTheme.cardFailureText);
    txt5.set_text_size(11);
    txt5.set_label_type(LabelType::None);
    txt5.set_color(MainTheme.backgroundMain);
    txt5.clear_visible_focus();
    txt5.set_pos(250, 215);
    txt5.set_frame(FrameType::FlatBox);
    txt5.show_cursor(false);
    
}
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
