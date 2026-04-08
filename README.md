# Void War Mod Manager

## Safety
The mod manager has no built-in protection against installation of malicious code. Only patch mods from trusted sources!

## Building
After cloning this repository, run this command in its root directory:

```
git submodule update --init --recursive
```

This project is designed to be built in [Visual Studio Code](https://code.visualstudio.com/) using the `run-release.bat` and `run-debug.bat` build scripts. Before building, you'll need to install [Rust](https://rust-lang.org/) and [Visual Studio 18+](https://visualstudio.microsoft.com/) with build tools for .NET 10 and C++. It is also reccomended to install the `rust-analyzer` extension in VSCode.

In order to automatically build `UTMTInterface` automatically when using the VSCode build scripts in `ModManager`, you'll need to add `msbuild` to your Path environment variable. Press the Windows key, type "Edit the system environment variables" and press enter. Click "Environment Variables...", double-click the Path system variable and click "New". For the community edition of Visual Studio 18, this should be the directory containing `MSBuild.exe`:

```
C:\Program Files\Microsoft Visual Studio\18\Community\MSBuild\Current\Bin
```

After you've added this to your Path, you can confirm it's correct by running `where msbuild` on your console.

## License
This project makes use of [Underanalyzer](https://github.com/UnderminersTeam/Underanalyzer) and [UndertaleModLib](https://github.com/UnderminersTeam/UndertaleModTool), which are licensed under the Mozilla Public License v2.0 and the GNU General Public License v3.0 respectively. The source code for this project is licensed under the GNU General Public License v3.0. This license does NOT extend to the project's assets. Fonts, images, music and sounds included in this project are not free for commercial use.
