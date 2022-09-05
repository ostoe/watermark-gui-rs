<script lang="ts" setup>
import { reactive, ref } from "vue";
// import { ElDrawer, ElMessageBox } from "element-plus";

const formLabelWidth = "80px";
//   let timer

const settingsDrawer = ref(true);
const loading = ref(false);
const qulity = ref(85);
const form = reactive({
  brands: [
    { brand: "canon", label: "佳能" },
    { brand: "nikon", label: "尼康" },
    { brand: "sony", label: "索尼" },
  ],
  name: "",
  brand: "",
  delivery: false,
  type: [],
  resource: "",
  desc: "",
});

const autoSelect = ref(true);

// const drawerRef = ref<InstanceType<typeof ElDrawer>>();
// const onClick = () => {
//   drawerRef.value!.close();
// };

// const saveSetting = () => {
//   console.log("save setting.");
// };

// const handleClose = (done: (_: boolean) => void) => {
//   console.log("close drawer" + done);
//   settingsDrawer.value = false;
// };

// const timerButtonArea = ref();
// const timerSettingsArea = ref();
// function menuShow() {
//   timerButtonArea.value = setTimeout(() => {
//     settingsDrawer.value = true;
//   }, 600);
// }

// function menuHidden() {
//   if (!settingsDrawer.value) {
//     clearTimeout(timerButtonArea.value);
//     // settingsDrawer.value = false
//   }
// }

// function settingShow() {
//   console.log("leave");
//   if (settingsDrawer.value) {
//     clearTimeout(timerSettingsArea.value);
//   }
// }

// function settingHidden() {
//   console.log("leave");
//   timerSettingsArea.value = setTimeout(() => {
//     console.log("leave");
//     settingsDrawer.value = false;
//   }, 5000);
// }

function handleChangeQulity() {}
</script>

<template>
  <div @mouseenter="menuShow" @mouseleave="menuHidden">
    <el-button
      @click="settingsDrawer = true"
      type="primary"
      color="#3f8418"
      plain="true"
    >
      <el-icon>
        <i-ep-d-arrow-left />
      </el-icon>
    </el-button>
  </div>
  <!-- <div @mouseenter="settingShow" @mouseleave="settingHidden">  -->

  <el-drawer
    v-model="settingsDrawer"
    title="基本设置"
    :before-close="handleClose"
    direction="rtl"
    size="30%"
  >
    <el-scrollbar wrap-class="max-height:200px">
      <el-checkbox
        v-model="autoSelect"
        label="根据exif自动设置"
        size="large"
        border
      />
      <el-select
        v-model="form.brand"
        :placeholder="form.brands[1].label"
        :disabled="autoSelect"
      >
        <el-option v-for="brand in form.brands" :value="brand.label" />
      </el-select>
      <el-input-number
        v-model="qulity"
        :min="1"
        :max="100"
        controls-position="right"
        size="large"
        step-strictly
        @change="handleChangeQulity"
      />

      <el-slider v-model="qulity" vertical height="200px" />
    </el-scrollbar>

    <div style="margin: 10px 0 20% 0; border-bottom: 0%">
      <el-row>
        <el-button type="primary" size="small"> reset </el-button>
        <el-button type="primary" size="large" @click="saveSetting">
          保存
        </el-button>
      </el-row>
    </div>

    <!-- <el-button type="primary" :loading="loading" @click="onClick">{{
            loading ? 'Submitting ...' : 'Submit'
          }}</el-button> -->
  </el-drawer>
  <!-- </div>  -->
</template>

<style scoped>
/* .el-drawer {
  --el-drawer-bg-color: var(--el-dialog-bg-color, var(--el-bg-color));
  --el-drawer-padding-primary: var(--el-dialog-padding-primary, 20px);
}
.el-drawer {
  position: absolute;
  box-sizing: border-box;
  background-color: var(--el-drawer-bg-color);
  display: flex;
  flex-direction: column;
  box-shadow: var(--el-box-shadow-dark);
  overflow: hidden;
  transition: all var(--el-transition-duration);
}
.el-drawer .rtl {
  transform: translate(0, 0);
}
.el-drawer .ltr {
  transform: translate(0, 0);
}
.el-drawer .ttb {
  transform: translate(0, 0);
}
.el-drawer .btt {
  transform: translate(0, 0);
}
.el-drawer__sr-focus:focus {
  outline: 0 !important;
}
.el-drawer__header {
  align-items: center;
  color: #72767b;
  display: flex;
  margin-bottom: 32px;
  padding: var(--el-drawer-padding-primary);
  padding-bottom: 0;
}
.el-drawer__header > :first-child {
  flex: 1;
}
.el-drawer__title {
  margin: 0;
  flex: 1;
  line-height: inherit;
  font-size: 1rem;
}
.el-drawer__footer {
  padding: var(--el-drawer-padding-primary);
  padding-top: 10px;
  text-align: right;
}
.el-drawer__close-btn {
  border: none;
  cursor: pointer;
  font-size: var(--el-font-size-extra-large);
  color: inherit;
  background-color: transparent;
  outline: 0;
}
.el-drawer__close-btn:focus i,
.el-drawer__close-btn:hover i {
  color: var(--el-color-primary);
}
.el-drawer__close-btn .el-icon {
  font-size: inherit;
  vertical-align: text-bottom;
}
.el-drawer__body {
  flex: 1;
  padding: var(--el-drawer-padding-primary);
  overflow: auto;
}
.el-drawer__body > * {
  box-sizing: border-box;
}
.el-drawer.ltr,
.el-drawer.rtl {
  height: 100%;
  top: 0;
  bottom: 0;
}
.el-drawer.btt,
.el-drawer.ttb {
  width: 100%;
  left: 0;
  right: 0;
}
.el-drawer.ltr {
  left: 0;
}
.el-drawer.rtl {
  right: 0;
}
.el-drawer.ttb {
  top: 0;
}
.el-drawer.btt {
  bottom: 0;
}
.el-drawer-fade-enter-active,
.el-drawer-fade-leave-active {
  transition: all var(--el-transition-duration);
}
.el-drawer-fade-enter-active,
.el-drawer-fade-enter-from,
.el-drawer-fade-enter-to,
.el-drawer-fade-leave-active,
.el-drawer-fade-leave-from,
.el-drawer-fade-leave-to {
  overflow: hidden !important;
}
.el-drawer-fade-enter-from,
.el-drawer-fade-leave-to {
  opacity: 0;
}
.el-drawer-fade-enter-to,
.el-drawer-fade-leave-from {
  opacity: 1;
}
.el-drawer-fade-enter-from .rtl,
.el-drawer-fade-leave-to .rtl {
  transform: translateX(100%);
}
.el-drawer-fade-enter-from .ltr,
.el-drawer-fade-leave-to .ltr {
  transform: translateX(-100%);
}
.el-drawer-fade-enter-from .ttb,
.el-drawer-fade-leave-to .ttb {
  transform: translateY(-100%);
}
.el-drawer-fade-enter-from .btt,
.el-drawer-fade-leave-to .btt {
  transform: translateY(100%);
} */
</style>
