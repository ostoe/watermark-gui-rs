<script lang="ts">
import HelloWorld from './HelloWorld.vue'
import H222 from './H222.vue'
import greet from '../main'
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'index',
  setup(){

  }
})
</script>

<template lang="">
    <div>
        111111111212121
    </div>
    <HelloWorld msg="Vite + Vue" class="hw"/>
    <H222 class="h2"/>
    <button @click="greet"> some button </button>
</template>

<style>
.hw {
    width: 50%;
    height: 50%;
}
</style>