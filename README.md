### 
### hm神秘口袋

<img src="https://cos.kiic.top//202305210932792.png?imageMogr2/quality/90/format/webp" alt="500px" style="zoom:25%;" />![image-20230513210848900](https://cos.kiic.top//202305140512835.webp?imageMogr2/quality/90/format/webp

![image-20230524061218771](C:\Users\kiic\AppData\Roaming\Typora\typora-user-images\image-20230524061218771.png)


-----------

#### 功能简介:

神秘口袋是一个全新的大饼项目，由于出于兼容性考虑，打算使用低级语言进行编写 ，此开源项目可以提前提出issues，抛砖引玉等待大家来讨论

功能非常复杂但也非常简单，软件的初衷就是用来摸鱼的，当前已经有个简单的框架

因为五年前 人不在的时候微信被同事查看了引起了纠纷闹得不愉快了，所以现在有能力了想写一款摸鱼护航的app
#### 快点给我新的意见灵感吧！！！！！
#### 秉持这你摸鱼我摸鱼老板路虎变青桔的态度我得方向如下
有意见就提出来，欢迎到GitHub提issues  欢迎点亮Star  欢迎一起合作维护
##### https://github.com/kihlh/hmPocketSecrets/issues
-----------
大致功能方向如下：
##### 1.将微信聊天隐射到一个工作软件中
 - 钉钉
 - 资源管理器
 - PS
 - VS code 
### 举个栗子

![](https://cos.kiic.top//202305150406963.png?imageMogr2/quality/90/format/webp)
##### 2.不隐射 但是将窗口透明度变低 例如30%  在1.5米范围后很难发现此窗口
- 自由设置透明度
-  鼠标移开窗口自动变成0透明度（完全不可见）
-  最少停留1.2秒才激活到30%的透明度  
-  可以关键词 比如我的是 *  才触发透明度 不然你领导在旁边不就发现了

##### 3.人离开电脑则上锁 （通过判断一个U盘是否插入做到）
 - 将一个USB设备作为密匙 （例如一个U盘） 哪怕是手机也可以
 - 移除就将微信透明度变成
 - 使用其他手段则发送钉钉消息给手机
 - 处理快 响应约为50ms
 - 自定义时间后关闭显示器
 - 尝试非密匙手段解锁则杀死所有相关进程（设置中允许）
 - 开机后第一次接入则 才启动 （用户设置  默认接入密匙才激活）

##### 4.其他
- 主进程适用rust+C++的组合达到低性能占用高效
- 视图层 使用轻量级的fltk 编写  当前测试界面内存 3MB
- 文件小 更新更快
- 视图美 2023了不能是远古界面了吧 我堂堂一个设计师最少得美的不像是开源软件
- **安全安全 还是得安全**
- 用户配置使用AES256保存 防止被篡改 密码不可恢复制度 包含配置文件
- 退出和修改密码和程序设置应当在主进程中调用 输入主密码盐化对比成功才启用
- 密码设置只能在Rust的主进程进行 而且用户输入完成必须马上盐化

##### 可扩展
- 使用py或者node可以简单的对窗口进行而外处理

---------------------

#### 程序界面畅想

查看原型：https://js.design/f/L9jghw?p=y57X5S-meg

<img src="https://cos.kiic.top//202305220514227.png?imageMogr2/quality/90/format/webp" alt="image-20230522051406961" style="zoom:50%;" />

画大饼的原型一览
![](https://cos.kiic.top//202305150400513.png?imageMogr2/quality/90/format/webp)

![image-20230514051559726](https://cos.kiic.top//202305140515799.png?imageMogr2/quality/90/format/webp)
------------------

#### 版权声明/君子协议：

| 名称                                                         | 授权/协议                                                    | 备注                        |
| ------------------------------------------------------------ | ------------------------------------------------------------ | --------------------------- |
| hm神秘口袋                                                   | MIT                                                          | 主要程序 不包含特殊模块     |
| 数据加密密匙                                                 | 不授权/ 只提供 只读的解密dll (用户密码只提供布尔值(哈希输入)) | 保证数据结构安全性          |
| hm神秘口袋 图标 <img src="https://cos.kiic.top//202305210936835.png?imageMogr2/quality/90/format/webp" alt="50px" style="zoom:33%;" /> | 禁止用于其他可执行文件                                       | 防止伪装或者商业性活动      |
| 微信嵌入钉钉  (内部工具)                                     | 不开源/禁止: 联网,存储数据,主动运行                          | 保证用户数据安全性/防止他用 |
| 微信嵌入资源管理器  (内部工具)                               | 不开源/禁止: 联网,存储数据,主动运行                          | 保证用户数据安全性/防止他用 |
|                                                              |                                                              |                             |





#### 编程语言：

Rust ，C++ ， HTML

##### 引用的程序包 排名不区分前后

- Rust

  |     库名称     |                           授权协议                           |          用途           |
  | :------------: | :----------------------------------------------------------: | :---------------------: |
  |    **fltk**    |        [MIT](https://choosealicense.com/licenses/mit)        |          界面           |
  |    fl2rust     |        [MIT](https://choosealicense.com/licenses/mit)        |        fltk辅助         |
  |   **regex**    | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |          界面           |
  | **serde_json** | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |        数据处理         |
  |     winres     |        [MIT](https://choosealicense.com/licenses/mit)        |    程序提权(管理员)     |
  |     winapi     | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |       系统API调用       |
  |   clipboard    | [MIT ](https://choosealicense.com/licenses/mit)OR [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) |         剪贴板          |
  |  magic-crypt   | [Apache-2.0](https://choosealicense.com/licenses/apache-2.0) | 用户配置内容加密 AES256 |
  |   heroicons    | [ MIT license](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE) |          图标           |
  |                |                                                              |                         |
  |                |                                                              |                         |
  |                |                                                              |                         |
  |                |                                                              |                         |
  |                |                                                              |                         |
  
  

