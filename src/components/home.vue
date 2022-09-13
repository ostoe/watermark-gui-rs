<script lang="ts" setup>
// import dragTest from "./dragTest.vue";
// import TextImageProcess from "./textImageProcess.vue";
// import sideBar from "./sideBar.vue";
// import topBar from "./topBar.vue";
// import picList from "./picList.vue";
// import baseSettingsDrawerVue from "./baseSettingsDrawerVue.vue";
// import previewWidget from "./previewWidget.vue";

import { ref, onMounted } from "vue";
import router from "../router/router";
// import { Context } from "vm";
import { sidebarReactives, image_progress } from "../scripts/reactives";
import { convertFileSrc } from "@tauri-apps/api/tauri";

// import { sidebarReactives } from "../scripts/reactives";
//dark mode
import { useDark, useToggle } from "@vueuse/core";
//引入路由
import { useRoute, useRouter } from "vue-router";

//自定义指令
const vResize = {
  mounted: (el: any, binding: { value: (arg0: { width: number }) => void }) => {
    let ResizeObserver = window.ResizeObserver;
    el._resizer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        // console.log(entry.contentRect.width)
        binding.value({ width: entry.contentRect.width });
      }
    });
    el._resizer.observe(el);
    // console.log(binding)
  },
  unmounted: (el: { _resizer: { disconnect: () => void } }) => {
    el._resizer.disconnect();
  },
};

//获取鼠标点击消除遮罩
const changeThisCollapse = () => {
  sidebarReactives.changeThisCollapse()
};

//监听dom变化
const dynamicWidth = ref(false);
const resizeSideBar = (width: any) => {
  let domWidth = width;
  let initWidth = 63;
  dynamicWidth.value = domWidth.width > initWidth ? false : true;
  // console.log(domWidth)
};

//路由
// const route = useRoute();
// const router = useRouter();
// const route2Main = () => {
//     router.push("/textImageProcess");
// };
// const route2Test = () => {
//   router.push("/HelloWorld");
// };

// 深色模式
const isDark = ref(true);
const isDarkMode = useDark();
const toggleDark = useToggle(isDarkMode);
const tooltipEffect = ref("light");
const toggleDarkMode = () => {
  toggleDark;
  isDark.value = isDark.value ? false : true;
};

//侧栏聚焦
// const activeMenuId = ref("1-1");


onMounted(() => {
  //   router.push("/textImageProcess");
});
//获取鼠标点击消除遮罩
let t: NodeJS.Timeout | null = null;
// const isCollapse = ref(true);
// const delay = ref(200);
// const extendPadding = ref("");
// 输出路径面包屑
// const selectedPath = ref<Array<string>>();
// const transferPath2Array = () => {
//   // if(image_progress.image_dir_path.indexOf("\")){
//   // }
// };
function handleSelect(key: string, keyPath: string) {
  console.log(key, keyPath);
}
nextTick(() => { });
    // const picSrc = ref("https://github.com/tauri-apps/tauri/blob/dev/.github/splash.png")
</script>
    
<template lang="">
      <el-container>
        <!-- <el-header> -->
          <!-- <sideBar/> -->
          <!-- <el-container > -->
          <el-aside width="50px">
            
            <el-menu  @select="handleSelect" router="true" default-active="" 
            class="elmenu" :collapse="sidebarReactives.isCollapse"
                v-resize:1="resizeSideBar" collapse-transition="false">
                <div style="margin-top: 30px"></div>
                <div @click="changeThisCollapse" class="extend"
                    :style="{ 'padding-left': sidebarReactives.extendPadding + 'px' }">
                    <el-icon>
                        <i-ep-arrow-right v-if="sidebarReactives.isCollapse" />
                        <i-ep-arrow-left v-else/>
                    </el-icon>
                </div>
                <template v-for="(item,index) in $router.options.routes">
                    
                    <el-sub-menu v-if="item.children && item.children.length > 0" :index="index" :key="'item-'+index" >
                      <template #title>
                        <!-- <el-icon class="i-ep-menu"></el-icon> -->
                        <el-icon v-if="item.iconid == 1">
                          <i-ep-menu /> 
                        </el-icon>
                        <el-icon v-else-if="item.iconid == 2">
                          <i-ep-document />
                        </el-icon>
                        <span>{{item.name}}</span>
                    </template>
                      <el-menu-item-group>
                        <!-- <template slot="title"><i :class="item.iconCls"></i><span>{{item.name}}</span></template> -->
                        <template v-for="(child,itemIndex) in item.children">
                            <el-menu-item v-if="!child.hidden"
                                          :index="child.path"
                                          :key="'child-'+itemIndex">{{child.name}}
                            </el-menu-item>
                        </template>
                      </el-menu-item-group>
                    </el-sub-menu>
                  
                  <el-menu-item v-else :index="item.path">
                       <template #title>
                        <!-- <el-icon class="i-ep-menu"></el-icon> -->
                        <el-icon v-if="item.iconid == 1">
                          <i-ep-menu /> 
                        </el-icon>
                        <el-icon v-else-if="item.iconid == 2">
                          <i-ep-document />
                        </el-icon>
                        <span>{{item.name}}</span>
                    </template>
                  </el-menu-item>
                </template>
            </el-menu>
          </el-aside>
        <!-- </el-container> -->
        <!-- </el-header> -->
        <div
            v-if="!sidebarReactives.isCollapse"
            class="shadowmask"
            @click="changeThisCollapse"
          ></div>
        <el-main>
            <transition name="el-fade-in-linear" mode="out-in">
                <router-view></router-view>
            </transition>
        </el-main>
    </el-container>


      <!-- <span class="spantest">fonttest1</span>
      <span class="spantest2">fonttest2</span>
      <el-select v-model="fontFamilyValue" class="select" placeholder="选择文字">
        <el-option
          v-for="item in fontOps"
          :key="item.fontFamily"
          :value="item.fontFamily"
        />
      </el-select> -->

      <!-- <div class="common-layout index">
        <div class="elcontainer"> -->
          <!-- <SideBar>
            <template #elmenu>
              <el-menu-item index="4">
                <el-icon>
                  <i-ep-document />
                </el-icon>
                <template #title>TODO2测试插槽</template>
              </el-menu-item>
            </template>
          </SideBar> -->
          
          
          
          
        <!-- </div>
      </div>
      <router-view></router-view> -->
    </template>
    
<style scoped>
@import "../assets/style/font.css";
/* 
.spantest {
  font-family: v-bind("fontFamilyValue");
}

.spantest2 {
  font-family: "FiraCode-Bold";
} */

.elmenu {
  z-index: 101;
  position: absolute;
  left: 0;
  height: 100%;
}

.footer-div {
  background-color: rgb(255, 255, 255);
  display: flex;
  height: 4vh;
  width: 100%;
  bottom: 0;
  right: 0;
  align-items: center;
}

/* .el-container {
  width: var(--el-aside-width,120px);
} */

.extend,
.darkBtn {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 10px 5px 20px 5px;
}

.copyrightSpan {
  font-size: xx-small;
}

.common-layout {
  height: 100%;
  width: auto;
}

.elcontainer {
  margin-left: 50px;
}

.footer-div {
  background-color: rgb(255, 255, 255);
  display: flex;
  height: 4vh;
  width: 100%;
  bottom: 0;
  right: 0;
  align-items: center;
}

/* .el-container {
      width: var(--el-aside-width,120px);
    } */

.common-layout .shadowmask {
  position: absolute;
  top: 0 auto;
  left: 0 auto;
  width: 100%;
  height: 100%;
  z-index: 100;
  background-color: #000;
  opacity: 0.7;
}
</style>
    