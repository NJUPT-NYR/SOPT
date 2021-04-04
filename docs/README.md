# Get Started

## 简体中文

欢迎使用 SOPT。这是一个现代化的 Private Tracker 框架，包括了上传、浏览、下载、做种情况和管理员等各种基本功能。同时也可以加载许多可选插件。

### 后端

```shell
git clone https://github.com/njupt-nyr/sopt.git
cd sopt
cp .env.example .env
```

在开始前，您需要安装以下依赖：

1. PostgreSQL >= 9.5
2. Rust >= 1.5
3. GCC >= 8.0 or Clang >= 11.0

修改 `.env` 文件如下示例：

```
# 服务地址，一般请保持默认
SERVER_ADDR=127.0.0.1:8000
# JWT 的密钥，建议采用随机字符串以提高安全性
SECRET_KEY=secret
# 数据库配置，请保证该数据库存在，否则使用 createdb 命令创建
DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/sopt
# Tracker 的对外地址
ANNOUNCE_ADDR=https://tracker.sopt.rs/announce
# SMTP 邮件服务器的配置信息
SMTP.SERVER=smtp.gmail.com
SMTP.USERNAME=brethland@gmail.com
SMTP.PASSWORD=fake_pass
# Rocksdb 的存储路径，一般保持默认
ROCKSDB_PATH=./rocksdb
# 对象存储的路径，支持一切s3协议的引擎
OSS_PATH=http://127.0.0.1:9000
```

编辑 `Cargo.toml` 以开关功能块：

```toml
[features]
# 将需要的功能填入该数组，以半角分号分隔
default = ["email-restriction", "message"]
# 使用邮箱注册白名单（无需邀请码）
email-restriction = []
# 私信功能
message = []
```

编辑 `filtered-email`，加入邮箱白名单，一行一个，全小写，可以为空。

编辑 `rank.sql` 来添加你的自定义用户等级设置，你也可以稍后在管理端添加和修改。

然后在终端运行以下命令：

```shell
cargo install sqlx-cli
sqlx migrate run
psql -U <PG_USER_NAME> -d sopt -f ./rank.sql
cargo build --release
```

将编译好的二进制包（路径为 `./target/release/sopt`) 与 `.env` 以及 `filtered-email`
一起复制到你喜欢的任何地方。

### Tracker
在开始前，您需要安装以下依赖：
1. Rust >= 1.48
2. Redis >= 6.0

``` shell
git clone https://github.com/NJUPT-NYR/SOPT-Tracker
cargo build --release
```

将编译好的二进制包（路径为 [`./target/release/ruapt_proxy`, `./target/release/libretracker.dylib`]）
与`.env`以及`etc/redis.conf`一起复制到你喜欢的任何地方。

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

### 后续升级

todo!

## English

SOPT is a modern private tracker framework, it supports basic functions like
uploading, find torrents, downloading, seeding status and admin panel. You can
load many optional features too.

### Backend

```shell
git clone https://github.com/njupt-nyr/sopt.git
cd sopt
cp .env.example .env
```

You need to install following dependencies:

1. PostgreSQL >= 9.5
2. Rust >= 1.5
3. GCC >= 8.0 or Clang >= 11.0

Edit `.env`:

```
# server address, keep default if nothing wrong.
SERVER_ADDR=127.0.0.1:8000
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
# path for rocksdb, keep default if nothing wrong.
ROCKSDB_PATH=./rocksdb
```

Edit `Cargo.toml` with selected features：

```toml
[features]
# load features seperated by ','
default = ["email-restriction", "message"]
# email whitlelist(so no invitation code is needed)
email-restriction = []
# private message
message = []
```

Edit `filtered-email`，add your own whitelist. one for a line, with
all lowercase(empty list is accepted too).

Edit `rank.sql` to add your own rank settings. You can edit via control
panel later as well.

Run following commands in terminal：

```shell
cargo install sqlx-cli
sqlx migrate run
psql -U <PG_USER_NAME> -d sopt -f ./rank.sql
cargo build --release
```

Copy compiled binary(path `./target/release/sopt`) and `.env`, `filtered-email`
to any path you like。

### Tracker

todo!

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

### Updating

todo!
