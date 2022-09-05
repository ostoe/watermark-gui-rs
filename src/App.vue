<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from 'vue'

// 设置ui语言
// import { ElConfigProvider } from "element-plus"
import zhCn from "element-plus/lib/locale/lang/zh-cn"
import BaseSettingsDrawer from './components/BaseSettingsDrawer.vue';
const locale = ref(zhCn)

</script>

<template>
  <el-config-provider :locale="locale">
    <!-- <router-view></router-view> -->
    <base-settings-drawer></base-settings-drawer>
  </el-config-provider>
  
</template>

<style>


</style>
