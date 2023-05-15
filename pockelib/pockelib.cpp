#include <iostream>
#include "./main.hpp"
//#include "weChat.cpp"
// #pragma comment(linker, "/subsystem:\"windows\" /entry:\"mainCRTStartup\"") // 设置入口地址

struct AllWeChatWindowItme
{
    string name;
    string className;
    string exeName;
    HWND hwnd;
    string infoName;

};
vector<AllWeChatWindowItme> getWeChatWindow()
{

    vector<AllWeChatWindowItme> hwndList = {};

    HWND hwnd = GetDesktopWindow();
    // 获取桌面子窗口句柄
    hwnd = GetWindow(hwnd, GW_CHILD);
    int counter = 0;
    for (size_t index = 0; hwnd != NULL; index++)
    {
        char lpClassName[MAX_PATH];
        GetClassNameA(hwnd, lpClassName, MAX_PATH);
        string strClassName;
        strClassName.append(lpClassName);
        char processName[MAX_PATH];
        DWORD dwProcessId = {};
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessId);
        if (hProcess != nullptr)
        {
            GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
        }
        
        // 获取窗口名
        CHAR windowTitle[MAX_PATH] = { 0 };
        GetWindowTextA(hwnd, windowTitle, MAX_PATH);

        CloseHandle(hProcess);
        cout << windowTitle << "      "   << processName << "      "  << strClassName << "      "  << dwProcessId << endl;
        if (processName == "WeChat.exe")
        {
           
            cout << processName << strClassName << dwProcessId << endl;
            // switch (strClassName)
            // {
            //     // 主界面
            // case "WeChatMainWndForPC":
            //     /* code */
            //     break;
            //     // 私聊
            // case "ChatWnd":
            //     /* code */
            //     break;
            //     // 文件管理
            // case "FileListMgrWnd":
            //     /* code */
            //     break;
            //     // 小程序选择面板
            // case "AppletPanelWnd":
            //     /* code */
            //     break;
            //     // 通讯录管理
            // case "ContactManagerWindow":
            //     /* code */
            //     break;
            // // 微信浏览器 公众号和视频号
            // case "Chrome_WidgetWin_0":
            // SnsWnd 朋友圈
            //     break;
            // default:

            //     break;
            // }
        }
        hwnd = GetNextWindow(hwnd, GW_HWNDNEXT);
    }
    return hwndList;
}


int main()
{
    // HWND hWnd = GetConsoleWindow();
    // hWnd &&ShowWindow(hWnd, SW_HIDE);
    getWeChatWindow();

}