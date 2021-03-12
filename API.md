# SOPT API Docs

## General Data

### GeneralResponse

    - data: Json
    - success: Boolean
    - errMsg: String

### DataWithCount

    - count: Int
    - ret: Json

### SlimUser

    - id: Int
    - email: String
    - username: String
    - passkey: String

## UserInfo

    - id: Int
    - username: String
    - register_time: String
    - last_activity: String
    - invitor: Option<String>
    - upload: Int
    - download: Int
    - money: Float
    - rank: Int
    - avatar: Option<String>(b64 encoded)
    - other: Option<Json>

### SlimInvitation

    - code: String
    # this is email address
    - sendTo: String
    - isUsed: Boolean

### TorrentInfo

    - id: Int
    - title: String
    - poster: String
    - description: String
    - downloaded: Int
    - visible: Boolean
    - tag: Vec<String>

### SlimTorrent

    - id: Int
    - title: String
    - poster: String
    - downloaded: Int
    - tag: Vec<String>
    - length: Int(size of torrent in bytes)

### Tag

    - id: Int
    - name: String
    - amount: Int

## User API

### /user/add_user
**Type**: POST

**Request**

    - email: String(200)
    - username: String(50)
    - password: String(200)
    - invite_code: Option<String>(200)

**Example**

```json
{
  "email": "brethland@gmail.com",
  "username": "brethland",
  "password": "password"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `SlimUser`

### /user/login
**Type**: POST

**Request**

    - username: String(50)
    - password: String(200)

**Example**

```json
{
  "username": "brethland",
  "password": "password"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with data a string represents jwt token

### /user/auth/check_identity
**Type**: POST

**Request**

    - password: String(200)

**Example**

```json
{
  "password": "password"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Default success `GeneralResponse`

**Comment**

This api is used for check current user's identity when it comes to some dangerous
actions like `reset_password` and `reset_passkey`. 
Then user can perform actions like a charm.

### /user/auth/reset_password
**Type**: POST

**Request**

    - passsword: String(200)

**Example**

```json
{
  "password": "password"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Default success `GeneralResponse`

### /user/auth/reset_passkey
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Default success `GeneralResponse`

### /user/auth/transfer_money
**Type**: POST

**Request**

    - to: String
    - amount: Float

**Example**

```json
{
  "to": "Tadokoro Koniji",
  "amount": 114514.1919
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Default success `GeneralResponse`

### user/upload_avatar
**Type**: POST

**Request**

Multipart Files

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Default success `GeneralResponse`

**Comment**

Allowed file types are `JPG` `PNG` and `WebP`.

Avatar will be stored in the database with base64 encoded.

### /user/personal_info_update
**Type**: POST

**Request**

    - info: Json

**Example**

```json
{
  "info": {
    "学校": "下北泽大学",
    "个人网站": "114514.com",
    "介绍": "24岁，是学生。"
  }
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `UserInfo`

**Comment**

Any key will be accepted and stored in Database without any change.

## Invitation API

### /invitation/send_invitation
**Type**: POST

**Request**

    - to: String
    - address: String(200)
    - body: String

**Example**

```json
{
  "to": "Hydrogen5",
  "address": "test@gmail.com",
  "body": "Enjoy your journey at SOPT!"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `SlimInvitation`

### /invitation/list_invitations
**Type** GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a list of `SlimInvitation`s

**Comment**

List all invitations current user sent before.

## Torrent API

### /torrent/add_torrent
**Type**: POST

**Request**

    - title: String
    - description: Option<String>
    - tags: Option<Vec<String>(5)>

**Example**

```json
{
  "title": "[喵萌奶茶屋&千夏字幕组&LoliHouse] 轻旅轻营△ SEASON2 / 摇曳露营△ SEASON2 / Yuru Camp S2 - 07 [WebRip 1920x1080 HEVC-10bit AAC][简繁内封字幕] ",
  "description": "人人为我，我为人人，为了各位观众能快速下载，请使用uTorrent / qBittorrent等正规 BT 软件下载，并保持开机上传，谢谢~",
  "tags": ["动画", "摇曳露营", "世界第一"]
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `TorrentInfo`

**Comment**

Post a new torrent without uploading file.

By default, post will be hide until checked by administer.

### /torrent/update_torrent
**Type**: POST

**Request**

    - id: Int
    - title: String
    - description: Option<String>
    - tags: Option<Vec<String>(5)>

**Example**

```json
{
  "id": 114514,
  "title": " [SweetSub&LoliHouse] 奇蛋物语 / Wonder Egg Priority - 07 [WebRip 1080p HEVC-10bit AAC][简繁内封字幕] ",
  "description": "人人为我，我为人人，为了各位观众能快速下载，请使用uTorrent / qBittorrent等正规 BT 软件下载，并保持开机上传，谢谢~",
  "tags": ["动画", "魔法少女", "原创"]
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `TorrentInfo`

### /torrent/upload_torrent
**Type**: POST

**Request**
Form data with following field:

    - id: String
    - torrent file

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Default success `GeneralResponse`

**Comment**
Only one torrent will be accepted.

### /torrent/hot_tags
**Type**: GET

**Request**

    - num: Option<Int>(>= 0)

**Example**

```
https://localhost:8000/torrent/hot_tags?num=20
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a list of `Tag`

### /torrent/list_torrents
**Type**: GET

**Request**

    - page: Option<Int>(>= 0)
    - tags: Option<Vec<String>>

**Example**

```
https://localhost:8000/torrent/list_torrents?page=0&tags[]=电影&tags[]=新浪潮
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with page count and a list of `SlimTorrent`(`DataWithCount`)

### /torrent/list_posted_torrent
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a list of `TorrentInfo`

**Comment**

List all torrents posted by current user.

### /torrent/show_torrent
**Type**: GET

**Request**

    - id: Int

**Example**

```
https://localhost:8000/torrent/show_torrent?id=1919810
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `TorrentInfo`

### /torrent/get_torrent
**Type**: GET

**Request**

    - id: Int

**Example**

```
https://localhost:8000/torrent/get_torrent?id=114514
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: multipart file