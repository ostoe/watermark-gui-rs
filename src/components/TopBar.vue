<script setup lang="ts">
import { main } from '@popperjs/core';
import { floor } from 'lodash';
import { onMounted, ref, reactive } from 'vue';
import { image_progress,previewwidget } from '../scripts/reactives';
import { invoke } from '@tauri-apps/api';
import { appDir, configDir, homeDir, localDataDir, logDir, resourceDir, fontDir } from '@tauri-apps/api/path';
import { ElMessage, ElNotification } from "element-plus";
import BaseSettingsDrawerVue from "./BaseSettingsDrawer.vue";

// const percentage = ref(90);
// const progress_count = ref({ completed: 0, total: 0 });
const colors = [
  { color: '#f56c6c', percentage: 0 },
  { color: '#e6a23c', percentage: 25 },
  { color: '#5cb87a', percentage: 50 },
  { color: '#1989fa', percentage: 75 },
  { color: '#6f7ad3', percentage: 100 },
];

async function test_some_f() {

  let a = [appDir, configDir, homeDir, localDataDir, logDir, resourceDir, fontDir];
  let b = ["appDir", "configDir", "homeDir", "localDataDir", "logDir", "resourceDir", "fontDir"];
  for (let i = 0; i < 6; i++) {
    let r = await a[i]();
    console.log(b[i] + ": " + r);
  }
}

//自定义指令
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
const ListenTopbarWidth = (width:any)=>{
  bigIcon.value=(width.width>=960)?true:false
  isNotTinyIcon.value=(width.width>=720)?true:false
}

const showDrawTable = ()=>{

}

const showPreviewWidget = ()=>{
  previewwidget.inputValue=true
}
const message=(msg: string)=> {
    ElNotification({
      message: msg,
      type: "success",
      title: "🐮----🍺",
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



function format(percentage: number) {
  return percentage === 100 ? "✔️" : `${image_progress.count.completed}/${image_progress.count.total}`;
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
  // #color=getColorstr((int(colorR),int(colorG),int(colorB)))#转换为16进制颜色
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
    //处理六位的颜色值
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
  if ((image_progress.count.total != 0) && (image_progress.count.completed != image_progress.count.total)) {
    let send_content = JSON.stringify(image_progress.image_paths);
    console.log(send_content);
    let res = await invoke("handle_front_select_files", { imagesObj: image_progress.image_paths });
    message("process_single_image result: " + res);
  } else {
    message("未选择文件或已完成");
  }

};

onMounted(() => {
  test_some_f();
})

defineExpose({
  image_progress
})

</script>


<template>
  <el-row class="row" v-resize="ListenTopbarWidth">
    <el-col :span="18" class="left">
      <div class="photoSelector">
        <el-progress id="progress-bar" :percentage="image_progress.value" :format="format" :color="color"
          v-if="isNotTinyIcon"></el-progress>
        <div>
          <div v-if="bigIcon">
            <el-button @click="image_progress.increase()"> + </el-button>
          </div>
          <div v-else class="increase">
            <el-icon>
              <i-ep-circle-plus @click="image_progress.increase()" />
            </el-icon>
          </div>
        </div>
        <div v-if="bigIcon">
          <el-button @click="image_progress.selectFiles()">选择文件</el-button>
          <el-button @click="image_progress.selectDirs()">输出目录</el-button>
          <el-button @click="process_image" color="#de4781" size="" plain>开始处理</el-button>
        </div>
        <div v-else class="medium">
          <el-icon style="margin-right:20px">
            <i-ep-document-add @click="image_progress.selectFiles()" />
          </el-icon>
          <el-icon style="margin-right:20px">
            <i-ep-folder-add @click="image_progress.selectDirs()" />
          </el-icon>
          <el-icon style="margin-right:20px">
            <i-ep-full-screen @click="process_image" />
          </el-icon>
        </div>
      </div>
    </el-col>
    <el-col :span="6" class="right">
      <div class="previewer" v-if="bigIcon">
        <el-button class="btn" @click="showPreviewWidget">预览图片</el-button>
      </div>
      <div v-else @click="showPreviewWidget" class="previewer-icon">
        <el-icon style="margin-right:20px">
          <i-ep-picture />
        </el-icon>
      </div>
      <div class="options" @click="showDrawTable">
        <base-settings-drawer></base-settings-drawer>
      </div>
    </el-col>
  </el-row>
</template>

<script setup lang="ts"></script>

<style scoped>
.left {
  display: flex;
}

.left .photoSelector {
  margin-left: 30px;
}

.row {
  padding-top: 15px;
  z-index: 99;
}

.previewer {
  margin: 2px 15px 2px 15px;
}

.photoSelector,
.right {
  display: flex;
  justify-content: center;
  flex-direction: row;
}

.el-progress--line {
  margin-left: 20px;
  margin-bottom: 8px;
  width: 250px;
}

.options {
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-left: 60px;
}

.increase,
.medium,
.previewer-icon {
  font-size: 25px;
  margin-right: 20px;
}
</style>
