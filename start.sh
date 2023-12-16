#!/bin/bash

# 脚本用途：管理 Rust 应用程序的启动和停止

# 设置常量
RUST_APP_NAME="midjourney-rust"
RUN_DIR="./run"
TARGET_DIR="./target/release"
LOG_FILE="$RUN_DIR/midjourney-rust.log"

# 创建 run 目录（如果不存在）
create_run_directory() {
    [ ! -d "$RUN_DIR" ] && mkdir "$RUN_DIR"
}

# 启动应用程序
start_application() {
    echo "Starting the application..."
    $RUN_DIR/$RUST_APP_NAME > "$LOG_FILE" 2>&1 &
}

# 关闭应用程序
stop_application() {
    CURRENT_PID=$(pgrep $RUST_APP_NAME)
    [ -n "$CURRENT_PID" ] && echo "Stopping the currently running $RUST_APP_NAME (PID: $CURRENT_PID)..." && kill $CURRENT_PID && sleep 2
}

# 构建并启动应用程序
build_and_start() {
    echo "Updating the project from Git..."
    git pull origin master
    echo "Building the Rust project..."
    cargo build --release

    # 移动可执行文件到 run 目录下
    if [ -f "$TARGET_DIR/$RUST_APP_NAME" ]; then
        create_run_directory
        mv "$TARGET_DIR/$RUST_APP_NAME" "$RUN_DIR/"
        # 启动应用程序
        start_application
    else
        echo "Error: Executable file not found. Build may have failed."
    fi
}

# 根据传入的参数执行相应的操作
case "$1" in
    "start") create_run_directory && build_and_start ;;
    "stop") stop_application ;;
    "build_and_start") build_and_start ;;
    *) echo "Usage: $0 {start|stop|build_and_start}" && exit 1 ;;
esac

echo "Done."
