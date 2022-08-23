<script lang="ts">
import { ref, onMounted, defineComponent } from 'vue'

export default defineComponent({
  name: 'H222',
  props:{
    number: Number,
    count: Number,
    increment: Function
  },
  setup(){
    const value = ref(0)
    // 响应式状态
    const count = ref(0)
    const number = ref("10")
    // 用来修改状态、触发更新的函数
    const increment = ()=> {
      count.value++
    }
    // 生命周期钩子
    onMounted(() => {
      console.log(`The initial count is ${count.value}.`)
    })

    return {
      value,
      count,
      number,
      increment
    }
  }
})
</script>

<template>
  <div>
    <button aria-disabled="false" type="button"><!--v-if-->
      <span >Click Me</span>
    </button>
    <input  v-model="number" placeholder="请输入价格"/>
    <div>{{ number }}</div>
    <button @click="increment">Count is: {{ count }}</button>
  </div>
  <div>
    <el-slider v-model="value" show-input />
  </div>
</template>

<style scoped>
</style>
