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

use fltk::{
    app::handle,
    button::{Button, self},
    enums::{Cursor, Event, Font, LabelType,Color,FrameType},
    draw::font,
    frame::Frame,
    group::{Group, self},
    input::{InputType, IntInput},
    text::TextDisplay,
    prelude::*, window::{Window, self, DoubleWindow}, tree,

};
use fltk_table::{SmartTable, TableOpts};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};


pub fn DeviceTubeMain(){

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 400, 300, "管理可解锁设备");
    
    let mut flex = group::Flex::default().with_size(win.width()-30, 200).column().center_of_parent();
    
    flex.set_pos(30,50);

    let mut bluetoothDevice = button::CheckButton::default().with_label("蓝牙音频设备(已匹配)连接时");
    bluetoothDevice.set_value(false);
    bluetoothDevice.set_label_color(Color::rgb_color(255, 255, 255));
    
    let mut wechatLock = button::CheckButton::default().with_label("键鼠锁定与微信锁同步 (较慢)");
    wechatLock.set_value(false);
    wechatLock.set_label_color(Color::rgb_color(255, 255, 255));
    
    // 同样可以使用 btn1.set_checked(true)
    let btn2 = button::CheckButton::default().with_label("Option 2");
    let mut btn3 = button::Button::default().with_label("Submit");
    
    flex.end();
    win.end();
    win.show();

    btn3.set_callback(move |btn3| {
       
    });


}
