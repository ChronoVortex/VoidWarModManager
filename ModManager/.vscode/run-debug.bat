msbuild ..\UTMTInterface\UTMTInterface.sln /p:Configuration=Release
set RUST_BACKTRACE=1
cargo run
