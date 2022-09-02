<script lang="ts" setup>
import dragTest from "./DragTest.vue";
import TextImageProcess from "./TextImageProcess.vue";
import SideBar from "./SideBar.vue";
import TopBar from "./TopBar.vue";
import picList from "./PicList.vue";

import { ref, onMounted } from "vue";
// import { Context } from "vm";
import { sidebarReactives } from "../scripts/reactives"
//获取鼠标点击消除遮罩
let t: NodeJS.Timeout | null = null;
// const isCollapse = ref(true);
// const delay = ref(200);
// const extendPadding = ref("");
const changeThisCollapse = () => {
  sidebarReactives.changeThisCollapse()
};


</script>

<template lang="">
  <div class="common-layout index">
    <el-container>
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
            <!-- <el-row > -->
            <!-- <HelloWorld msg="Vite + Vue" /> -->
            <!-- <TextImageProcess /> -->
            <!-- </el-row> -->
          </el-container>
        </el-main>
        <PicList></PicList>
      </el-container>
    </el-container>
  </div>
  <router-view></router-view>
</template>

<style scoped>
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
