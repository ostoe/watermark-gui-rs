# tauri + rust + vue3 + vite


ref tauri event:
```js
  async listen(event: 'tauri://scale-change', handler: EventCallback<ScaleFactorChanged>): Promise<UnlistenFn>
  async listen(event: 'tauri://menu', handler: EventCallback<string>): Promise<UnlistenFn>
  async listen(event: 'tauri://file-drop', handler: EventCallback<{ type: 'drop', paths: string[] }>): Promise<UnlistenFn>
  async listen(event: 'tauri://file-drop-hover', handler: EventCallback<{ type: 'hover', paths: string[] }>): Promise<UnlistenFn>
  async listen(event: 'tauri://file-drop-cancelled', handler: EventCallback<{ type: 'cancel' }>): Promise<UnlistenFn>
  async listen(event: 'tauri://theme-changed', handler: EventCallback<Theme>): Promise<UnlistenFn>
  async listen(event: 'tauri://close-requested', handler: EventCallback<CloseRequestedEvent>): Promise<UnlistenFn>
  async listen<T>(event: EventName, handler: EventCallback<T>): Promise<UnlistenFn> {
     // actual implementation
  }



import { event } from '@tauri-apps/api';

const dropzoneElement = document.querySelector(/* ... */);

event.listen('tauri://file-drop-hover', (e) => {
  const hoveredElement = document.elementFromPoint(e.x, e.y);

  if (dropzoneElement.contains(hoveredElement)) {
    // ...
  }
});


```


## TODO [list]
 - 图片logo位置和比例优化下 [v]
 - 图片质量 [v]
 - 支持目录 or 文件 做成开关 [v] 
 - 支持自定义字体 [] 
 - 拖动文件 [v]
 - 显示输出文件夹
 - 自定义后缀 or 自定义名称
 - 自定义布局
 - 自定义后缀 防止覆盖文件
 - 集成其他工具，如计算景深 超焦距之类
 - 进度条 [v]
 - 字体啊，logo位置、比例，还有日期格式，自定义显示光圈iso与否，放在高级设置里面了
 - 记录用户数据，输出文件夹之类的。
 - font size test...
 - replace Dynamic with ImageBuffer<u8>....
 - 处理等待时，其他组件的控制
 - reset 后强制提示



-------------------
```error
水印比例：

0.13

15000 1758

16  9  0.28
4   3  0.156
相机直出4128 2735    3:2   0.1758
```

unplugin autoimport 最佳实践
ref: https://github.com/zzzgydi/clash-verge/blob/66ccbf70f8d51153ef097fb62deae833c683b26c/src-tauri/src/main.rs




图像插值使用：[参考](https://stackoverflow.com/questions/23853632/which-kind-of-interpolation-best-for-resizing-image)

如果要放大图像，您应该更喜欢使用INTER_LINEAR或INTER_CUBIC插值。如果要缩小图像，您应该更喜欢使用INTER_AREA插值。
三次插值在计算上更复杂，因此比线性插值慢。但是，生成的图像的质量会更高。



### 自定义名称规范
#### 前缀 | 后缀
 - ""   :   空
 - "不含$x"   : 自定义
 - "含$x"   : 序号+自定义：解析$x 比如 [$x]

#### 中间
 - "\_\_keep\_\_"   :   保持原名字
 - "\_\_today\_\_"   :  今天的日期
 - "不含$x"   : 自定义