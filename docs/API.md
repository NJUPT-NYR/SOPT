# SOPT API Docs

* [General Data](#general-data)
  * [GeneralResponse](#generalresponse)
  * [DataWithCount](#datawithcount)
* [Admin](#admin-api)
  * [/api/admin/torrent/accept_torrents](#apiadmintorrentaccept_torrents)
  * [/api/admin/torrent/stick_torrents](#apiadmintorrentstick_torrents)
  * [/api/admin/torrent/free_torrents](#apiadmintorrentfree_torrents)
  * [/api/admin/torrent/show_invisible_torrents](#apiadmintorrentshow_invisible_torrents)
  * [/api/admin/user/ban_user](#apiadminuserban_user)
  * [/api/admin/user/unban_user](#apiadminuserunban_user)
  * [/api/admin/user/list_banned_user](#apiadminuserlist_banned_user)
  * [/api/admin/user/group_awards](#apiadminusergroup_awards)
  * [/api/admin/user/change_permission](#apiadminuserchange_permission)
  * [/api/admin/site/get_email_whitelist](#apiadminsiteget_email_whitelist)
  * [/api/admin/site/update_email_whitelist](#apiadminsiteupdate_email_whitelist)
  * [/api/admin/site/get_rank](#apiadminsiteget_rank)
  * [/api/admin/site/update_rank](#apiadminsiteupdate_rank)
* [Invitation](#invitation-api)
  * [/api/invitation/send_invitation](#apiinvitationsend_invitation)
  * [/api/invitation/list_invitations](#apiinvitationlist_invitations)
* [Torrent](#torrent-api)
  * [/api/torrent/add_torrent](#apitorrentadd_torrent)
  * [/api/torrent/update_torrent](#apitorrentupdate_torrent)
  * [/api/torrent/hot_tags](#apitorrenthot_tags)
  * [/api/torrent/list_torrents](#apitorrentlist_torrents)
  * [/api/torrent/search_torrents](#apitorrentsearch_torrents)
  * [/api/torrent/show_torrent](#apitorrentshow_torrent)
  * [/api/torrent/list_posted_torrent](#apitorrentlist_posted_torrent)
  * [/api/torrent/upload_torrent](#apitorrentupload_torrent)
  * [/api/torrent/get_torrent](#apitorrentget_torrent)
* [Tracker](#tracker-api)
  * [/api/tracker/get_announce](#apitrackerget_announce)
* [User](#user-api)
  * [/api/user/add_user](#apiuseradd_user)
  * [/api/user/login](#apiuserlogin)
  * [/api/user/personal_info_update](#apiuserpersonal_info_update)
  * [/api/user/upload_avatar](#apiuserupload_avatar)
  * [/api/user/show_user](#apiusershow_user)
  * [/api/user/show_torrent_status](#apiusershow_torrent_status)
  * [/api/user/auth/reset_password](#apiuserauthreset_password)
  * [/api/user/auth/reset_passkey](#apiuserauthreset_passkey)
  * [/api/user/auth/transfer_money](#apiuserauthtransfer_money)
* [Role Design](#role-design)
* [Response Data](#response-data)
  * [TorrentId](#torrentid)
  * [SlimTorrent](#slimtorrent)
  * [Full Torrent](#full-torrent)
  * [Tag](#tag)
  * [PersonalTorrent](#personaltorrent)
  * [Account](#account)
  * [User](#user)
  * [Invitation](#invitation)
  * [Rank](#rank)

## General Data

### GeneralResponse

    - data: Json
    - success: Boolean
    - errMsg: String

### DataWithCount

    - count: Int
    - ret: Json

## Admin API

### /api/admin/torrent/accept_torrents
**Type**: POST

**Request**

    - ids: Vec<i64>

**Example**

```json
{
  "ids": [114, 514, 1919810]
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Accept torrents, or in another word, make them visible to ordinary
users. 

Only user with torrent admin role can access.

### /api/admin/torrent/stick_torrents
**Type**: POST

**Request**

    - ids: Vec<i64>

**Example**

```json
{
  "ids": [114, 514, 1919810]
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Make a list of torrents stick. Stick torrents will be displayed at the top
torrent list page no matter of sorting.

By default, you can add infinite number of stick torrents, 
but we recommend the number is less than 20.

Only user with torrent admin role can access.

### /api/admin/torrent/free_torrents
**Type**: POST

**Request**

    - ids: Vec<i64>

**Example**

```json
{
  "ids": [114, 514, 1919810]
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Make a list of torrents free.

Only user with torrent admin role can access.

### /api/admin/torrent/show_invisible_torrents
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `SlimTorrent`

**Comment**

List all torrents that is invisible to users.

Only user with torrent admin role can access.

### /api/admin/user/ban_user
**Type**: GET

**Request**

    - id: i64

**Example**

```
http://localhost:8000/api/admin/ban_user?id=114
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Ban definite user, just delete role bit 0.

Only user with user admin role can access.

### /api/admin/user/unban_user
**Type**: GET

**Request**

    - id: i64

**Example**

```
http://localhost:8000/api/admin/unban_user?id=114
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Unban definite user, just add role bit 0.

Only user with user admin role can access.

### /api/admin/user/list_banned_user
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `Account`

**Comment**

List all users current banned.

Only user with user admin role can access.

### /api/admin/user/group_awards
**Type**: POST

**Request**

    - ids: Vec<i64>
    - amount: f64

**Example**

```json
{
  "ids": [114, 514],
  "amount": 1919.810
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Give(or take if minus) a group of users some money.

Only user with user admin role can access.

### /api/admin/user/change_permission
**Type**: POST

**Request**

    - id: i64
    - give: Vec<i32>
    - take: Vec<i32>

**Example**

```json
{
  "id": 1919810,
  "give": [62, 61],
  "take": []
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Give(and take away) someone definite permissions. See
`role design` at this page for more information.

Only super user can access.

### /api/admin/site/get_email_whitelist
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `String`

**Comment**

Return current email whitelist(no invitation code is needed).

Only user with site admin role can access.

### /api/admin/site/update_email_whitelist
**Type**: POST

**Request**

    - add: Vec<String>
    - delete: Vec<String>

**Example**

```json
{
  "add": ["nju.edu.cn", "uni-leipzig.de"],
  "delete": ["gmail.com"]
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Update current email whitelist. It is strongly discouraged since
maybe block. You can reload filtered email via reboot server

Only user with site admin role can access.

### /api/admin/site/get_rank
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `Rank`

**Comment**

Return current rank definition.

Only user with site admin role can access.

### /api/admin/site/update_rank
**Type**: POST

**Request**

A `Rank` Struct.

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Update or Add new rank, you can edit everything about a rank.

Only user with site admin role can access.

## Invitation API

### /api/invitation/send_invitation
**Type**: POST

**Request**

    - to: String
    - address: String
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
2. Success: `GeneralResponse` with a single `Invitation`

**Comment**

Send invitation to someone. It will consume some money(default is 5000)
and send an email. If email sent is failed, you can still give out invitation
code manually.

Banned user or user without invitation permission role cannot access.

### /api/invitation/list_invitations
**Type** GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `Invitation`

**Comment**

List all invitations and usage current user sent before.

## Torrent API

### /api/torrent/add_torrent
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
2. Success: `GeneralResponse` with a single `TorrentId`

**Comment**

Post a new torrent without uploading file.
By default, post will be hide until checked by administer.

Banned user cannot access.

### /api/torrent/update_torrent
**Type**: POST

**Request**

    - id: i64
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
2. Success: `GeneralResponse` with a single `TorrentId`

**Comment**

Update existed torrent, it will be replaced but not append.

Only creator and user with torrent admin role can access.

### /api/torrent/hot_tags
**Type**: GET

**Request**

    - num: Option<usize>(>= 0)

**Example**

```
https://localhost:8000/torrent/hot_tags?num=20
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `Tag`

**Comment**

Return hottest tags with number wanted, default number is 10.

### /api/torrent/list_torrents
**Type**: GET

**Enum**
```rust
enum Sort {
    Title,
    Poster,
    LastEdit,
    Length,
    Downloading,
    Uploading,
    Finished,
}

enum SortType {
    ASC,
    DESC,
}
```

**Request**

    - page: Option<usize>(>= 0)
    - tags: Option<Vec<String>>
    - freeonly: bool
    - sort: Option<Sort>
    - type: Option<SortType>

**Example**

```
https://localhost:8000/torrent/list_torrents?page=0&tags[]=电影&tags[]=新浪潮&freeonly=false&sort=Uploading&type=DESC
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with page count and an array of `SlimTorrent`(`DataWithCount`)

**Comment**

List torrents with options.
1. `page` : control the pagination, we can now display 20 torrents in a page.
2. `tags` : arbitrary tags filtering the torrents
3. `freeonly`: only show free torrents
4. `sort`: Sort torrents with fields supplied
5. `type`: increment or decrement

Stick torrents will always appear at first.

### /api/torrent/search_torrents
**Type**: GET

**Enum**
```rust
enum Sort {
    Title,
    Poster,
    LastEdit,
    Length,
    Downloading,
    Uploading,
    Finished,
}

enum SortType {
    ASC,
    DESC,
}
```

**Request**

    - page: Option<usize>(>= 0)
    - keywords: Vec<String>
    - freeonly: bool
    - sort: Option<Sort>
    - type: Option<SortType>

**Example**

```
https://localhost:8000/torrent/search_torrents?page=0&keywords[]=回转企鹅罐&freeonly=false&sort=Uploading&type=DESC
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `SlimTorrent`

**Comment**

Search torrents with options.
1. `page` : control the pagination, we can now display 20 torrents in a page.
2. `keywords` : arbitrary keywords, just do what you do on Google.
3. `freeonly`: only show free torrents
4. `sort`: Sort torrents with fields supplied
5. `type`: increment or decrement

### /api/torrent/show_torrent
**Type**: GET

**Request**

    - id: i64

**Example**

```
https://localhost:8000/torrent/show_torrent?id=1919810
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `FullTorrent`

**Comment**
Show a torrent.

Invisible torrent can only be accessed by the creator or user with
torrent admin role.

### /api/torrent/list_posted_torrent
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with an array of `SlimTorrent`

**Comment**

List all torrents posted by current user.

### /api/torrent/upload_torrent
**Type**: POST

**Request**
Form data with following field:

    - id: String(will be pared into i64)
    - torrent file: Binary File

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**
Only one torrent will be accepted.

Only the creator and user with torrent admin role can upload.

### /api/torrent/get_torrent
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

**Comment**

Download torrents.

Invisible torrents can only be downloaded by the creator or
user with torrent admin role.

Banned user cannot download.

## Tracker API

### /api/tracker/get_announce
**Type**: GET

**Enum**
```rust
enum Action {
    Start = 0,
    Complete,
    Stop,
}
```

**Request**

    - uid: i64
    - tid: i64
    - download: i64
    - upload: i64
    - action: Option<Action>

**Example**
```
https://localhost:8000/api/tracker/get_announce?uid=114&tid=514&download=276212&upload=0
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Used by the tracker for announcing.

## User API

### /api/user/add_user
**Type**: POST

**Request**

    - email: String
    - username: String(50)
    - password: String
    - invite_code: Option<String>

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
2. Success: `GeneralResponse` with a single `Account`

**Comment**

Sign up. Default rank is first rank.

### /api/user/login
**Type**: POST

**Request**

    - username: String(50)
    - password: String

**Example**

```json
{
  "username": "brethland",
  "password": "password"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a string represents jwt token

**Comment**

Sign in. It will also check for rank up. Since role checking uses
jwt token so if you are updated on roles please re-login.

JWT expires in 3 days.

### /api/user/personal_info_update
**Type**: POST

**Request**

    - info: Json
    - privacy: i32

**Example**

```json
{
  "info": {
    "学校": "下北泽大学",
    "个人网站": "114514.com",
    "介绍": "24岁，是学生。"
  },
  "privacy": 0
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Update user defined json fields and privacy level.

Any key in info will be accepted and stored in Database without any change.

Privacy level, by default, is 0 which means everyone can see your profile.

### /api/user/upload_avatar
**Type**: POST

**Request**

Multipart Files

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Allowed file types are `JPG` `PNG` and `WebP`.

Avatar will be stored in the database with base64 encoded.


### /api/user/show_user
**Type**: GET

**Request**

    - username: String

**Example**
```
https://localhost:8000/api/user/show_user?username=brethland
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with a single `User`

**Comment**

Show definite user. 

If user set the privacy, and you are not the user with user admin role,
you cannot access.

Passkey can only be accessed by user himself.

### /api/user/show_torrent_status
**Type**: GET

**Request**

    - username: String

**Example**
```
https://localhost:8000/api/user/show_torrent_status?username=brethland
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: `GeneralResponse` with four lists of `PersonalTorrent`, which are
`downloading`, `uploading`, `finished` and `unfinished`.

**Comment**

Show definite user's torrent seeding status.

### /api/user/auth/reset_password
**Type**: POST

**Request**

    - passsword: String

**Example**

```json
{
  "password": "password"
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

### /api/user/auth/reset_passkey
**Type**: GET

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

### /api/user/auth/transfer_money
**Type**: POST

**Request**

    - to: String
    - amount: f64

**Example**

```json
{
  "to": "Tadokoro Koniji",
  "amount": 114514.1919
}
```

**Response**
1. Error: `GeneralResponse` with `errMsg`
2. Success: Empty `GeneralResponse`

**Comment**

Give away some money to users.

Banned user cannot access.

## Role Design
This is a map from bit to bool(or 01 string). 0 is unset and 1 is set.

0: ordinary user

1: can invite

...

60: site admin

61: user admin

62: torrent admin

63: super user

## Response Data

### TorrentId

    - id: i64
    - visible: bool

### SlimTorrent

    - id: i64
    - title: String
    - poster: String
    - tag: Option<Vec<String>>
    - lastEdit: String(DateTime)
    - length: i64(in byte)
    - free: bool
    - downloading: i32
    - uploading: i32
    - finished: i64

### Full Torrent

    - id: i64
    - title: String
    - poster: String
    - description: Option<String>
    - visible: bool
    - tag: Option<Vec<String>>
    - createTime: String(DateTime)
    - lastEdit: String(DateTime)
    - free: bool
    - downloading: i32
    - uploading: i32
    - finished: i64
    - length: i64(in byte)
    - files: Vec<String>
    - infohash: String

### Tag

    - name: String
    - amount: i32

### PersonalTorrent

    - id: i64
    - title: String
    - length: i64(in byte)
    - upload: i64(in byte)
    - download: i64(in byte)

### Account

    - id: i64
    - email: String
    - username: String
    - passkey: String
    - role: i64

### User

    - id: i64
    - username: String
    - registerTime: String(DateTime)
    - lastActivity: String(DateTime)
    - invitor: Option<String>
    - upload: i64(in byte)
    - download: i64(in byte)
    - money: f64
    - rank: String
    - avatar: Option<String>
    - other: Option<Json>
    - privacy: i32
    - email: String
    - passkey: String

### Invitation

    - sender: Option<String>
    - code: String
    - address: String
    - usage: bool

### Rank

    - id: i32
    - name: String
    - role: Vec<i16>
    - upload: i64(in byte)
    - age: i64(in second)
    - next: Option<i32>