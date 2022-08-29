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
 - 选择字体
 - 拖动文件
 - .....
 - 自定义布局
 - 自定义后缀 防止覆盖文件
 - 集成其他工具，如计算景深 超焦距之类

