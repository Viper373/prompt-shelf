---
title: PromptShelf
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.30"

---

# PromptShelf

Base URLs:

# Authentication

# Default

## GET 状态查询

GET /status

> 返回示例

> 200 Response

```json
{
  "status": "string",
  "uptime_seconds": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|string|true|none||none|
|» uptime_seconds|integer|true|none||none|

## POST 注册

POST /user/signup

> Body 请求参数

```json
{
  "email": "string",
  "password": "string",
  "username": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» email|body|string| 是 |none|
|» password|body|string| 是 |none|
|» username|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "email": "string",
    "id": 0,
    "role": "string",
    "token": "string",
    "username": "string"
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» email|string|true|none||none|
|»» id|integer|true|none||none|
|»» role|string|true|none||none|
|»» token|string|true|none||none|
|»» username|string|true|none||none|
|» status|string|true|none||none|

## POST 登录

POST /user/signin

> Body 请求参数

```json
{
  "email": "string",
  "password": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» email|body|string| 是 |none|
|» password|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "email": "string",
    "id": 0,
    "role": "string",
    "token": "string",
    "username": "string"
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» email|string|true|none||none|
|»» id|integer|true|none||none|
|»» role|string|true|none||none|
|»» token|string|true|none||none|
|»» username|string|true|none||none|
|» status|string|true|none||none|

## POST 创建提示词/项目

POST /prompt/create_prompt

> Body 请求参数

```json
{
  "name": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» name|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "id": 0
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» id|integer|true|none||none|
|» status|string|true|none||none|

## POST 创建节点/版本

POST /prompt/create_node

> Body 请求参数

```json
{
  "prompt_id": 0,
  "version": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» prompt_id|body|number| 是 |none|
|» version|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": null,
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|null|true|none||none|
|» status|string|true|none||none|

## POST 创建提交

POST /prompt/create_commit

> Body 请求参数

```json
{
  "prompt_id": 0,
  "version": "string",
  "desp": "string",
  "content": "string",
  "as_latest": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» prompt_id|body|number| 是 |none|
|» version|body|string| 是 |none|
|» desp|body|string| 是 |none|
|» content|body|string| 是 |none|
|» as_latest|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "commit_id": "string"
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» commit_id|string|true|none||none|
|» status|string|true|none||none|

## GET 查询prompt

GET /prompt/query

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|id|query|number| 否 |ID 编号|
|Authorization|header|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": [
    {
      "created_at": "string",
      "id": 0,
      "latest_commit": "string",
      "latest_version": "string",
      "org_id": null,
      "prompt": {
        "id": "string",
        "name": "string",
        "nodes": [
          {
            "commits": null,
            "updated_at": null,
            "version": null
          }
        ]
      },
      "updated_at": "string",
      "user_id": 0
    }
  ],
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|[object]|true|none||none|
|»» created_at|string|false|none||none|
|»» id|integer|false|none||none|
|»» latest_commit|string|false|none||none|
|»» latest_version|string|false|none||none|
|»» org_id|null|false|none||none|
|»» prompt|object|false|none||none|
|»»» id|string|true|none||none|
|»»» name|string|true|none||none|
|»»» nodes|[object]|true|none||none|
|»»»» commits|[object]|true|none||none|
|»»»»» author|string|true|none||none|
|»»»»» commit_id|string|true|none||none|
|»»»»» created_at|string|true|none||none|
|»»»»» desp|string|true|none||none|
|»»»» updated_at|string|true|none||none|
|»»»» version|string|true|none||none|
|»» updated_at|string|false|none||none|
|»» user_id|integer|false|none||none|
|» status|string|true|none||none|

## DELETE 删除prompt

DELETE /prompt

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|id|query|number| 是 |ID 编号|
|Authorization|header|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "id": 0
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» id|integer|true|none||none|
|» status|string|true|none||none|

## GET 获取最新提交

GET /prompt/latest

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|id|query|number| 是 |ID 编号|
|Authorization|header|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "commit": {
      "author": "string",
      "commit_id": "string",
      "created_at": "string",
      "desp": "string"
    },
    "content": "string"
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» commit|object|true|none||none|
|»»» author|string|true|none||none|
|»»» commit_id|string|true|none||none|
|»»» created_at|string|true|none||none|
|»»» desp|string|true|none||none|
|»» content|string|true|none||none|
|» status|string|true|none||none|

## GET 获取指定commit的内容

GET /prompt/content

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|prompt_id|query|number| 是 |none|
|version|query|string| 是 |none|
|commit_id|query|string| 是 |none|
|Authorization|header|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": "string",
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|string|true|none||none|
|» status|string|true|none||none|

## POST 启用/关闭注册

POST /control/register

> Body 请求参数

```json
{
  "enable_register": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» enable_register|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": null,
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|null|true|none||none|
|» status|string|true|none||none|

## GET 获取用户列表

GET /control/list/user

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": [
    {
      "created_at": "string",
      "email": "string",
      "id": 0,
      "role": "string",
      "updated_at": "string",
      "username": "string",
      "valid": true
    }
  ],
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|[object]|true|none||none|
|»» created_at|string|true|none||none|
|»» email|string|true|none||none|
|»» id|integer|true|none||none|
|»» role|string|true|none||none|
|»» updated_at|string|true|none||none|
|»» username|string|true|none||none|
|»» valid|boolean|true|none||none|
|» status|string|true|none||none|

## POST 版本回退

POST /prompt/rollback

> Body 请求参数

```json
{
  "prompt_id": 0,
  "version": "string",
  "commit_id": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» prompt_id|body|number| 是 |none|
|» version|body|string| 是 |none|
|» commit_id|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "id": 0
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» id|integer|true|none||none|
|» status|string|true|none||none|

## POST 回退至上一次提交

POST /prompt/revert

> Body 请求参数

```json
{
  "prompt_id": 0
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» prompt_id|body|number| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": {
    "id": 0
  },
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|object|true|none||none|
|»» id|integer|true|none||none|
|» status|string|true|none||none|

## POST 启用/禁用用户

POST /control/disable/user

> Body 请求参数

```json
{
  "user_id": 0,
  "disable": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|Authorization|header|string| 是 |none|
|body|body|object| 否 |none|
|» user_id|body|number| 是 |none|
|» disable|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": null,
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|null|true|none||none|
|» status|string|true|none||none|

## DELETE 删除用户

DELETE /control/user/{user_id}

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|user_id|path|string| 是 |none|
|Authorization|header|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "msg": "string",
  "result": null,
  "status": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» msg|string|true|none||none|
|» result|null|true|none||none|
|» status|string|true|none||none|

# 数据模型

