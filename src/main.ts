import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
// 全局引入elementui
// import ElementPlus from 'element-plus'
// import 'element-plus/dist/index.css'
// import zhCn from 'element-plus/es/locale/lang/zh-cn'
import {invoke} from '@tauri-apps/api'
import {routers} from './router'

const app = createApp(App)

// 全局引入elementui，此处不设置语言
// app.use(ElementPlus, {
//   locale: zhCn,
// })

app.use(routers)
app.mount('#app')

// createApp(App).mount('#app')

// 调用命令
// 在应用窗口中右键，打开开发者工具
// 你会看到控制台上输出了 "Hello, World!"！


export default async function greet(params: string) {
  console.log("will run greet");
  let res = await invoke('greet', {name: 'World'});
  console.log(res);
}
