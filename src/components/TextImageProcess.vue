<script setup lang="ts">
import { nextTick, ref, onMounted, Ref } from "vue";
// import { image_progress } from "../main";
import { elmessage, image_progress } from "../scripts/reactives";
import { emit, listen } from "@tauri-apps/api/event";
import { event, invoke } from "@tauri-apps/api";
import { sidebarReactives } from "../scripts/reactives";

// import { ElMessage, ElNotification } from "element-plus";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";
import { pictureDir } from "@tauri-apps/api/path";
// import { watch } from "fs";
import baseSettingsDrawerVue from "./BaseSettingsDrawer.vue";
import ExifReader from "exifreader";
import type {
  UploadFile,
  UploadFiles,
  UploadInstance,
  UploadProps,
  UploadRawFile,
} from "element-plus";

const isCollapse = ref(true);
// const progress_count = ref({ completed: 0, total: 0 });
const count = ref(0);
const text = ref("./tests/img/jpg/gps/DSCN0010.jpg");
const selectImage = ref("");
const isPlain = ref(true);

// const message=(msg: string)=> {
//     ElNotification({
//       message: msg,
//       type: "success",
//       title: "🐮----🍺",
//       position: "bottom-left",
//     });
//   }

// {count: selected.length, image_paths: [selected]}

function changeCollapse() {
  console.log(isCollapse.value);
  isCollapse.value = isCollapse.value ? false : true;
}
// 这是个异步函数
async function greetTest() {
  if (text.value == "") {
    text.value = "World!";
  }
  let res = await invoke("greet", { name: text.value });
  console.log(res);
}

// async function update_user_data2BD(key: string, value: string) {
//   let res = await invoke("handle_front_update_data", { key: key, value: value });
//   message("update output dir: " + res);
// };

async function drap_hover_event_handle() {
  const dropzoneElement = document.querySelector("#drap-area-sq1");
  const unlisten = await listen("tauri://file-drop-hover", (e) => {
    console.log(e);
    // const hoveredElement = document.elementFromPoint(e.x, e.y);

    // if (dropzoneElement != null && dropzoneElement.contains(hoveredElement)) {
    //   // ...
    //   console.log("drag in la" + hoveredElement);
    //   // toggleActive();
    // }
  });
}

const send_event_test = async () => {
  await nextTick(() => {
    console.log("will send_event");
    let res = invoke("send_event");
    console.log("send_event ok");
  });
};

const send_event = async () => {
  let res = await invoke("send_event");

  // elmessage("[测试发送]: "+ res);
};

function handleFileChange(e: InputEvent) {
  const el = e.target as HTMLInputElement;
  if (!el.files || el.files?.length === 0) {
    return;
  }
  // console.log(el.files);
  console.log(el ? el.files[0] : null);
  // this.$emit('input', e.target.files[0])
  // console.log(e.target.files[0].path);
  // console.log(typeof (e.target));
  selectImage.value = "./tests/img/jpg/gps/DSCN0010.jpg";
}

onMounted(() => {
  // backend_event_recv();
  // // for (let i = 0; i < 5; i += 1) {
  // //   tableData.value.push({ id: i, msg: " " + i + " " + i + " " + i });
  // // }
  // drag_event_handle();
  // // this.set_drap_hover_evet(); // e.x e.y invalid.
  // init_output_dir();
});

// test event

const exifTags = ref<ExifReader.Tags>({});
interface exifDetailData {
  label?: string;
  data?: string;
}

interface exifData {
  label: string;
  data: Array<exifDetailData>;
}

interface summaryData {
  models: exifData;
  exposure: exifData;
  speed: exifData;
  focal: exifData;
  color: exifData;
  date: exifData;
}
const summaryTemplate = {
  models: {
    label: "器材",
    data: [],
  },
  exposure: {
    label: "模式",
    data: [],
  },
  speed: {
    label: "曝光",
    data: [],
  },
  focal: {
    label: "焦距",
    data: [],
  },
  color: {
    label: "色彩",
    data: [],
  },
  date: {
    label: "时间",
    data: [],
  },
} as summaryData;
const summaryInfo = ref<summaryData>(summaryTemplate);
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

// 概览
const size = ref("small");
const tableExifData = ref<Array<exifDetailData>>([]);
watch(exifTags, (newValue, oldValue) => {
  //清空对象
  for (let value of Object.values(summaryInfo.value)) {
    value.data = [];
  }
  if (newValue.Model?.value[0]) {
    summaryInfo.value.models.data.push({
      label: "相机",
      data: newValue.Model?.value[0],
    } as exifDetailData);
  }
  if (newValue.LensModel?.value) {
    summaryInfo.value.models.data.push({
      label: "详细",
      data: newValue.LensModel?.description,
    } as exifDetailData);
  }
  // summaryInfo.value.models.data = [{ label: "相机", data: newValue.Model?.value[0] } as exifDetailData, { label: "详细", data: newValue.LensModel?.value } as exifDetailData]
  if (newValue.ExposureProgram?.description) {
    summaryInfo.value.exposure.data.push({
      label: "曝光模式",
      data: newValue.ExposureProgram?.description,
    } as exifDetailData);
  }
  if (newValue.MeteringMode?.description) {
    summaryInfo.value.exposure.data.push({
      label: "测光模式",
      data: newValue.MeteringMode?.description,
    } as exifDetailData);
  }
  if (newValue.ExposureBiasValue?.description) {
    let parseExposureBiasValue = parseFloat(
      exifTags.value.ExposureBiasValue!.description
    ).toFixed(2);
    summaryInfo.value.exposure.data.push({
      label: "曝光补偿",
      data: parseExposureBiasValue,
    } as exifDetailData);
  }
  // summaryInfo.value.exposure.data = [{ label: "曝光模式", data: newValue.ExposureProgram?.description } as exifDetailData, { label: "测光模式", data: newValue.MeteringMode?.description } as exifDetailData, { label: "曝光补偿", data: newValue.ExposureBiasValue?.description } as exifDetailData]
  if (newValue.ApertureValue?.description) {
    summaryInfo.value.speed.data.push({
      label: "光圈",
      data: newValue.ApertureValue?.description,
    } as exifDetailData);
  }
  if (newValue.ShutterSpeedValue?.description) {
    summaryInfo.value.speed.data.push({
      label: "快门",
      data: newValue.ShutterSpeedValue?.description,
    } as exifDetailData);
  }
  if (newValue.ISOSpeedRatings?.description) {
    summaryInfo.value.speed.data.push({
      label: "ISO",
      data: newValue.ISOSpeedRatings?.description.toString(),
    } as exifDetailData);
  }
  // summaryInfo.value.speed.data = [{ label: "光圈", data: newValue.ApertureValue?.description } as exifDetailData, { label: "快门", data: newValue.ShutterSpeedValue?.description } as exifDetailData, { label: "ISO", data: newValue.ISOSpeedRatings?.description.toString() } as exifDetailData]
  if (newValue.FocalLength?.description) {
    summaryInfo.value.focal.data.push({
      label: "焦距",
      data: newValue.FocalLength?.description,
    } as exifDetailData);
  }
  // summaryInfo.value.focal.data = [{ label: "焦距", data: newValue.FocalLength?.description } as exifDetailData ]
  if (newValue.WhiteBalance?.description) {
    summaryInfo.value.color.data.push({
      label: "白平衡",
      data: newValue.WhiteBalance?.description,
    } as exifDetailData);
  }
  if (newValue.ColorSpace?.description) {
    summaryInfo.value.color.data.push({
      label: "色彩空间",
      data: newValue.ColorSpace?.description,
    } as exifDetailData);
  }
  // summaryInfo.value.color.data = [{ label: "白平衡", data: newValue.WhiteBalance?.description } as exifDetailData, { label: "色彩空间", data: newValue.ColorSpace?.description } as exifDetailData]
  if (newValue.DateTime?.description) {
    summaryInfo.value.date.data.push({
      label: "创建时间",
      data: newValue.DateTime?.description,
    } as exifDetailData);
  }
  // summaryInfo.value.date.data = [{ label: "创建时间", data: newValue.DateTime?.description } as exifDetailData]
  tableExifData.value = [];
  var reg = "^[ ]+$";
  var re = new RegExp(reg);
  console.log(newValue);
  for (const [key, value] of Object.entries(newValue)) {
    if (key == "MakerNote") {
      console.log(value.value);
      console.log(typeof value.value);
    }
    // console.log(key, value);
    if (
      typeof value.description === "string" &&
      value.description != "" &&
      !re.test(value.description)
    ) {
      tableExifData.value.push({ label: key, data: value.description });
    }
  }
});

// const summaryInfo = computed(() => {
//   return {
//     models: {
//       label: "器材",
//       data: {
//         model: { label: "相机", data: exifTags.value.Model?.value[0] },
//         lensModel: { label: "", data: exifTags.value.LensModel?.value },
//       }
//     },
//     exposure: {
//       label: "模式",
//       data: {
//         exposureProgram: { label: "曝光模式", data: exifTags.value.ExposureProgram?.description },
//         meteringMode: { label: "测光模式", data: exifTags.value.MeteringMode?.description },
//         exposureBiasValue: { label: "曝光补偿", data: exifTags.value.ExposureBiasValue?.description }
//       }
//     },
//     speed: {
//       label: "曝光",
//       data: {
//         apertureValue: { label: "光圈", data: exifTags.value.ApertureValue?.description },
//         shutterSpeedValue: { label: "快门", data: exifTags.value.ShutterSpeedValue?.description },
//         ISOSpeedRatings: { label: "ISO", data: exifTags.value.ISOSpeedRatings?.description.toString() }
//       }
//     },
//     focal: {
//       label: "焦距",
//       data: {
//         focalLength: { label: "", data: exifTags.value.FocalLength?.description }
//       }
//     },
//     color: {
//       label: "色彩",
//       data: {
//         whiteBalance: { label: "白平衡", data: exifTags.value.WhiteBalance?.description },
//         colorSpace: { label: "色彩空间", data: exifTags.value.ColorSpace?.description }
//       }
//     },
//     date: {
//       label: "时间",
//       data: {
//         date: { label: "", data: exifTags.value.DateTime?.description }
//       }
//     }
//   };
// });

// async function selectedFile(e: globalThis.Event) {
//   const target = e.target as HTMLInputElement;
//   const file = target.files![0]; // TODO 判断
//   console.log(file);
//   if (file.type === "image/jpeg") {
//     // dropzoneFile.value = document.querySelector(".dropzoneFile").files[0]; ?????
//     exifTags.value = await ExifReader.load(file);
//     console.log(`output->tags`, exifTags.value);
//     // image_progress.selectFiles();
//   } else {
//     elmessage("please choose a jpeg file");
//   }
// }

async function selectFile(file: File) {
  console.log(file);
  if (file.type === "image/jpeg") {
    // dropzoneFile.value = document.querySelector(".dropzoneFile").files[0]; ?????
    exifTags.value = await ExifReader.load(file);
    console.log(`output->tags`, exifTags.value);
    // image_progress.selectFiles();
  } else {
    elmessage("please choose a jpeg file");
  }
}
const upload = ref<UploadInstance>();
const handleExceed: UploadProps["onExceed"] = (files) => {
  upload.value!.clearFiles();
  const file = files[0] as UploadRawFile;
  upload.value!.handleStart(file);
  selectFile(file);
};
const handleOnChange: UploadProps["onChange"] = (uploadFile) => {
  console.log(uploadFile);
  selectFile(uploadFile.raw!);
};

const getMaxHeight=computed(()=>{
  return (sidebarReactives.curWindowHeight-50)
})

// :on-exceed="handleExceed"
</script>
<template lang="">
  <el-row class="exifinput" style="margin-top:22px;margin-bottom:10px;">
    <el-col :span="20">
      <div style="margin-left: 25px;font-size: 20px;">读取信息</div>
    </el-col>
    <el-col :span="4">
      <!-- <input type="file" @change="selectedFile" /> -->
      <el-upload
        ref="upload"
        :limit="1"
        :auto-upload="false"
        :drag="false"
        :on-change="handleOnChange"
        :show-file-list="false"
      >
        <template #trigger>
          <el-icon style="margin-right: 20px; font-size: 25px">
            <i-ep-upload />
          </el-icon>
        </template>
      </el-upload>
    </el-col>
  </el-row>
  <el-divider style="margin-top: 0px"></el-divider>
  <el-scrollbar height="100%" :max-height="getMaxHeight">
    <div class="thumb"></div>
    <el-descriptions
      class="margin-top"
      title="概览"
      :column="1"
      :size="size"
      border
    >
      <el-descriptions-item v-for="info in summaryInfo">
        <template #label>
          <div>{{ info.label }}</div>
        </template>
        <div>
          <el-descriptions class="margin-top" :column="2" :size="size" border>
            <el-descriptions-item v-for="value in info.data">
              <template #label v-if="value.data">
                <div>{{ value.label }}</div>
              </template>
              <span v-for="v in value.data" v-if="value.data">{{ v }}</span>
            </el-descriptions-item>
          </el-descriptions>
        </div>
      </el-descriptions-item>
    </el-descriptions>
    <el-table
      :data="tableExifData"
      stripe
      style="width: 100%"
      class="table"
      :default-sort="{ prop: 'label', order: 'descending' }"
    >
      <el-table-column prop="label" label="名称" width="380" />
      <el-table-column prop="data" label="详细参数" width="380" class="col" />
    </el-table>
    <el-container class="a-border">
      <div id="drap-area-sq1">
        <!-- <div style="margin: 20px 0" /> -->
        <div style="margin: 20px 5% 20px 5%">
          <el-input
            v-model="text"
            type="textarea"
            size="large"
            :autosize="{ minRows: 2, maxRows: 6 }"
            placeholder="Please input"
          />
        </div>
        <div>
          <!-- <el-container direction="vertical"> -->
          <suspense>
            <!-- <el-col > -->
            <el-container direction="horizontal">
              <el-button
                @click="send_event"
                color="#de4781"
                size="large"
                :plain="isPlain"
                >[s]测试event</el-button
              >
              <!-- </suspense>
        <suspense> -->
              <el-button
                @click="greetTest"
                color="#322aef"
                size="large"
                :plain="isPlain"
                >[i]测试Rust
              </el-button>
              <!-- </el-col> -->
            </el-container>
          </suspense>
          <el-row class="left">
            <el-col :span="24" style="margin: 15px 0 15px 0">
              <!-- <div class="photoSelector"> -->
              <label class="file-select">
                <div class="select-button">
                  <span v-if="selectImage"
                    >Selected File: {{ selectImage.name }}</span
                  >
                  <span v-else>Select File</span>
                </div>
                <input type="file" @change="handleFileChange" />
              </label>
              <div class="b-border">
                <el-button class="btn" @click="image_progress.selectFiles()"
                  >选择文件</el-button
                >
                <!-- </div> -->
                <!-- <div class="photoSelector"> -->
                <el-button class="btn" @click="image_progress.selectDirs()"
                  >选择目录</el-button
                >
              </div>

              <!-- </div> -->
            </el-col>
            <text> {{ selectImage }}</text>
          </el-row>
          <!-- <button @click="greetTest" >测试调用rust</button> -->
          <!-- </el-col> -->

          <!-- </el-container> -->
        </div>
      </div>
      <!-- <BaseSettingsDrawerVue /> -->
      <!-- <div style="margin-right=10px margin:auto" >
      </div> -->
    </el-container>
  </el-scrollbar>
</template>

<style>

.a-border {
  border: 1px solid rgb(8, 210, 255);
  margin: 20px 10% 20px 10%;
  text-align: center;
  box-shadow: 0 0 10px rgb(79, 223, 255);
  padding: 20px 25px 20px 25px;
  align-items: center;
}

.b-border {
  /* border: 1px solid #de4781; */
  margin: 10px auto 10px auto;
  text-align: center;
  /* line-height: 2; */
  /* text-align: center; */
  /* box-shadow: 0 0 10px rgb(79, 223, 255); */
  /* padding: 20px 25px 20px 25px; */
}
</style>

<style scoped>
.file-select > .select-button {
  padding: 1rem;
  width: 10rem;
  color: white;
  background-color: #2ea169;
  justify-content: center;
  border-radius: 0.3rem;
  text-align: center;
  font-weight: bold;
  display: flex;
}

.file-select > input[type="file"] {
  display: none;
}

.thumb {
  background: v-bind(thumbBase64);
  width: 30%;
  height: 100px;
}

.exifinput {
}

:deep(.cell) {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
