// hash history模式
// https://router.vuejs.org/guide/essentials/history-mode.html#hash-mode
import { createRouter,createWebHashHistory} from "vue-router"

const helloworld = () => import('../components/HelloWorld.vue') 
const textImageProcess = () => import('../components/TextImageProcess.vue')
const wrapper = () => import('../components/Wrapper.vue')

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
        path:'/textImageProcess',
        name:'textImageProcess',
        component:textImageProcess
    }]
}]

export const routers = createRouter({
    history: createWebHashHistory(),
    routes: router
})
