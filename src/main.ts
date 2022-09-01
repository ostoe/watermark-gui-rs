import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
// 全局引入elementui
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import { invoke } from '@tauri-apps/api'
import { routers } from './router'
import { reactive } from 'vue'
import { open } from "@tauri-apps/api/dialog";
import { ElMessage, ElNotification } from "element-plus";


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


// export default async function greet(params: string) {

// }

interface ImageProps {
  image_paths: [string],
  count: number
}

export const tools = reactive({
  message(msg: string) {
    ElNotification({
      message: msg,
      type: "success",
      title: "🐮----🍺",
      position: 'bottom-left',
    });
  },

})

export const image_progress = reactive({

  value: 90,
  count: { completed: 0, total: 0 },
  image_paths: { count: 0, image_paths: [""] },
  increase() {
    if (this.value <= 98) {
      this.value += 2
      if (this.value < 0) {
        this.value = 0
      }
    }
  },
  // 
  reset_progress() {
    this.value = 0;
    this.count.completed = 0;
    this.count.total = 0;
    this.image_paths = { count: 0, image_paths: [""] };
  },
  increase_one() {
    this.count.completed++;
    if (this.count.total > 0) {
      let value = this.count.completed / this.count.total;
      if (value < 0) { this.value = 0; } 
      else if
        (value > 100) { this.value = 100; }
      else {
        this.value = Math.round((value + Number.EPSILON) * 10000) / 100;
      }
    }
  },
  update_progress(completed: number, total: number) {
    this.count.completed = completed;
    this.count.total = total;
    if (total > 0) {
      let value = completed / total;
      if (value < 0) { value = 0; } else if (value > 100) { value = 100; }
      this.value = Math.round((value + Number.EPSILON) * 10000) / 100;
    }

    // color();
  },

  //


  async selectFiles() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          extensions: ["jpg", "jpeg"],
        },
      ],
    });
    if (Array.isArray(selected) && selected?.length != 0) {
      // console.log(selected);
      this.image_paths = { count: selected.length, image_paths: selected } as ImageProps;
      image_progress.update_progress(0, selected.length);
      tools.message("selected: " + this.image_paths);
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
      ElMessage({
        message: "null file selected.",
        type: "warning",
      });
    } else if (typeof (selected) === "string") {
      // console.log("single fil: " + selected);
      this.image_paths = { count: 1, image_paths: [selected] };
      // this.message("handle_json: " + handle_json.count);
      //   await process_single_image(handle_json);
      image_progress.update_progress(0, 1);
      tools.message("selected: " + this.image_paths);
    }
  },

  async update_user_data2BD(key: string, value: string) {
    let res = await invoke("handle_front_update_data", { key: key, value: value });
    message("update output dir: " + res);
  },
  //

  async selectDirs() {
    const selected = await open({
      directory: true,
      multiple: false,
      // defaultPath: await appDir(),
    });
    if (Array.isArray(selected)) {
      console.log("selected dirs" + selected);
      message("selected dirs" + selected);
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
      ElMessage({
        message: "null dir selected.",
        type: "warning",
      });
    } else {
      console.log("selected single dir " + selected);
      message("selected single dir " + selected);
      this.update_user_data2BD("output_dir", selected);
    }
  },

})

function message(msg: string) {
  ElNotification({
    message: msg,
    type: "success",
    title: "🐮----🍺",
    position: 'bottom-left',
  });
};