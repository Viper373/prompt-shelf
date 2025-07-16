# PromptShelf - 提示词版本控制系统

## 系统概述
PromptShelf是一个基于Rust构建的提示词版本控制系统，提供类似Git的功能来管理提示词的创建、版本跟踪和历史查询。

## 功能特性
- 提示词版本控制：创建、提交和回滚提示词版本
- 用户认证：基于JWT的身份验证和授权
- REST API：完整的API接口用于提示词管理操作
- 缓存层：使用Dragonfly(兼容Redis)提高查询性能
- Docker部署：包含MySQL、Redis和应用服务的完整Docker配置

## 快速开始

### 使用Docker Compose
```bash
# 启动所有服务
docker-compose up --build -d

# 查看日志
docker-compose logs -f
```

## 环境变量配置
以下环境变量可以在docker-compose.yml中配置：
- `DATABASE_URL`：MySQL数据库连接URL
- `REDIS_URI`：Dragonfly/Redis连接字符串
- `JWT_SECRET`：JWT签名密钥
- `JWT_EXPIRATION`：JWT过期时间(秒)
- `ALLOW_REGISTER`：是否允许用户注册(true/false)

## API文档
详细的API文档，请参考[Markdown文档](./doc/PromptShelf.md)

### 核心API端点摘要

#### 认证
- `POST /user/signup`：用户注册
- `POST /user/signin`：用户登录，获取JWT令牌

#### 提示词管理
- `POST /prompt/create_prompt`：创建新提示词
- `POST /prompt/create_node`：为提示词添加节点
- `POST /prompt/create_commit`：提交提示词更改
- `GET /prompt/query`：查询提示词历史
- `GET /prompt/latest`：获取最新版本提示词
- `POST /prompt/rollback`：回滚到之前版本
- `POST /prompt/revert`：回退到当前版本上一次提交
- `DELETE /prompt/`：删除提示词

#### 系统状态
- `GET /status`：获取系统状态和运行时间

#### 管理员控制
- `POST /control/register`：切换用户注册功能
- `GET /control/list/user`：列出所有用户(仅管理员)
- `DELETE /control/user/{user_id}`: 删除用户
- `POST /control/disable/user`: 禁用/启用用户

## 项目结构
```
├── src/
│   ├── db/           # 数据库模型和操作
│   ├── routes/       # API路由处理
│   │   ├── prompt.rs # 提示词相关接口
│   │   ├── user.rs   # 用户认证接口
│   │   └── ...
│   └── main.rs       # 应用入口
├── app/              # 前端React应用
├── doc/              # 文档文件
└── docker-compose.yml # Docker配置
```

## 技术栈
- **后端**：Rust, Axum, SeaORM
- **数据库**：MySQL
- **缓存**：Dragonfly(兼容Redis)
- **认证**：JWT, Argon2密码哈希
- **前端**：React, TypeScript, Vite
