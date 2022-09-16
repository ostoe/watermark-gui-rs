<script setup lang="ts">
// import { defineComponent, ref } from "vue";

// import TextImageProcess from "./textImageProcess.vue";
import sideBar from "./sideBar.vue";
import TopBar from "./topBar.vue";
import PicList from "./picList.vue";
// import BaseSettingsDrawerVue from "./baseSettingsDrawerVue.vue";
import PreviewWidget from "./previewWidget.vue";
import ExifReader from "exifreader";
// import { image_progress } from "../scripts/reactives";
import { sum } from "lodash";
// const pic = convertFileSrc("/Users/dongyifan/watermark-gui-rs/src/assets/20220807-_DSC0279-3.jpg")
// const tags = await ExifReader.load(pic);
// console.log(`output->tags`,tags)

//获取tags
// async function getTags() {
//   const tags = await ExifReader.load(image_progress.converted_exif_path)
//   //预留reactives数据
//   exifTags.value=tags
//   console.log(`output->tags`, exifTags.value)
// }

function test_pro() {
  console.log("fff");
  // update_progress();
  //
}

const exifTags = ref({} as ExifReader.Tags);


// 缩略图
const thumbBase64 = computed(() => {
  if (!exifTags.value.Thumbnail?.base64) {
    return "";
  } else {
    return (
      "url(data:image/png;base64," +
      exifTags.value.Thumbnail!.base64 +
      ") no-repeat"
    );
  }
});

// const computeExposureBiasValue = computed(() => {
//   if (exifTags.value.ExposureBiasValue!.description) {
//     return parseFloat(exifTags.value.ExposureBiasValue!.description).toFixed(2);
//   } else {
//     return "222"
//   }
// })

// 概览

const size = ref('')
const summaryInfo = computed(() => {
  return {
    models: {
      label: "器材",
      data: {
        model: { label: "相机", data: exifTags.value.Model?.value[0] },
        lensModel: { label: "", data: exifTags.value.LensModel?.value },
      }
    },
    exposure: {
      label: "模式",
      data: {
        exposureProgram: { label: "曝光模式", data: exifTags.value.ExposureProgram?.description },
        meteringMode: { label: "测光模式", data: exifTags.value.MeteringMode?.description },
        exposureBiasValue: { label: "曝光补偿", data: exifTags.value.ExposureBiasValue?.description }
      }
    },
    speed: {
      label: "曝光",
      data: {
        apertureValue: { label: "光圈", data: exifTags.value.ApertureValue?.description },
        shutterSpeedValue: { label: "快门", data: exifTags.value.ShutterSpeedValue?.description },
        ISOSpeedRatings: { label: "ISO", data: exifTags.value.ISOSpeedRatings?.description.toString() }
      }
    },
    focal: {
      label: "焦距",
      data: {
        focalLength: { label: "", data: exifTags.value.FocalLength?.description }
      }
    },
    color: {
      label: "色彩",
      data: {
        whiteBalance: { label: "白平衡", data: exifTags.value.WhiteBalance?.description },
        colorSpace: { label: "色彩空间", data: exifTags.value.ColorSpace?.description }
      }
    },
    date: {
      label: "时间",
      data: {
        date: { label: "", data: exifTags.value.DateTime?.description }
      }
    }
  };
});

async function selectedFile(e: globalThis.Event) {
  const target = e.target as HTMLInputElement
  const file = target.files![0]; // TODO 判断
  console.log(file);
  // dropzoneFile.value = document.querySelector(".dropzoneFile").files[0]; ?????
  exifTags.value = await ExifReader.load(file);
  console.log(`output->tags`, exifTags.value);
  // image_progress.selectFiles();
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
        <div class="exifinput">
          <span>读取信息</span>
          <input type="file" @change="selectedFile" />
        </div>
        <div class="thumb"></div>
        <el-descriptions class="margin-top" title="summary" :column="1" :size="size" border>
          <el-descriptions-item v-for="info in summaryInfo">
            <template #label>
              <div>{{info.label}}</div>
            </template>
            <span>
              <el-descriptions class="margin-top" :column="2" :size="size" border>
                <el-descriptions-item v-for="value in info.data">
                  <template #label>
                    <div>{{value.label}}</div>
                  </template>
                  <span v-for="v in value.data">{{v}}</span>
                </el-descriptions-item>
              </el-descriptions>
            </span>
          </el-descriptions-item>
        </el-descriptions>
        <!-- <span>器材：{{ summaryInfo.models.model }}
          {{ summaryInfo.models.lensModel }}</span>
        <span>模式：{{ summaryInfo.exposure.exposureProgram }}
          {{ summaryInfo.exposure.meteringMode }} {{summaryInfo.exposure.exposureBiasValue}}
        </span>
        <span>曝光：{{summaryInfo.speed.apertureValue}} {{summaryInfo.speed.shutterSpeedValue}}
          {{summaryInfo.speed.ISOSpeedRatings}}</span>
        <span>焦距：{{summaryInfo.focal.focalLength}}</span>
        <span>色彩：{{summaryInfo.color.whiteBalance}} {{summaryInfo.color.colorSpace}}</span>
        <span>时间：{{summaryInfo.date}}</span> -->
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

<style scoped>
.thumb {
  background: v-bind(thumbBase64);
  width: 30%;
  height: 100px;
}

.exifinput {
  padding: 8px 12px;
  color: #fff;
  /* width: 50px; */
  background-color: #25dce3;
  transition: 0.3s ease all;
  border-radius:30px;
}
</style>
