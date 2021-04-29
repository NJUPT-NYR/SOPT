# Get Started

## 简体中文

欢迎使用 SOPT。这是一个现代化的 Private Tracker 框架，包括了上传、浏览、下载、做种情况和管理员等各种基本功能。同时也可以加载许多可选插件。

### 后端 & Tracker

```shell
git clone https://github.com/njupt-nyr/sopt.git
cd sopt
cp .env.example .env
```

在开始前，您需要安装以下依赖：

1. PostgreSQL >= 9.5
2. Redis >= 6.0
3. Rust >= 1.50

修改 `.env` 文件如下示例：

```
# 后端地址，一般请保持默认
SERVER_ADDR=127.0.0.1:8000
# Tracker 地址，一般请保持默认
TRACKER_ADDR=127.0.0.1:8080
# JWT 的密钥，建议采用随机字符串以提高安全性
SECRET_KEY=secret
# 数据库配置，请保证该数据库存在，否则使用 createdb 命令创建
DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/sopt
# Tracker 的对外地址
ANNOUNCE_ADDR=https://tracker.sopt.rs/announce
SMTP.SERVER=smtp.gmail.com
SMTP.USERNAME=brethland@gmail.com
SMTP.PASSWORD=fake_pass
REDIS_URI=redis://127.0.0.1:6379/
```

编辑 `./backend/Cargo.toml` 以开关功能块：

```toml
[features]
# 将需要的功能填入该数组，以半角分号分隔
# 你必须选择一个 K-V 的存储层，默认是 sled
# 可选项有 sled, csv 和 rocksdb
default = ["email-restriction", "message", "sled"]
# 使用邮箱注册允许名单（无需邀请码）
email-restriction = []
# 私信功能
message = []
```

编辑 `./config/filtered-email`，加入邮箱白名单，一行一个，全小写，可以为空。

编辑 `./config/rank.sql` 来添加你的自定义用户等级设置，你也可以稍后在管理端添加和修改。

然后在终端运行以下命令：

```shell
cargo install sqlx-cli
sqlx migrate run
psql -U <PG_USER_NAME> -d sopt -f ./config/rank.sql
cargo build --release
```

将编译好的二进制包（路径为 `./target/release/sopt`,
`./target/release/sopt_proxy`, `./target/release/libretracker.dylib`) 
与 `.env` 以及 `./config/` 一起复制到你喜欢的任何地方。

### 前端

```shell
git clone https://github.com/NJUPT-NYR/SOPT-Frontend.git
cd SOPT-Frontend
cp .env.example .env.development
cp .env.example .env.production
```

在开始前，您需要安装以下依赖：

1. node >= 14.15.4
2. yarn >= 1.22.4

修改 `.env.*` 文件如下示例

```
# 启用mock server，一般用于开发环境
NEXT_PUBLIC_ENABLE_MOCK=true
# 服务端请求的接口的API地址
API_GATEWAY_URL=https://tracker.sopt.rs/api
# 客户端请求的接口的API地址
NEXT_PUBLIC_CLIENT_API_GATEWAY_URL=/api
```

```shell
# 开发环境
yarn install
yanr dev

# 生产环境
yarn install
yarn build
yarn start
```

## English

SOPT is a modern private tracker framework, it supports basic functions like
uploading, find torrents, downloading, seeding status and admin panel. You can
load many optional features too.

### Backend & Tracker

```shell
git clone https://github.com/njupt-nyr/sopt.git
cd sopt
cp .env.example .env
```

You need to install following dependencies:

1. PostgreSQL >= 9.5
2. Redis >= 6.0
3. Rust >= 1.50

Edit `.env`:

```
# server address, keep default if nothing wrong.
SERVER_ADDR=127.0.0.1:8000
# tracker address, keep default if nothing wrong.
TRACKER_ADDR=127.0.0.1:8080
# key for JWT Auth, you can generate some random strings
SECRET_KEY=secret
# database configuration, make sure you have DB already,
# or use createdb command
DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/sopt
# tracker's public address
ANNOUNCE_ADDR=https://tracker.sopt.rs/announce
# SMTP mail server configuration
SMTP.SERVER=smtp.gmail.com
SMTP.USERNAME=brethland@gmail.com
SMTP.PASSWORD=fake_pass
REDIS_URI=redis://127.0.0.1:6379/
```

Edit `./backend/Cargo.toml` with selected features：

```toml
[features]
# load features seperated by ','
# you must choose a kv storage backend
# default is sled, options are sled, csv and rocksdb
default = ["email-restriction", "message", "sled"]
# email allowlist(so no invitation code is needed)
email-restriction = []
# private message
message = []
```

Edit `./config/filtered-email`，add your own whitelist. one for a line, with
all lowercase(empty list is accepted too).

Edit `./config/rank.sql` to add your own rank settings. You can edit via control
panel later as well.

Run following commands in terminal：

```shell
cargo install sqlx-cli
sqlx migrate run
psql -U <PG_USER_NAME> -d sopt -f ./rank.sql
cargo build --release
```

Copy compiled binary(path `./target/release/sopt`,
`./target/release/sopt_proxy`, `./target/release/libretracker.dylib`) 
and `.env`, `./config/` to any path you like。

### Frontend

```shell
git clone https://github.com/NJUPT-NYR/SOPT-Frontend.git
cd SOPT-Frontend
cp .env.example .env.development
cp .env.example .env.production
```

You need to install following dependencies:

1. node >= 14.15.4
2. yarn >= 1.22.4

Edit `.env.*`:

```
# enable mock server, for development mode
NEXT_PUBLIC_ENABLE_MOCK=true
# api gateway url for server side
API_GATEWAY_URL=https://tracker.sopt.rs/api
# api gateway url for client side
NEXT_PUBLIC_CLIENT_API_GATEWAY_URL=/api
```

```shell
# development mode
yarn install
yanr dev

# production mode
yarn install
yarn build
yarn start
```
