# 接口
## 1. 文件
- 获取文件列表 `/api/program` `get`
- 新建文件`/api/program` `post`
- 删除文件 `/api/program/:id` `delete`
- 编辑 `/api/program/:id` `put`
- 运行 `/api/program/run` `post`
- 获取参数配置 `JSON Schema` `/api/program/schema/:id` `get` 
## 2. 插件
- 获取、编辑插件配置`JSONSchema` `/api/plugin/schema/:id` `get post put`
- 实时生成插件文件 `/api/plugin/:id` `post`
- 下载插件文件 `/api/plugin/:id` `get`
- 删除插件 `/api/plugin/:id` `delete`

## 3. 协同