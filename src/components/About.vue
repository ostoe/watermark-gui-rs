<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
const appVersion = ref("aaa");
async function getMyVersion() { appVersion.value = await getVersion() };

const size = ref('')
const blockMargin = computed(() => {
  const marginMap = {
    large: '32px',
    default: '28px',
    small: '24px',
  }
  return {
    marginTop: marginMap[size.value] || marginMap.default,
  }
})


onMounted(() => {
    getMyVersion();
})
</script>

<template>
    <el-header>
        <span>About Me: {{}}</span>
        <div>Version: {{appVersion}} </div>
    </el-header>


    <el-icon>
        <i-ep-camera-filled></i-ep-camera-filled>
    </el-icon>
    <el-descriptions title="Watermark-gui" :column="4" :size="size" direction="vertical"
        :style="blockMargin">
        <el-descriptions-item label="Name" :span="2">watermark-gui</el-descriptions-item>
        <el-descriptions-item label="Remarks">
            <el-tag size="small">version</el-tag>
            <span>{{appVersion}}</span>
        </el-descriptions-item>
    </el-descriptions>
</template>

<style scoped>
    .el-descriptions {
      margin-top: 20px;
    }
    </style>