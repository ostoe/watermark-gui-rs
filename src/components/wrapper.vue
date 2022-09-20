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
import { sidebarReactives, image_progress } from "../scripts/reactives";
import { convertFileSrc } from "@tauri-apps/api/tauri";
//获取鼠标点击消除遮罩
let t: NodeJS.Timeout | null = null;
// const isCollapse = ref(true);
// const delay = ref(200);
// const extendPadding = ref("");
const changeThisCollapse = () => {
  sidebarReactives.changeThisCollapse();
};
// 输出路径面包屑
// const selectedPath = ref<Array<string>>();
// const transferPath2Array = () => {
//   // if(image_progress.image_dir_path.indexOf("\")){
//   // }
// };
const fontOps = [{
  fontFamily: 'Aleo-Bold-2',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'Aleo-Italic-4',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'Aleo-Light-5',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'Aleo-LightItalic-6',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'Aleo-Regular-7',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'FiraCode-Bold',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'FiraCode-Medium',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'FiraCode-Regular',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'FiraCode-Retina',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'FiraCode-SemiBold',
  fontWeight: 1000,
  fontSize: '150px'
},{
  fontFamily: 'DejaVuSans',
  fontWeight: 1000,
  fontSize: '150px'
}]

const fontFamilyValue = ref("")
console.log(`output->fontFamilyValue`,fontFamilyValue)
nextTick(() => {});
// const picSrc = ref("https://github.com/tauri-apps/tauri/blob/dev/.github/splash.png")
</script>

<template lang="">
  <span class="spantest">fonttest1</span>
  <span class="spantest2">fonttest2</span>
  <el-select v-model="fontFamilyValue" class="select" placeholder="选择文字">
    <el-option
      v-for="item in fontOps"
      :key="item.fontFamily"
      :value="item.fontFamily"
    />
  </el-select>
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
              <el-image src="https://fuss10.elemecdn.com/a/3f/3302e58f9a181d2509f3dc0fa68b0jpeg.jpeg"></el-image>
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
@import "../assets/style/font.css";

.spantest {
  font-family: v-bind("fontFamilyValue");
}

.spantest2 {
  font-family: "FiraCode-Bold";
}

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
