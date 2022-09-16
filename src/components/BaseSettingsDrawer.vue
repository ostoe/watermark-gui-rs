
<script lang="ts" setup>
import { resolve, resourceDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
// import { reactive, ref } from 'vue'
//   import { ElDrawer, ElMessageBox } from 'unp'
import { invoke } from '@tauri-apps/api';
import { user_conf, UserDataType, UserSettings, RenameType } from '../scripts/reactives'
import { elmessage } from '../scripts/reactives';
import { Ref } from 'vue';
import { InfoFilled } from '@element-plus/icons-vue';



const formLabelWidth = '80px'
//   let timer
const renamePreffix = ref({
    SD: [{ id: 1, label: "空", value: "" }, { id: 2, label: "自定义", value: "__custom__" }, { id: 3, label: "自定义+【序号】", value: "__serial_number__" }],
    value: { id: 1, label: "空", value: "" },
    input: "",
    valid: true,
});
const renameCenter = ref({
    SD: [{ id: 1, label: "原名称", value: "" }, { id: 2, label: "自定义", value: "__custom__" }],
    value: { id: 1, label: "原名称", value: "" },
    input: "",
    valid: true,
});
const renameSuffix = ref({
    SD: [{ id: 1, label: "空", value: "" }, { id: 2, label: "自定义", value: "__custom__" }, { id: 3, label: "自定义+【序号】", value: "__serial_number__" }],
    value: { id: 2, label: "自定义", value: "__custom__" },
    input: "-WM",
    valid: true,
});

const reg = /^[a-zA-Z0-9\u4e00-\u9fa5_ ~!@#$%^&*()_+=～！@#¥%……&*–—‘’“”…、。〈〉《》「」『』【】〔〕！（），．：；？、\-\$\[\]]+$/;
function check_input_prefix() {
    check_input(renamePreffix, 0);
}

function check_input_center() {
    check_input(renameCenter, 1);
}
function check_input_suffix() {
    check_input(renameSuffix, 2);
}

function resetConfirmEvent() {
    baseForm.value.autoUseBrand = true;
    baseForm.value.qulity = 85;
    baseForm.value.brand = "nikon";
    baseForm.value.autoUseBrand = true;
    baseForm.value.renameSuffix = "";
    renamePreffix.value.input = "";
    renamePreffix.value.valid = true;
    renamePreffix.value.value = renamePreffix.value.SD[0];
    renameCenter.value.input = "";
    renameCenter.value.valid = true;
    renameCenter.value.value = renameCenter.value.SD[0];
    renameSuffix.value.input = "-WM";
    renameSuffix.value.valid = true;
    renameSuffix.value.value = renameSuffix.value.SD[1];

}

const preview_filename = ref(["", "", "", ".jpg"]);

function check_input(rename: Ref<RenameType>, preview_index: number) {
    console.log("-----" + "value");

    // let a = [renamePreffix, renameCenter, renameSuffix];
    // for (let i = 0; i < 3; i++) {
    let t = rename.value;
    if (t.value.id == 2) {
        if (reg.test(t.input)) {
            t.valid = true;
            preview_filename.value[preview_index] = t.input;
        }
        else {
            t.valid = false;
        }
    } else if (t.value.id == 3) {
        if (t.input.includes("$x") && reg.test(t.input)) {
            t.valid = true;
            preview_filename.value[preview_index] = t.input.replaceAll("$x", "1");
        } else {
            t.valid = false;
        }
    } else {
        if (preview_index == 1) {
            preview_filename.value[preview_index] = "basename";
        }
        // == 1 ???
    }
    // }
    // 判断是否全空?
}

function check_select_prefix() {
    if (renamePreffix.value.value.id == 1) {
        renamePreffix.value.valid = true
        preview_filename.value[0] = "";
    }
}
function check_select_center() {
    if (renameCenter.value.value.id == 1) {
        renameCenter.value.valid = true
        preview_filename.value[1] = "basename";
    }
}

function check_select_suffix() {
    if (renameSuffix.value.value.id == 1) {
        renameSuffix.value.valid = true
        preview_filename.value[2] = "";
    }
}




const settingsDrawer = ref(false);
const loading = ref(false);

//   const autoSelect = ref(true);

const resource_imge_patten = ref("");

function get_image_url(value: string) {
    // const resource_dir = await resourceDir();
    // \\?\G:\workspace\watermark-app\src-tauri\target\debug\
    // return convertFileSrc("C:\\Users\\fly\\Pictures\\DSC0118.-w.jpg");
    // console.log(resource_imge_patten.value);
    const p = resource_imge_patten.value.replace("__value__", value + ".png");
    let ap = convertFileSrc(p);
    // console.log(ap);
    return ap;
}

const baseForm: Ref<UserDataType> = ref({
    qulity: 0,
    latestSelectedDirPath: "",
    latestSelectedOutputPath: "",
    outputPathHistory: [],
    autoUseBrand: false,
    brand: "",
    font: "",
    renameSuffix: "",
    renamePreffix: "",
    renameCenter: "",
    brands: [],

})





// interface UserSettings {
//     output_dir: string,
//     qulity: number,
//     auto_user_brand: boolean,
//     brand: string,
//     filename_pattern: Array<string>,
// }

async function saveSetting() {
    if (renamePreffix.value.valid && renameCenter.value.valid && renameSuffix.value.valid) {
        // send backend
        let filename_tmp = ["", "", ""];
        // 
        if (renamePreffix.value.value.value != "") { filename_tmp[0] = baseForm.value.renamePreffix = renamePreffix.value.value.value + renamePreffix.value.input; }
        if (renameCenter.value.value.value != "") { filename_tmp[1] = baseForm.value.renameCenter = renameCenter.value.value.value + renameCenter.value.input; }
        if (renameSuffix.value.value.value != "") { filename_tmp[2] = baseForm.value.renameSuffix = renameSuffix.value.value.value + renameSuffix.value.input; }
        // console.log(filename_tmp);
        // output_dir: "" means backend not update "output_dir" key.
        let user_data: UserSettings = { output_dir: "", qulity: baseForm.value.qulity, auto_user_brand: baseForm.value.autoUseBrand, brand: baseForm.value.brand, filename_pattern: filename_tmp };
        //   console.log(user_data);
        let res: string = await invoke("handle_front_update_user_data", { userData: user_data });
        user_conf.save_user_conf(baseForm.value);

        elmessage(res);
    } else {
        ElMessage({
            showClose: true,
            message: '输入有误，保存失败，注意特殊字符',
            type: 'error',
        })
    }

}

function init_saved_conf() {
    user_conf.B2A(baseForm.value, user_conf);
    // parse __custom__
    // prefix
    if (baseForm.value.renamePreffix == "") {
        renamePreffix.value.value = renamePreffix.value.SD[0];
    } else if (baseForm.value.renamePreffix.startsWith("__custom__")) {
        renamePreffix.value.value = renamePreffix.value.SD[1];
        renamePreffix.value.input = baseForm.value.renamePreffix.replace("__custom__", "");
    } else if (baseForm.value.renamePreffix.startsWith("__serial_number__")) {
        renamePreffix.value.value = renamePreffix.value.SD[2];
        renamePreffix.value.input = baseForm.value.renamePreffix.replace("__serial_number__", "");
    }
    // center
    if (baseForm.value.renameCenter == "") {
        renameCenter.value.value = renameCenter.value.SD[0];

    } else if (baseForm.value.renameCenter.startsWith("__custom__")) {
        renameCenter.value.value = renameCenter.value.SD[1];
        renameCenter.value.input = baseForm.value.renameCenter.replace("__custom__", "");
    }
    // suffix
    if (baseForm.value.renameSuffix == "") {
        renameSuffix.value.value = renameSuffix.value.SD[0];
    } else if (baseForm.value.renameSuffix.startsWith("__custom__")) {
        renameSuffix.value.value = renameSuffix.value.SD[1];
        renameSuffix.value.input = baseForm.value.renameSuffix.replace("__custom__", "");
    } else if (baseForm.value.renameSuffix.startsWith("__serial_number__")) {
        renameSuffix.value.value = renameSuffix.value.SD[2];
        renameSuffix.value.input = baseForm.value.renameSuffix.replace("__serial_number__", "");
    }

    check_input(renamePreffix, 0);
    check_input(renameCenter, 1);
    check_input(renameSuffix, 2);


}


const handleClose = (done: (_: boolean) => void) => {
    console.log("close drawer" + done);
    settingsDrawer.value = false;
}

const timerButtonArea = ref();
const timerSettingsArea = ref();
function menuShow() {
    timerButtonArea.value = setTimeout(() => {
        loadDrawer()
    }, 600);
}

function menuHidden() {
    if (!settingsDrawer.value) {
        clearTimeout(timerButtonArea.value);
        // settingsDrawer.value = false
    }
}

function loadDrawer() {
    user_conf.load_conf(baseForm.value);
    settingsDrawer.value = true
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
    const _r2 = await resolve(r1, "resources", "__value__").then(value => resource_imge_patten.value = value);
}

onMounted(() => {
    get_image_src_patten();
    init_saved_conf();
})

</script>
    

<template>
    <div @mouseenter="menuShow" @mouseleave="menuHidden">
        <el-button @click="loadDrawer" type="primary" color="#3f8418" plain>
            <el-icon>
                <i-ep-d-arrow-left />
            </el-icon>
        </el-button>

    </div>
    <!-- <div @mouseenter="settingShow" @mouseleave="settingHidden">  -->

    <el-drawer v-model="settingsDrawer" title="基本设置" :before-close="handleClose" direction="rtl" size="30%">
        <!-- <el-image style="width: 150px; height: 150px" :src="get_image_url('a')" fit="contain" loading="eager" /> -->
        <el-scrollbar wrap-class="max-height:200px">
            <el-checkbox v-model="baseForm.autoUseBrand" label="根据exif自动设置" size="large" border />
            <el-select v-model="baseForm.brand" :placeholder="baseForm.brands[1].label"
                :disabled="baseForm.autoUseBrand" style="margin: 20px 0 20px 0">
                <el-option v-for="brand in baseForm.brands" :key="brand.value" :label="brand.label"
                    :value="brand.value">
                    <span style="float:left">{{ brand.label }}</span>
                    <span style="float: right;color: var(--el-text-color-secondary);font-size: 20px;">
                        <!-- <el-image style="width: 50px; height: 50px" :src="get_image_url(brand.value)" fit="fill" loading="eager" /> -->
                        <el-image style="width: 35px; height: 35px" :src="get_image_url(brand.value)" fit="contain"
                            loading="eager" />
                        <!-- <el-icon>
                            <i-ep-picture />
                        </el-icon> -->
                    </span>
                </el-option>
            </el-select>
            <el-input-number v-model="baseForm.qulity" :min="1" :max="100" controls-position="right" size="large"
                step-strictly @change="handleChangeQulity" />

            <el-slider v-model="baseForm.qulity" vertical height="200px" />

            <el-row>
                <el-col :span="8">
                    <div class="grid-content ep-bg-purple" />
                    <p style="margin-left: 10px">名称前缀</p>
                    <el-select v-model="renamePreffix.value" class="m-2" placeholder="Select" value-key="id"
                        size="large" :change="check_select_prefix()">
                        <el-option v-for="item in renamePreffix.SD" :key="item.id" :label="item.label" :value="item" />
                    </el-select>

                </el-col>
                <el-col :span="8">
                    <div class="grid-content ep-bg-purple-light" />
                    <p style="margin-left: 10px">名称中间</p>
                    <el-select v-model="renameCenter.value" class="m-2" placeholder="Select" value-key="id" size="large"
                        :change="check_select_center()">
                        <el-option v-for="item in renameCenter.SD" :key="item.id" :label="item.label" :value="item" />
                    </el-select>
                </el-col>
                <el-col :span="8">
                    <div class="grid-content ep-bg-purple" />
                    <p style="margin-left: 10px">名称后缀</p>
                    <el-select v-model="renameSuffix.value" class="m-2" placeholder="Select" value-key="id" size="large"
                        :change="check_select_suffix()">
                        <el-option v-for="item in renameSuffix.SD" :key="item.id" :label="item.label" :value="item" />
                    </el-select>
                </el-col>
            </el-row>
            <el-row>
                <el-col :span="8">
                    <div class="grid-content ep-bg-purple" />
                    <el-input v-model="renamePreffix.input" :disabled="renamePreffix.value.id == 1"
                        :blur="check_input_prefix()"> "自定义后缀"
                    </el-input>

                </el-col>
                <el-col :span="8">
                    <div class="grid-content ep-bg-purple-light" />
                    <el-input v-model="renameCenter.input" :disabled="renameCenter.value.id == 1"
                        :blur="check_input_center()">
                        "自定义后缀" </el-input>

                </el-col>
                <el-col :span="8" id="invalidInputCss">
                    <div class="grid-content ep-bg-purple" />
                    <el-input v-model="renameSuffix.input" :disabled="renameSuffix.value.id == 1"
                        :blur="check_input_suffix()">
                        "自定义后缀" </el-input>
                </el-col>
            </el-row>
            <el-row>
                <el-button key="button.text" type="primary" text> {{ preview_filename.join("") }} </el-button>

            </el-row>

        </el-scrollbar>

        <div style="margin: 10px 0 20% 0; border-bottom: 0%;">
            <el-row>
                <el-popconfirm confirm-button-text="是" cancel-button-text="否" :icon="InfoFilled" icon-color="#626AEF"
                    title="重置（仍未保存）" @confirm="resetConfirmEvent" @cancel="">
                    <template #reference>
                        <el-button type="primary" size="small" plain color="#0FCAC7">默认</el-button>
                    </template>
                </el-popconfirm>
                <el-button type="primary" size="large" @click="saveSetting"> 保存 </el-button>
            </el-row>
        </div>

        <!-- <el-button type="primary" :loading="loading" @click="onClick">{{
            loading ? 'Submitting ...' : 'Submit'
          }}</el-button> -->
    </el-drawer>
    <!-- </div>  -->

</template>
  

<style lang="scss">
.el-row {
    margin-bottom: 20px;
}

.el-row:last-child {
    margin-bottom: 0;
}

.el-col {
    border-radius: 4px;
}

.grid-content {
    border-radius: 4px;
    min-height: 36px;
}

#invalidInputCss .el-input {
    --el-input-border-color: #f70909;
}

#defaultInputCss .el-input {
    --el-input-border-color: #dcdfe6;
}

// .el-input__inner {
//     --el-input-focus-border: #b24444;
//     --el-input-text-color: #b24444;
//     --el-text-color-regular:#b24444;
//     // --el-input-focus-border: #b24444;
// }

// .el-input__inner::placeholder {
//   color:  #b24444;
// }
// /* google */
// .el-input__inner::-webkit-input-placeholder {
//   color:  #b24444;
// }
// /* firefox */
// .el-input__inner:-moz-placeholder {
//   color:  #b24444;
// }
// /*ie*/
// .el-input__inner:-ms-input-placeholder {
//   color:  #b24444;
// }
</style>