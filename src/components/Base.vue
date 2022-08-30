
<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
    setup() {
    const isCollapse = ref(true);
    return {
      isCollapse,
    };
  },
  data() {
    return {
      count: 0,
    };
  },
  "name": "ffdsf",
    methods: {
        // 这是个异步函数
        // test() {
        //     update_progress(0, selected.length);
        // }

    test_pro() {
        console.log("fff");
        // update_progress();
        //
    },
    }

});

</script>

<template>
    <div></div>
</template>
    
<style>
</style>