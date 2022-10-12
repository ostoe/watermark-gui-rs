
<script setup lang="ts">
// @ is an alias to /src
// import DropZone from "@/components/DropZone.vue";
import { ref, onMounted, defineExpose } from "vue";
import {image_progress} from "../scripts/reactives";
import { event } from '@tauri-apps/api';
//引入路由
import { useRouter } from "vue-router";
import { Event } from "@tauri-apps/api/event";

const active = ref(false);
const router = useRouter();
const backRoute = () => {
  router.push("/");
};


function toggleActive() {
  // test_pro()
  // console.log("toggle once ");
  active.value = !active.value;
}
// export default {
//   name: "Home",
//   components: {
//     DropZone,
//   },
//   setup() {f
const dropzoneFile = ref("");
const dropzoneElement = document.querySelector("#drap-area-sq1");


function drop(e: DragEvent) {

}

function selectedFile(e: globalThis.Event) {
  const target = e.target as HTMLInputElement
      const file = target.files![0]; // TODO 判断
      console.log(file);
  // dropzoneFile.value = document.querySelector(".dropzoneFile").files[0]; ?????

  // image_progress.selectFiles();
}

const x = ref(0)
function onMousemove(e: MouseEvent) {
  console.log(typeof e);
  x.value = e.clientX
}


onMounted(() => {

  event.listen('tauri://file-drop-hover', (e) => {
  // console.log(e.x, e.y); // undifined 
  toggleActive();
});


event.listen('tauri://file-drop-cancelled', (e) => {
  toggleActive();
});
});


</script>


<template>
  <!-- <div class="home"> -->
  <el-button @click="backRoute">back</el-button>

  <h1>DropZone</h1>
  <div @dragenter.prevent="toggleActive" @dragleave.prevent="toggleActive" @dragover.prevent @drop.prevent="drop"
    @change="selectedFile" :class="{ 'active-dropzone': active }" class="dropzone" id="drap-area-sq1">
    <span>Drag or Drop File</span>
    <span>OR</span>
    <label for="dropzoneFile">Select File</label>
    <input type="file" id="dropzoneFile" class="dropzoneFile" @change="selectedFile"/>
    
    <div
  @mousemove="onMousemove"
  :style="{ backgroundColor: `hsl(${x}, 80%, 50%)` }"
  class="movearea"
>
  <p>Move your mouse across this div...</p>
  <p>x: {{ x }}</p>
</div>

  </div>

  <!-- <DropZone @drop.prevent="drop" @change="selectedFile" /> -->
  <span class="file-info">File: {{ dropzoneFile }}</span>
  <!-- </div> -->
</template>

<!-- 
<style lang="scss" scoped>
.home {
  height: auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: #f1f1f1;

  h1 {
    font-size: 40px;
    margin-bottom: 32px;
  }

  .file-info {
    margin-top: 32px;
  }
}
.movearea {
  transition: 0.3s background-color ease;
}
.dropzone {
  width: 400px;
  height: 200px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  row-gap: 16px;
  border: 2px dashed #41b883;
  background-color: #fff;
  transition: 0.3s ease all;

  label {
    padding: 8px 12px;
    color: #fff;
    background-color: #41b883;
    transition: 0.3s ease all;
  }

  input {
    display: none;
  }
}

.active-dropzone {
  color: #fff;
  border-color: #fff;
  background-color: #41b883;

  label {
    background-color: #fff;
    color: #41b883;
  }
}
</style> -->
