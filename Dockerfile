# 构建前端
FROM node:18-alpine AS frontend
RUN npm install -g pnpm
WORKDIR /app/frontend
COPY app/ .
RUN pnpm install && pnpm build

# 构建后端
FROM rust:1.87 AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# 最终镜像
FROM nginx:alpine
COPY --from=frontend /app/frontend/dist /usr/share/nginx/html
COPY --from=backend /app/target/release/prompt-shelf /app/
COPY nginx-with-api.conf /etc/nginx/nginx.conf
EXPOSE 80
# 需要启动脚本同时运行nginx和rust服务
