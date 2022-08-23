<script lang="ts">
import { ref, onMounted, defineComponent } from 'vue'
const value = ref(0)
// 响应式状态
const count = ref(0)
const number = ref("10")
// 用来修改状态、触发更新的函数
function increment() {
  count.value++
}

// 生命周期钩子
onMounted(() => {
  console.log(`The initial count is ${count.value}.`)
})

export default defineComponent({
  name: 'H222',
  setup(){

  }
})
</script>

<template>
    <button class="el-button" aria-disabled="false" type="button"><!--v-if--><span class="">Click Me</span></button>
    
    <input  v-model="number" placeholder="请输入价格"/>
    <div>{{number}}</div>>
  <button @click="increment">Count is: {{ count }}</button>
    <div class="slider-demo-block">
    <el-slider v-model="value" show-input />
  </div>
</template>

<style scoped>
.slider-demo-block {
  display: flex;
  align-items: center;
}
.slider-demo-block .el-slider {
  margin-top: 0;
  margin-left: 12px;
}
</style>
