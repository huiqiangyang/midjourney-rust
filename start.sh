#!/bin/bash

# 脚本用途：管理 Rust 应用程序的启动和停止

# 设置常量
RUST_APP_NAME="midjourney-rust"
RUN_DIR="./run"
RELEASE_DIR="./release"
TARGET_DIR="./target/x86_64-unknown-linux-musl/release"
LOG_FILE="$RUN_DIR/midjourney-rust.log"

# 创建目录（如果不存在）
create_directory() {
    [ ! -d "$1" ] && mkdir -p "$1"
}

# 启动应用程序
start_application() {
    echo "Start pulling the latest code..."
    git pull origin master

    stop_application
    # 先删除目录，再创建目录
    rm -rf "$RUN_DIR"
    create_directory "$RUN_DIR"

    cp "$RELEASE_DIR/$RUST_APP_NAME" "$RUN_DIR/"

    echo "Starting the application..."
    $RUN_DIR/$RUST_APP_NAME > "$LOG_FILE" 2>&1 &
}

# 关闭应用程序
stop_application() {
    CURRENT_PID=$(pgrep $RUST_APP_NAME)
    [ -n "$CURRENT_PID" ] && echo "Stopping the currently running $RUST_APP_NAME (PID: $CURRENT_PID)..." && kill $CURRENT_PID && sleep 2
}

# 构建应用程序
build() {
    echo "Building the Rust project..."
    cargo build --release --target x86_64-unknown-linux-musl

    # 复制可执行文件到 release 目录下
    [ -f "$TARGET_DIR/$RUST_APP_NAME" ] && cp "$TARGET_DIR/$RUST_APP_NAME" "$RELEASE_DIR/" || echo "Error: Executable file not found. Build may have failed."
}

# 根据传入的参数执行相应的操作
case "$1" in
    "start") start_application ;;
    "stop") stop_application ;;
    "build") build ;;
    *) echo "Usage: $0 {start|stop|build}" && exit 1 ;;
esac

echo "Done."
