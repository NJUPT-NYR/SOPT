# SOPT API Docs

## General Data

### User

    - id: Int
    - email: String
    - username: String
    - password: String
    - passkey: String

### Invitation

    - id: Int
    - sender: String
    - code: String
    # this is email address
    - send_to: String
    - is_used: Boolean

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
1. Error: Human Readable String
2. Success: a single `User`

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
1. Error: Human Readable String
2. Success: Http 200

### /user/logout
**Type**: GET

**Response**
1. Success: Http 200

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
1. Error: Human Readable String
2. Success: Http 200

**Comment**

This api is used for check current user's identity when it comes to some dangerous
actions like `reset_password` and `reset_passkey`. We will store the passed identity
for 5 minutes, then user can perform actions like a charm.

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
1. Error: Human Readable String
2. Success: A single `User`

### /user/auth/reset_passkey
**Type**: GET

**Response**
1. Error: Human Readable String
2. Success: A single `User`

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
1. Error: Human Readable String
2. Success: a single `Invitation`

### /invitation/list_invitations
**Type** GET

**Response**
1. Error: Human Readable String
2. Success: a list of `Invitation`s

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
1. Error: Human Readable String
2. Success: a single `TorrentInfo`

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
1. Error: Human Readable String
2. Success: a single `TorrentInfo`

### /torrent/list_torrents
**Type**: GET

**Request**

    - tags: Option<Vec<String>>

**Example**

```
    https://localhost:8000/torrent/list_torrents?tags=电影,新浪潮
```

**Response**
1. Error: Human Readable String
2. Success: a list of `SlimTorrent`

### /torrent/list_posted_torrent
**Type**: GET

**Response**
1. Error: Human Readable String
2. Success: a list of `TorrentInfo`

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
1. Error: Human Readable String
2. Success: a single `TorrentInfo`