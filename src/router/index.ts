// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter, createWebHashHistory } from "vue-router";

const dragTest = () => import("../components/DragTest.vue");
const textImageProcess = () => import("../components/TextImageProcess.vue");
const wrapper = () => import("../components/Wrapper.vue");
const base = () => import("../components/Base.vue");
const topbar = () => import("../components/TopBar.vue");

const routes  = [
  {
    path: "/",
    name: "index",
    component: wrapper,
    children: [
      {
        path: "/textImageProcess",
        name: "textImageProcess",
        component: textImageProcess,
      },
      {
        path: "/dragTest",
        name: "dragTest",
        component: dragTest,
      },
      {
        path: "/Base",
        name: "base",
        component: base,
      },
      {
        path: "/Topbar",
        name: "topbar",
        component: topbar,
      },
    ],
  },


];

export const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
