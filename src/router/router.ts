// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter, createWebHashHistory } from "vue-router";

const dragTest = () => import("../components/dragTest.vue");
const textImageProcess = () => import("../components/textImageProcess.vue");
// const wrapper = () => import("../components/wrapper.vue");
const base = () => import("../components/base.vue");
// const topBar = () => import("../components/topBar.vue");
const home = () => import("../components/home.vue");


// const cusIcon = {
//   template: '<{{ $route.params.id }}/> '
// }

// 这些都会传递给 `createRouter`
const routes = [
  {
    path: "/",
    name: "基础",
    component: home,
    iconid: 1,
    children: [
      {
        path: "base",
        name: "base",
        component: base,
      },
      {
        path: "textImageProcess",
        name: "textImageProcess",
        component: textImageProcess,
        
      },
    ],
  },
  // { path: 'cus/:id', component: cusIcon }// solve icon ...
  {
    path: "/",
    name: "hello",
    component: home,
    iconid: 2,
    children: [
      {
        path: "dragTest",
        name: "dragTest",
        component: dragTest,
      },
      {
        path: "dragTest1",
        name: "dragTest1",
        component: textImageProcess,
      }
    ]
  },


];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
export default router;
