#include "./main.hpp"


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

