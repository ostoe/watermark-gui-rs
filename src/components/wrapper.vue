<script lang="ts">
import HelloWorld from './HelloWorld.vue'
import H222 from './H222.vue'
import greet from '../main'
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'index',
  methods: {
    // 这是个异步函数
    greetTest(){
      greet("")
    }
  }
})
</script>

<template lang="">
    <div>
        111111111212121
    </div>
    <HelloWorld msg="Vite + Vue"/>
    <H222/>
    <suspense>
      <button @click="greetTest"> some button </button>
    </suspense>
</template>

<style>
</style>