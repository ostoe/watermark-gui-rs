
<script setup lang="ts">
import { defineComponent, ref } from "vue";
const isCollapse = ref(true);


function test_pro() {
  console.log("fff");
  // update_progress();
  //
};

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
        <!-- <el-row > -->
        <!-- <HelloWorld msg="Vite + Vue" /> -->
        <!-- <TextImageProcess /> -->
        <!-- </el-row> -->
      </el-container>
    </el-main>
    <PicList></PicList>
  </el-container>
  <!-- <div></div> -->
</template>
    
<style>

</style>