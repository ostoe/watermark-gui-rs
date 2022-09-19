<script setup lang="ts">
// import { defineComponent, ref } from "vue";

// import TextImageProcess from "./TextImageProcess.vue";
// import sideBar from "./SideBar.vue";
import TopBar from "./TopBar.vue";
// import PicList from "./PicList.vue";
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

// const src = new URL("../assets/test.jpeg", import.meta.url).toString();
const src =
  'https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg'

  const watermark_ratio = ref(0.1172 * 0.8)
  const watermark_scale = ref(0.50)
  const logo_ratio = ref(0.70)
  const logo_spacing_ratio = ref(0.35)
  const position_ratio = ref(0.6267)
  const split_line_spacing = ref(30)
  const font_scale = ref(1.0)

  /* ref
  let mut font_scale = 1.0;
  let mut watermark_ratio = 0.1172f32 * 0.8;
  let mut WATERMARK_SCALE = 0.50;
  let mut logo_ratio = 0.70f32;
  let mut logo_spacing_ratio = 0.35f32; // if nokon logo should 1
  let mut position_ratio = 0.6267f32;
  let mut split_line_spacing = 30u32; // px doubel = 10*/
  // const font_path: String,

const cusstyle = ref(
  {
    "background-color": "#898989",
    width: "100%",
    height: 882 * watermark_ratio.value + "px",
  }
)

onMounted(() => {
  console.log(cusstyle.value.height)
  // setTimeout(() => {
  //    // invalid: sacale.value = 0.2;
  //   // valid
  //   let con = document.getElementById('banner') as HTMLElement ;
  //   con.style.height = 882 * watermark_ratio.value + "px";
  // }, 3000);
 
})



</script>

<template>
  <el-container>
    <el-header height="40px">
      <TopBar />
    </el-header>
    <el-divider></el-divider>
    <el-main>
      <div class="demo-image__placeholder">
        <div class="block">
          <!-- <span class="demonstration">Custom</span> -->
          <el-image :src="src" fit="fill">
            <template #placeholder>
              <div class="image-slot">Loading<span class="dot">...</span></div>
            </template>
            
          </el-image>
        </div>
      </div>
      <div class="container">
        <div id="banner" :style="cusstyle"></div>
        <!-- <img :src="src" alt="Snow" style="width:100%;">
        <div background="#887863" style="width:100%;"></div> -->
        <div class="model-device">Nikon ZFC</div>
        <div class="date-time">2030:05:10 14:31:33</div>
        <div class="inter-content">50mm f/1.8 iso 2000</div>
      </div>
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
.container {
  width: 882px;
  position: relative;
  text-align: center;
  color: white;
}

.inter-content {
  position: absolute;
  bottom: 60%;
  left: 67%;
}
.model-device {
  position: absolute;
  bottom: 60%;
  left: 5%;
}
.date-time {
  position: absolute;
  bottom: 20%;
  left: 67%;
}

.demo-image__placeholder .block {
  position: relative;
  padding: 0px 0;
  text-align: center;
  border-right: solid 1px var(--el-border-color);
  display: inline-block;
  width: 100%;
  box-sizing: border-box;
  vertical-align: top;
  max-width: 882px;
  max-height: auto;
}

.demo-image__placeholder .demonstration {
  display: block;
  color: var(--el-text-color-secondary);
  font-size: 14px;
  margin-bottom: 20px;
}

.demo-image__placeholder .el-image {
  padding: 0 0px;

}

.demo-image__placeholder.image-slot {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  background: var(--el-fill-color-light);
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.demo-image__placeholder .dot {
  animation: dot 2s infinite steps(3, start);
  overflow: hidden;
}
</style>
