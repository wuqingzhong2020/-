
## ubuntu 环境下安装QT
https://blog.csdn.net/zvui_/article/details/108214959


# 镜像源 
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'sjtu'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"
```

# 1.Win11安装Ubuntu子系统
https://blog.csdn.net/Tim_Cookerr/article/details/127189008

## 去微软应用下载
terminal 
ubuntu22.04.3
    // 不知道怎么下载qt6
    ubuntu24.04 就没有意义， 不用下载 
    // 最后下载的版本是5.15.3


## 更改镜像源
```

sudo sed -i "s@http://.*archive.ubuntu.com@https://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list
sudo sed -i "s@http://.*security.ubuntu.com@https://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list

sudo apt update
sudo apt upgrade 

```

## 安装c++ 编译开发环境
```
sudo apt install vim
sudo apt install cmake 
sudo apt intall sqlite3
# 图形化下载
sudo apt install sqlitebrowser
# 编译器
sudo apt-get install build-essentia
```

## ubuntu 环境下安装QT
```
https://blog.csdn.net/zvui_/article/details/108214959
sudo apt-get install build-essential
sudo apt-get install qtbase5-dev qtchooser qt5-qmake qtbase5-dev-tools
sudo apt-get install qtcreator  # 编辑软件
sudo apt-get install qt5*

# 怎么下载qt6
```

## 在使用 sqlite 时遇到的奇怪问题的正解
https://blog.csdn.net/m0_47505062/article/details/138044982
以前遇到的问题是test.db放在有权限限制的目录下， 权限被限制
图形软件自发在某个系统目录下复制该test.db， 
以后我们打开文件是， 默认用复制的的文件 // Note:因此产生一系列的问题


## 如果将windows磁盘挂载在ubuntu上面
默认是挂载的在 /mnt/ 下面 


