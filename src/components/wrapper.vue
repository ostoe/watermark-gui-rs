<script lang="ts" setup>
import dragTest from "./DragTest.vue";
import TextImageProcess from "./TextImageProcess.vue";
import SideBar from "./SideBar.vue";
import TopBar from "./TopBar.vue";
import picList from "./PicList.vue";
import BaseSettingsDrawerVue from "./BaseSettingsDrawer.vue";
import PreviewWidget from "./PreviewWidget.vue";

import { ref, onMounted } from "vue";
// import { Context } from "vm";
import { sidebarReactives } from "../scripts/reactives";
import { convertFileSrc } from "@tauri-apps/api/tauri"
//获取鼠标点击消除遮罩
let t: NodeJS.Timeout | null = null;
// const isCollapse = ref(true);
// const delay = ref(200);
// const extendPadding = ref("");
const changeThisCollapse = () => {
  sidebarReactives.changeThisCollapse();
};

 const src = ()=>{
  return convertFileSrc("/Users/dongyifan/Library/Mobile Documents/comappleCloudDocs/Desktop/pic/wallhaven-6od3px.jpeg")
 }

 const picSrc = ref(src())
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
              <el-image :src="picSrc"></el-image>
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
  .common-layout{
    height: 100%;
    width: auto;
  }
  .elcontainer{
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
