
  <script lang="ts" setup>
 import { resolve, resourceDir } from '@tauri-apps/api/path';
 import { convertFileSrc } from '@tauri-apps/api/tauri';
  import { reactive, ref } from 'vue'
  //   import { ElDrawer, ElMessageBox } from 'unp'
  import { invoke } from '@tauri-apps/api';
  const formLabelWidth = '80px'
  //   let timer

  
  
  const settingsDrawer = ref(false);
  const loading = ref(false);
  const qulity = ref(85);
  const form = reactive({
      brands: [{ value: 'canon', label: "佳能" }, { value: 'nikon', label: "尼康" }, { value: 'sony', label: "索尼" }, 
                {value: "panasonic", label: "松下"}, {value: "fujifilm", label: "富士"} ],
      name: '',
      brand: '',
      delivery: false,
      type: [],
      resource: '',
      desc: '',
  })
  
  const autoSelect = ref(true);
 
  const resource_imge_patten = ref("");

function  get_image_url(value: string) {
        // const resource_dir = await resourceDir();
        // \\?\G:\workspace\watermark-app\src-tauri\target\debug\
        // return convertFileSrc("C:\\Users\\fly\\Pictures\\DSC0118.-w.jpg");
        console.log(resource_imge_patten.value);
        const p = resource_imge_patten.value.replace("---value---", value+".png");
        let ap = convertFileSrc(p);
        console.log(ap);
        return ap;
  }


  
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

  async function get_image_src_patten() {
    const r1 = await resourceDir();
    const _r2 = await resolve(r1, "resources" ,"---value---").then(value => resource_imge_patten.value = value);
  }
  
  onMounted(() => {
     get_image_src_patten();
  })

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
        <!-- <el-image style="width: 150px; height: 150px" :src="get_image_url('a')" fit="contain" loading="eager" /> -->
        <el-scrollbar wrap-class="max-height:200px">
            <el-checkbox v-model="autoSelect" label="根据exif自动设置" size="large" border />
            <el-select v-model="form.brand" :placeholder="form.brands[1].label" :disabled="autoSelect" style="margin: 20px 0 20px 0">
                <el-option v-for="brand in form.brands" :key="brand.value" :label="brand.label" :value="brand.value">
                    <span style="float:left">{{ brand.label }}</span>
                    <span style="float: right;color: var(--el-text-color-secondary);font-size: 20px;">
                        <!-- <el-image style="width: 50px; height: 50px" :src="get_image_url(brand.value)" fit="fill" loading="eager" /> -->
                        <el-image style="width: 35px; height: 35px" :src="get_image_url(brand.value)" fit="contain" loading="eager" />
                        <!-- <el-icon>
                            <i-ep-picture />
                        </el-icon> -->
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
  