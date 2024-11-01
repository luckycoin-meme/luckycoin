#!/bin/bash

# 指定要查找的端口号
PORT=8899
FAUCET_PORT=9900
GOSSIP_PORT=1024
TPU_PORT=1027

# 查找占用指定端口的进程
PID=$(lsof -t -i:"$PORT")

if [ -z "$PID" ]; then
    echo "没有找到占用端口 $PORT 的进程。"
else
    echo "找到占用端口 $PORT 的进程 PID: $PID"
    kill -9 "$PID"
    echo "已终止进程 $PID。"
fi

# 检查faucet端口是否被占用
FAUCET_PID=$(lsof -t -i:"$FAUCET_PORT")
if [ -n "$FAUCET_PID" ]; then
    echo "端口 $FAUCET_PORT 已被占用，PID: $FAUCET_PID"
    kill -9 "$FAUCET_PID"
    echo "已终止进程 $FAUCET_PID。"
fi

# 检查Gossip和TPU端口
for port in $GOSSIP_PORT $TPU_PORT; do
    PORT_PID=$(lsof -t -i:"$port")
    if [ -n "$PORT_PID" ]; then
        echo "端口 $port 已被占用，PID: $PORT_PID"
        kill -9 "$PORT_PID"
        echo "已终止进程 $PORT_PID。"
    fi
done

# 清理旧的测试账本
cd .. && rm -rf test-ledger

# 启动 solana-test-validator
echo "启动 solana-test-validator..."
solana-test-validator