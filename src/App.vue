<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import H222 from './components/H222.vue'
import greet from './main'

</script>

<template>
  <el-container>

    <el-header style="height: 30px; border: 1px solid #eee">
      <a>header</a>
    </el-header>

    <el-aside width="30px" style="background-color: rgb(238, 241, 246)">
      <el-menu>
        基本
      </el-menu>
      <el-menu>
        高级
      </el-menu>
    </el-aside>


    <el-main width="600px" style="background-color: rgb(238, 241, 246)">
      <div>
        <!-- <button @click="count1++">Count is: {{ count1 }}</button> -->
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo" alt="Vite logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
          <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
      </div>
      <HelloWorld msg="Vite + Vue" />
      <H222 />
      <button @click="greet"> some button </button>
    </el-main>

    <el-footer></el-footer>

  </el-container>



</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
