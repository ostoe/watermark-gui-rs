<script setup lang="ts">
// import { defineComponent, ref } from "vue";
import { Files, FolderChecked } from '@element-plus/icons-vue'
import { image_progress, previewwidget, user_conf, is_dir } from '../scripts/reactives';
import { Calendar, Search, SuccessFilled, WarningFilled } from '@element-plus/icons-vue'
import { sidebarReactives } from "../scripts/reactives";
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

const selectType = ref(true);

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

const inputInvalid = ref(true);
const inputDir = ref(user_conf.latestSelectedOutputPath);

const querySearch = (queryString: string, cb: any) => {
  cb(user_conf.outputPathHistory);
}

async function handleSelect(item: any) {
  if (await is_dir(inputDir.value)) {
    inputInvalid.value = true;
    if (item.value != user_conf.latestSelectedOutputPath) {
      user_conf.updated_output_dir(item.value);
      // backend update dir
    }
  } else {
    inputInvalid.value = false;
  }

}
const handleIconClick = (ev: Event) => {
  console.log(ev)
}

async function input_value_change(v: string) {
  console.log(inputDir.value);
  if (await is_dir(inputDir.value)) {
    inputInvalid.value = true;
    if (v != user_conf.latestSelectedOutputPath) {
      user_conf.updated_output_dir(v);
      // backend update dir
    }
  } else {
    inputInvalid.value = false;
  }
}
const getMaxHeight = computed(() => {
  return (sidebarReactives.curWindowHeight - 89)
})



</script>

<template>
  <el-container>
    <el-header height="40px">
      <TopBar />
    </el-header>
    <el-divider></el-divider>
    <el-row>
      <el-col :span="14">
        <el-tooltip :content="'选择' + (selectType ? '文件' : '文件夹')" placement="bottom-end" effect="light">
          <el-button v-if="selectType" @click="image_progress.selectFiles()"  style="margin-left:20px; margin-right: 5px;">选择</el-button>
          <el-button v-else @click="image_progress.selectDirs()">选择</el-button>
        </el-tooltip>
        <el-tooltip :content="'输入模式：' + (selectType ? '文件' : '文件夹')" placement="bottom-end" effect="light">
          <el-switch v-model="selectType" style="--el-switch-on-color: #38D6BF; --el-switch-off-color: #D4BE94"
            inline-prompt :active-icon="Files" :inactive-icon="FolderChecked" />
        </el-tooltip>
      </el-col>
      <el-col :span="10">
        <el-row>
          <el-col :span="14">
            <el-autocomplete v-model="inputDir" class="inline-input w-50" size="default" placeholder="Please Input"
              :fetch-suggestions="querySearch" @change="input_value_change" @select="handleSelect">
              <template #suffix>
                <el-icon v-if="inputInvalid" color="#33CC33" class="el-input__icon" @click="handleIconClick">
                  <SuccessFilled />
                </el-icon>
                <el-icon v-else class="el-input__icon" color="#FF3333" @click="handleIconClick">
                  <WarningFilled />
                </el-icon>
              </template>
            </el-autocomplete>
          </el-col>
          <el-col :span="10">
            <el-button @click="user_conf.selectOutputDirs()">输出目录</el-button>
          </el-col>
        </el-row>
      </el-col>
    </el-row>
    <el-scrollbar height="100%" :max-height="getMaxHeight">
      <el-main>
        <div class="container">
          <div class="demo-image__placeholder">
            <!-- <span class="demonstration">Custom</span> -->
            <el-image :src="src" fit="contain">
              <template #placeholder>
                <div class="image-slot">Loading<span class="dot">...</span></div>
              </template>
            </el-image>
          </div>
          <div class="model">
            <div id="banner" :style="cusstyle"></div>
            <!-- <img :src="src" alt="Snow" style="width:100%;">
            <div background="#887863" style="width:100%;"></div> -->
            <div class="device">Nikon ZFC</div>
            <div class="wrapper">
              <div class="detail">
                <div class="icon"></div>
                <div class="divider"></div>
                <div class="font">
                  <div class="date-time">2030:05:10 14:31:33</div>
                  <div class="inter-content">50mm f/1.8 iso 2000</div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <el-container direction="vertical">
          <PreviewWidget>
            <!-- <el-tooltip :content="'输入模式：' + (selectType ? '文件' : '文件夹')" placement="bottom-end" effect="light">
            <el-switch v-model="selectType" style="--el-switch-on-color: #38D6BF; --el-switch-off-color: #D4BE94"
              inline-prompt :active-icon="Files" :inactive-icon="FolderChecked" />
          </el-tooltip>
          <el-tooltip :content="'选择' + (selectType ? '文件' : '文件夹')" placement="bottom-end" effect="light">
            <el-button v-if="selectType" @click="image_progress.selectFiles()">选择</el-button>
            <el-button v-else @click="image_progress.selectDirs()">选择</el-button>
          </el-tooltip>

          <el-button @click="user_conf.selectOutputDirs()">输出目录</el-button>

          <el-autocomplete v-model="inputDir" class="inline-input w-50" size="default" placeholder="Please Input"
            :fetch-suggestions="querySearch" @change="input_value_change" @select="handleSelect">
            <template #suffix>
              <el-icon v-if="inputInvalid" color="#33CC33" class="el-input__icon" @click="handleIconClick">
                <SuccessFilled />
              </el-icon>
              <el-icon v-else class="el-input__icon" color="#FF3333" @click="handleIconClick">
                <WarningFilled />
              </el-icon>
            </template>
          </el-autocomplete> -->
          </PreviewWidget>
          <!-- <el-button @click="image_progress.selectSingleFile()">读取信息</el-button> -->
          <!-- <el-row > -->
          <!-- <HelloWorld msg="Vite + Vue" /> -->
          <!-- <TextImageProcess /> -->
          <!-- </el-row> -->
        </el-container>


      </el-main>
    </el-scrollbar>
  </el-container>
  <!-- <div></div> -->
</template>

<style scoped>
/* .container {
  width: 882px;
  position: relative;
  text-align: center;
  color: white;
} */

/* .inter-content {
  position: absolute;
  bottom: 60%;
  left: 67%;
} */

.device {
  position: absolute;
  bottom: 60%;
  left: 5%;
}

/* .date-time {
  position: absolute;
  bottom: 20%;
  left: 67%;
} */

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

:deep(.el-input__wrapper) {
  box-shadow: 0 0 0 0;
}

.model {
  position: relative;
  bottom: 140px;
  color: white;
  opacity: 0.8;
}

.model .wrapper {
  position: absolute;
  left: 60%;
  bottom: 20%;
}

.model .detail {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.model .icon {
  width: 50px;
  height: 50px;
  background-image: url("../assets/resources/canon.png");
  background-position: center;
  background-repeat: no-repeat;
  background-size: contain;
}

.model .divider {
  width: 1px;
  border-width: 0 0 0 1px;
  border-style: solid;
  margin: 0 10px 0 10px;
  height: 35px;
}

.model .font {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

}
</style>
