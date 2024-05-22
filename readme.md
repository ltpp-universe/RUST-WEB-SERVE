# 基于RUST的WEB资源服务器

> 该项目于2024年5月1日开始开发

## 预期功能

| 功能                                       | 支持情况 | 当前情况 |
|--------------------------------------------|----------|----------|
| 多线程支持                                  | 是       | 是       |
| 服务支持配置化                              | 是       | 是       |
| 防盗链支持                                  | 是       | 否       |
| gzip支持                                   | 是       | 否       |
| SSL支持                                    | 是       | 否       |
| 反向代理支持                                | 是       | 否       |
| 自定义状态码对应资源文件                     | 是       | 是       |
| 日志支持                                    | 是       | 是       |
| 负载均衡支持                                | 是       | 否       |
| 域名绑定解析支持                             | 是       | 是       |
| 资源解析                                    | 是       | 是       |
| history模式支持                             | 是       | 是       |
| 自定义响应头                                | 是       | 是       |

## 目前进度

* 自定义响应头
* 多线程支持
* 配置化
* 日志输出 
* 资源解析
* history模式支持
* 域名绑定支持
* 一个端口对应多域名支持

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
            "bind_server_name": {
                "127.0.0.1": {
                    "root_path": "./",
                    "log_dir_path": "./logs",
                    "ssl_certificate_path": "./ssl/certificate.crt",
                    "ssl_certificate_key_path": "./ssl/certificate.key",
                    "empty_path_try_files_path": "./index.html",
                    "response_header": "Server: RUST-WEB-SERVER\r\nGit: https://git.ltpp.vip/root/RUST-WEB-SERVE.git"
                },
                "localhost": {
                    "root_path": "./",
                    "log_dir_path": "./logs",
                    "ssl_certificate_path": "./ssl/certificate.crt",
                    "ssl_certificate_key_path": "./ssl/certificate.key",
                    "empty_path_try_files_path": "./index.html",
                    "response_header": "Server: RUST-WEB-SERVER\r\nGit: https://git.ltpp.vip/root/RUST-WEB-SERVE.git"
                }
            }
        },
        {
            "listen_ip": "127.0.0.1",
            "listen_port": 81,
            "buffer_size": 1024,
            "bind_server_name": {
                "127.0.0.1": {
                    "root_path": "./",
                    "log_dir_path": "./logs",
                    "ssl_certificate_path": "./ssl/certificate.crt",
                    "ssl_certificate_key_path": "./ssl/certificate.key",
                    "empty_path_try_files_path": "./index.html",
                    "response_header": "Server: RUST-WEB-SERVER\r\nGit: https://git.ltpp.vip/root/RUST-WEB-SERVE.git"
                }
            }
        }
    ]
}
```

> 以上配置将当前目录作为访问地址的根目录，
> 监听了80和81端口，绑定IP和域名为127.0.0.1和localhost用来处理请求，
> 当访问为空则重写路径到当前目录的index.html（适用于Vue等打包后的资源），
> 日志保存在当前目录logs下

> PS:listen_ip为服务端IP, bind_server_name下的key为域名或者IP, 一般listen_ip为127.0.0.1，
> 如果bind_server_name配置了localhost域名，那么可以使用localhost访问，
> 但是不支持127.0.0.1，如需支持127.0.0.1，在bind_server_name中添加即可
> 如果本地做了映射，需要同时添加映射的域名和127.0.0.1
> 配置如下：

```json
"bind_server_name": {
    "localhost": {
        "root_path": "./",
        "log_dir_path": "./logs",
        "ssl_certificate_path": "./ssl/certificate.crt",
        "ssl_certificate_key_path": "./ssl/certificate.key",
        "empty_path_try_files_path": "./index.html",
        "response_header": "Server: RUST-WEB-SERVER\r\nGit: https://git.ltpp.vip/root/RUST-WEB-SERVE.git"
    },
    "127.0.0.1": {
        "root_path": "./",
        "log_dir_path": "./logs",
        "ssl_certificate_path": "./ssl/certificate.crt",
        "ssl_certificate_key_path": "./ssl/certificate.key",
        "empty_path_try_files_path": "./index.html",
        "response_header": "Server: RUST-WEB-SERVER\r\nGit: https://git.ltpp.vip/root/RUST-WEB-SERVE.git"
    },
    "test.com": {
        "root_path": "./",
        "log_dir_path": "./logs",
        "ssl_certificate_path": "./ssl/certificate.crt",
        "ssl_certificate_key_path": "./ssl/certificate.key",
        "empty_path_try_files_path": "./index.html",
        "response_header": "Server: RUST-WEB-SERVER\r\nGit: https://git.ltpp.vip/root/RUST-WEB-SERVE.git"
    }
}
```

## 日志

> 支持配置，日期和完整请求记录

```php
[2024-05-22 10:33:49]
HttpRequest {
    method: "GET",
    path: "/_static/out/browser/serviceWorker.js",
    headers: {
        "Host": "127.0.0.1",
        "Service-Worker": "script",
        "Sec-Fetch-Site": "same-origin",
        "Sec-Fetch-Mode": "same-origin",
        "Referer": "http://127.0.0.1/_static/out/browser/serviceWorker.js",
        "Accept-Language": "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
        "Connection": "keep-alive",
        "Cookie": "SameSite=Lax; Hm_lvt_9793f42b498361373512340937deb2a0=1689155374; _ga=GA1.1.39230079.1707025003; _ga_69MPZE94D5=GS1.1.1707025002.1.1.1707026740.0.0.0; pmaUser-1=kQPDRgPaTO%2FrEE7aszZUy7I1J297glrDTv3jOeSXCxxCLe2kFFUpFi4%2FdHo%3D",
        "Cache-Control": "max-age=0",
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
        "Accept": "*/*",
        "Sec-Fetch-Dest": "serviceworker",
        "Accept-Encoding": "gzip, deflate, br, zstd",
    },
    body: {
        "Sec-Fetch-Site": [
            "same-origin",
        ],
        "Host": [
            "127.0.0.1",
        ],
        "Accept-Encoding": [
            "gzip, deflate, br, zstd",
        ],
        "Service-Worker": [
            "script",
        ],
        "Accept": [
            "*/*",
        ],
        "Referer": [
            "http://127.0.0.1/_static/out/browser/serviceWorker.js",
        ],
        "Sec-Fetch-Mode": [
            "same-origin",
        ],
        "Cookie": [
            "SameSite=Lax; Hm_lvt_9793f42b498361373512340937deb2a0=1689155374; _ga=GA1.1.39230079.1707025003; _ga_69MPZE94D5=GS1.1.1707025002.1.1.1707026740.0.0.0; pmaUser-1=kQPDRgPaTO%2FrEE7aszZUy7I1J297glrDTv3jOeSXCxxCLe2kFFUpFi4%2FdHo%3D",
        ],
        "Sec-Fetch-Dest": [
            "serviceworker",
        ],
        "Cache-Control": [
            "max-age=0",
        ],
        "User-Agent": [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
        ],
        "Accept-Language": [
            "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
        ],
        "Connection": [
            "keep-alive",
        ],
    },
}

[2024-05-22 10:33:49]
"Resource load fail:./_static/out/browser/serviceWorker.js"

```

## 实际效果

![alt text](markdown-images/image.png)

### 控制台输出

![alt text](markdown-images/image-1.png)

### 404

![alt text](markdown-images/image-2.png)
