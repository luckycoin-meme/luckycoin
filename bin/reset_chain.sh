#!/bin/bash

# 指定要查找的端口号
PORT=8899

# 查找占用指定端口的进程
PID=$(lsof -t -i:"$PORT")

if [ -z "$PID" ]; then
    echo "没有找到占用端口 $PORT 的进程。"
else
    # 显示进程信息
    echo "找到占用端口 $PORT 的进程 PID: $PID"

    # 杀死进程
    kill -9 "$PID"

    echo "已终止进程 $PID。"
fi

cd .. && rm -rf test-ledger

# 启动 solana-test-validator
echo "启动 solana-test-validator..."
solana-test-validator