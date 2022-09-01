<script lang="ts">
import HelloWorld from "./HelloWorld.vue";
import TextImageProcess from "./TextImageProcess.vue";
import TopBar from "./TopBar.vue";

import { defineComponent, ref } from "vue";
// import { Context } from "vm";

//dark mode
import { useDark, useToggle } from "@vueuse/core";
//引入路由
import { useRoute, useRouter } from "vue-router";

export default defineComponent({
  name: "index",
  //自定义指令
  directives: {
    resize: {
      mounted(el, binding) {
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
      unmounted(el) {
        el._resizer.disconnect();
      },
    },
  },

  setup() {
    const route = useRoute();
    const router = useRouter();
    const isDark = ref(true);
    const isCollapse = ref(true);
    const delay = ref(200);
    const tooltipEffect = ref("light");
    const dynamicWidth = ref(false);
    const extendPadding = ref("");
    const count = ref(0);
    const tableData = ref([]);
    const text = ref("./tests/img/jpg/gps/DSCN0010.jpg");
    let t: NodeJS.Timeout | null = null;

    //获取鼠标点击消除遮罩
    const changeThisCollapse = () => {
      let firstClick = !t;
      if (firstClick) {
        isCollapse.value = isCollapse.value ? false : true;
        extendPadding.value = isCollapse.value ? "" : "70";
      }
      if (t) {
        clearTimeout(t);
      }
      t = setTimeout(() => {
        t = null;
        if (!firstClick) {
          isCollapse.value = isCollapse.value ? false : true;
          extendPadding.value = isCollapse.value ? "" : "70";
        }
      }, delay.value);
    };

    //监听dom变化
    const resizeSideBar = (width: any) => {
      let domWidth = width;
      let initWidth = 63;
      dynamicWidth.value = domWidth.width > 63 ? false : true;
      // console.log(domWidth)
    };

    //路由
    const route2Main = () => {
      console.log("main");
      router.push("/textImageProcess")
    }
    const route2Test = () =>{
      router.push("/helloWorld")
    }

    const isDarkMode = useDark();
    const toggleDark = useToggle(isDarkMode);
    const toggleDarkMode = () => {
      toggleDark;
      isDark.value = isDark.value ? false : true;
    };

    return {
      isCollapse,
      delay,
      tooltipEffect,
      dynamicWidth,
      extendPadding,
      count,
      tableData,
      text,
      isDark,
      changeThisCollapse,
      resizeSideBar,
      toggleDarkMode,
      route2Main,
      route2Test
    };
  },

  watch: {},

  mounted() {
    this.route2Main();
    // 在其他方法或是生命周期中也可以调用方法
    // this.test_event_recv();
    // for (let i = 0; i < 5; i += 1) {
    //   this.tableData.push({ id: i, msg: " " + i + " " + i + " " + i });
    // }
  },
});

</script>

<template lang="">
  <div class="common-layout index">
    <el-container>
      <el-aside width="50px">
        <el-menu
          default-active="1-1"
          class="elmenu"
          :collapse="isCollapse"
          :delay="delay"
          v-resize:1="resizeSideBar"
        >
          <div style="margin-top: 30px"></div>
          <div
            @click="changeThisCollapse"
            class="extend"
            :style="{ 'padding-left': extendPadding + 'px' }"
          >
            <el-icon>
              <i-ep-arrow-right v-if="isCollapse" />
              <i-ep-arrow-left v-else />
            </el-icon>
          </div>
          <el-sub-menu index="1">
            <template #title>
              <el-icon><i-ep-menu /></el-icon>
              <span>基本</span>
            </template>
            <el-menu-item-group>
              <!-- <router-link to="/">Go to Home</router-link>
              <router-link to="/about">Go to About</router-link> -->
              <el-menu-item index="1-1"  @click="route2Main">测试页</el-menu-item>
              <el-menu-item index="1-2" @click="route2Test">HelloWorld</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
          <el-menu-item index="2">
            <el-icon><i-ep-add-location /></el-icon>
            <template #title>高级</template>
          </el-menu-item>
          <el-menu-item index="3">
            <el-icon><i-ep-document /></el-icon>
            <template #title>TODO</template>
          </el-menu-item>
          <el-footer>
            <div class="footer-div">
              <el-divider content-position="center">
                <el-tooltip
                  class="tooltip"
                  :effect="tooltipEffect"
                  content="gui测试"
                  placement="right-start"
                  v-if="dynamicWidth"
                >
                  <span>&copy;</span>
                </el-tooltip>
                <span v-else class="copyrightSpan">&copy;gui</span>
              </el-divider>
            </div>
          </el-footer>
          <div @click="toggleDarkMode" class="darkBtn">
            <el-icon>
              <i-ep-arrow-right v-if="isDark" />
              <i-ep-arrow-left v-else />
            </el-icon>
          </div>
        </el-menu>
      </el-aside>
      <div
        v-if="!isCollapse"
        class="shadowmask"
        @click="changeThisCollapse"
      ></div>
      <el-container>
        <el-header height="40px">
          <TopBar />
        </el-header>
        <el-divider></el-divider>
        <el-main>
          <el-container direction="vertical">
            <!-- <el-row > -->
            <!-- <HelloWorld msg="Vite + Vue" /> -->
            <!-- <TextImageProcess /> -->
            <!-- </el-row> -->
          </el-container>
        </el-main>
      </el-container>
    </el-container>
  </div>
  <router-view></router-view>
</template>

<style>
.elmenu {
  z-index: 99;
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

.common-layout .shadowmask {
  position: absolute;
  top: 0 auto;
  left: 0 auto;
  width: 100%;
  height: 100%;
  z-index: 98;
  background-color: #000;
  opacity: 0.7;
}

.common-layout .extend,
.darkBtn {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 10px 5px 20px 5px;
}

.copyrightSpan {
  font-size: xx-small;
}
</style>
