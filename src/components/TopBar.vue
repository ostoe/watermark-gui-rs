<script setup lang="ts">
import { Files, FolderChecked } from '@element-plus/icons-vue'
import { main } from '@popperjs/core';
import { floor } from 'lodash';
import { onMounted, ref, reactive } from 'vue';
import { elmessage, image_progress, previewwidget, user_conf, is_dir } from '../scripts/reactives';
import { invoke } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { appDir, configDir, homeDir, localDataDir, logDir, resourceDir, fontDir } from '@tauri-apps/api/path';
import { ElMessage, ElNotification } from "element-plus";
import BaseSettingsDrawer from "./BaseSettingsDrawer.vue";
import WaveProgress, { drawCarbo, drawCircle, drawText } from "../scripts/wave-progress-plugin";
import { objectPick } from '@vueuse/shared';
import { Calendar, Search, SuccessFilled, WarningFilled } from '@element-plus/icons-vue'

const percentage = ref(90);
const progress_count = ref({ completed: 0, total: 0 });
const inputInvalid = ref(true);
const inputDir = ref(user_conf.latestSelectedOutputPath);
// const inputHistory = ref<{value: string}[]>([]);
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

// invalid
// const test_blur = (e: any) => {console.log("test_blur", e)}
// const test_focus = (e: any) => {console.log("focus", e)}
// const test_input = (e: any) => {console.log("input", e)}
const colors = [
  { color: '#f56c6c', percentage: 0 },
  { color: '#e6a23c', percentage: 25 },
  { color: '#5cb87a', percentage: 50 },
  { color: '#1989fa', percentage: 75 },
  { color: '#6f7ad3', percentage: 100 },
];

const selectType = ref(true);

async function test_some_f() {

  let a = [appDir, configDir, homeDir, localDataDir, logDir, resourceDir, fontDir];
  let b = ["appDir", "configDir", "homeDir", "localDataDir", "logDir", "resourceDir", "fontDir"];
  for (let i = 0; i < 6; i++) {
    let r = await a[i]();
    console.log(b[i] + ": " + r);
  }
}

//???????????????
const vResize = {
  mounted: (el: any, binding: { value: (arg0: { width: number }) => void }) => {
    let ResizeObserver = window.ResizeObserver;
    el._resizer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        // console.log(entry.contentRect.width)
        binding.value({ width: entry.contentRect.width });
      }
    });
    el._resizer.observe(el);
    // console.log(binding)
  },
  unmounted: (el: { _resizer: { disconnect: () => void } }) => {
    el._resizer.disconnect();
  },
};

const bigIcon = ref(true)
const isNotTinyIcon = ref(true)
const ListenTopbarWidth = (width: any) => {
  bigIcon.value = (width.width >= 10) ? true : false // 960 --> 10
  isNotTinyIcon.value = (width.width >= 720) ? true : false
}

const showDrawTable = () => {

}

const showPreviewWidget = () => {
  previewwidget.inputValue = (previewwidget.inputValue) ? false : true
}
const message = (msg: string) => {
  ElNotification({
    message: msg,
    type: "success",
    title: "????----????",
    position: "bottom-left",
  });
}

function color() {
  const index = floor(image_progress.value / 25.01);
  // console.log(index);
  const startC = colorRgb(colors[index].color);
  const endC = colorRgb(colors[index + 1].color);
  return gerColorOfWeight1(1, 25, startC, endC, image_progress.value % 25);
}

const format_vlue = "";


function format(percentage: number) {
  return percentage === 100 ? "??????" : `${image_progress.count.completed}/${image_progress.count.total}`;
}

interface colorObj {
  red: number,
  green: number,
  blue: number,
}

function gerColorOfWeight1(minNum: number, maxNum: number, colorStart: colorObj, colorEnd: colorObj, number: number) {
  const colorR =
    ((colorEnd.red - colorStart.red) * (number - minNum)) / (maxNum - minNum) + colorStart.red;
  const colorG =
    ((colorEnd.green - colorStart.green) * (number - minNum)) / (maxNum - minNum) +
    colorStart.green;
  const colorB =
    ((colorEnd.blue - colorStart.blue) * (number - minNum)) / (maxNum - minNum) +
    colorStart.blue;
  const color = `rgb(${floor(colorR).toString()},${floor(colorG).toString()},${floor(
    colorB
  ).toString()})`;
  // console.log(typeof colorR);
  // #color=getColorstr((int(colorR),int(colorG),int(colorB)))#?????????16????????????
  return color;
}

function colorRgb(color: string) {
  var reg = /^#([0-9a-fA-f]{3}|[0-9a-fA-f]{6})$/;
  var sColor = color.toLowerCase();
  if (sColor && reg.test(sColor)) {
    if (sColor.length === 4) {
      var sColorNew = "#";
      for (var i = 1; i < 4; i += 1) {
        sColorNew += sColor.slice(i, i + 1).concat(sColor.slice(i, i + 1));
      }
      sColor = sColorNew;
    }
    //????????????????????????
    var sColorChange = [];
    for (var i = 1; i < 7; i += 2) {
      sColorChange.push(parseInt("0x" + sColor.slice(i, i + 2)));
    }
    return { red: sColorChange[0], green: sColorChange[1], blue: sColorChange[2] }
    // return "rgba(" + sColorChange.join(",") + ")";
  } else {
    return { red: 128, green: 128, blue: 128 };
  }
};

async function process_image() {
  // control other compo
  if (image_progress.canIHandle) {
    image_progress.isHandling = true
    image_progress.status_toogle();
    image_progress.process_image();
  } else {
    elmessage("please choose a picture first!")
  }
};

function get_image_url(value: string) {
  let ap = convertFileSrc(value);
  console.log(ap);
  return ap;
}
// const goutouUrl = ref(get_image_url())
// const goutouUrl = ref(get_image_url('/Users/dongyifan/watermark-gui-rs/src/assets/goutou.jpeg'))

enum progressSettings {
  progressSpeed = 2.0,
  characterNum = 2,
  characterWidth = 0.01,
  characterHeight = 5,
  lineWidth = 4,
  fontSize = 10
}
const getProgress = (completed: number = image_progress.count.completed, total: number = image_progress.count.total) => {

  if (completed != null && total != 0 && total != null) {
    return Math.round((completed / total + Number.EPSILON) * 10000) / 100
  } else {
    return 0;
  }
}
// const waveProgressRef = ref<HTMLCanvasElement>(new Object as HTMLCanvasElement);
const waveInit = ref<WaveProgress>(new Object as WaveProgress);
// const waveInit11 = ref<WaveProgress>(new Object as WaveProgress);
const setCanvasSize = (canvas: HTMLCanvasElement) => {
  canvas.width = 35;
  canvas.height = 35;
}
onMounted(() => {
  // test_some_f();

  // init user_conf
  // load user_conf 
  // inputHistory.value = [{value:"/Users/fly/Pictures/"}, {value: "fdf"}];
  // inputHistory.value = user_conf.outputPathHistory;
  // ???????????????
  const canvas = document.getElementById("waveProgress")! as HTMLCanvasElement
  window.addEventListener('resize', () => {
    setCanvasSize(canvas)
  })
  const waveRun = new WaveProgress({
    canvas: canvas,
    progress: getProgress(),
    waveSpeed: 0.05,
    progressSpeed: progressSettings.progressSpeed,
    waveCharacter: {
      color: '197,140,19',
      number: progressSettings.characterNum,
      waveWidth: progressSettings.characterWidth,
      waveHeight: progressSettings.characterHeight,
    },
  })!;
  waveInit.value = waveRun;
  // waveInit.value.usePlugin(drawCircle, { lineWidth: progressSettings.lineWidth });
  // waveInit.value.usePlugin(drawText, { fontSize: progressSettings.fontSize });
  waveInit.value.setProgress({
    to: 0,// getProgress(image_progress.count.completed, image_progress.count.total),
    from: 0, animated: false,
  })
  waveInit.value.render();


  // another icon
  // // ???????????????
  // const canvas11 = document.getElementById("waveProgress11")! as HTMLCanvasElement
  // // console.log(`output->canvas`, canvas11)
  // const waveRun11 = new WaveProgress({
  //   canvas: canvas11,
  //   progress: getProgress(),
  //   waveSpeed: 0.05,
  //   progressSpeed: 2.0,
  //   waveCharacter: {
  //     color: '70,194,240',
  //     number: 2,
  //     waveWidth: 0.01,
  //     waveHeight: 5,
  //   },
  // })!;
  // waveInit11.value = waveRun11;
  // // waveRun11.usePlugin(drawCarbo, { lineWidth: progressSettings.lineWidth });
  // waveInit11.value.usePlugin(drawCircle, { lineWidth: 4 });
  // waveInit11.value.usePlugin(drawText, { fontSize: 10 });
  // waveInit11.value.setProgress({
  //   to:  0,// getProgress(image_progress.count.completed, image_progress.count.total),
  //   from: 0, animated: false,
  // })
  // waveInit11.value.render();
});
watch([() => image_progress.count.completed, () => image_progress.count.total, () => image_progress.image_dir_path], (newValue, oldValue) => {
  // console.log(`output->oldValue`, oldValue);
  let fromData = getProgress(oldValue[0], oldValue[1]);
  let toData = getProgress(newValue[0], newValue[1]);
  //????????????????????????
  if (!image_progress.selectType) {
    console.log(image_progress.selectType)
    //????????????????????????
    if (newValue[2] != "") {
      image_progress.canIHandle = true
    } else {
      image_progress.canIHandle = false
    }
  } else {
    if (newValue[0] === 0 && newValue[1] != 0) {
      image_progress.canIHandle = true
    } else {
      image_progress.canIHandle = false
    }
  }
  waveInit.value.setProgress({
    from: fromData,
    to: toData,
    animated: true,
  });
  if (image_progress.isHandling && getProgress(newValue[0], newValue[1]) === 100) {
    image_progress.isHandling = false
  }
  // test
  // waveInit11.value.setProgress({
  //   from: fromData,
  //   to: toData,
  //   animated: true,
  // })
})


nextTick(() => {
  //   const ctx = document.getElementById("waveProgress")!.getContext('2d')
  // console.log(`output->ctx`,ctx)

})

</script>


<template>
  <el-row class="elrow" v-resize="ListenTopbarWidth">
    <el-col :span="16" class="left">
      <div class="photoSelector">
        <!-- <rotate-square4 v-if="image_progress.status"></rotate-square4>
        <ping-pong v-else></ping-pong> -->
        <!-- <canvas width="40" height="40" id="waveProgress11"></canvas> -->
        <!-- <el-progress id="progress-bar" :percentage="image_progress.value" :format="format" :color="color"
          v-if="isNotTinyIcon"></el-progress> -->
        <!-- <div> -->
        <!-- <div v-if="bigIcon"> -->
        <!-- <el-button @click="image_progress.increase()"> + </el-button> -->
        <!-- </div>
          <div v-else class="increase">
            <el-icon>
              <i-ep-circle-plus @click="image_progress.increase()" />
            </el-icon>
          </div>
        </div> -->
        <!-- <el-tooltip :content="'???????????????' + (selectType ? '??????' : '?????????')" placement="bottom-end" effect="light">
            <el-switch v-model="selectType" style="--el-switch-on-color: #38D6BF; --el-switch-off-color: #D4BE94"
              inline-prompt :active-icon="Files" :inactive-icon="FolderChecked" />
          </el-tooltip>
          <el-tooltip :content="'??????' + (selectType ? '??????' : '?????????')" placement="bottom-end" effect="light">
            <el-button v-if="selectType" @click="image_progress.selectFiles()">??????</el-button>
            <el-button v-else @click="image_progress.selectDirs()">??????</el-button>
          </el-tooltip>

          <el-button @click="user_conf.selectOutputDirs()">????????????</el-button>

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
        <!-- <el-button class="btn" @click="showPreviewWidget">??????/??????</el-button> -->
        <div class="left">
          <div class="font">????????????</div>
          <div class="progress">
            <div class="goutou-wrapper">
              <div class="goutou"></div>
              <canvas ref="waveProgress" width="35" height="35" id="waveProgress"
                style="border-radius: 48%;z-index: -1;"></canvas>
            </div>
            <el-button key="button.text" :type="image_progress.status ? 'success' : 'primary'" text> {{
            `${image_progress.count.completed}/${image_progress.count.total}`
            }} </el-button>
          </div>
        </div>
      </div>
    </el-col>
    <el-col :span="8" class="right">
      <div class="wrapper" style="padding-right: 15px;">
        <div class="previewer" v-if="bigIcon">
          <el-button @click="process_image" color="#de4781" size="" round :loading="image_progress.isHandling">????????????
          </el-button>
        </div>
        <div v-else @click="showPreviewWidget" class="previewer-icon">
          <el-icon style="margin-right:20px">
            <i-ep-picture />
          </el-icon>
        </div>
        <div class="options" @click="showDrawTable">
          <BaseSettingsDrawer></BaseSettingsDrawer>
        </div>
      </div>
    </el-col>
  </el-row>
</template>

<script setup lang="ts"></script>

<style scoped>
.left {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.left .photoSelector {
  margin-left: 30px;
  width: 37%;
}

@media(max-width:600px){
.left>.font{
  display: none;
}
}

.left>.font {
  font-size: 20px;
}

.left >.progress{
  display: flex;
  align-items: center;
}

.previewer {
  display: flex;
}

.photoSelector,
.right {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-around;
}

.right>.wrapper {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-around;
}

.el-progress--line {
  margin-left: 20px;
  margin-bottom: 8px;
  width: 250px;
}

.options {
  display: flex;
  flex-direction: column;
}

.increase,
.medium,
.previewer-icon {
  font-size: 25px;
  margin-right: 20px;
}

.goutou {
  /* background-image: v-bind(goutouUrl); */
  -webkit-mask-image: url('../assets/dog_eye.png');
  mask-image: url('../assets/dog_eye.png');
  -webkit-mask-repeat: no-repeat;
  mask-repeat: no-repeat;
  -webkit-mask-size: contain;
  mask-size: contain;
  height: 35px;
  position: absolute;
  width: 35px;
  z-index: 1;
  background-color: white;
}

.progress> .goutou-wrapper{
  margin-right: 3px;
  display: flex;
  align-items: center;
}
</style>
