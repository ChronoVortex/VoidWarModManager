if not exist "target\release" mkdir target\release
for %%f in (*.dll) do copy %%f target\release
copy ModManLib.runtimeconfig.json target\release
cargo build --release
target\release\modman
