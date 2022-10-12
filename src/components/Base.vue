
<template>
  <el-container>
    <el-header height="65px" class="elheader">
      <div style="width:100%">
        <TopBar />
      </div>
    </el-header>
    <div style="padding-top: 8px;background: #f9f9f9;padding-bottom: 5px;">
      <el-row>
        <el-col :span="15">
          <div class="left">
            <el-tooltip :content="'选择' + (image_progress.selectType ? '文件' : '文件夹')" placement="bottom-end"
              effect="light">
              <el-button v-if="image_progress.selectType" @click="handleSelectInputFiles()"
                style="margin-left:20px; margin-right: 5px;">选择</el-button>
              <el-button v-else @click="handleSelectInputDir()">选择</el-button>
            </el-tooltip>
            <el-tooltip :content="'输入模式：' + (image_progress.selectType ? '文件' : '文件夹')" placement="bottom-end"
              effect="light">
              <el-switch v-model="image_progress.selectType"
                style="--el-switch-on-color: #38D6BF; --el-switch-off-color: #D4BE94" inline-prompt :active-icon="Files"
                :inactive-icon="FolderChecked" />
            </el-tooltip>
            <div style="width: auto;">
              <el-button text type="info">
                {{ selectedInputContent }}
              </el-button>
            </div>
          </div>
        </el-col>
        <el-col :span="9">
          <div class="right">
            <div>
              <el-button @click="user_conf.selectOutputDirs()">输出目录</el-button>
            </div>
            <div>
              <el-autocomplete v-model="inputDir" class="inline-input w-50" size="default" placeholder="Please Input"
                :fetch-suggestions="querySearch" @change="input_value_change" @select="handleSelectHistoriesDir">
                <template #suffix>
                  <el-icon v-if="inputInvalid" color="#33CC33" class="el-input__icon" @click="handleIconClick">
                    <SuccessFilled />
                  </el-icon>
                  <el-icon v-else class="el-input__icon" color="#FF3333" @click="handleIconClick">
                    <WarningFilled />
                  </el-icon>
                </template>
              </el-autocomplete>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>
    <el-scrollbar height="100%" :max-height="getMaxHeight">
      <el-main class="elmain">
        <div class="grid-wrapper">
          <el-card class="slider-wrapper">
            <div class="card" style="display:flex;flex-direction:column;">
              <div class="slide">
                <div class="font">banner长宽比例</div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="watermark_wh_ratio" :step="0.001" :min="0.01" :max="0.5"
                        @input="_wh_ratio_f" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="watermark_wh_ratio" controls-position="right" size="small"
                      @change="_wh_ratio_f" :step="0.001" :min="0.01" :max="0.5"></el-input-number>
                  </div>
                </div>
              </div>
              <div class="slide">
                <div class="font">第一水平线</div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="watermark_text_h_scale" :step="0.001" :min="0.2" :max="1.0"
                        @input="_text_h_scale" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="watermark_text_h_scale" controls-position="right" :step="0.001" :min="0.2"
                      :max="1.0" @change="_text_h_scale" size="small"></el-input-number>
                  </div>
                </div>
              </div>
              <div class="slide">
                <div class="font">第二水平线</div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="datetime_posi_percent" :step="0.001" :min="0.2" :max="1.0"
                        @input="datetime_posi_percent_f" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="datetime_posi_percent" :step="0.001" :min="0.2" :max="1.0"
                      controls-position="right" size="small" @change="datetime_posi_percent_f">
                    </el-input-number>
                  </div>
                </div>
              </div>
              <div class="slide">
                <div class="font">Logo比例</div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="logo_ratio" :step="0.001" :min="0.2" :max="1.0" @input="logo_ratio_f" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="logo_ratio" size="small" :step="0.001" :min="0.2" :max="1.0"
                      controls-position="right" @change="logo_ratio_f"></el-input-number>
                  </div>
                </div>
              </div>
              <div class="slide">
                <div class="font">横向位置</div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="position_ratio" :step="0.001" :min="0.2" :max="1.0"
                        @input="position_ratio_f" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="position_ratio" :step="0.001" :min="0.2" :max="1.0" size="small"
                      controls-position="right" @change="position_ratio_f"></el-input-number>
                  </div>
                </div>
              </div>
              <div class="slide">
                <div class="font">字体比例</div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="font_scale" :step="0.001" :min="0.2" :max="2.0" @input="font_scale_f" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="font_scale" :step="0.001" :min="0.2" :max="2.0" size="small"
                      controls-position="right" @change="font_scale_f"></el-input-number>
                  </div>
                </div>
              </div>
              <div class="slide">
                <div class="font">间距
                  <el-tooltip content="不要随意修改此项，固定像素，当图片像素为2kw or 4kw时, 间距距会变的很小">
                    <el-icon>
                      <i-ep-Warning />
                    </el-icon>
                  </el-tooltip>
                </div>
                <div class="wrapper">
                  <div style="display:flex;width: 50%;">
                    <div style="width:100%">
                      <el-slider v-model="split_line_spacing" :step="0.5" :min="1" :max="100"
                        @input="split_line_spacing_f" />
                    </div>
                  </div>
                  <div style="width:50%">
                    <el-input-number v-model="split_line_spacing" :step="0.5" :min="1" :max="100" size="small"
                      controls-position="right" @change="split_line_spacing_f"></el-input-number>
                  </div>
                </div>
              </div>
            </div>
          </el-card>
          <div class="watermask">
            <div class="demo-image__placeholder">
              <div class="block">
                <!-- <span class="demonstration">Custom</span> -->
                <el-image :src="src" fit="contain">
                  <template #placeholder>
                    <div class="image-slot">Loading<span class="dot">...</span></div>
                  </template>

                </el-image>
              </div>
            </div>
            <div class="container">
              <div id="banner" :style="cusstyle">
                <div class="logo"></div>
                <div class="split-line"></div>
                <div class="model-device">Nikon ZFC</div>
                <div class="date-time">2030:05:10 14:31:33</div>
                <div class="inter-content">50mm f/1.8 1/2500s ISO 2000</div>
              </div>
              <!-- <div background="#887863" style="width:100%;"></div> -->

              <!-- <div class="model-device">Nikon ZFC</div> -->
              <!-- <div class="wrapper">
                <div class="detail">
                  <div class="logo"></div>
                  <div class="divider"></div>
                  <div class="font">
                    <div class="date-time">2030:05:10 14:31:33</div>
                    <div class="inter-content">50mm f/1.8 iso 2000</div>
                  </div>
                </div>
                </div> -->
            </div>
          </div>
          <div class="selector">
            <!-- 颜色选择器 -->
            <el-card class="color">
              <div class="card-font">颜色选择器</div>
              <div class="colorpicker">
                <div class="color-wrapper">
                  <div class="font">
                    <div>背景</div>
                  </div>
                  <el-color-picker v-model="banner_bg_color" />
                </div>
                <div class="color-wrapper">
                  <div class="font">
                    <div>型号</div>
                  </div>
                  <el-color-picker v-model="camera_color" />
                </div>
                <div class="color-wrapper">
                  <div class="font">
                    <div>标注</div>
                  </div>
                  <el-color-picker v-model="focus_color" />
                </div>
                <div class="color-wrapper">
                  <div class="font">
                    <div>日期</div>
                  </div>
                  <el-color-picker v-model="datetime_color" />
                </div>
                <div class="color-wrapper">
                  <div class="font">
                    <div>分割线</div>
                  </div>
                  <el-color-picker v-model="splite_line_color" />
                </div>
              </div>
            </el-card>
            <!-- 设置 -->
            <el-card class="set">
              <div class="setup">
                <el-collapse v-model="extendCollapse">
                  <el-collapse-item title="设置" name="1">
                    <div class="save-btn">
                      <el-popconfirm confirm-button-text="是" cancel-button-text="否" :icon="InfoFilled"
                        icon-color="#626AEF" title="重置（仍未保存）" @confirm="resetConfirmEvent" @cancel="">
                        <template #reference>
                          <el-button type="primary" size="small" plain color="#0FCAC7" class="default-btn">默认
                          </el-button>
                        </template>
                      </el-popconfirm>
                      <!-- 此处防止两个按钮碰撞 -->
                      <div style="margin-top:5px;"></div>
                      <el-button type="primary" size="large" @click="saveSetting" class="save-btn"> 保存 </el-button>
                    </div>
                  </el-collapse-item>
                </el-collapse>
              </div>
            </el-card>
          </div>
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
    </el-scrollbar>
  </el-container>
  <!-- <div></div> -->
</template>



<script setup lang="ts">
// import { defineComponent, ref } from "vue";
import { Files, FolderChecked } from '@element-plus/icons-vue'
import { image_progress, previewwidget, user_conf, is_dir } from '../scripts/reactives';
import { Calendar, Search, SuccessFilled, WarningFilled, InfoFilled } from '@element-plus/icons-vue'
import { sidebarReactives } from "../scripts/reactives";
// import TextImageProcess from "./TextImageProcess.vue";
// import sideBar from "./SideBar.vue";
import TopBar from "./TopBar.vue";
// import PicList from "./PicList.vue";
// import BaseSettingsDrawerVue from "./BaseSettingsDrawerVue.vue";
import PreviewWidget from "./PreviewWidget.vue";
// import { image_progress } from "../scripts/reactives";
import { elmessage, UserSeniorSettings } from "../scripts/reactives";
import { resolve, resourceDir } from "@tauri-apps/api/path";
// import { Arrayable } from "element-plus/es/utils";
import { StyleValue } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/tauri';
// const pic = convertFileSrc("/Users/dongyifan/watermark-gui-rs/src/assets/20220807-_DSC0279-3.jpg")
// const tags = await ExifReader.load(pic);
// console.log(`output->tags`,tags)

function test_pro() {
  console.log("fff");
  // update_progress();
  //
}
const selectedInputContent = ref("");
// const selectType = ref(true);

// const src = new URL("../assets/test.jpeg", import.meta.url).toString();
const src = 'https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg'

const GLOBAL_WIDTH = 1024;  // 窗口缩放的时候，图片缩放了，但是横幅并未缩放。
const GLOBAL_WIDTH_PX = ref(GLOBAL_WIDTH + "px");

const watermark_wh_ratio = ref(Math.round((0.1172 * 0.8 + Number.EPSILON) * 10000000) / 10000000);
const watermark_text_h_scale = ref(0.5); // <----watermark_scale
const datetime_posi_percent = ref(0.55);
const logo_ratio = ref(0.70);
const logo_spacing_ratio = ref(0.35); // unused
const position_ratio = ref(0.6267);
const split_line_spacing = ref(30); // fixed 30 px.
const font_scale = ref(1.0);
const font_size = ref(10);

const font_size_device_px = ref("10px");
const font_size_focus_px = ref("10px");
const font_size_datetime_px = ref("10px");

const device_x_shift = ref("10px");
// const watermark_wh_ratio_px = ref()
const st_line_y_shift = ref("10%");
// const st_line_y_shift_fixed = ref("10%");
const focus_length_text_x = ref("10%");
const datetime_posi_percent_px = ref("10%");
const logo_y_ratio_px = ref("10%");
const logo_x_ratio_px = ref("10%");
const logo_width_px = ref("10px");
const logo_spacing_ratio_px = ref("10%");
const position_ratio_px = ref("10%");
const split_line_spacing_px = ref("10%");
const split_line_height = ref("10%");
const font_scale_px = ref("10%");

const camera_color = ref("#4A4A4A");
const focus_color = ref("#4A4A4A");
const datetime_color = ref("#4A4A4A");
const splite_line_color = ref("#4A4A4A");
const banner_bg_color = ref("#ffffff");
const cusstyle = ref(
  {
    "background-color": "#757575",
    width: "100%",
    height: GLOBAL_WIDTH * watermark_wh_ratio.value + "px",
  }
)
const font_path = ref("");

async function get_font_path() {
  const r1 = await resourceDir();
  const _r2 = await resolve(r1, "resources", "OPPOSans-H.ttf");
  font_path.value = `url('${convertFileSrc(_r2)}') format('ttf')`;

  // font_path.value = new URL(_r2, import.meta.url).toString();
  console.log(_r2, font_path.value); // /usr....   http://local /user///
}

function _wh_ratio_f(value: any) { // Arrayable<number>) {
  // console.log(value);
  value = value as number
  font_size.value = GLOBAL_WIDTH * value;
  font_size_device_px.value = font_size.value * font_scale.value * 0.25 + "px";
  font_size_focus_px.value = font_size.value * font_scale.value * 0.20 + "px";
  font_size_datetime_px.value = font_size.value * font_scale.value * 0.18 + "px";
  logo_width_px.value = GLOBAL_WIDTH * value * logo_ratio.value + "px";
  cusstyle.value.height = GLOBAL_WIDTH * value + "px";
}

function _text_h_scale(value: any) { // Arrayable<number>) {
  value = value as number;
  st_line_y_shift.value = (1 - value) / 2 * font_scale.value * 100 + "%";
  // st_line_y_shift_fixed.value = (1 - value) / 2 * font_scale.value * 100 - 2.0 + "%";
  split_line_height.value = value * 100 + "%";
}

function datetime_posi_percent_f(value: any) { // Arrayable<number>) {
  value = value as number;
  datetime_posi_percent_px.value = value * 100 + "%";
}

function logo_ratio_f(value: any) { // Arrayable<number>) {
  value = value as number;
  logo_y_ratio_px.value = (1 - value) / 2 * 100 + "%";
  logo_width_px.value = GLOBAL_WIDTH * watermark_wh_ratio.value * logo_ratio.value + "px";
}

function position_ratio_f(value: any) {
  value = value as number;
  focus_length_text_x.value = (value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%";
  logo_x_ratio_px.value = (1 - value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%";
  position_ratio_px.value = value * 100 + "%";
  split_line_spacing_px.value = value * 100 + "%";
}

function split_line_spacing_f(value: any) { // Arrayable<number>) {
  value = value as number;
  focus_length_text_x.value = (position_ratio.value + value / GLOBAL_WIDTH) * 100 + "%";
  logo_x_ratio_px.value = (1 - position_ratio.value + value / GLOBAL_WIDTH) * 100 + "%";
}

function font_scale_f(value: any) { // Arrayable<number>) {
  value = value as number;
  font_size_device_px.value = (font_size.value * value * 0.25 + "px");
  font_size_focus_px.value = (font_size.value * value * 0.20 + "px");
  font_size_datetime_px.value = (font_size.value * value * 0.18 + "px");
}

function update_px_from_value() {
  cusstyle.value.height = GLOBAL_WIDTH * watermark_wh_ratio.value + "px";
  font_size.value = GLOBAL_WIDTH * watermark_wh_ratio.value;

  font_size_device_px.value = font_size.value * font_scale.value * 0.25 + "px";
  font_size_focus_px.value = font_size.value * font_scale.value * 0.20 + "px";
  font_size_datetime_px.value = font_size.value * font_scale.value * 0.18 + "px";

  device_x_shift.value = GLOBAL_WIDTH * 0.03 + "px";
  //   watermark_wh_ratio_px .value = )
  st_line_y_shift.value = (1 - watermark_text_h_scale.value) / 2 * 100 + "%";
  // st_line_y_shift_fixed.value = (1 - watermark_text_h_scale.value) / 2 * 100 - 2.0 + "%";
  focus_length_text_x.value = (position_ratio.value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%";
  datetime_posi_percent_px.value = datetime_posi_percent.value * 100 - 1.0 + "%";
  logo_y_ratio_px.value = (1 - logo_ratio.value) / 2 * 100 + "%";
  logo_x_ratio_px.value = (1 - position_ratio.value + split_line_spacing.value / GLOBAL_WIDTH) * 100 + "%";
  logo_width_px.value = GLOBAL_WIDTH * watermark_wh_ratio.value * logo_ratio.value + "px";
  logo_spacing_ratio_px.value = logo_spacing_ratio.value * 100 + "%";
  position_ratio_px.value = position_ratio.value * 100 + "%";
  split_line_spacing_px.value = position_ratio.value * 100 + "%";
  split_line_height.value = watermark_text_h_scale.value * 100 + "%";
  font_scale_px.value = font_scale.value * 100 + "%";
}

function resetConfirmEvent() {
  watermark_wh_ratio.value = 0.09376;
  watermark_text_h_scale.value = 0.5;
  datetime_posi_percent.value = 0.55;
  logo_ratio.value = 0.7;
  position_ratio.value = 0.6267;
  split_line_spacing.value = 30;
  font_scale.value = 1.0;

  camera_color.value = "";
  focus_color.value = "";
  datetime_color.value = "";
  splite_line_color.value = "";
  banner_bg_color.value = "";
  update_px_from_value();

}

function saveSetting() {
  let sets = {
    watermark_wh_ratio: watermark_wh_ratio.value,
    watermark_text_h_scale: watermark_text_h_scale.value,
    datetime_posi_percent: datetime_posi_percent.value,
    logo_ratio: logo_ratio.value,
    position_ratio: position_ratio.value,
    split_line_spacing: split_line_spacing.value,
    font_scale: font_scale.value,
    camera_color: camera_color.value,
    focus_color: focus_color.value,
    datetime_color: datetime_color.value,
    splite_line_color: splite_line_color.value,
    banner_bg_color: banner_bg_color.value,
  } as UserSeniorSettings
  user_conf.save_senior_settings(sets);

}

onMounted(() => {
  // get_font_path();
  console.log(cusstyle.value.height);
  // init senior settings     begin----
  watermark_wh_ratio.value = user_conf.seniorSettings.watermark_wh_ratio;
  watermark_text_h_scale.value = user_conf.seniorSettings.watermark_text_h_scale;
  datetime_posi_percent.value = user_conf.seniorSettings.datetime_posi_percent;
  logo_ratio.value = user_conf.seniorSettings.logo_ratio;
  position_ratio.value = user_conf.seniorSettings.position_ratio;
  split_line_spacing.value = user_conf.seniorSettings.split_line_spacing;
  font_scale.value = user_conf.seniorSettings.font_scale;

  camera_color.value = user_conf.seniorSettings.camera_color;
  focus_color.value = user_conf.seniorSettings.focus_color;
  datetime_color.value = user_conf.seniorSettings.datetime_color;
  splite_line_color.value = user_conf.seniorSettings.splite_line_color;
  banner_bg_color.value = user_conf.seniorSettings.banner_bg_color;
  // ------init senior settings end----
  update_px_from_value();

  // -------init data end ------
  // console.log("111  ", font_size_device_px, font_size_focus_px, font_size_datetime_px);

  // setTimeout(() => {
  //    // invalid: sacale.value = 0.2;
  //   // valid
  //   let con = document.getElementById('banner') as HTMLElement ;
  //   con.style.height =  GLOBAL_WIDTH * watermark_wh_ratio.value + "px";
  // }, 3000);
})

const inputInvalid = ref(true);
const inputDir = ref(user_conf.latestSelectedOutputPath);
watch([() => user_conf.latestSelectedOutputPath], (nv, ov) => {
  inputDir.value = nv[0];
});
const querySearch = (queryString: string, cb: any) => {
  cb(user_conf.outputPathHistory);
}

async function handleSelectInputFiles() {
  if (await image_progress.selectFiles()) {
    selectedInputContent.value = user_conf.latestSelectedDirPath;
  }

}
async function handleSelectInputDir() {
  if (await image_progress.selectDirs()) {
    selectedInputContent.value = user_conf.latestSelectedDirPath;
  }
}

// output dir
async function handleSelectHistoriesDir(item: any) {
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

// 展开手风琴
const extendCollapse = ref(["1"])

</script>


<style scoped>
/* @import "../../src-tauri/resources/OPPOSans-H.ttf"; */
@font-face {
  font-family: OPPOSans-H;
  src: url("~@asset/resources/OPPOSans-H.ttf") format("ttf")
    /*v-bind(font_path);*/
}

.elheader {
  display: flex;
  align-items: center;
  padding: 0;
  /** #dcdfe6 **/
  border-bottom: 1px var(--el-border-color);
  border-bottom-style: solid;
}

:deep(.elmain) {
  --el-main-padding: 20px;
  display: block;
  flex: 1;
  flex-basis: auto;
  overflow: hidden;
  box-sizing: border-box;
  padding: var(--el-main-padding);
}

.right,
.left {
  display: flex;
  flex-direction: row;
}

.right {
  justify-content: right;
  margin-right: 15px;
}

.banner {
  /*unuse*/
  background-color: rgb(90, 87, 87);
  width: 100%,
    /* height: v-bind(), */

}

.logo {
  width: v-bind(logo_width_px);
  height: v-bind(logo_width_px);
  background-color: rgb(0, 136, 255);
  position: absolute;
  right: v-bind(logo_x_ratio_px);
  top: v-bind(logo_y_ratio_px);
}

.split-line {
  width: 2px;
  height: v-bind(split_line_height);
  background-color: rgb(237, 233, 40);
  position: absolute;
  left: v-bind(split_line_spacing_px);
  top: v-bind(st_line_y_shift);
}

.container {
  display: block;
  /* font-display:calc(); */
  /* border: 1px solid rgb(8, 210, 255); */
  /* font-family: 'OPPOSans-H'; */
  max-width: v-bind(GLOBAL_WIDTH_PX);
  position: relative;
  text-align: center;
  color: rgb(255, 255, 255);
  margin: 0px;
  /* max-width: v-bind(GLOBAL_WIDTH_PX); */
  position: relative;
  ;
  /* box-shadow: 0 0 10px rgb(79, 223, 255); */
  /* font-size: v-bind(font_size_px); */
}

.model-device {
  /* display: flex; */
  vertical-align: top;
  width: auto;
  position: absolute;
  top: v-bind(st_line_y_shift);
  left: v-bind(device_x_shift);
  font-size: v-bind(font_size_device_px);
  line-height: v-bind(font_size_device_px);
}

.inter-content {
  position: absolute;
  top: v-bind(st_line_y_shift);
  left: v-bind(focus_length_text_x);
  font-size: v-bind(font_size_focus_px);
  line-height: v-bind(font_size_focus_px);
}

.date-time {
  /* display: flex; */
  position: absolute;
  top: v-bind(datetime_posi_percent_px);
  left: v-bind(focus_length_text_x);
  font-size: v-bind(font_size_datetime_px);
  line-height: v-bind(font_size_datetime_px);
}

/* --------------------------------------------------------- */

.grid-wrapper {
  display: grid;
  grid-auto-flow: column;
  grid-gap: 10px;
  grid-template-columns: 20% 60% 20%;
}

.selector {
  width: 90%;
}

.selector .set{
  margin-top: 10px;
}

.colorpicker {
  display: flex;
  flex-wrap: wrap;
}

.card-font {
  font-size: 8px;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

.color-wrapper {
  width: 50px;
  flex-grow: 1;
  border: solid #f3f3f3 1px;

  display: flex;
  flex-direction: column;

}

.color-wrapper .font {
  font-size: 10px;
  color: #858585;
}

:deep(.el-color-picker) {
  display: flex;
  justify-content: center;
}

.setup .save-btn {
  display: grid;
}

.slide .wrapper {
  display: flex;
}

.slide .font {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 10px;
}

.el-input-number--small {
  width: 80px;
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

:deep(.inline-input) {
  border-style: solid;
  border-width: 0 0 1px 0;
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
