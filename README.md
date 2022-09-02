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
 - 图片logo位置和比例优化下[v]
 - 图片质量
 - 支持目录
 - 选择字体
 - 拖动文件
 - .....
 - 自定义布局
 - 自定义后缀 防止覆盖文件
 - 集成其他工具，如计算景深 超焦距之类
 - 进度条
 - 字体啊，logo位置、比例，还有日期格式，自定义显示光圈iso与否，放在高级设置里面了
 - 记录用户数据，输出文件夹之类的。



-------------------
```error
image/tiff
read exif ok
read exif---2.4659ms
read image---9.0118174s
453 x 453
resize ---9.5345489s
copy 126---14.2708617s
copy 139---14.6111246s
draw 153---14.7396533s
draw 162---14.739949s
write image---42.3066473s
write ok
```


ref: https://github.com/zzzgydi/clash-verge/blob/66ccbf70f8d51153ef097fb62deae833c683b26c/src-tauri/src/main.rs