// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter, createWebHashHistory } from "vue-router";

const helloworld = () => import("../components/HelloWorld.vue");
const textImageProcess = () => import("../components/TextImageProcess.vue");
const wrapper = () => import("../components/Wrapper.vue");
const base = () => import("../components/Base.vue");
const topbar = () => import("../components/TopBar.vue");

const router = [
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
    ],
  },
  {
    path: "/HelloWorld",
    name: "helloworld",
    component: helloworld,
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
];

export const routers = createRouter({
  history: createWebHashHistory(),
  routes: router,
});
