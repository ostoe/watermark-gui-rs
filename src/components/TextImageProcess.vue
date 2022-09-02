<script setup lang="ts">
import { nextTick, ref, onMounted } from "vue";
// import { image_progress } from "../main";
import { image_progress } from '../scripts/reactives';
import { emit, listen } from "@tauri-apps/api/event";
import { event, invoke } from "@tauri-apps/api";
import { ElMessage, ElNotification } from "element-plus";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";
import { pictureDir } from '@tauri-apps/api/path';
import { watch } from "fs";
import BaseSettingsDrawerVue from "./BaseSettingsDrawer.vue";

const isCollapse = ref(true);
// const progress_count = ref({ completed: 0, total: 0 });
const count = ref(0);
const text = ref("./tests/img/jpg/gps/DSCN0010.jpg");
const selectImage = ref("");
const isPlain = ref(true)
//  const      progress_count = ,
interface MsgProps {
  message: string,
  state_code: number
}
interface ImageProps {
  image_paths: [string],
  count: number
}


const message=(msg: string)=> {
    ElNotification({
      message: msg,
      type: "success",
      title: "🐮----🍺",
      position: "bottom-left",
    });
  }

// {count: selected.length, image_paths: [selected]}

function changeCollapse() {
  console.log(isCollapse.value);
  isCollapse.value = isCollapse.value ? false : true;
};
// 这是个异步函数
async function greetTest() {
  if (text.value == "") {
    text.value = "World!";
  }
  let res = await invoke("greet", { name: text.value });
  console.log(res);
};




// async function update_user_data2BD(key: string, value: string) {
//   let res = await invoke("handle_front_update_data", { key: key, value: value });
//   message("update output dir: " + res);
// };

async function backend_event_recv() {
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  // emits the `click` event with the object payload

  const unlisten = await listen<MsgProps>("front-backend", (event) => {
    // 是一个循环函数
    // console.log(
    //   `[r]: ${event.payload.stateCode, event.payload.message}`
    // );
    // let state_code = Number(event.payload.message.substring(0, 4));
    // let data = event.payload.message.substring(4);
    switch (event.payload.state_code) {
      case 200:
        image_progress.increase_one();
        // progress.update_progress(progress.count.completed, progress.count.total);
        break;
      case 300:
        console.log("skip file: " + event.payload.message);
        image_progress.increase_one();
        // progress.update_progress(progress.count.completed, progress.count.total);
        break;
      case 500:
        console.log("500...");
        break;
      default: console.log("unknown nofitication.: " + event.payload.message);
    }
    if (image_progress.count.completed == image_progress.count.total) {
      message(
        `[r] : 已完成处理！`
      );
    }

  });
};

async function drap_hover_event_handle() {
  const dropzoneElement = document.querySelector("#drap-area-sq1");
  const unlisten = await listen("tauri://file-drop-hover", (e) => {
    console.log(e);
    // const hoveredElement = document.elementFromPoint(e.x, e.y);

    // if (dropzoneElement != null && dropzoneElement.contains(hoveredElement)) {
    //   // ...
    //   console.log("drag in la" + hoveredElement);
    //   // toggleActive();
    // }
  });

};

async function drag_event_handle() {
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  // emits the `click` event with the object payload

  const unlisten = await listen<string>("tauri://file-drop", (event) => {
    // 是一个循环函数
    console.log(event.payload);
    message(
      `drap payload: ${event.payload}`
    );
  });
};

const send_event_test = async ()=>{
  await nextTick(()=>{
    console.log("will send_event");
    let res = invoke("send_event");
    console.log("send_event ok");
  })
}


function handleFileChange(e: InputEvent) {
  const el = e.target as HTMLInputElement;
  if (!el.files || el.files?.length === 0) {
    return;
  }
  // console.log(el.files);
  console.log(el ? el.files[0] : null);
  // this.$emit('input', e.target.files[0])
  // console.log(e.target.files[0].path);
  // console.log(typeof (e.target));
  selectImage.value = "./tests/img/jpg/gps/DSCN0010.jpg";
};

async function init_output_dir() {
  const pictureDirPath = await pictureDir();
  image_progress.update_user_data2BD("output_dir", pictureDirPath);
};

onMounted(() => {
  backend_event_recv();
  // for (let i = 0; i < 5; i += 1) {
  //   tableData.value.push({ id: i, msg: " " + i + " " + i + " " + i });
  // }
  drag_event_handle();
  // this.set_drap_hover_evet(); // e.x e.y invalid.
  init_output_dir();
});

// test event
</script>

<template lang="">
  
  <el-container  class="a-border">
  
  <div id="drap-area-sq1">
    <!-- <div style="margin: 20px 0" /> -->
    <div style="margin: 20px 5% 20px 5%">
      <el-input
        v-model="text"
        type="textarea"
        size="large"
        :autosize="{ minRows: 2, maxRows: 6 }"
        placeholder="Please input"
      />
    </div>
    <div>
    <!-- <el-container direction="vertical"> -->
        <suspense>
          <!-- <el-col > -->
        <el-container direction="horizontal">
          <el-button @click="send_event" color="#de4781" size="large" :plain="isPlain">[s]测试event</el-button>
        <!-- </suspense>
        <suspense> -->
         <el-button @click="greetTest"  color="#322aef"  size="large"  :plain="isPlain" >[i]测试Rust </el-button>
        <!-- </el-col> -->
        </el-container>
        </suspense>
        <el-row class="left">
      <el-col :span="24" style="margin: 15px 0 15px 0 ">
        <!-- <div class="photoSelector"> -->
        <label class="file-select">
          <div class="select-button">
            <span v-if="selectImage">Selected File: {{selectImage.name}}</span>
            <span v-else>Select File</span>
          </div>
          <input type="file" @change="handleFileChange"/>
        </label>
      <div class="b-border">
          <el-button class="btn" @click="image_progress.selectFiles()">选择文件</el-button>
        <!-- </div> -->
        <!-- <div class="photoSelector"> -->
          <el-button class="btn" @click="image_progress.selectDirs()">选择目录</el-button>
          
        </div>


        <!-- </div> -->
      </el-col>
      <text> {{ selectImage }}</text>
    </el-row>
        <!-- <button @click="greetTest" >测试调用rust</button> -->
      <!-- </el-col> -->
  
    <!-- </el-container> -->
    </div>
  </div>
  <div style="margin-right=10px margin:auto" >
          <BaseSettingsDrawerVue/>
  </div>
  </el-container>

</template>

<style>
.a-border {
  border: 1px solid rgb(8, 210, 255);
  margin: 20px 10% 20px 10%;
  text-align: center;
  box-shadow: 0 0 10px rgb(79, 223, 255);
  padding: 20px 25px 20px 25px;
  align-items: center;
}

.b-border {
  /* border: 1px solid #de4781; */
  margin: 10px auto 10px auto;
  text-align: center;
  /* line-height: 2; */
  /* text-align: center; */
  /* box-shadow: 0 0 10px rgb(79, 223, 255); */
  /* padding: 20px 25px 20px 25px; */
}
</style>


<style scoped>
.file-select>.select-button {
  padding: 1rem;
  width: 10rem;
  color: white;
  background-color: #2ea169;
  justify-content: center;
  border-radius: 0.3rem;
  text-align: center;
  font-weight: bold;
  display: flex;
}

.file-select>input[type="file"] {
  display: none;
}
</style>