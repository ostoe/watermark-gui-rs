<!-- <script lang="ts">
import { ref, defineComponent } from 'vue'
import { onMounted, onUnmounted } from 'vue'
const events = ['dragenter', 'dragover', 'dragleave', 'drop']

const emit = defineEmits(['files-dropped'])
export default defineComponent({
  name: 'HelloWorld',
  props: {
    msg: String
  },
  setup(props){
    const input = ref('')
    const count = ref(0)
    const active = ref(false)
    console.log(props)
    return {
      input,
      count,
      active
    }
  },

onMounted()  {
    events.forEach((eventName) => {
        document.body.addEventListener(eventName, this.preventDefaults)
    })
},

onUnmounted() {
    events.forEach((eventName) => {
        document.body.removeEventListener(eventName, this.preventDefaults)
    })
},

  methods: {


 setActive() {
    active.value = true
}
 setInactive() {
    active.value = false
}
    onDrop(e) {
      setInactive(); // add this line too;
        emit('files-dropped', [...e.dataTransfer.files]);
    },

    preventDefaults(e) {
        e.preventDefault()
    },
  },
  data() {
    return {
      count: 0,
    };
  },
})
</script> -->

<script lang="ts">
import { ref, onMounted, onUnmounted, defineEmits } from "vue"
//引入路由
import { useRoute, useRouter } from "vue-router";

export default {
  props: ["msg"],
  setup() {
    const route = useRoute();
    const router = useRouter();
    const backRoute = () => {
      router.push("/")
    }
    return {
      backRoute
    }
  }
};
const emit = defineEmits(["files-dropped"]);



// Create `active` state and manage it with functions
let active = ref(false);
let inActiveTimeout = null;
function setActive() {
  active.value = true
  clearTimeout(inActiveTimeout) // clear the timeout
}
function setInactive() {
  // wrap it in a `setTimeout`
  inActiveTimeout = setTimeout(() => {
    active.value = false
  }, 50)
}

function onDrop(e) {
  setInactive(); // add this line too
  emit("files-dropped", [...e.dataTransfer.files]);
}

function preventDefaults(e) {
  e.preventDefault();
}

const events = ["dragenter", "dragover", "dragleave", "drop"];

onMounted(() => {
  events.forEach((eventName) => {
    document.body.addEventListener(eventName, preventDefaults);
  });
});

onUnmounted(() => {
  events.forEach((eventName) => {
    document.body.removeEventListener(eventName, preventDefaults);
  });
});

</script>



<template>
  <h1>{{ msg }}</h1>
  <div style="width:fit-content">
    <button @click="backRoute">back</button>

    <div :data-active="active" @dragenter.prevent="setActive" @dragover.prevent="setActive"
      @dragleave.prevent="setInactive" @drop.prevent="onDrop">
      <!-- share state with the scoped slot -->
      <slot :dropZoneActive="active"></slot>
    </div>

    <div class="card">
    </div>
    <!-- <el-input v-model="input" placeholder="Please input" /> -->

    <div id="drop-area">
      <form class="my-form">
        <p> dashed region</p>
        <input type="file" id="fileElem" multiple accept="image/*" onchange="handleFiles(this.files)">
        <label class="button" for="fileElem">Select some files</label>
      </form>
      <progress id="progress-bar" max=100 value=0></progress>
      <div id="gallery"></div>
    </div>
  </div>



</template>

<style scoped>
.read-the-docs {
  color: #888;
}

body {
  font-family: sans-serif;
}

a {
  color: #369;
}

.note {
  width: 500px;
  margin: 50px auto;
  font-size: 1.1em;
  color: #333;
  text-align: justify;
}

#drop-area {
  border: 2px dashed #ccc;
  border-radius: 20px;
  width: 480px;
  margin: 50px auto;
  padding: 20px;
}

#drop-area.highlight {
  border-color: purple;
}

p {
  margin-top: 0;
}

.my-form {
  margin-bottom: 10px;
}

#gallery {
  margin-top: 10px;
}

#gallery img {
  width: 150px;
  margin-bottom: 10px;
  margin-right: 10px;
  vertical-align: middle;
}

.button {
  display: inline-block;
  padding: 10px;
  background: #ccc;
  cursor: pointer;
  border-radius: 5px;
  border: 1px solid #ccc;
}

.button:hover {
  background: #ddd;
}

#fileElem {
  display: none;
}
</style>
