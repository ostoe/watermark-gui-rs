
<script lang="ts" setup>
import { resolve, resourceDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
// import { reactive, ref } from 'vue'
//   import { ElDrawer, ElMessageBox } from 'unp'
import { invoke } from '@tauri-apps/api';
import { user_conf, UserDataType, UserSendSettings, RenameType, UserBaseSettings } from '../scripts/reactives'
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
    SD: [{ id: 1, label: "原名称", value: "__keep__" }, { id: 2, label: "自定义", value: "__custom__" }],
    value: { id: 1, label: "原名称", value: "__keep__" },
    input: "",
    valid: true,
});
const renameSuffix = ref({
    SD: [{ id: 1, label: "空", value: "" }, { id: 2, label: "自定义", value: "__custom__" }, { id: 3, label: "自定义+【序号】", value: "__serial_number__" }],
    value: { id: 3, label: "自定义", value: "__serial_number__-后缀[$x]" },
    input: "-后缀[$x]",
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
const inputBorder_preffix = ref("")
const inputBorder_center = ref("")
//预留后缀判断
const inputBorder_suffix = ref("")
function chooseInput(t: RenameType, status: Boolean) {
    if (status) {
        switch (t) {
            case renamePreffix.value:
                inputBorder_preffix.value = ""
                break
            case renameCenter.value:
                inputBorder_center.value = ""
                break
            case renameSuffix.value:
                inputBorder_suffix.value = ""
                break
        }

    } else {
        switch (t) {
            case renamePreffix.value:
                inputBorder_preffix.value = "solid red"
                break
            case renameCenter.value:
                inputBorder_center.value = "solid red"
                break
            case renameSuffix.value:
                inputBorder_suffix.value = "solid red"
                break
        }

    }

}
function check_input(rename: Ref<RenameType>, preview_index: number) {
    // console.log("-----" + "value");
    // let a = [renamePreffix, renameCenter, renameSuffix];
    // for (let i = 0; i < 3; i++) {
    console.log(rename.value)
    let t = rename.value;
    if (t.value.id == 2) {
        if (reg.test(t.input)) {
            t.valid = true;
            preview_filename.value[preview_index] = t.input;
            chooseInput(t,true)
        }
        else {
            t.valid = false;
            chooseInput(t,false)
        }
    } else if (t.value.id == 3) {
        if (t.input.includes("$x") && reg.test(t.input)) {
            t.valid = true;
            preview_filename.value[preview_index] = t.input.replaceAll("$x", "1");
            chooseInput(t,true)
        } else {
            // console.log(`output->border`)
            t.valid = false;
            chooseInput(t,false)
        }
    } else {
        if (preview_index == 1) {
            preview_filename.value[preview_index] = "basename";
        }
        chooseInput(t,true)
        // == 1 ???
    }
    // }
    // 判断是否全空?
}

function check_select_prefix() {
    if (renamePreffix.value.value.id == 1) {
        renamePreffix.value.valid = true
        renamePreffix.value.input = ""
        preview_filename.value[0] = "";
    }
}
function check_select_center() {
    if (renameCenter.value.value.id == 1) {
        renameCenter.value.valid = true
        renameCenter.value.input = ""
        preview_filename.value[1] = "basename";
    }
}

function check_select_suffix() {
    if (renameSuffix.value.value.id == 1) {
        renameSuffix.value.valid = true
        renameSuffix.value.input = ""
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

// 
const baseForm: Ref<UserBaseSettings> = ref({
    qulity: 0,
    autoUseBrand: false,
    brand: "",
    renameSuffix: "",
    renamePreffix: "",
    renameCenter: "",
    brands: [],
})







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
        let user_data: UserSendSettings = { output_dir: "", qulity: baseForm.value.qulity, auto_user_brand: baseForm.value.autoUseBrand, brand: baseForm.value.brand, filename_pattern: filename_tmp };
        //   console.log(user_data);
        user_conf.save_user_conf(baseForm.value, user_data);
    } else {
        ElMessage({
            showClose: true,
            message: '输入有误，保存失败，注意特殊字符',
            type: 'error',
        })
    }

}

function load_saved_conf() {
    user_conf.load_base_setting_B2A(baseForm.value, user_conf);
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
    settingsDrawer.value = true
    load_saved_conf();
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
    load_saved_conf();
})

const extendCollapse = ref(["1", "2", "3"])


</script>
    

<template>
    <div @mouseenter="menuShow" @mouseleave="menuHidden">
        <!-- <el-button @click="loadDrawer" type="primary" color="#3f8418" plain> -->
        <div @click="loadDrawer" style="height:18px">
            <el-icon size="large">
                <i-ep-setting />
            </el-icon>
        </div>
        <!-- </el-button> -->

    </div>
    <!-- <div @mouseenter="settingShow" @mouseleave="settingHidden">  -->
    <el-drawer v-model="settingsDrawer" title="基本设置" :before-close="handleClose" direction="rtl" size="30%">
        <!-- <el-image style="width: 150px; height: 150px" :src="get_image_url('a')" fit="contain" loading="eager" /> -->
        <el-scrollbar wrap-class="max-height:200px">
            <el-collapse v-model="extendCollapse">
                <el-collapse-item title="选择logo图片" name="1">
                    <el-checkbox v-model="baseForm.autoUseBrand" label="根据exif自动设置" size="large" border />
                    <el-select v-model="baseForm.brand" :placeholder="baseForm.brands[1].label"
                        :disabled="baseForm.autoUseBrand" style="margin: 20px 0 20px 0">
                        <el-option v-for="brand in baseForm.brands" :key="brand.value" :label="brand.label"
                            :value="brand.value">
                            <span style="float:left">{{ brand.label }}</span>
                            <span style="float: right;color: var(--el-text-color-secondary);font-size: 20px;">
                                <!-- <el-image style="width: 50px; height: 50px" :src="get_image_url(brand.value)" fit="fill" loading="eager" /> -->
                                <el-image style="width: 35px; height: 35px" :src="get_image_url(brand.value)"
                                    fit="contain" loading="eager" />
                                <!-- <el-icon>
                            <i-ep-picture />
                        </el-icon> -->
                            </span>
                        </el-option>
                    </el-select>
                </el-collapse-item>
                <el-collapse-item title="选择图片质量" name="2">
                    <div class="quality">
                        <el-slider v-model="baseForm.qulity" height="200px" />
                        <el-input-number v-model="baseForm.qulity" :min="1" :max="100" controls-position="right"
                            size="default" step-strictly @change="handleChangeQulity" />
                    </div>
                </el-collapse-item>
                <el-collapse-item title="选择图片名" name="3">
                    <el-row>
                        <el-col :span="8">
                            <div class="grid-content ep-bg-purple" />
                            <p style="margin-left: 10px">名称前缀</p>
                            <el-select v-model="renamePreffix.value" class="m-2" placeholder="Select" value-key="id"
                                size="large" :change="check_select_prefix()">
                                <el-option v-for="item in renamePreffix.SD" :key="item.id" :label="item.label"
                                    :value="item" />
                            </el-select>

                        </el-col>
                        <el-col :span="8">
                            <div class="grid-content ep-bg-purple-light" />
                            <p style="margin-left: 10px">名称中间</p>
                            <el-select v-model="renameCenter.value" class="m-2" placeholder="Select" value-key="id"
                                size="large" :change="check_select_center()">
                                <el-option v-for="item in renameCenter.SD" :key="item.id" :label="item.label"
                                    :value="item" />
                            </el-select>
                        </el-col>
                        <el-col :span="8">
                            <div class="grid-content ep-bg-purple" />
                            <p style="margin-left: 10px">名称后缀</p>
                            <el-select v-model="renameSuffix.value" class="m-2" placeholder="Select" value-key="id"
                                size="large" :change="check_select_suffix()">
                                <el-option v-for="item in renameSuffix.SD" :key="item.id" :label="item.label"
                                    :value="item" />
                            </el-select>
                        </el-col>
                    </el-row>
                    <el-row>
                        <el-col :span="8">
                            <div class="grid-content ep-bg-purple" />
                            <el-input class="elinput preffix" v-model="renamePreffix.input"
                                :disabled="renamePreffix.value.id == 1" :blur="check_input_prefix()"
                                :style="{'border':inputBorder_preffix+' 1px'}"> "自定义后缀"
                            </el-input>

                        </el-col>
                        <el-col :span="8">
                            <div class="grid-content ep-bg-purple-light" />
                            <el-input class="elinput center" v-model="renameCenter.input"
                                :disabled="renameCenter.value.id == 1" :blur="check_input_center()"
                                :style="{'border':inputBorder_center+' 1px'}">
                                "自定义后缀" </el-input>

                        </el-col>
                        <el-col :span="8" id="invalidInputCss">
                            <div class="grid-content ep-bg-purple" />
                            <el-input class="elinput suffix" v-model="renameSuffix.input"
                                :disabled="renameSuffix.value.id == 1" :blur="check_input_suffix()"
                                :style="{'border':inputBorder_suffix+' 1px'}">
                                "自定义后缀" </el-input>
                        </el-col>
                    </el-row>
                    <el-row>
                        <el-button key="button.text" type="primary" text>文件名预览： {{ preview_filename.join("") }} </el-button>
                        <el-button text size="small">提示： 自定义+序号 默认替换“$x” 为序号 1、2、3……</el-button>
                        <el-button text size="small">尝试在后缀自定义+序号中输入[$x] 预览</el-button>
                        <!-- <el-button text size="small"></el-button> -->
                    </el-row>
                </el-collapse-item>
            </el-collapse>
        </el-scrollbar>

        <div style="" class="row">
            <div class="container">
                <div class="btn">
                    <el-popconfirm confirm-button-text="是" cancel-button-text="否" :icon="InfoFilled"
                        icon-color="#626AEF" title="重置（仍未保存）" @confirm="resetConfirmEvent" @cancel="">
                        <template #reference>
                            <el-button type="primary" size="large" plain color="#0FCAC7" class="default-btn">默认
                            </el-button>
                        </template>
                    </el-popconfirm>
                </div>
                <div class="btn">
                    <el-button type="primary" size="large" @click="saveSetting" class="save-btn"> 保存 </el-button>
                </div>
            </div>
        </div>

        <!-- <el-button type="primary" :loading="loading" @click="onClick">{{
            loading ? 'Submitting ...' : 'Submit'
          }}</el-button> -->
    </el-drawer>
    <!-- </div>  -->

</template>
  

<style lang="css" scoped>
.row {
    position: absolute;
    bottom: 8px;
    width: 100%;
    z-index: 90;
}

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
    
}

#invalidInputCss .el-input {
    --el-input-border-color: #f70909;
}

#defaultInputCss .el-input {
    --el-input-border-color: #dcdfe6;
}

.row .container {
    display: inline-flex;
    width: 86%;
}

.row .container .btn {
    width: 50%;
}

.default-btn {
    width: 100%;
    height: 100%;
}

.save-btn {
    width: 100%;
    height: 100%;
}

.quality {
    display: inline-flex;
    width: 97%;
}

.elinput {
    border-radius: 4px;
}

</style>


<!-- 
// :deep(.elinput, .preffix) {
    //     border: v-bind(inputBorder_preffix);
    // }
    
    // :deep(.elinput, .center) {
    //     border: v-bind(inputBorder_center);
    // }
    
    // :deep(.elinput, .suffix) {
    //     border: v-bind(inputBorder_suffix);
    // }
    
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
    // } -->