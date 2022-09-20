
<template>
  <el-container>
    <el-header height="40px">
      <TopBar />
    </el-header>
    <el-divider></el-divider>
    <el-main>
      <el-row>
        <el-col :span="19">
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
            <div class="logo"></div>
            <div class="split-line"></div>
            <div class="model-device">Nikon ZFC</div>
            <div class="date-time">2030:05:10 14:31:33</div>
            <div class="inter-content">50mm f/1.8 1/2500 ISO 2000</div>
          </div>

        </el-col>
        <el-col :span="5" style="align-items: center;">

          <el-row style="margin-bottom: 2px;"><span class="demonstration">banner长宽比例：</span></el-row>
          <el-row>
            <el-col :span="14">
              <el-slider v-model="watermark_WH_ratio" :step="0.001" :min="0.01" :max="0.5" @input="WH_ratio" />
            </el-col>
            <el-col :span="10">
              <el-input-number v-model="watermark_WH_ratio" controls-position="right" size="default" @change="WH_ratio"
                :step="0.001" :min="0.01" :max="0.5"></el-input-number>
            </el-col>
          </el-row>

          <el-row style="margin-bottom: 2px;"><span class="demonstration">第一水平线</span></el-row>
          <el-row>
            <el-col :span="12">
              <el-slider v-model="watermark_text_h_scale" :step="0.001" :min="0.2" :max="1.0" @input="_text_h_scale" />
            </el-col>
            <el-col :span="12">
              <el-input-number v-model="watermark_text_h_scale" controls-position="right" :step="0.001" :min="0.2"
                :max="1.0" @change="_text_h_scale"></el-input-number>
            </el-col>
          </el-row>

          <el-row style="margin-bottom: 2px;"><span class="demonstration">第二水平线</span></el-row>
          <el-row>
            <el-col :span="12">
              <el-slider v-model="datetime_scale" :step="0.001" :min="0.2" :max="1.0" @input="datetime_scale_f" />
            </el-col>
            <el-col :span="12">
              <el-input-number v-model="datetime_scale" :step="0.001" :min="0.2" :max="1.0" controls-position="right"
                size="default" @change="datetime_scale_f"></el-input-number>
            </el-col>
          </el-row>

          <el-row style="margin-bottom: 2px;"><span class="demonstration">Logo比例</span></el-row>
          <el-row>
            <el-col :span="12">
              <el-slider v-model="logo_ratio" :step="0.001" :min="0.2" :max="1.0" @input="logo_ratio_f" />
            </el-col>
            <el-col :span="12">
              <el-input-number v-model="logo_ratio" size="default" :step="0.001" :min="0.2" :max="1.0"
                controls-position="right" @change="logo_ratio_f"></el-input-number>
            </el-col>
          </el-row>

          <el-row style="margin-bottom: 2px;"><span class="demonstration">横向位置</span></el-row>
          <el-row>
            <el-col :span="12">
              <el-slider v-model="position_ratio" :step="0.001" :min="0.2" :max="1.0" @input="position_ratio_f" />
            </el-col>
            <el-col :span="12">
              <el-input-number v-model="position_ratio" :step="0.001" :min="0.2" :max="1.0" size="default"
                controls-position="right" @change="position_ratio_f"></el-input-number>
            </el-col>
          </el-row>

          <el-row style="margin-bottom: 2px;">
            <el-tooltip content="不要随意修改此项，固定像素，\n当图片像素为2kw or 4kw时,\n间距距会变的很小" effect="customized">
              <span >分割线边距（像素） <el-icon>
                <i-ep-Warning/>
              </el-icon></span>
             
            </el-tooltip>
          </el-row>
          <el-row>
            <el-col :span="12">
              <el-slider v-model="split_line_spacing" :step="0.5" :min="1" :max="100" @input="split_line_spacing_f" />
            </el-col>
            <el-col :span="12">
              <el-input v-model="split_line_spacing" :step="0.5" :min="1" :max="100" size="default"
                controls-position="right" @change="split_line_spacing_f"></el-input>
            </el-col>
          </el-row>

          <el-row style="margin-bottom: 2px;"><span class="demonstration">型号｜标注｜日期｜分隔线</span></el-row>
          <el-row>
            <el-col :span="6">
              <el-color-picker v-model="camera_color" />
            </el-col>
            <el-col :span="6">
              <el-color-picker v-model="focus_color" />
            </el-col>
            <el-col :span="6">
              <el-color-picker v-model="datetime_color" />
            </el-col>
            <el-col :span="6">
              <el-color-picker v-model="splite_line_color" />
            </el-col>
          </el-row>

        </el-col>
      </el-row>
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
import { resolve, resourceDir } from "@tauri-apps/api/path";
import { Arrayable } from "element-plus/es/utils";
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

const GLOBAL_WIDTH = 1024;
const GLOBAL_WIDTH_PX = ref(GLOBAL_WIDTH + "px");

const watermark_WH_ratio = ref(Math.round((0.1172 * 0.8 + Number.EPSILON) * 10000000) / 10000000);
const watermark_text_h_scale = ref(0.5); // <----watermark_scale
const datetime_scale = ref(0.55);
const logo_ratio = ref(0.70);
const logo_spacing_ratio = ref(0.35);
const position_ratio = ref(0.6267);
const split_line_spacing = ref(30); // fixed 30 px.
const font_scale = ref(1.0);
const font_size = ref(GLOBAL_WIDTH * watermark_WH_ratio.value);

const font_size_device_px = ref(font_size.value * 0.25 + "px");
const font_size_focus_px = ref(font_size.value * 0.20 + "px");
const font_size_datetime_px = ref(font_size.value * 0.18 + "px");

const device_x_shift = ref(GLOBAL_WIDTH * 0.03 + "px");
// const watermark_WH_ratio_px = ref()
const st_line_y_shift = ref(watermark_text_h_scale.value / 2 * 100 + "%");
const focus_length_text_x = ref((position_ratio.value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%");
const datetime_scale_px = ref(datetime_scale.value * 100 + "%");
const logo_y_ratio_px = ref((1 - logo_ratio.value) / 2 * 100 + "%");
const logo_x_ratio_px = ref((1 - position_ratio.value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%");
const logo_width_px = ref(GLOBAL_WIDTH * watermark_WH_ratio.value * logo_ratio.value + "px");
const logo_spacing_ratio_px = ref(logo_spacing_ratio.value * 100 + "%");
const position_ratio_px = ref(position_ratio.value * 100 + "%");
const split_line_spacing_px = ref(position_ratio.value * 100 + "%");
const split_line_height = ref((1 - watermark_text_h_scale.value) * 100 + "%");
const font_scale_px = ref(font_scale.value * 100 + "%");

const camera_color = ref("#4A4A4A");
const focus_color = ref("#4A4A4A");
const datetime_color = ref("#4A4A4A");
const splite_line_color = ref("#4A4A4A");
const cusstyle = ref(
  {
    "background-color": "#757575",
    width: "100%",
    height: GLOBAL_WIDTH * watermark_WH_ratio.value + "px",
  }
)
const font_path = ref("");

async function get_font_path() {
  const r1 = await resourceDir();
  const _r2 = await resolve(r1, "resources", "OPPOSans-H.ttf");
  console.log(_r2);
  font_path.value = new URL(_r2, import.meta.url).toString();
}

function WH_ratio(value: Arrayable<number>) {
  // console.log(value);
  value = value as number
  const font_size = GLOBAL_WIDTH * value;
  font_size_device_px.value = font_size * 0.25 + "px";
  font_size_focus_px.value = font_size * 0.20 + "px";
  font_size_datetime_px.value = font_size * 0.18 + "px";
  logo_width_px.value = GLOBAL_WIDTH * value * logo_ratio.value + "px";
  cusstyle.value.height = GLOBAL_WIDTH * value + "px";
}

function _text_h_scale(value: Arrayable<number>) {
  value = value as number;
  st_line_y_shift.value = value / 2 * 100 + "%";
  split_line_height.value = (1 - value) * 100 + "%";
}

function datetime_scale_f(value: Arrayable<number>) {
  value = value as number;
  datetime_scale_px.value = value * 100 + "%";
}

function logo_ratio_f(value: Arrayable<number>) {
  value = value as number;
  logo_y_ratio_px.value = (1 - value) / 2 * 100 + "%";
  logo_width_px.value = GLOBAL_WIDTH * watermark_WH_ratio.value * logo_ratio.value + "px";
}

function position_ratio_f(value: Arrayable<number>) {
  value = value as number;
  focus_length_text_x.value = (value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%";
  logo_x_ratio_px.value = (1 - value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%";
  position_ratio_px.value = value * 100 + "%";
  split_line_spacing_px.value = value * 100 + "%";
}

function split_line_spacing_f(value: Arrayable<number>) {
  value = value as number;
  focus_length_text_x.value = (position_ratio.value + value / GLOBAL_WIDTH) * 100 + "%";
  logo_x_ratio_px.value = (1 - position_ratio.value + value / GLOBAL_WIDTH) * 100 + "%";
}

onMounted(() => {
  get_font_path();
  console.log(cusstyle.value.height);
  // setTimeout(() => {
  //    // invalid: sacale.value = 0.2;
  //   // valid
  //   let con = document.getElementById('banner') as HTMLElement ;
  //   con.style.height =  GLOBAL_WIDTH * watermark_WH_ratio.value + "px";
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

}

  .split-line {
  width: 2px;
  height: v-bind(split_line_height);
    </el-scrollbar>
  background-color: rgb(237, 233, 40);
  position: absolute;
  left: v-bind(split_line_spacing_px);
  top: v-bind(st_line_y_shift);
}

.container {
  /* border: 1px solid rgb(8, 210, 255); */
  font-family: 'OPPOSans-H';
  width: v-bind(GLOBAL_WIDTH_PX);
  position: relative;
  text-align: center;
  color: rgb(255, 255, 255);
  margin: 0px;
  /* box-shadow: 0 0 10px rgb(79, 223, 255); */
  /* font-size: v-bind(font_size_px); */
}

.model-device {
  position: absolute;
  top: v-bind(st_line_y_shift);
  left: v-bind(device_x_shift);
  font-size: v-bind(font_size_device_px);
}

.inter-content {
  position: absolute;
  top: v-bind(st_line_y_shift);
  left: v-bind(focus_length_text_x);
  font-size: v-bind(font_size_focus_px);
}

.date-time {
  position: absolute;
  top: v-bind(datetime_scale_px);
  left: v-bind(focus_length_text_x);
  font-size: v-bind(font_size_datetime_px);
}

<<<<<<< HEAD
/* .date-time {
  position: absolute;
  bottom: 20%;
  left: 67%;
} */
=======
.date-time {
  position: absolute;
  top: v-bind(datetime_scale_px);
  left: v-bind(focus_length_text_x);
  font-size: v-bind(font_size_datetime_px);
}
>>>>>>> 9e14e133071e4a3d94178cc6b16e1fa332999c72

.demo-image__placeholder .block {
  position: relative;
  padding: 0px 0;
  text-align: center;
  border-right: solid 1px var(--el-border-color);
  display: inline-block;
  width: 100%;
  box-sizing: border-box;
  vertical-align: top;
  max-width: v-bind(GLOBAL_WIDTH_PX);
  max-height: 500px;
  /* TODO auto */
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

.el-popper.is-customized {
  /* Set padding to ensure the height is 32px */
  padding: 6px 12px;
  background: linear-gradient(90deg, rgb(159, 229, 151), rgb(204, 229, 129));
}
.el-popper.is-customized .el-popper__arrow::before {
  background: linear-gradient(45deg, #b2e68d, #bce689);
  right: 0;
}
</style>
