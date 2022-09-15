import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
// import home from "./components/home.vue";
// 全局引入elementui
// import zhCn from 'element-plus/es/locale/lang/zh-cn'
import  router from './router/router'
// import { TroisJSVuePlugin } from 'troisjs';
const app = createApp(App)

// 全局引入elementui，此处不设置语言
// app.use(ElementPlus, {
//   locale: zhCn,
// })

app.use(router)
// app.use(TroisJSVuePlugin);
app.mount('#app')

// createApp(App).mount('#app')