
  <script lang="ts" setup>
  import { reactive, ref } from 'vue'
  //   import { ElDrawer, ElMessageBox } from 'unp'
  import { invoke } from '@tauri-apps/api';
  const formLabelWidth = '80px'
  //   let timer
  
  const settingsDrawer = ref(true);
  const loading = ref(false);
  const qulity = ref(85);
  const form = reactive({
      brands: [{ brand: 'canon', label: "佳能" }, { brand: 'nikon', label: "尼康" }, { brand: 'sony', label: "索尼" },],
      name: '',
      brand: '',
      delivery: false,
      type: [],
      resource: '',
      desc: '',
  })
  
  const autoSelect = ref(true);
  
  
  interface UserSettings {
      output_dir: string,
      qulity: number,
      auto_user_brand: boolean,
      brand: string,
  }
  
  async function saveSetting() {
      //  invoke("handle_front_select_files", { imagesObj: image_progress.image_paths });
      let user_data: UserSettings = { output_dir: "", qulity: qulity.value, auto_user_brand: autoSelect.value, brand: form.brand };
      console.log(user_data);
  
      let res = await invoke("handle_front_update_user_data", { userData: user_data });
      console.log(res);
  }
  
  
  const handleClose = (done: (_: boolean) => void) => {
      console.log("close drawer" + done);
      settingsDrawer.value = false;
  }
  
  const timerButtonArea = ref();
  const timerSettingsArea = ref();
  function menuShow() {
      timerButtonArea.value = setTimeout(() => {
          settingsDrawer.value = true
      }, 600);
  }
  
  function menuHidden() {
      if (!settingsDrawer.value) {
          clearTimeout(timerButtonArea.value);
          // settingsDrawer.value = false
      }
  }
  
  function settingShow() {
      console.log("leave");
      if (settingsDrawer.value) {
          clearTimeout(timerSettingsArea.value);
      }
  }
  
  function settingHidden() {
      console.log("leave");
      timerSettingsArea.value = setTimeout(() => {
          console.log("leave");
          settingsDrawer.value = false;
      }, 5000);
  }
  
  function handleChangeQulity() { }
  
  
  </script>
    

<template>
    <div @mouseenter="menuShow" @mouseleave="menuHidden">
        <el-button @click="settingsDrawer = true" type="primary" color="#3f8418" plain>
            <el-icon>
                <i-ep-d-arrow-left />
            </el-icon>
        </el-button>

    </div>
    <!-- <div @mouseenter="settingShow" @mouseleave="settingHidden">  -->

    <el-drawer v-model="settingsDrawer" title="基本设置" :before-close="handleClose" direction="rtl" size="30%">
        <el-scrollbar wrap-class="max-height:200px">
            <el-checkbox v-model="autoSelect" label="根据exif自动设置" size="large" border />
            <el-select v-model="form.brand" :placeholder="form.brands[1].label" :disabled="autoSelect">
                <el-option v-for="brand in form.brands" :key="brand.brand" :label="brand.label" :value="brand.brand">
                    <span style="float:left">{{ brand.label }}</span>
                    <span style="float: right;color: var(--el-text-color-secondary);font-size: 20px;">
                        <el-icon>
                            <i-ep-picture />
                        </el-icon>
                    </span>
                </el-option>
            </el-select>
            <el-input-number v-model="qulity" :min="1" :max="100" controls-position="right" size="large" step-strictly
                @change="handleChangeQulity" />

            <el-slider v-model="qulity" vertical height="200px" />
        </el-scrollbar>

        <div style="margin: 10px 0 20% 0; border-bottom: 0%;">
            <el-row>
                <el-button type="primary" size="small"> reset </el-button>
                <el-button type="primary" size="large" @click="saveSetting"> 保存 </el-button>
            </el-row>
        </div>

        <!-- <el-button type="primary" :loading="loading" @click="onClick">{{
            loading ? 'Submitting ...' : 'Submit'
          }}</el-button> -->
    </el-drawer>
    <!-- </div>  -->

</template>
  