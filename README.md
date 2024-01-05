# postman-backup

对 `postman` 数据进行备份，按照 `workspace` 分组，备份 `collection` 和 `environment` 数据，备份的数据可以直接导入 `postman` 中，备份文件夹结构如下
```
backup
├── user_info.json
└── workspace
    ├── workspace_id11111
        ├── workspace_info.json
        ├── archive.json
        ├── collection
            ├── collection_id111111.json
            ├── collection_id222222.json
            └── collection_id333333.json
        └── environment
            ├── environment_id111111.json
            ├── environment_id222222.json
            └── environment_id333333.json
    └── workspace_id22222
        ├── workspace_info.json
        ├── archive.json
        ├── collection
            ├── collection_id111111.json
            ├── collection_id222222.json
            └── collection_id333333.json
        └── environment
            ├── environment_id111111.json
            ├── environment_id222222.json
            └── environment_id333333.json
```
## 使用

### Windows & Macos

1. 下载 [`Release`](https://github.com/NPCDW/postman-backup/releases)
2. 去 `postman` 控制台生成一个 `api_key`，[点击这里去生成](https://go.postman.co/settings/me/api-keys)
3. 设置环境变量 `POSTMAN_API_KEY` 为第一步得到的 `api_key`
4. 运行程序

### Docker

建议复制该 [`docker-compose.yml`](docker-compose.yml) 文件到服务器，并在同目录下运行
```bash
docker compose up
```