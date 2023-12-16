#!/bin/bash

# 设置常量
RUST_APP_NAME="midjourney-rust"

# 创建 run 目录（如果不存在）
create_run_directory() {
    RUN_DIR="$SCRIPT_DIR/run"
    [ ! -d "$RUN_DIR" ] && mkdir "$RUN_DIR"
}

# 启动应用程序
start_application() {
  stop_application
    echo "Starting the application..."
    ./target/release/$RUST_APP_NAME
}

# 关闭应用程序
stop_application() {
    CURRENT_PID=$(pgrep $RUST_APP_NAME)
    [ -n "$CURRENT_PID" ] && echo "Stopping the currently running $RUST_APP_NAME (PID: $CURRENT_PID)..." && kill $CURRENT_PID && sleep 2
}

# 构建并启动应用程序
build_and_start() {
    echo "Updating the project from Git..."
    git pull origin main
    echo "Building the Rust project..."
    cargo build --release
    start_application
}

# 根据传入的参数执行相应的操作
case "$1" in
    "start") create_run_directory && start_application ;;
    "stop") stop_application ;;
    "build_and_start") create_run_directory && build_and_start ;;
esac

echo "Done."
