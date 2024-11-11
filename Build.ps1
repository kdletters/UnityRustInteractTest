# 设置路径变量
$UNITY_PATH = "E:\Software\Unity\6000.0.22f1c1\Editor\Unity.exe"
$BUILD_PATH = "Build\Windows\UnityRustInteractTest.exe"
$RUST_PATH = "rust-unity"
$RUST_BUILD_TOOL_PATH = "rust-unity\build-tool"
$DLL_PATH = "Assets\RustLib\dlls"

# 获取当前路径
$basePath = $PWD

# 构建 Rust 项目
cd $RUST_PATH
cargo build --release
cd $basePath

# 运行 Rust 构建工具
cd $RUST_BUILD_TOOL_PATH
cargo run --release
cd $basePath

# 运行 Unity 构建命令
Write-Output "Starting Unity build..."
Start-Process $UNITY_PATH "-quit -batchmode -buildWindows64Player $BUILD_PATH" -Wait
Write-Output "Unity build completed. $LASTEXITCODE"

# 删除 DLL 目录（如果它存在且为空）
if (Test-Path $DLL_PATH) {
    Remove-Item -Path $DLL_PATH -Recurse -Force
    Remove-Item -Path "$DLL_PATH.meta" -Force
}

Write-Output "Build completed successfully!"

Start-Process $BUILD_PATH