# 概要
该项目主要依赖于charming项目，将charming部分api封装成jni方便javaer使用。

# 关于libc版本
项目使用github actions构建的，动态链接库依赖于libc版本的影响，可能不通用。如果你构建出来了某个平台某个libc版本的链接库欢迎提交pr 方便其他人使用。

动态链接库存放在 artifact 目录中


# 开发记录
charming 0.6 版本将：
  deno_core 0.311 升级 0.354
  serde_v8  0.220 升级 0.263
导致linux系统上无法编译为动态连结库，大概原因是deno tls 一些变动导致无法编译为动态库。如果使用需要给版本降级。

但 0.6 版本支持chart结构体反序列化了，无需增加 xx_by_json 方法。

当前可以编译，但运行时会报错，，，，