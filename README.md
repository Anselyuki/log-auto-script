# 自动日志框架

你是否遇到领导想要你每天写具体的工作日志，但是你又不是那么想写日志呢

如果你平日里写 Git commit 遵守一定的规范,在每次提交内都会简单描述你的工作内容,那么这个工具就适合你自动化生成日志呢

使用大模型去提炼你的提交信息,然后生成日志,这样你就不用每天写日志了

## 支持大模型列表

- [x] 通义千问
- [ ] 文心一言

## 初始化设置与配置文件

下载 Release 中的可执行文件，然后运行即可,首次使用需要进行一些配置

### 默认配置文件

创建默认的配置文件,下述两条命令等价

```shell
./log_auto_script config --default
./log_auto_script config -d
```

配置文件的路径为`$HOME/.auto-log.yaml`,该配置文件在 MacOS 下遵守 XDG 规范,即创建的配置文件夹为
`~/.config/log-auto-script`

配置文件结构如下

```yaml
repo_path:
  - C:\Users\AnselYuki\project_1
  - C:\Users\AnselYuki\project_2
branches:
  - master
  - develop
model_type: QianWen
authors:
  - anselyuki
  - admin
authorization: <model token>
```

其中需要填写的字段有

| 字段名        | 字段含义       | 取值示例               | 描述                |
|------------|------------|--------------------|-------------------|
| repo_path  | 代码仓库路径     | 见示例,注意各个平台路径差别     | 对多个仓库执行了提交可以均填写至此 |
| branches   | 代码仓库分支     | `master`,`develop` |                   |
| model_type | 大模型选项      | `QianWen`          | 目前仅支持通义千问         |
| authors    | 你的 Git 用户名 | `anselyuki`        | 多个提交别名在此配置        |

### 打开配置文件

使用 [open](https://docs.rs/open/2.1.3/open) 这个 create 提供的能力实现使用本地默认编辑器打开配置文件

> **Open**: Use this library to open a path or URL using the program configured on the system

```shell
log_auto_script config
```

如果没有配置默认编辑器,系统则会提示选择一个编辑器打开配置文件

# TODO

对接常见的日志系统,我这边用的轻流和禅道,所以会优先对接这两个系统