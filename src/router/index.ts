// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter,createWebHashHistory } from "vue-router"

const helloworld = () => import('../components/HelloWorld.vue') 
const h222 = () => import('../components/H222.vue')
const wrapper = () => import('../components/wrapper.vue')

const router = [{
    path:'/',
    name:'index',
    component:wrapper,
    children:[{
        path:'/HelloWorld',
        name:'helloworld',
        component:helloworld
    },
    {
        path:'/h222',
        name:'h222',
        component:h222
    }]
}]

export const routers = createRouter({
    history: createWebHashHistory(),
    routes: router
})
