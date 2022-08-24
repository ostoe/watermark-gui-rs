<script lang="ts">
import { ref, defineComponent } from 'vue'


export default defineComponent({
  name: 'HelloWorld',
  props: {
    msg: String
  },
  setup(props){
    const input = ref('')
    const count = ref(0)
    console.log(props)
    return {
      input,
      count
    }
  }
})
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>
  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
   <el-input v-model="input" placeholder="Please input" />
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
