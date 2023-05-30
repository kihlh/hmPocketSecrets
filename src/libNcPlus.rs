use std::ffi::{c_int, c_long,c_void,};

#[link(name = "libNcPlus")]
extern "C" {
    // 获取当前已经匹配的进程数量
    fn cp_getProcessMatchSize() -> c_int;
    // 获取可解锁设备名称列表的数量(多少设备匹配)
    fn cp_getUnlockDriverNamesize() -> c_int;
    // 获取已经设置的进程匹配名称数量
    fn cp_getProcessNameQuencySize() -> c_int;
    // 获取今天已经锁定的次数上限99
    fn cp_getLockFrequency() -> c_int;
    // 返回是否启用了桌面锁定
    fn cp_hasEnableDesktopLock() -> bool;
    // 最高级别开关 启用/关闭核心功能
    fn cp_hasDisableDesktopLockKernel() -> bool;
    // 设置是否启用桌面锁
    fn cp_setEnableDesktopLock(flag: bool) -> bool;
    // 设置是否禁用全局锁
    fn cp_setDisableDesktopLockKernel(flag: bool) -> bool;
    
    // --------------------hmpPlus-----------------------
    // 初始化主窗口并获取其句柄 并将其传递到libNcPlus 处理
    fn wp_initializeHandle (_hWnd:c_long)->c_void;
}

/**
 * 设置 程序禁止使用（禁用桌面锁定内核）
 */
pub fn setDisableDesktopLockKernel(flag: bool) -> bool {
    return unsafe { cp_setDisableDesktopLockKernel(flag) } == true;
}

/**
 * 设置 启用桌面锁定
 */
pub fn setEnableDesktopLock(flag: bool) -> bool {
    return unsafe { cp_setEnableDesktopLock(flag) } == true;
}

/**
 * 状态 程序禁止使用（禁用桌面锁定内核）
 */
pub fn hasDisableDesktopLockKernel() -> bool {
    return unsafe { cp_hasDisableDesktopLockKernel() } == true;
}

/**
 * 状态 启用桌面锁定
 */
pub fn hasEnableDesktopLock() -> bool {
    return unsafe { cp_hasEnableDesktopLock() } == true;
}

/**
 * 次数 启用桌面锁定(今日已护航次数)
 * - 返回最高数值99
 */
pub fn getLockFrequency() -> i32 {
    return unsafe { cp_getLockFrequency() };
}

/**
 * 名称数量 获取已经设置的进程匹配名称数量
 * - 返回最高数值99
 */
pub fn getProcessNameQuencySize() -> i32 {
    return unsafe { cp_getProcessNameQuencySize() };
}

/**
 * 获取当前允许匹配的驱动器设备数量
 */
pub fn getUnlockDriverNamesize() -> i32 {
    return unsafe { cp_getUnlockDriverNamesize() };
}

/**
 * 当前已存在匹配的驱动器设备数量
 */
pub fn hasUnlockDriverSuccess() -> bool {
    return getUnlockDriverNamesize() != 0;
}

/**
 * 获取当前已经匹配的进程/线程数量
 */
pub fn getProcessMatchSize() -> i32 {
    return unsafe { cp_getProcessMatchSize() };
}

/**
 * 初始化窗口 并将句柄回传到libNcPlus处理
 * - 移除窗口边框 (因为原生的会导致cpu莫名其面的高)
 */
pub fn sendInitializeHandle(_hwnd:i64) {    
 unsafe { wp_initializeHandle(_hwnd.try_into().unwrap()) };
}
