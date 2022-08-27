<script lang="ts">
import HelloWorld from "./HelloWorld.vue";
import TextImageProcess from "./TextImageProcess.vue";
import TopBar from "./TopBar.vue";

import { defineComponent, ref } from "vue";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import { ElMessage } from "element-plus";
import { Context } from "vm";

function changeCollapse(that: Context) {
  that.isCollapse = that.isCollapse ? false : true;
  that.extendPadding = that.isCollapse ? "" : "70";
}

export default defineComponent({
  setup() {
    const isCollapse = ref(true);
    const tooltipEffect = ref("light");
    const dynamicWidth = ref(false);
    const extendPadding = ref("")
    const count = ref(0)
    const tableData = ref([])
    const text = ref("./tests/img/jpg/gps/DSCN0010.jpg")

    return {
      isCollapse,
      tooltipEffect,
      dynamicWidth,
      extendPadding,
      count,
      tableData,
      text

    };

  },
  watch: {

  },

  data() {
    return {
    };
  },
  name: "index",
  methods: {
    changeThisCollapse() {
      changeCollapse(this);
    },
    //获取鼠标点击消除遮罩
    clickMask() {
      changeCollapse(this);
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

// test event
</script>

<template lang="">
  <div class="common-layout index">
    <el-container>
      <el-aside width="50px">
        <el-menu default-active="1-1" class="elmenu" :collapse="isCollapse">
          <div style="margin-top: 30px"></div>
          <div @click="changeThisCollapse" class="extend" :style="{ 'padding-left': extendPadding + 'px' }">
            <el-icon>
              <i-ep-arrow-right v-if="isCollapse" />
              <i-ep-arrow-left v-if="!isCollapse" />
            </el-icon>
          </div>
          <el-sub-menu index="1">
            <template #title>
              <el-icon><i-ep-menu /></el-icon>
              <span>基本</span>
            </template>
            <el-menu-item-group>
              <el-menu-item index="1-1">不知道写啥</el-menu-item>
              <el-menu-item index="1-2">不知道写啥</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
          <el-menu-item index="2">
            <el-icon><i-ep-add-location /></el-icon>
            <template #title>高级</template>
          </el-menu-item>
          <el-menu-item index="3">
            <el-icon><i-ep-document /></el-icon>
            <template #title>TODO</template>
          </el-menu-item>
          <el-footer>
            <div class="footer-div">
              <el-divider content-position="center">
              <el-tooltip
                class="tooltip"
                :effect="tooltipEffect"
                content="gui测试"
                placement="right-start"
                v-if="isTooltip"
              >
              <span>&copy;</span>
              </el-tooltip>
              <span v-else class="copyrightSpan">&copy;gui</span>
              </el-divider>
            </div>
          </el-footer>
        </el-menu>
      </el-aside>
      <div v-if="!isCollapse" class="shadowmask" @click="clickMask"></div>
      <el-container>
        <el-header>
          <el-tooltip></el-tooltip>
          <TopBar />
        </el-header>
        <el-main>
          <el-container direction="vertical">
            <!-- <el-row > -->
            <HelloWorld msg="Vite + Vue" />
            <TextImageProcess />
            <!-- </el-row> -->
          </el-container>
        </el-main>
      </el-container>
    </el-container>
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

.common-layout .extend {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 10px 5px 20px 5px;
}

.copyrightSpan {
  font-size: xx-small;
}
</style>
