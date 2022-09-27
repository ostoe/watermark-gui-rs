<script setup lang="ts">
import { nextTick, ref, onMounted, Ref } from "vue";
// import { image_progress } from "../main";
import { elmessage, image_progress } from "../scripts/reactives";
import { emit, listen } from "@tauri-apps/api/event";
import { event, invoke } from "@tauri-apps/api";
import { sidebarReactives } from "../scripts/reactives";
import { platform, type as os_type } from '@tauri-apps/api/os';
import { Command } from '@tauri-apps/api/shell';
// import { ElMessage, ElNotification } from "element-plus";
import { open } from "@tauri-apps/api/dialog";
import { appDir, resolve, resourceDir } from "@tauri-apps/api/path";
import { pictureDir } from "@tauri-apps/api/path";
// import { watch } from "fs";
import baseSettingsDrawerVue from "./BaseSettingsDrawer.vue";
// import ExifReader from "exifreader";
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
  console.log(file, file.webkitRelativePath, );
  if (file.type === "image/jpeg") {
    const osType = await os_type(); // Returns 'Linux' 'Darwin'  'Windows_NT'
    if (osType.includes('Darwin')) {
      // TODO  run ./
      const r1 = await resourceDir();
      const exiftool_path = await resolve(r1, "resources", "exiftool");
      //  exiftool -j ~/Pictures/100NCZ_7/DSC_0595.JPG
      const reader = new FileReader(); // TODO精简exif的库，好多用不到
      // reader.readAsDataURL(file);
      console.log(exiftool_path);
      // const output = await Command.sidecar("resources/exiftool",  [ exiftool_path, "-j" , "/Users/fly/Pictures/100NCZ_7/DSC_0595.JPG"]).execute();
      const output = await new Command("win-exif-run", [ exiftool_path, "-j" , "/Users/fly/Pictures/100NCZ_7/DSC_0595.JPG"]).execute();
      console.log(output, output.stdout,);
      // Command::new("powershell")
      // .args(&[
      //   "./src/extractIcon.ps1",
      //   file_path.as_str(),
      //   icon_path.to_str().unwrap(),
      // ])
      // .creation_flags(0x08000000)

    } else if(osType.includes('Windows_NT')) {
      const r1 = await resourceDir();
      const exiftool_path = await resolve(r1, "resources", "exiftool.exe");
      console.log(exiftool_path);
      // const output = await Command.sidecar("resources/exiftool",  [ "-j",  "X:\\Z7\\001\\2022_07_19_016_DSC_0610.JPG"]).execute();
      const output = await new Command("win-exif-run", [ "-j" , "X:\\Z7\\001\\2022_07_19_016_DSC_0610.JPG"]).execute();
      console.log(output, output.stdout,);
    }
    // exifTags.value = await ExifReader.load(file);
    console.log(`output->tags`, exifTags.value);
    // image_progress.selectFiles();
  } else {
    elmessage("please choose a jpeg file");
  }
}

const emptyStatus = ref(true);
const upload = ref<UploadInstance>();
const handleExceed: UploadProps["onExceed"] = (files) => {
  upload.value!.clearFiles();
  const file = files[0] as UploadRawFile;
  upload.value!.handleStart(file);
  if (file.type === "image/jpeg") {
    emptyStatus.value = false;
    selectFile(file);
  }
};
const handleOnChange: UploadProps["onChange"] = (uploadFile) => {
  console.log(uploadFile.raw!);
  if (uploadFile.raw!.type === "image/jpeg") {
    emptyStatus.value = false;
    selectFile(uploadFile.raw!);
  } else {
    elmessage("please choose a jpeg file");
  }
};

const getMaxHeight = computed(() => {
  return sidebarReactives.curWindowHeight - 50;
});
interface skeleton {
  sideWidth: number,
  sideMargin: number,
  textWidth: number,
  textMargin: number
}
const loading = ref(true);
const skeletonTemplate = [{
  sideWidth: 10,
  sideMargin: 50,
  textWidth: 20,
  textMargin: 16
}, {
  sideWidth: 10,
  sideMargin: 50,
  textWidth: 40,
  textMargin: 16
}, {
  sideWidth: 10,
  sideMargin: 50,
  textWidth: 85,
  textMargin: 16
}, {
  sideWidth: 10,
  sideMargin: 50,
  textWidth: 45,
  textMargin: 16
}, {
  sideWidth: 10,
  sideMargin: 50,
  textWidth: 65,
  textMargin: 16
}, {
  sideWidth: 10,
  sideMargin: 50,
  textWidth: 15,
  textMargin: 16
}] as Array<skeleton>
</script>
<template lang="">
  <div>{{ testData }}</div>
  <el-row class="exifinput" style="margin-top: 22px; margin-bottom: 10px">
    <el-col :span="20">
      <div style="margin-left: 25px; font-size: 20px">
        读取信息
      </div>
    </el-col>
    <el-col :span="4">
      <!-- <input type="file" @change="selectedFile" /> -->
      <el-upload
        ref="upload"
        :limit="1"
        :auto-upload="false"
        :drag="false"
        :on-exceed="handleExceed"
        :on-change="handleOnChange"
        :show-file-list="false"
      >
        <template #trigger>
          <el-icon style="margin-right: 20px; font-size: 25px">
            <i-ep-upload />
          </el-icon>
        </template>
      </el-upload>
      <!-- <el-skeleton style="width: 240px" :loading="loading" animated>
        <template #template>
          <el-skeleton-item variant="text" style="width: 1%" />
        </template>
        <template #default>
          <span>1111</span>
        </template>
      </el-skeleton> -->
    </el-col>
  </el-row>
  <el-divider style="margin-top: 0px"></el-divider>
  <template v-if="emptyStatus">
    <el-empty :image-size="200" description="请先上传图片" />
  </template>
  <el-scrollbar height="100%" :max-height="getMaxHeight" v-else>
    <el-skeleton style="width: 100%; height: 100%" :loading="loading" animated :throttle="500">
      <template #template>
        <el-card class="box-card">
          <div class="wrapper">
            <div class="pic-skeleton">
              <el-skeleton-item
                variant="image"
                style="width: 100%; height: 50%; align-self: center"
              />
            </div>
            <div class="eldescription-skeleton">
              <div
                style="
                  display: flex;
                  flex-direction: column;
                  width: 100%;
                  align-self: center;
                "
                v-for="value in skeletonTemplate"
              >
                <div style="display: flex; margin-bottom: 15px">
                  <el-skeleton-item
                    variant="text"
                    :style="{'width': value.sideWidth+'%','margin-left': value.sideMargin+'px'}"
                  />
                  <el-skeleton-item
                    variant="text"
                    :style="{'width': value.textWidth+'%','margin-left': value.textMargin+'px'}"
                  />
                </div>
              </div>
            </div>
          </div>
        </el-card>
        <el-divider></el-divider>
        <div style="display:flex;">
          <div style="margin-bottom:18px;width:50%">
            <div style="width:30%">
              <el-skeleton-item variant="text" />
            </div>
          </div>
          <div style="margin-bottom:18px;width:50%">
            <div style="width:30%">
              <el-skeleton-item variant="text" />
            </div>
          </div>
        </div>
        <div v-for="index of 6">
          <div style="margin-bottom:28px;display:flex;">
            <el-skeleton-item variant="text"/>
            <el-skeleton-item variant="text" />
          </div>
        </div>
      </template>
      <template #default>
        <el-card class="box-card">
          <div class="wrapper">
            <div class="pic">
              <div class="thumb"></div>
            </div>
            <el-descriptions
              class="eldescription"
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
                  <el-descriptions :column="2" :size="size" border>
                    <el-descriptions-item v-for="value in info.data">
                      <template #label v-if="value.data">
                        <div>{{ value.label }}</div>
                      </template>
                      <span v-for="v in value.data" v-if="value.data">{{
                        v
                      }}</span>
                    </el-descriptions-item>
                  </el-descriptions>
                </div>
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-card>
        <el-divider></el-divider>
        <el-table
          :data="tableExifData"
          stripe
          style="width: 100%"
          class="table"
          :default-sort="{ prop: 'label', order: 'descending' }"
        >
          <el-table-column
            class="table-col"
            prop="label"
            label="名称"
            resizable
            sortable
          />
          <el-table-column
            class="table-col"
            prop="data"
            label="详细参数"
            resizable
          />
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
      </template>
    </el-skeleton>
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
.wrapper {
  display: flex;
  width: 100%;
  height: 300px;
  overflow: auto;
}

.eldescription,
.eldescription-skeleton {
  width: 70%;
}

.eldescription-skeleton {
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: 100%;
}

.pic,
.pic-skeleton {
  width: 30%;
  height: 100%;
  display: flex;
}

.pic {
  padding: 15px;
}

.pic-skeleton {
  margin-left: 15px;
}

.pic>.thumb {
  width: 100%;
  height: 50%;
  align-self: center;
  background: v-bind(thumbBase64);
  background-repeat: no-repeat;
  background-size: contain;
  background-position: center;
}


@media (max-width:600px) {
  .wrapper{
    flex-direction: column;
    height: 50%;
    -webkit-transition: width 1s ease;
    -moz-transition: width 1s ease;
    -o-transition: width 1s ease;
    -ms-transition: width 2s ease;
    transition: width 1s ease;
  }
  .pic{
    width: 100%;
  }

  .pic-skeleton{
    width: 50%;
  }
  .pic,.pic-skeleton{
    height: 190px;
    align-self: center;
  }
  .eldescription{
    width: 95%;
    align-self: center;
  }
}

.file-select>.select-button {
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

.file-select>input[type="file"] {
  display: none;
}

.exifinput {
  overflow: hidden;
}

:deep(.cell) {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.table-col {
  width: 100%;
}
</style>
