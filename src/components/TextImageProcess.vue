<script setup lang="ts">
import { defineComponent, ref, onMounted } from "vue";
import update_progress from "./TopBar.vue";
import { emit, listen } from "@tauri-apps/api/event";
import { event, invoke } from "@tauri-apps/api";
import { ElMessage } from "element-plus";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";
import { pictureDir } from '@tauri-apps/api/path';

const isCollapse = ref(true);
const progress_count = ref({ completed: 0, total: 0 });
const count = ref(0);
const tableData = ref([]);
const text = ref("./tests/img/jpg/gps/DSCN0010.jpg");
const selectImage = ref("");
//  const      progress_count = ,
interface MsgProps {
  message: string,
  stateCode: number
}

interface ImageProps {
  image_paths: [string],
  count: number
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

async function process_single_image(image_path: ImageProps) {
  let send_content = JSON.stringify(image_path);
  console.log(send_content);
  let res = await invoke("handle_front_select_files", { imagesObj: image_path });
  message("process_single_image result: " + res);
};

async function handle_front_update_data(key: string, value: string) {
  let res = await invoke("handle_front_update_data", { key: key, value: value });
  message("update output dir: " + res);
};

async function test_event_recv() {
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  // emits the `click` event with the object payload

  const unlisten = await listen<MsgProps>("front-backend", (event) => {
    // 是一个循环函数
    console.log(
      `[r] : ${event.payload}`
    );
    // let state_code = Number(event.payload.message.substring(0, 4));
    // let data = event.payload.message.substring(4);
    switch (event.payload.stateCode) {
      case 200:
        progress_count.value.completed += 1;
        update_progress(progress_count.value.completed, progress_count.value.total);
      case 300:
        console.log("skip file: " + event.payload.message);
        progress_count.value.completed += 1;
        update_progress(progress_count.value.completed, progress_count.value.total);
      case 500:
        ;
      default: console.log("unknown nofitication.: " + event.payload)
    }
    if (progress_count.value.completed == progress_count.value.total) {
      message(
        `[r] : 已完成处理！`
      );
    }

  });
};

async function set_drap_hover_evet() {
  const dropzoneElement = document.querySelector("#drap-area-sq1");
  const unlisten = await listen("tauri://file-drop-hover", (e) => {
    console.log(e);
    const hoveredElement = document.elementFromPoint(e.x, e.y);

    if (dropzoneElement != null && dropzoneElement.contains(hoveredElement)) {
      // ...
      console.log("drag in la" + hoveredElement);
      // toggleActive();
    }
  });

};

async function test_drag_event_recv() {
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
async function send_event() {
  console.log("will send_event");
  let res = await invoke("send_event");
  console.log("send_event ok");
};

function message(msg: string) {
  ElMessage({
    message: msg,
    type: "success",
  });
};

function handleFileChange(e: InputEvent) {
  const el = e.target as HTMLInputElement;
  if (!el.files || el.files?.length === 0) {
    return;
  }
  // console.log(el.files);
  console.log(e.target ? e.target.files[0] : null);
  // this.$emit('input', e.target.files[0])
  // console.log(e.target.files[0].path);
  // console.log(typeof (e.target));
  selectImage.value = "./tests/img/jpg/gps/DSCN0010.jpg";
};

async function selectFiles() {
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
    let handle_json = { count: selected.length, image_paths: selected };
    update_progress(0, selected.length);
    // user selected multiple files
  } else if (selected === null) {
    // user cancelled the selection
    ElMessage({
      message: "null file selected.",
      type: "warning",
    });
  } else {
    // console.log("single fil: " + selected);
    let handle_json: ImageProps = { count: selected.length, image_paths: [selected] };
    // this.message("handle_json: " + handle_json.count);
    await process_single_image(handle_json);
  }
};
async function selectDirs() {
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
    handle_front_update_data("output_dir", selected)
  }
};
async function update_output_dir() {
  const pictureDirPath = await pictureDir();
  handle_front_update_data("output_dir", pictureDirPath);
};

onMounted(() => {
  test_event_recv();
  for (let i = 0; i < 5; i += 1) {
    tableData.value.push({ id: i, msg: " " + i + " " + i + " " + i });
  }
  test_drag_event_recv();
  // this.set_drap_hover_evet(); // e.x e.y invalid.
  update_output_dir();
});

defineExpose({
  selectFiles
})

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
        autosize="{ minRows: 2, maxRows: 6 }"
        placeholder="Please input"
      />
    </div>
    <div class=".b-border">
    <!-- <el-container direction="vertical"> -->
        <suspense>
          <!-- <el-col > -->
        <el-container direction="horizontal">
          <el-button @click="send_event" color="#de4781" size="large" plain=true>[s]测试event</el-button>
        <!-- </suspense>
        <suspense> -->
         <el-button @click="greetTest"  color="#322aef"  size="large"  plain=true >[i]测试Rust </el-button>
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
          <el-button class="btn" @click="selectFiles">选择文件</el-button>
        <!-- </div> -->
        <!-- <div class="photoSelector"> -->
          <el-button class="btn  .b-border" @click="selectDirs">选择目录</el-button>
      </div>

        <!-- </div> -->
      </el-col>
      <text> {{ this.selectImage }}</text>
    </el-row>
        <!-- <button @click="greetTest" >测试调用rust</button> -->
      <!-- </el-col> -->
  
    <!-- </el-container> -->
    </div>
  </div>
  </el-container>

  <div>
    
</div>

</template>

<style>
.a-border {
  border: 1px solid rgb(8, 210, 255);
  margin: 20px auto 20px auto;
  text-align: center;
  box-shadow: 0 0 10px rgb(79, 223, 255);
  padding: 20px 25px 20px 25px;
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

  border-radius: 0.3rem;
  text-align: center;
  font-weight: bold;
}

.file-select>input[type="file"] {
  display: none;
}
</style>