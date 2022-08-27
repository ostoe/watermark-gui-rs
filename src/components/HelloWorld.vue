<template>
  <!-- <div class="home"> -->
    <h1>DropZone</h1>

    <div @dragenter.prevent="toggleActive" @dragleave.prevent="toggleActive" @dragover.prevent
      @drop.prevent="drop" @change="selectedFile" :class="{ 'active-dropzone': active }" class="dropzone">
      <span>Drag or Drop File</span>
      <span>OR</span>
      <label for="dropzoneFile">Select File</label>
      <input type="file" id="dropzoneFile" class="dropzoneFile" />
    </div>

    <!-- <DropZone @drop.prevent="drop" @change="selectedFile" /> -->
    <span class="file-info">File: {{ dropzoneFile.name }}</span>
  <!-- </div> -->
</template>

<script setup>
// @ is an alias to /src
// import DropZone from "@/components/DropZone.vue";
import { ref } from "vue";
const active = ref(false);

function toggleActive() {
  console.log("toggle once ");
  active.value = !active.value;
}
// export default {
//   name: "Home",
//   components: {
//     DropZone,
//   },
//   setup() {
let dropzoneFile = ref("");

function drop(e) {
  console.log(e);
  dropzoneFile.value = e.dataTransfer.files[0];
};

function selectedFile() {
  dropzoneFile.value = document.querySelector(".dropzoneFile").files[0];
};

//     return { dropzoneFile, drop, selectedFile };
//   },
// };
</script>

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
</style>
