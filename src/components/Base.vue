<script setup lang="ts">
// import { defineComponent, ref } from "vue";

// import TextImageProcess from "./TextImageProcess.vue";
import sideBar from "./SideBar.vue";
import TopBar from "./TopBar.vue";
import PicList from "./PicList.vue";
// import BaseSettingsDrawerVue from "./BaseSettingsDrawerVue.vue";
import PreviewWidget from "./PreviewWidget.vue";
// import { image_progress } from "../scripts/reactives";
import { elmessage } from "../scripts/reactives";
// const pic = convertFileSrc("/Users/dongyifan/watermark-gui-rs/src/assets/20220807-_DSC0279-3.jpg")
// const tags = await ExifReader.load(pic);
// console.log(`output->tags`,tags)

function test_pro() {
  console.log("fff");
  // update_progress();
  //
}

</script>

<template>
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
        <!-- <el-button @click="image_progress.selectSingleFile()">读取信息</el-button> -->
        <!-- <el-row > -->
        <!-- <HelloWorld msg="Vite + Vue" /> -->
        <!-- <TextImageProcess /> -->
        <!-- </el-row> -->
      </el-container>
    </el-main>
  </el-container>
  <!-- <div></div> -->
</template>

<style scoped>

</style>
