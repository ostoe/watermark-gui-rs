<script lang="ts" setup>
import dragTest from "./DragTest.vue";
import TextImageProcess from "./TextImageProcess.vue";
import SideBar from "./SideBar.vue";
import TopBar from "./TopBar.vue";
import picList from "./PicList.vue";
import BaseSettingsDrawerVue from "./BaseSettingsDrawer.vue";
import PreviewWidget from "./PreviewWidget.vue";

// import { square } from "../scripts/wave-progress";
import { ref, onMounted, nextTick } from "vue";
// import { Context } from "vm";
import { sidebarReactives } from "../scripts/reactives";
//获取鼠标点击消除遮罩
let t: NodeJS.Timeout | null = null;
// const isCollapse = ref(true);
// const delay = ref(200);
// const extendPadding = ref("");
const changeThisCollapse = () => {
  sidebarReactives.changeThisCollapse();
};
// const square = {
//   hook: "beforeProgress",
//   install({ ctx, configs, scopedData }, opts = {}) {
//     ctx.beginPath();
//     ctx.lineWidth = (opts && opts.lineWidth) || 2;
//     ctx.strokeStyle =
//       (opts && opts.lineColor) || `rgba(${configs.waveCharacter.color}, 1)`;
//     const r = Math.min(configs.canvasWidth, configs.canvasHeight) / 2;
//     ctx.arc(r, r, r + 1, 0, 2 * Math.PI);
//     ctx.stroke();
//     console.log(ctx);
//     ctx.beginPath();
//     ctx.lineWidth = 5;
//     ctx.arc(100, 200, 80, 0, Math.PI * 2);
//     // ctx.strokeStyle = '#1c86d1'
//     ctx.stroke();
//     // ctx.closePath()
//     console.log(ctx);
//   },
// };
nextTick(() => {});
// const picSrc = ref("https://github.com/tauri-apps/tauri/blob/dev/.github/splash.png")
</script>

<template lang="">
  <div class="common-layout index">
    <div class="elcontainer">
      <SideBar>
        <template #elmenu>
          <el-menu-item index="4">
            <el-icon>
              <i-ep-document />
            </el-icon>
            <template #title>TODO2测试插槽</template>
          </el-menu-item>
        </template>
      </SideBar>
      <div
        v-if="!sidebarReactives.isCollapse"
        class="shadowmask"
        @click="changeThisCollapse"
      ></div>
      <el-container>
        <el-header height="40px">
          <TopBar />
        </el-header>
        <el-divider></el-divider>
        <el-main>
          <el-container direction="vertical">
            <PreviewWidget>
              <el-image
                src="https://fuss10.elemecdn.com/a/3f/3302e58f9a181d2509f3dc0fa68b0jpeg.jpeg"
              ></el-image>
            </PreviewWidget>
            <!-- <el-row > -->
            <!-- <HelloWorld msg="Vite + Vue" /> -->
            <!-- <TextImageProcess /> -->
            <!-- </el-row> -->
          </el-container>
        </el-main>
        <PicList></PicList>
      </el-container>
    </div>
  </div>
  <router-view></router-view>
</template>

<style scoped>
.common-layout {
  height: 100%;
  width: auto;
}

.elcontainer {
  margin-left: 50px;
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
  z-index: 100;
  background-color: #000;
  opacity: 0.7;
}
</style>
