#include "./main.hpp"



EnvInfo getEnvInfo(int argc, char *char_argv[])
{
    DWORD this_pid = GetCurrentProcessId();
    string exePath = getProcessidFilePath(this_pid);

    char cwdPath[MAX_PATH];
    PathCombineA(cwdPath, exePath.c_str(), "..");

    char parentPath[MAX_PATH];
    PathCombineA(parentPath, cwdPath, "..");
    EnvInfo envInfo;
    envInfo.args = getArgv(argc,char_argv);
    envInfo.cwdPath = cwdPath;
    envInfo.exePath = exePath;
    envInfo.parentPath = parentPath;
    envInfo.pid = this_pid;
    return envInfo;
}

vector<string> getArgv(int argc, char *char_argv[])
{
    vector<string> argv = {};
    string str_argv = "";

    for (size_t i = 0; i < argc; i++)
    {
        string arg = string(char_argv[i]);
        argv.push_back(arg);
        str_argv.append(arg);
    }
    return argv;
}
// 判断 x64 系统
BOOL isSystemFor64bit()
{
    SYSTEM_INFO SystemInfo;
    GetNativeSystemInfo(&SystemInfo);
    if (SystemInfo.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_IA64 || SystemInfo.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64)
        return TRUE;
    else
        return FALSE;
}

// 执行注销，关机，重启
BOOL ReSetWindows(DWORD dwReason, BOOL aims)
{
    if (dwReason != EWX_LOGOFF && dwReason != EWX_REBOOT && dwReason != EWX_SHUTDOWN)
        return FALSE;
    OSVERSIONINFO osvi = {0};
    osvi.dwOSVersionInfoSize = sizeof(OSVERSIONINFO);
    dwReason |= (aims != FALSE) ? EWX_FORCE : EWX_FORCEIFHUNG;
    return ExitWindowsEx(dwReason, 0);
}