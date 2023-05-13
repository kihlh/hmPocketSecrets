### 
### hm神秘口袋

<img src ="https://cos.kiic.top//202305140512647.webp?imageMogr2/quality/90/format/webp" style="zoom:50%;" ></img>![image-20230513210848900](https://cos.kiic.top//202305140512835.webp?imageMogr2/quality/90/format/webp)





-----------

#### 功能简介:

神秘口袋是一个全新的大饼项目，由于出于兼容性考虑，打算使用低级语言进行编写 ，此开源项目可以提前提出issues，抛砖引玉等待大家来讨论

功能非常复杂但也非常简单，软件的初衷就是用来摸鱼的，当前已经有个简单的框架

- **构建畅想**

  - 神秘口袋的主进程使用C++ 加Rust 的结合运行  所以界面非常节省内存和CPU
    - 界面测试得 3.7 MB

  - 界面设置要丰富 所以选择使用 web + tcp的方式进行设置 

  - 安全很重要，用户配置将使用AES256进行加密 开源的代码的公匙只用于开源，正式二进制中将会使用其他的公匙，以此防止被篡改

  - 密码设置只能在Rust的主进程进行 而且用户输入完成必须马上盐化

  - 用户密码加密使用  公匙+ 用户密码 合并进行哈希盐化

  - 界面灵感来自于 很棒的软件 《FxSound》

  - 退出和修改密码和程序设置应当在主进程中调用 输入主密码盐化对比成功才启用

- **摸鱼护航**

  - 当鼠标移开指定的微信对话框   例如备注名称带有*号
    - 透明度变成0（窗口不可见）
  - 鼠标移入
    - 透明度变成XX  比如50的透明度 1.5米后看不到

- **隐私保护**.

  - 将一个USB设备作为密匙 （例如一个U盘）
    - **拔出**
      1. 禁用电脑设备的键盘鼠标
      2. 自定义时间后关闭显示器
      3. 尝试非密匙手段解锁则杀死所有相关进程
      4. 
    - **连接**
      	1. 开机后第一次接入则 启动

  
---------------------

#### 程序界面畅想

![image-20230514051559726](https://cos.kiic.top//202305140515799.png?imageMogr2/quality/90/format/webp)
------------------

#### 编程语言：

Rust ，C++ ， HTML

##### 引用的程序包 排名不区分前后

- Rust

  |     库名称     |                           授权协议                           |           用途            |
  | :------------: | :----------------------------------------------------------: | :-----------------------: |
  |   hm神秘口袋   |                             MIT                              | 如果引用了GPL将可能会变更 |
  |    **fltk**    |        [MIT](https://choosealicense.com/licenses/mit)        |           界面            |
  |    fl2rust     |        [MIT](https://choosealicense.com/licenses/mit)        |         fltk辅助          |
  |   **regex**    | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |           界面            |
  | **serde_json** | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |         数据处理          |
  |     winres     |        [MIT](https://choosealicense.com/licenses/mit)        |     程序提权(管理员)      |
  |     winapi     | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |        系统API调用        |
  |   clipboard    | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |          剪贴板           |
  |  magic-crypt   | [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |  用户配置内容加密 AES256  |
  |   heroicons    | [ MIT license](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE) |           图标            |
  |                |                                                              |                           |
  |                |                                                              |                           |
  |                |                                                              |                           |
  |                |                                                              |                           |
  
  

