## **Day01**
### **Rust历史了解**
#### **有趣的开发经历**
在 2006 年的某天，作者工作到精疲力尽后，本想回公寓享受下生活，结果发现电梯的程序出 Bug 崩溃了，要知道在国外，修理工可不像在中国那样随时待岗，还要知道，他家在 20 多楼！最后，他选择了妥协，去酒店待几天等待电梯的修理。

当然，一般人可能就这样算了，毕竟忍几天就过去了嘛。但是这名伟大的程序员显然也不是一般人，他面对害他流离失所的电梯拿起了屠龙宝刀 - Rust。

自此，劈开一个全新的编程世界。
#### **Rust效率**
##### 学习效率
通过了解，Rust的学习曲线不可谓不陡峭。

Rust 之难，不在于语言特性，这些都可以很容易学到，而在于：
- 实践中如何融会贯通的运用
- 遇到了坑时（生命周期、借用错误，自引用等）如何迅速、正确的解决
+ 大量的标准库方法记忆及熟练使用，这些是保证开发效率的关键
心智负担较重，特别是初中级阶段
##### 运行效率
得益于各种零开销抽象、深入到底层的优化潜力、优质的标准库和第三方库实现，Rust 具备非常优秀的性能，和 C、C++ 是 一个级别。

同时 Rust 有一个极大的优点：只要按照正确的方式使用 Rust，无需性能优化，就能有非常优秀的表现，不可谓不惊艳。
##### 开发效率
Rust 的开发效率可以用先抑后扬来形容。

当你熟悉各种标准库、生命周期和所有权的常用解决方法，乃至形成肌肉记忆时，开发效率将大大提升，可以写出高质量的原生代码了。
#### **总结**
连续 6 年最受欢迎的语言当然不是浪得虚名。 无 GC、效率高、工程性强、强安全性以及能同时得到工程派和学院派认可，这些令 Rust 拥有了自己的特色和生存空间。社区的友善，生态的快速发展，大公司的重仓跟进，一切的一切都在说明 Rust 的璀璨未来。

#### **学习过程中的建议**
- 要提前做好会遇到困难的准备，因为如上所说，学习 Rust 不仅仅是在学习一门编程语言
- 不要抱着试一试的心态去试一试，否则是浪费时间和消耗学习激情，作为连续七年荣获全世界最受喜欢桂冠的语言，Rust 不仅仅是值得试一试 :)
- 深入学习一本好书或教程

历经九九八十一难，立地成大佬。(哈哈😀，作者真是十分有趣)


### **关于Rust的安装和配置**
#### **Windows下安装**
因为个人使用windows系统，所以只记录windows下的rust安装，且并非标准。

Windows 上安装 Rust 需要有 C++ 环境，以下为安装的两种方式：

**1. x86_64-pc-windows-msvc（官方推荐）**
   
先安装 `Microsoft C++ Build Tools`，勾选安装 C++ 环境即可。安装时可自行修改缓存路径与安装路径。安装完成后，Rust 所需的 msvc 命令行程序需要手动添加到环境变量中，否则安装 Rust 时 rustup-init 会提示未安装 Microsoft C++ Build Tools，其位于：`%Visual Studio 安装位置%\VC\Tools\MSVC\%version%\bin\Hostx64\x64`（请自行替换其中的 %Visual Studio 安装位置%、%version% 字段）下。

如果你不想这么做，可以选择安装 Microsoft C++ Build Tools 新增的“定制”终端 `Developer Command Prompt for %Visual Studio version%` 或 `Developer PowerShell for %Visual Studio version%`，在其中运行 `rustup-init.exe`。

准备好 C++ 环境后开始安装 Rust：

在 RUSTUP-INIT 下载系统相对应的 Rust 安装程序，一路默认即可。
```
PS C:\Users\Hehongyuan> rustup-init.exe
......
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```
#### **Linux**
Linux环境为VMware虚拟机建立虚拟环境，其中磁盘映像由`rCore-Tutorial-Book-v3`实验文档所提供,其中Rust以及C语言相关环境已经配置完全。故没有了解其安装方式，后续若有需要会学习。
#### **更新和卸载**
检查是否安装成功

```
$ rustc -V
rustc 1.72.1 (d5c2e9c34 2023-09-13)
$ cargo -V
rustc 1.72.1 (d5c2e9c34 2023-09-13)
```
更新

`$ rustup update`

卸载

`$ rustup self uninstall`
