// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter, createWebHashHistory } from "vue-router";

const DragTest = () => import("../components/DragTest.vue");
const TextImageProcess = () => import("../components/TextImageProcess.vue");
// const wrapper = () => import("../components/wrapper.vue");
const Base = () => import("../components/Base.vue");
const TopBar = () => import("../components/TopBar.vue");
const BaseSettingsDrawer = () => import("../components/BaseSettingsDrawer.vue");
const DepthField = () => import("../components/DepthField.vue");



// const cusIcon = {
//   template: '<{{ $route.params.id }}/> '
// }

// 这些都会传递给 `createRouter`
const routes = [
  {
    path: "/",
    name: "基础",
    component: Base,
    iconid: 1,
    
  },
  {
    path: "/TextImageProcess",
    name: "Exif查询",
    component: TextImageProcess,
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
    path: "/DragTest",
    name: "DragTest",
    component: DragTest,
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
