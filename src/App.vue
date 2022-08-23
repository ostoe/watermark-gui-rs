<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from 'vue'

// 设置ui语言
import { ElConfigProvider } from "element-plus"
import zhCn from "element-plus/lib/locale/lang/zh-cn"
const locale = ref('zhCn')

</script>

<template>
  <!-- <div>
    <button @click="count1++">Count is: {{ count1 }}</button>
  </div> -->
  <el-config-provider :locale="locale">
    <router-view></router-view>
  </el-config-provider>
  
</template>

<style>

</style>
