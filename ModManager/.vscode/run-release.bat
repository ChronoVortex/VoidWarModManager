msbuild ..\UTMTInterface\UTMTInterface.sln /p:Configuration=Release
if not exist "target\release" mkdir target\release
for %%f in (*.dll) do copy %%f target\release
cargo build --release
target\release\modman
