<script setup lang="ts">
import { floor } from 'lodash';
import { onMounted, ref } from 'vue';


const percentage = ref(90);
const progress_count = ref({ completed: 0, total: 0 });
const colors = [
  { color: '#f56c6c', percentage: 0 },
  { color: '#e6a23c', percentage: 25 },
  { color: '#5cb87a', percentage: 50 },
  { color: '#1989fa', percentage: 75 },
  { color: '#6f7ad3', percentage: 100 },
]

function color() {
  const index = floor(percentage.value / 25.01);
  console.log(index);
  const startC = colorRgb(colors[index].color);
  const endC = colorRgb(colors[index + 1].color);
  return gerColorOfWeight1(1, 25, startC, endC, percentage.value % 25);
}

function update_progress(completed: number, total: number) {
  progress_count.value.completed = completed;
  progress_count.value.total = total;
  if (total > 0) {
    let value = completed / total;
    if (value < 0) { value = 0; } else if (value > 100) { value = 100; }
    percentage.value = Math.round((value + Number.EPSILON) * 10000) / 100;
  }

  // color();
}

const increase = () => {
  if (percentage.value <= 98) {
    percentage.value += 2
    if (percentage.value < 0) {
      percentage.value = 0
    }
  }
}


function format(percentage: number) {
  return percentage === 100 ? "✔️" : `${progress_count.value.completed}/${progress_count.value.total}`;
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

  const color = `rgb(${parseInt(colorR).toString()},${parseInt(colorG).toString()},${parseInt(
    colorB
  ).toString()})`;
  console.log(color);
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
}

</script>


<template>
  <!-- jindutiao -->
  <el-row class="row">
    <el-col :span="18" class="left">
      <div class="photoSelector">
        <el-button class="btn">选择图片</el-button>
        <el-progress id="progress-bar" :percentage="percentage" :format="format" :color="color"></el-progress>
        <div>
          <div>
            <el-button @click="increase"> +++ </el-button>
          </div>
        </div>
      </div>
    </el-col>
    <el-col :span="6" class="right">
      <div class="previewer">
        <el-button class="btn">预览图片</el-button>
      </div>
      <div class="options">
        <el-icon>
          <i-ep-arrow-right />
        </el-icon>
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
  width: 350px;
}

.options {
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-left: 60px;
}
</style>
