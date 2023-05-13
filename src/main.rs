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

use fltk::input::{InputType, IntInput};
// use fltk::{app, prelude::*, window::Window};
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, *};
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
// use std::hash::{Hash, Hasher};

struct Config {
    /**启用功能 */
    enable: bool,
    /**自启动 */
    SelfStart: bool,
    /**自动消息机器人 */
    AotoMsg: AotoMsg,
    /**高级设置 */
    advanced: Advanced,
}

struct AotoMsg {
    /** 0 不启用 1 钉钉机器人 2 飞书机器人 */
    types: i32,
    /**启用 */
    enable: bool,
    /**机器人链接 */
    key: String,
    /**解锁时提醒 */
    HAS_unlock: bool,
    /**名单中的进程可见时候提醒 */
    HAS_showExList: bool,
    /**键盘或者鼠标被激活 的时候 */
    HAS_key: bool,
    /**USB接入但是不是可解锁设备 */
    HAS_USBfailure: bool,
    /**USB效应成功 */
    HAS_USBsuccess: bool,
}

struct Advanced {
    /**开机后是否USB接入后才开启保密功能 */
    Devil: bool,
    /**尝试阻止关机 */
    BlockShutdown: bool,
    /**应急密码的哈希值 */
    Pass: String,
    /**当尝试被迫解锁达到六次时候 允许通过结束所有进程 并禁用功能 */
    Forced: bool,
}
const CONFIGKEY: &str = "KXO3NB7XFSAB0-KXO3NB7XFSAB0-LF3UGBU8MPP84";
const CONFIGPATH: &str = "config.hmcg";

static CONFIG: Mutex<Config> = Mutex::new(Config {
    enable: true,
    SelfStart: false,
    AotoMsg: AotoMsg {
        types: 0,
        enable: false,
        key: String::new(),
        HAS_unlock: false,
        HAS_showExList: false,
        HAS_key: false,
        HAS_USBfailure: false,
        HAS_USBsuccess: false,
    },
    advanced: Advanced {
        Devil: true,
        BlockShutdown: false,
        Pass: String::new(),
        Forced: true,
    },
});

fn firstActivationItem() {
    // 设置颜色
    let mut Color_show_btn = Color::rgb_color(33, 37, 43);
    let mut Color_show_visible: Color = Color::rgb_color(215, 218, 224);
    let mut Color_focus_btn: Color = Color::rgb_color(56, 62, 74);
    let mut Color_back = Color::rgb_color(40, 44, 52);
    let mut Color_show_err: Color = Color::rgb_color(204, 71, 78);
    let mut Color_show_label: Color = Color::rgb_color(191, 191, 191);
    let mut Color_err_show_label: Color = Color::rgb_color(255, 235, 59);
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();

    let appMain = app::App::default();
    let mut appMainWin = fltk::window::Window::new(2800, 100, 350, 200, "HM神秘口袋");
    appMainWin.set_color(Color_back);
    appMainWin.set_border(false);

    let mut flex = group::Flex::default()
    .with_size(300, 90)
    // .column()
    // .center_of_parent();
    ;
    flex.set_pos(25, 20);
    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(200, 90)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_text_color(Color_show_label);
    txt.set_color(Color_back);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    buf.set_text("作者 @KIIC ： https://github.com/kihlh ");
    buf.append("\n");
    buf.append("软件开源协议 Mit 2.0    版本：1.0.0 ");
    buf.append("\n\n");
    buf.append("HM神秘口袋 是免费的自由软件 如果付费请维权退款");
    buf.append("\n");
    buf.append("在此 kiic 向所有引用的开源项目表示感谢");
    flex.end();

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

    // 允许调整窗口大小
    // appMainWin.make_resizable(true);

    let mut flex = group::Flex::default()
        .with_size(130, 48)
        // .column()
        // .center_of_parent();
        ;
    flex.set_pos(110, 130);
    let mut serviceStartup: button::Button = button::Button::default().with_label("我知道了");
    serviceStartup.set_label_type(fltk::enums::LabelType::Normal);
    serviceStartup.set_color(Color_show_btn);
    serviceStartup.set_label_color(Color_show_visible);
    serviceStartup.set_pos(10, 0);
    serviceStartup.set_size(45, 31);
    serviceStartup.set_selection_color(Color_focus_btn);
    serviceStartup.set_label_color(Color_show_label);

    serviceStartup.set_callback(move |serviceStartup| {
        appMain.quit();
    });
    flex.end();
    appMainWin.end();
    appMainWin.show();
    appMainWin.visible_focus(true);
    appMain.run().unwrap();
}

fn storeConfig() {
    let file_config = Path::new(CONFIGPATH);
    let mc: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(CONFIGKEY, 256);
    let mut config: std::sync::MutexGuard<Config> = CONFIG.lock().unwrap();
    let mut config_json = json!({
        "enable":config.enable,
        "SelfStart":config.SelfStart,
        "AotoMsg":  {
            "types": config.AotoMsg.types,
            "enable": config.AotoMsg.enable,
            "key": config.AotoMsg.key,
            "HAS_unlock":  config.AotoMsg.HAS_unlock,
            "HAS_showExList":  config.AotoMsg.HAS_showExList,
            "HAS_key":  config.AotoMsg.HAS_key,
            "HAS_USBfailure":  config.AotoMsg.HAS_USBfailure,
            "HAS_USBsuccess":  config.AotoMsg.HAS_USBsuccess,
        },
        "advanced":  {
            "Devil":  config.advanced.Devil,
            "BlockShutdown":  config.advanced.BlockShutdown,
            "Pass": config.advanced.Pass,
            "Forced":  config.advanced.Forced,
        },
    });
    let decrypted: String = mc.encrypt_str_to_base64(&config_json.to_string().as_str());

    fs::write(file_config, decrypted.as_bytes()).unwrap_or_else(|_| {
        msgbox::create(
            "初始化失败",
            "无法在程序目录创建配置文件 程序无法运行",
            IconType::Error,
        )
        .expect("");
    });

    drop(config);
}

fn storeConfigRead() {
    let file_config = Path::new(CONFIGPATH);
    let mc: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(CONFIGKEY, 256);
    let mut config: std::sync::MutexGuard<Config> = CONFIG.lock().unwrap();

    // 没有则创建
    if (!file_config.exists()) {
        storeConfig();
    }

    if (!file_config.is_file()) {
        msgbox::create(
            "初始化失败",
            "配置文件存在但不是一个文件 无法创建配置文件",
            IconType::Error,
        )
        .expect("");
        process::exit(2);
    }

    // 读取为文本
    let mut file_config_str = fs::read_to_string(file_config).unwrap_or_else(|_| {
        msgbox::create(
            "配置文件异常",
            "程序配置文件无法被正确读取 可能文件被删除或者无权限",
            IconType::Error,
        )
        .expect("");
        String::new()
    });

    // 解密
    let decrypted = mc
        .decrypt_base64_to_string(&file_config_str)
        .unwrap_or_else(|_| {
            if mb_confirm("配置文件解密失败","配置文件解密失败 密文无法被解析 是否删除错误的配置文件并退出") {
                fs::remove_file(file_config).expect("remove_file => config error");
            }
            process::exit(3);
        });

    drop(config);
}
/**
 * 
 */
fn mb_confirm(title: &str, info: &str) -> bool{
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
    if(result == 1){
        return true;
    }
    return false;
}

fn main() {
    // 获取命令行内容并转为数组
    let args: std::env::Args = std::env::args();
    let mut args: Vec<String> = Vec::new();
    std::env::args().for_each(|value| {
        args.push(value);
    });

    let file_config = Path::new(CONFIGPATH);
    let current_exe_path = Path::new(args[0].as_str());
    let mut as_exe_cwd = current_exe_path.join("..");
    // let mut as_open_path = current_exe_path.join("..").join("c.exe");

    // 首次打开时候显示一个关于页面 以防止该软件被不法变卖 损害了正常用户利益 看得懂源码的人应该不会在乎这点蝇头小利 所以意义就在于保护信息差用户的利益
    if (!file_config.exists()) {
        storeConfig();
        let mut has_firstActivationItem = true;

        for value in args.clone() {
            if (str_eq_ostr(value, "--open")) {
                has_firstActivationItem = false;
            }
        }

        firstActivationItem();
    }
    storeConfigRead();

    // 初始化窗口
    let appMain = app::App::default();
    let mut appMainWin = fltk::window::Window::new(2400, 100, 350, 550, "HM神秘口袋");
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();

    // 设置颜色
    let mut Color_show_btn = Color::rgb_color(33, 37, 43);
    let mut Color_show_visible: Color = Color::rgb_color(215, 218, 224);
    let mut Color_focus_btn: Color = Color::rgb_color(56, 62, 74);
    let mut Color_back = Color::rgb_color(40, 44, 52);
    let mut Color_show_err: Color = Color::rgb_color(204, 71, 78);
    let mut Color_show_label: Color = Color::rgb_color(191, 191, 191);
    let mut Color_err_show_label: Color = Color::rgb_color(255, 235, 59);

    let mut has_serviceStartup = false;

    appMainWin.set_color(Color_back);
    // 允许调整窗口大小
    appMainWin.make_resizable(true);
    appMainWin.visible_focus(true);
    //  设置窗口图标
    let ICON1 = image::IcoImage::load("D:\\source\\rust\\hmPocketSecrets\\src\\icon\\ICON1.ico")
        .expect("set main icon error");
    appMainWin.set_icon(Some(ICON1.clone()));

    let mut flex = group::Flex::default()
        .with_size(350, 48)
        // .column()
        // .center_of_parent();
        ;
    flex.set_pos(0, 500 - 5);
    // let mut oidConfig_str: &str = config.as_str().unwrap();

    let mut serviceStartup: button::Button =
        button::Button::default().with_label("● 服 务 启 动 中... ");
    serviceStartup.set_color(Color_show_btn);
    serviceStartup.set_label_color(Color_show_visible);
    serviceStartup.set_pos(10, 0);
    serviceStartup.set_size(45, 31);
    serviceStartup.set_selection_color(Color_focus_btn);
    serviceStartup.set_label_color(Color_show_label);
    serviceStartup.set_label_type(fltk::enums::LabelType::Normal);

    let mut usetup = button::Button::default().with_label("在 浏 览 器 中 设 置");
    usetup.set_color(Color_show_btn);
    usetup.set_label_color(Color_show_visible);
    usetup.set_pos(50, 0);
    usetup.set_size(45, 31);
    usetup.set_selection_color(Color_focus_btn);
    usetup.set_label_type(fltk::enums::LabelType::Shadow);

    flex.end();

    serviceStartup.set_callback(move |serviceStartup| {
        has_serviceStartup = !has_serviceStartup;
        if (has_serviceStartup) {
            serviceStartup.set_color(Color_show_btn);
            serviceStartup.set_label("● 服 务 启 动 中... ");
            serviceStartup.set_tooltip("服务正在启动中 将会自动处理隐私内容保护");
        } else {
            serviceStartup.set_label("○ 服 务 已 关 闭... ");
            serviceStartup.set_tooltip("服务被关闭 将停止处理隐私内容保护");
            serviceStartup.set_color(Color_show_err);
        }
    });

    // 密码设置

    let mut flex = group::Flex::default()
    .with_size(350, 25)
    // .column()
    // .center_of_parent();
    ;
    flex.set_pos(0, 500 - 5 - 50 - 0);

    let mut pass_input = IntInput::default();
    pass_input.set_type(InputType::Secret);
    // pass_input.set_value(config["advanced"]["Pass"].as_str().expect(""));
    let mut config: std::sync::MutexGuard<Config> = CONFIG.lock().unwrap();
    pass_input.set_value(config.advanced.Pass.as_str());
    drop(config);
    let mut usetup = button::Button::default().with_label("确认密码");
    usetup.set_color(Color_show_btn);
    usetup.set_label_color(Color_show_visible);
    usetup.set_pos(50, 0);
    usetup.set_size(45, 31);
    usetup.set_selection_color(Color_focus_btn);
    usetup.set_label_type(fltk::enums::LabelType::Shadow);
    let mut hash_password = format!("");
    let mut set_password = false;

    usetup.set_callback(move |usetup| {
        let mut config: std::sync::MutexGuard<Config> = CONFIG.lock().unwrap();
        let is_pass_empty = config.advanced.Pass.is_empty();
        drop(config);

        if (!is_pass_empty) {
            msgbox::create("无法修改", "当前密码已经存在", IconType::Info).expect("");

            return;
        } else {
        }

        if (pass_input.clone().value().is_empty()) {
            if (set_password) {
                msgbox::create("密码未记录", "请重复输入您的密码", IconType::Info).expect("");
            } else {
                msgbox::create("密码设置失败", "您没有输入密码", IconType::Info).expect("");
            }
            return;
        } else {
            if (!set_password) {
                set_password = true;
                hash_password.clear();
                hash_password.push_str(&calculate_hash(&pass_input.clone().value()).to_string());
                usetup.set_label("重复一遍");
                pass_input.clone().set_value("");
            } else {
                if (!str_eq_str(
                    hash_password.to_string(),
                    calculate_hash(&pass_input.clone().value()).to_string(),
                )) {
                    msgbox::create("密码设置失败", "两次输入的密码不一致", IconType::Error)
                        .expect("");
                    pass_input.clone().set_value("");
                    usetup.set_label("确认密码");
                    set_password = false;
                    hash_password.clear();
                } else {
                    usetup.set_label("确认密码");
                    msgbox::create("密码已设置", "密码已设置", IconType::Info).expect("");
                    pass_input.clone().set_value("");
                    let mut config: std::sync::MutexGuard<Config> = CONFIG.lock().unwrap();
                    config.advanced.Pass.push_str(hash_password.as_str());
                    drop(config);
                    storeConfig();
                    pass_input.clone().hide();
                    usetup.resize(5, 0, 100, 31);
                }
                set_password = false;
                hash_password.clear();
            }
        }
    });

    flex.end();

    appMainWin.end();
    appMainWin.show();
    appMain.run().unwrap();
}

/**
 * 文本是否相等
 */
fn str_eq_str(str: String, eq_str: String) -> bool {
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
fn str_eq_ostr(str: String, eq_str: &str) -> bool {
    return str_eq_str(str, String::from(eq_str));
}
// json 数据的文本是否重合
fn json_eq_str(value: &serde_json::Value, key: &str, eq_str: &str) -> bool {
    return value[key].is_string() && value[key] == eq_str;
}

// JSON 的数字数据是否重合
fn json_eq_num(value: &serde_json::Value, key: &str, eq_i32: i32) -> bool {
    return value[key].is_number() && value[key] == eq_i32;
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
