import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
// 全局引入elementui
// import zhCn from 'element-plus/es/locale/lang/zh-cn'
import { router } from './router'

const app = createApp(App)

// 全局引入elementui，此处不设置语言
// app.use(ElementPlus, {
//   locale: zhCn,
// })

app.use(router)
app.mount('#app')

// createApp(App).mount('#app')