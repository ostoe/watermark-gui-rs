<script lang="ts">
import { defineComponent, ref } from "vue";
import { emit, listen } from "@tauri-apps/api/event";
import { event, invoke } from "@tauri-apps/api";
import { ElMessage } from "element-plus";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";

export default defineComponent({
  setup() {
    const isCollapse = ref(true);
    return {
      isCollapse,
    };
  },
  data() {
    return {
      count: 0,
      tableData: [],
      text: "./tests/img/jpg/gps/DSCN0010.jpg",
      selectImage: "",
    };
  },
  name: "index",
  methods: {
    changeCollapse() {
      console.log(this.isCollapse);
      this.isCollapse = this.isCollapse ? false : true;
    },
    // 这是个异步函数
    async greetTest() {
      if (this.text == "") {
        this.text = "World!";
      }
      let res = await invoke("greet", { name: this.text });
      console.log(res);
    },
    async test_event_recv() {
      // listen to the `click` event and get a function to remove the event listener
      // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
      // emits the `click` event with the object payload

      const unlisten = await listen<string>("click", (event) => {
        // 是一个循环函数
        console.log(
          `window name: ${event.windowLabel}, payload: ${event.payload.message}`
        );
        this.message(
          `window name: ${event.windowLabel}, payload: ${event.payload.message}`
        );
      });
      // console.log("recv ok " + this.count);
    },

    async test_drag_event_recv() {
      // listen to the `click` event and get a function to remove the event listener
      // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
      // emits the `click` event with the object payload

      const unlisten = await listen<string>("tauri://file-drop", (event) => {
        // 是一个循环函数
        console.log(
          `drap payload: ${event.id} ${event.payload}`
        );
        console.log(event.payload);
        this.message(
          `drap payload: ${event.payload}`
        );
      });
      // console.log("recv ok " + this.count);
    },

    async test_event_send() {
      // listen to the `click` event and get a function to remove the event listener
      // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
      // emits the `click` event with the object payload
      this.count++;

      emit("click", {
        theMessage: "send content: " + this.count,
      });
      console.log("send ok " + this.count);
    },
    async send_event() {
      console.log("will send_event");
      let res = await invoke("send_event");
      console.log("send_event ok");
    },

    message(msg: string) {
      ElMessage({
        message: msg,
        type: "success",
      });
    },
    handleFileChange(e: InputEvent) {
      const el = e.target as HTMLInputElement;
      if (!el.files || el.files?.length === 0) {
        return;
      }
      // console.log(el.files);
      console.log(e.target.files[0]);
      // this.$emit('input', e.target.files[0])
      // console.log(e.target.files[0].path);
      // console.log(typeof (e.target));
      this.selectImage = "./tests/img/jpg/gps/DSCN0010.jpg";
    },

    async selectFiles() {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "Image",
            extensions: ["png", "jpeg"],
          },
        ],
      });
      if (Array.isArray(selected)) {
        console.log("selected files" + selected);
        this.message("selected files" + selected);
        // user selected multiple files
      } else if (selected === null) {
        // user cancelled the selection
        ElMessage({
          message: "null file selected.",
          type: "warning",
        });
      } else {
        console.log("selected single file" + selected);
        this.message("selected single file" + selected);
        // user selected a single file
      }
    },
    async selectDirs() {
      const selected = await open({
        directory: true,
        multiple: false,
        // defaultPath: await appDir(),
      });
      if (Array.isArray(selected)) {
        console.log("selected dirs" + selected);
        this.message("selected dirs" + selected);
        // user selected multiple files
      } else if (selected === null) {
        // user cancelled the selection
        ElMessage({
          message: "null dir selected.",
          type: "warning",
        });
      } else {
        console.log("selected single dir" + selected);
        this.message("selected single dir" + selected);
        // user selected a single file
      }
    },
    //
  },

  mounted() {
    // 在其他方法或是生命周期中也可以调用方法
    this.test_event_recv();
    for (let i = 0; i < 5; i += 1) {
      this.tableData.push({ id: i, msg: " " + i + " " + i + " " + i });
    }
    this.test_drag_event_recv();
  },
});

// test event
</script>

<template lang="">
  
  <el-container  class="a-border">
  
  <div >
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