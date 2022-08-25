<script lang="ts">
import HelloWorld from "./HelloWorld.vue";
import H222 from "./H222.vue";
import TopBar from "./TopBar.vue";

import { defineComponent } from "vue";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import { ElMessage } from "element-plus";

export default defineComponent({
  data() {
    return {
      count: 0,
      tableData: [],
      text: "./tests/img/jpg/gps/DSCN0010.jpg",
    };
  },
  name: "index",
  methods: {
    // 这是个异步函数
    async greetTest() {
      if (this.text == "") {this.text = "World!"}
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
  },

  mounted() {
    // 在其他方法或是生命周期中也可以调用方法
    this.test_event_recv();
    for (let i = 0; i < 5; i += 1) {
      this.tableData.push({ id: i, msg: " " + i + " " + i + " " + i });
    }
  },
});

// test event
</script>

<template lang="">
    <HelloWorld msg="Vite + Vue"/>
    <div>--------------测试分割线头部----------------</div>
    <div>
      <!-- <div style="margin: 20px 0" /> -->
      <div style="margin: 20px 30% 20px 30%">
      <el-input v-model="text" size="large" clearable="true" placeholder="Please input"/>
      </div>
      <!-- <div style="margin: 20px 0" /> -->
    <suspense>
      <el-col>
      <el-button @click="send_event" color="#626aef" :dark="isDark" size="large" plain="true"> [s]测试event</el-button>
      <button @click="greetTest"> 测试调用rust </button> 
      </el-col>
    </suspense>
    <div>
      <el-table :data="tableData" height="200" style="width: 100%">
        <el-table-column sortable="true" sort-by="id"  sort-orders="descending" prop="msg" label="Date" width="300" />
      </el-table>
    </div>
    <div>--------------测试分割线尾部----------------</div>
    </div>
    <H222/>
</template>

<style>
</style>

