<script lang="ts">
import HelloWorld from "./HelloWorld.vue";
import TextImageProcess from "./TextImageProcess.vue";
import TopBar from "./TopBar.vue";

import { defineComponent, ref } from "vue";
// import { Context } from "vm";

//dark mode
import { useDark, useToggle } from "@vueuse/core";
//引入路由
import { useRoute, useRouter } from "vue-router";

export default defineComponent({
  setup() {
    const isCollapse = ref(true)
    return({
      isCollapse
    })
  },
  data() {
    return {
      count: 0,
      tableData: [],
      text: "./tests/img/jpg/gps/DSCN0010.jpg",
    };
  },
  name: "index",
  methods: {
    changeCollapse() {
      console.log(this.isCollapse)
      this.isCollapse = (this.isCollapse)?false:true
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
    // this.test_event_recv();
    // for (let i = 0; i < 5; i += 1) {
    //   this.tableData.push({ id: i, msg: " " + i + " " + i + " " + i });
    // }
  },
});

</script>

<template lang="">
  <el-menu
    default-active="2-1"
    class="elmenu"
    :collapse="isCollapse"
  >
    <el-menu-item index="1"  @click="changeCollapse">
      <el-icon><i-ep-arrow-right/></el-icon>
    </el-menu-item>
    <el-sub-menu index="2">
      <template #title>
        <el-icon><i-ep-menu /></el-icon>
        <span>基本</span>
      </template>
      <el-menu-item-group>
        <el-menu-item index="2-1">不知道写啥</el-menu-item>
        <el-menu-item index="2-2">不知道写啥</el-menu-item>
      </el-menu-item-group>
    </el-sub-menu>
    <el-menu-item index="3">
      <el-icon><i-ep-add-location /></el-icon>
      <template #title>高级</template>
    </el-menu-item>
    <el-menu-item index="4">
      <el-icon><i-ep-document /></el-icon>
      <template #title>TODO</template>
    </el-menu-item>
  </el-menu>
  <TopBar></TopBar>
  <HelloWorld msg="Vite + Vue" />
  <div>--------------测试分割线头部----------------</div>
  <div>
    <!-- <div style="margin: 20px 0" /> -->
    <div style="margin: 20px 30% 20px 30%">
      <el-input
        v-model="text"
        size="large"
        clearable="true"
        placeholder="Please input"
      />
    </div>
    <!-- <div style="margin: 20px 0" /> -->
    <suspense>
      <el-col>
        <el-button
          @click="send_event"
          color="#626aef"
          :dark="isDark"
          size="large"
          plain="true"
        >
          [s]测试event</el-button
        >
        <button @click="greetTest">测试调用rust</button>
      </el-col>
    </suspense>
    <div>
      <el-table :data="tableData" height="200" style="width: 100%">
        <el-table-column
          sortable="true"
          sort-by="id"
          sort-orders="descending"
          prop="msg"
          label="Date"
          width="300"
        />
      </el-table>
    </div>
    <div>--------------测试分割线尾部----------------</div>
  </div>
</template>

<style>
.elmenu {
  z-index: 99;
  position: absolute;
  left: 0;
  height: 100%;
}

.footer-div {
  background-color: rgb(255, 255, 255);
  display: flex;
  height: 4vh;
  width: 100%;
  bottom: 0;
  right: 0;
  align-items: center;
}

/* .el-container {
  width: var(--el-aside-width,120px);
} */

.common-layout .shadowmask {
  position: absolute;
  top: 0 auto;
  left: 0 auto;
  width: 100%;
  height: 100%;
  z-index: 98;
  background-color: #000;
  opacity: 0.7;
}

.common-layout .extend,
.darkBtn {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 10px 5px 20px 5px;
}

.copyrightSpan {
  font-size: xx-small;
}
</style>
