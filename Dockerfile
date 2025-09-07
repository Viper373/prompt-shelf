# 构建前端
FROM node:18-alpine AS frontend-builder
RUN npm install -g pnpm
WORKDIR /app
COPY app/package.json app/pnpm-lock.yaml ./
COPY app/ .
RUN pnpm install
RUN pnpm build

# 构建后端
FROM rust:1.87 AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# 最终运行镜像 - 基于ubuntu
FROM ubuntu:22.04

# 安装nginx和后端运行依赖
RUN apt-get update -y && \
    apt-get install -y nginx libssl-dev ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# 复制前端构建产物
COPY --from=frontend-builder /app/dist /usr/share/nginx/html

# 复制后端可执行文件
COPY --from=backend-builder /app/target/release/prompt-shelf /app
RUN chmod +x /app

# 暴露两个端口
EXPOSE 80 8000

# 启动脚本：同时启动nginx和后端服务
RUN echo '#!/bin/bash\nnginx &\n/app\nwait' > /start.sh && chmod +x /start.sh

CMD ["/start.sh"]
