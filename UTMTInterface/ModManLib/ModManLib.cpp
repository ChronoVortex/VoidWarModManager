#include <filesystem>

#define EXPORT_SPEC extern "C" __declspec(dllexport)
#define MANAGED_STR(pString) System::Runtime::InteropServices::Marshal::PtrToStringAnsi(System::IntPtr((char*)pString))

EXPORT_SPEC double EX_ModmanInstallMod(const char* dataPath, const char* modPath) {
    UndertaleProjectMan::ProjectInstaller^ pj = gcnew UndertaleProjectMan::ProjectInstaller();
    pj->InstallProject(MANAGED_STR(dataPath), MANAGED_STR(modPath));
    return 1.0;
}
