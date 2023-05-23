#include <iostream>
#include "./main.hpp"

// 微信窗口分类
struct WeChatWindowType
{
    string name;
    string className;
    string baseName;
    bool fuzzy;
};

vector<WeChatWindowType> WE_CHAT_WINDOW_TYPE_DEFAULT;

// 枚举微信窗口返回值
struct AllWeChatWindowItme
{
    string title;
    string className;
    string exeName;
    HWND hwnd;
    string infoName;
};

// 获取进程可执行文件路径
string getProcessidFilePath(int ProcessID)
{
    string Run_lpFilename = "";
    HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, ProcessID);
    char lpFilename[1024];
    if (hProcess == nullptr)
    {
        CloseHandle(hProcess);
        return Run_lpFilename;
    }
    GetModuleFileNameExA(hProcess, NULL, (LPSTR)lpFilename, 1024);
    CloseHandle(hProcess);
    return string(lpFilename);
}

// 权限提升
BOOL EnableShutDownPriv()
{
    HANDLE Handle_Token = NULL;
    TOKEN_PRIVILEGES PermissionAttribute = {0};
    // 打开当前程序的权限令牌
    bool is_Open_Process_Token = OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &Handle_Token);
    if (!is_Open_Process_Token)
    {
        return FALSE;
    }
    // 获得某一特定权限的权限标识LUID 保存到权限属性中
    if (!LookupPrivilegeValue(NULL, SE_SHUTDOWN_NAME, &PermissionAttribute.Privileges[0].Luid))
    {
        CloseHandle(Handle_Token);
        return FALSE;
    }
    PermissionAttribute.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
    PermissionAttribute.PrivilegeCount = 1;
    // 提升到系统权限
    if (!AdjustTokenPrivileges(Handle_Token, FALSE, &PermissionAttribute, sizeof(TOKEN_PRIVILEGES), NULL, NULL))
    {
        CloseHandle(Handle_Token);
        return FALSE;
    }
    return TRUE;
}

// 终止进程
bool killProcessID(DWORD ProcessID)
{
    bool Kill_info_bool = false;
    EnableShutDownPriv();
    HANDLE killHandle = OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION | PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, FALSE, ProcessID);
    if (killHandle == NULL)
    {
        Kill_info_bool = false;
    }
    else
    {
        bool Terminate_Kill_info_bool = TerminateProcess(killHandle, 0);
        Kill_info_bool = Terminate_Kill_info_bool;
    }
    return Kill_info_bool;
}

// 获取句柄对应的进程名称
string getHandleBaseName(HWND hwnd)
{
    string result = "";
    if (GetWindow(hwnd, GW_OWNER) == (HWND)0 && IsWindowVisible(hwnd))
    {
        DWORD dwProcessID = 0;
        ::GetWindowThreadProcessId(hwnd, &dwProcessID);
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);
        if (hProcess != nullptr)
        {
            char processName[MAX_PATH];
            GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
            result.append(processName);
        }
        CloseHandle(hProcess);
    }
    return result;
}

// 判断窗口是否是微信的
bool hasClassNameWeChatWindow(string className, HWND hwnd = NULL)
{
    for (size_t i = 0; i < WE_CHAT_WINDOW_TYPE_DEFAULT.size(); i++)
    {
        WeChatWindowType data = WE_CHAT_WINDOW_TYPE_DEFAULT[i];
        if (!data.fuzzy)
        {
            if (data.className == className)
                return true;
        }
    }

    // Chrome视图
    if ("Chrome_WidgetWin_0" == className || "Chrome_RenderWidgetHostHWND" == className)
    {
        if (hwnd == NULL)
            return false;
        string pBaseName = getHandleBaseName(hwnd);
        if (pBaseName == "WeChatAppEx.exe" || pBaseName == "WeChat.exe")
        {
            return true;
        }
        // 这个是谷歌浏览器视图 需要打开句柄判断
        return false;
    }

    return false;
}

string getClassNameWeChatWindowHumanName(string className, HWND hwnd = NULL)
{
    string result = "未知";
    if (hasClassNameWeChatWindow(className, hwnd))
    {
        // Chrome视图
        if ("Chrome_WidgetWin_0" == className || "Chrome_RenderWidgetHostHWND" == className)
        {
            if (hwnd == NULL)
                return "不存在";
            string pBaseName = getHandleBaseName(hwnd);
            char windowTitle[MAX_PATH] = {0};
            GetWindowTextA(hwnd, windowTitle, MAX_PATH);
            if (pBaseName == "WeChatAppEx.exe" || pBaseName == "WeChat.exe")
            {
                if (className == "Chrome_RenderWidgetHostHWND")
                {
                    result = "小程序";
                }
                else if (windowTitle == "微信")
                {
                    result = "浏览页";
                }
                else if (windowTitle == "视频号")
                {
                    result = "视频号";
                }
                else if (windowTitle == "小程序")
                {
                    result = "小程序";
                }
                else if (windowTitle == "公众号")
                {
                    result = "公众号";
                }
                else
                {
                    result = "小程序";
                }
            }
        }
        for (size_t i = 0; i < WE_CHAT_WINDOW_TYPE_DEFAULT.size(); i++)
        {
            WeChatWindowType data = WE_CHAT_WINDOW_TYPE_DEFAULT[i];
            if (!data.fuzzy)
            {
                if (data.className == className)
                    result = data.name;
            }
        }
    }
    return result;
}

vector<AllWeChatWindowItme> getWeChatAllWindow()
{

    vector<AllWeChatWindowItme> hwndList = {};
    EnableShutDownPriv();
    HWND hwnd = GetDesktopWindow();
    // 获取桌面子窗口句柄
    hwnd = GetWindow(hwnd, GW_CHILD);
    int counter = 0;
    for (size_t index = 0; hwnd != NULL; index++)
    {
            EnableShutDownPriv();
        if (GetWindow(hwnd, GW_OWNER) == (HWND)0 && IsWindowVisible(hwnd))
        {
            char lpClassName[MAX_PATH];
            GetClassNameA(hwnd, lpClassName, MAX_PATH);
            if (hasClassNameWeChatWindow((string)lpClassName, hwnd))
            {
                char processName[MAX_PATH] = { 0 };
                DWORD dwProcessId = {};
                char windowTitle[MAX_PATH] = {0};

                // get -
                GetWindowTextA(hwnd, windowTitle, MAX_PATH);
                ::GetWindowThreadProcessId(hwnd, &dwProcessId);
                HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessId);
                if (hProcess != nullptr)
                {
                    GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
                }
                CloseHandle(hProcess);
                AllWeChatWindowItme wecharhwnd;
                wecharhwnd.className = lpClassName;
                wecharhwnd.exeName = getProcessidFilePath(dwProcessId);
                wecharhwnd.hwnd = hwnd;
                wecharhwnd.infoName = getClassNameWeChatWindowHumanName((string)lpClassName, hwnd);
                wecharhwnd.title = windowTitle;
                hwndList.push_back(wecharhwnd);
                cout<< "className:"<< wecharhwnd.className << endl;
                cout<< "exeName:"<< wecharhwnd.exeName << endl;
                cout<< "infoName:"<< wecharhwnd.infoName << endl;
                cout<< "hwnd:"<< to_string((int64_t)wecharhwnd.hwnd) << endl;
                cout<< "title:"<< wecharhwnd.title << endl;
                cout<< "------------------------------------------------------------------\n" << endl;
            }
        }
        // 获取窗口名

        hwnd = GetNextWindow(hwnd, GW_HWNDNEXT);
    }
    return hwndList;
}

void push_type_to_global_wechat_type(string name, string className, string BaseName, bool fuzzyMatch)
{
    WeChatWindowType _WeChatType;
    _WeChatType.name = name;
    _WeChatType.className = className;
    _WeChatType.baseName = BaseName;
    _WeChatType.fuzzy = fuzzyMatch;
    WE_CHAT_WINDOW_TYPE_DEFAULT.push_back(_WeChatType);
}

void SetWeChatWindowTypeDefault()
{
    /*

    let d = "";
    for (let data of fs.readJsonSync("D:\\source\\rust\\hmPocketSecrets\\pockelib\\weChatWinType.json"))
    {
        d += `push_type_to_global_wechat_type("${data.name}", "${data.className}", "${data.baseName}", ${data.fuzzy});\n`
    }

    copy(d)
    */
    push_type_to_global_wechat_type("主界面", "WeChatMainWndForPC", "", false);
    push_type_to_global_wechat_type("单聊窗口", "ChatWnd", "", false);
    push_type_to_global_wechat_type("文件管理", "FileListMgrWnd", "", false);
    push_type_to_global_wechat_type("小程序选择", "AppletPanelWnd", "", false);
    push_type_to_global_wechat_type("笔记", "FavNoteWnd", "", false);
    push_type_to_global_wechat_type("通讯录管理", "ContactManagerWindow", "", false);
    push_type_to_global_wechat_type("any", "Chrome_RenderWidgetHostHWND", "WeChatAppEx.exe", true);
    push_type_to_global_wechat_type("any", "Chrome_WidgetWin_0", "WeChatAppEx.exe", true);
    push_type_to_global_wechat_type("设置", "SettingWnd", "", false);
    push_type_to_global_wechat_type("备份窗口", "BackupRestoreEntryWnd", "", false);
    push_type_to_global_wechat_type("朋友圈", "SnsWnd", "", false);
    push_type_to_global_wechat_type("公众号", "H5SubscriptionProfileWnd", "", false);
    push_type_to_global_wechat_type("收藏预览", "FavRecordWnd", "", false);
    push_type_to_global_wechat_type("图片查看", "ImagePreviewWnd", "", false);
    push_type_to_global_wechat_type("登录界面", "WeChatLoginWndForPC", "", false);
    push_type_to_global_wechat_type("分享", "SelectContactWnd", "", false);
    push_type_to_global_wechat_type("托盘消息", "TrayNotifyWnd", "", false);
    push_type_to_global_wechat_type("简约名片", "NewContactProfileWnd", "", false);
    push_type_to_global_wechat_type("文件传输助手选择", "HandoffMenuWnd", "", false);
    push_type_to_global_wechat_type("选择联系人", "AddMemberWnd", "", false);
    push_type_to_global_wechat_type("聊天文件", "FileManagerWnd", "", false);
    push_type_to_global_wechat_type("通话", "TalkRoomWnd", "", false);
    push_type_to_global_wechat_type("通话邀请", "GroupVoipTrayWnd", "", false);
    push_type_to_global_wechat_type("联系人侧边栏", "SessionChatRoomDetailWnd", "", false);
    push_type_to_global_wechat_type("修改群聊名称", "RoomInfoModifyDialog", "", false);
}

int main()
{

    // HWND hWnd = GetConsoleWindow();
    // hWnd &&ShowWindow(hWnd, SW_HIDE);
    EnableShutDownPriv();
    SetWeChatWindowTypeDefault();
    vector<AllWeChatWindowItme> weChatAllWindow = getWeChatAllWindow();

}