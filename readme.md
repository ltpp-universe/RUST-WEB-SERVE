# 基于RUST的WEB资源服务器

> 该项目于2024年5月1日开始开发

## 预期功能

| 功能                                       | 支持情况 | 当前情况 |
|--------------------------------------------|----------|----------|
| 使用线程池处理请求                          | 是       | 否       |
| 服务支持配置化                              | 是       | 是       |
| 防盗链支持                                  | 是       | 否       |
| gzip支持                                   | 是       | 否       |
| SSL支持                                    | 是       | 否       |
| 反向代理支持                                | 是       | 否       |
| 自定义状态码对应资源文件                     | 是       | 是       |
| 日志支持                                    | 是       | 是       |
| 负载均衡支持                                | 是       | 否       |
| 域名绑定解析支持                             | 是       | 否       |
| 资源解析                                    | 是       | 是       |

## 目前进度

* 配置化
* 多线程处理请求
* 日志输出 
* 资源解析

## 错误页

* 在root_path下名称为对应状态码.html，例如404对应页面404.html

## JSON示例

> 首次运行会自动生成config.json配置文件，填写好后重新运行即可

```json
{
    "server": [
        {
            "listen_ip": "127.0.0.1",
            "listen_port": 80,
            "buffer_size": 1024,
            "root_path": "./",
            "log_dir_path": "./logs",
            "server_name": "localhost",
            "ssl_certificate_path": "./ssl/certificate.crt",
            "ssl_certificate_key_path": "./ssl/certificate.key",
            "empty_path_try_files_path": "./index.html"
        },
        {
            "listen_ip": "127.0.0.1",
            "listen_port": 81,
            "buffer_size": 1024,
            "root_path": "./",
            "log_dir_path": "./logs",
            "server_name": "localhost",
            "ssl_certificate_path": "./ssl/certificate.crt",
            "ssl_certificate_key_path": "./ssl/certificate.key",
            "empty_path_try_files_path": "./index.html"
        }
    ]
}
```

> 以上配置将当前目录作为访问地址的根目录，监听了80和81端口用来处理请求，当访问为空则重写路径到当前目录的index.html（适用于Vue等打包后的资源），日志保存在当前目录logs下
