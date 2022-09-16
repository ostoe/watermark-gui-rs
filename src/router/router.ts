// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter, createWebHashHistory } from "vue-router";

const dragTest = () => import("../components/dragTest.vue");
const textImageProcess = () => import("../components/textImageProcess.vue");
// const wrapper = () => import("../components/wrapper.vue");
const base = () => import("../components/base.vue");
const topBar = () => import("../components/topBar.vue");
const baseSettingsDrawer = () => import("../components/baseSettingsDrawer.vue");
const DepthField = () => import("../components/DepthField.vue");



// const cusIcon = {
//   template: '<{{ $route.params.id }}/> '
// }

// 这些都会传递给 `createRouter`
const routes = [
  {
    path: "/",
    name: "基础",
    component: base,
    iconid: 1,
    
  },
  {
    path: "/textImageProcess",
    name: "Exif查询",
    component: textImageProcess,
    iconid: 2,
  },
  // { path: 'cus/:id', component: cusIcon }// solve icon ...
  {
    path: "/DepthField",
    name: "景深计算",
    component: DepthField,
    iconid: 3,
    // children: [
    //   {
    //     path: "dragTest",
    //     name: "dragTest",
    //     component: dragTest,
    //   },
    //   {
    //     path: "dragTest1",
    //     name: "dragTest1",
    //     component: textImageProcess,
    //   }
    // ]
  },
  {
    path: "/dragTest",
    name: "DragTest",
    component: dragTest,
    iconid: 4,
  },
  {
    path: "/animation",
    name: "时间",
    component: () => import("../components/animation.vue"),
    iconid: 5,
  },
  {
    path: "/About",
    name: "About",
    component: () => import("../components/About.vue"),
    iconid: 6,
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
export default router;
