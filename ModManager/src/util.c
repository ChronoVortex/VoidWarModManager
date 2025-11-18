#include <windows.h>

int ex_run_program(LPSTR program) {
    // Additional information
    STARTUPINFOA si;
    PROCESS_INFORMATION pi;

    // Set the size of the structures
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    // Start the program up
    if (CreateProcessA(program, NULL, NULL, NULL, FALSE, CREATE_NEW_CONSOLE, NULL, NULL, &si, &pi)) {
        // Close process and thread handles
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);

        return 1;
    }

    return 0; 
}
