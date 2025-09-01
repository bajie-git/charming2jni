# 概要
该项目主要依赖于charming项目，将charming部分api封装成jni方便javaer使用。

# 关于libc版本
项目使用github actions构建的，动态链接库依赖于libc版本的影响，可能不通用。如果你构建出来了某个平台某个libc版本的链接库欢迎提交pr 方便其他人使用。

动态链接库存放在 artifact 目录中


该版本
deno_core 0.311  变  0.354
serde_v8  0.263  变  0.220

导致linux上无法编译