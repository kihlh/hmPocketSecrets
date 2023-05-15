// 基础c++库
#include <iostream>
#include <string>
#include <stdlib.h>
#include <vector>
#include <windows.h>
#include <regex>

// 实现进程结束
#include <process.h>
#include <Tlhelp32.h>
#include <iostream>
#include <Psapi.h>
#include <Dwmapi.h>

// 与用户交互
#include <winuser.h>
#include <tchar.h>

// 文件io
#include <fstream>
#include <shlwapi.h>
#pragma comment(lib, "shlwapi.lib")
#include <filesystem>

// 读取 dawn_launcher_profile/ 配置 的  文件
#include <codecvt>
#include <locale>
#include <stringapiset.h>
#include <shlobj.h>

#include <thread>

// W字符 A字符 自适应
#ifdef UNICODE
typedef LPWSTR LPTSTR;
typedef LPCWSTR LPCTSTR;
#else
typedef LPSTR LPTSTR;
typedef LPCSTR LPCTSTR;
#endif

using namespace std;





/*

vector<string> getArgv(int argc, char* char_argv[]);
EnvInfo getEnvInfo(int argc, char *char_argv[]);
string getProcessidFilePath(int ProcessID);
BOOL EnableShutDownPriv();
bool killProcessID(DWORD ProcessID);
BOOL isSystemFor64bit();
BOOL ReSetWindows(DWORD dwReason, BOOL aims);
vector<AllWeChatWindowItme> getAllWeChatWindow();
HWND GetSystemTrayHwnd();
BOOL lockSystemInteraction(bool lockb );
int GetSystemMetricsLen();
vector<RECT> GetDeviceCapsAll();
RECT GetCurrentMonitorRect();

*/