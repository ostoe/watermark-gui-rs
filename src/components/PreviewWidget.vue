<template>
  <div class="widget" v-if="previewwidget.inputValue" :style="{
    'top': realTop + 'px',
    'right': realRight + 'px',
    'height': realHeight + 'px',
    'width': realWidth + 'px',
    'left': realLeft + 'px'
  }">
    <div class="mini-widget-handle" @mousedown="startMove">
      <div></div>
      <span class="mini-widget-close" @click="close">×</span>
    </div>
    <div class="widget-body">
      <slot></slot>
    </div>

    <div class="realTop-handle" @mousedown="startChangeHeightAndTop">
    </div>
    <div class="bottom-handle" @mousedown='startChangeHeightAndBottom'>
    </div>
    <div class="left-handle" @mousedown="startChangeWidthAndLeft">
    </div>
    <div class="realRight-handle" @mousedown="startChangeWidthAndRight">
    </div>
    <div class="left-realTop-handle" @mousedown="startChangeLeftTop">
    </div>
    <div class="realRight-realTop-handle" @mousedown="startChangeRightTop">
    </div>
    <div class="left-bottom-handle" @mousedown="startChangeLeftBottom">
    </div>
    <div class="realRight-bottom-handle" @mousedown="startChangeRightBottom">
    </div>
  </div>
</template>
    
<script setup lang='ts'>
import { ref, onMounted ,reactive,computed} from "vue";
import { previewwidget } from "../scripts/reactives"
// const inputValue = computed({
//   get:()=>{
//     return value.value
//   },
//   set:(val)=>{
//     console.log("val"+val)
//   }
// })

onMounted(()=>{
  document.addEventListener('mouseup', onMouseUp)
})

onBeforeUnmount(()=>{
  document.removeEventListener('mouseup', onMouseUp)
})
const lastX = ref(0)
const lastY = ref(0)
const realTop = ref(90)
const realHeight = ref(55)
const realLeft = ref(70)
const realRight = ref(0)
const realWidth = ref(450)
const close = () => {
  previewwidget.inputValue = false
}
const startMove = (event:any) => {
  document.addEventListener('mousemove', onMove)
  //$rxbus.$on('canvasMouseMove', mouseMove)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const startChangeHeightAndTop = (event:any) => {
  document.addEventListener('mousemove', onExpandTop)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const startChangeHeightAndBottom = (event:any) => {
  document.addEventListener('mousemove', onExpandBottom)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const startChangeWidthAndLeft = (event:any) => {
  document.addEventListener('mousemove', onExpandLeft)
  lastX.value = event.screenX
  forbidSelect()
}
const startChangeWidthAndRight = (event:any) => {
  document.addEventListener('mousemove', onExpandRight)
  lastX.value = event.screenX
  forbidSelect()
}
const startChangeLeftTop = (event:any) => {
  document.addEventListener('mousemove', onExpandLeftTop)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const startChangeRightTop = (event:any) => {
  document.addEventListener('mousemove', onExpandRightTop)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const startChangeLeftBottom = (event:any) => {
  document.addEventListener('mousemove', onExpandLeftBottom)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const startChangeRightBottom = (event:any) => {
  document.addEventListener('mousemove', onExpandRightBottom)
  lastX.value = event.screenX
  lastY.value = event.screenY
  forbidSelect()
}
const onMove = (event:any) => {
  if (realLeft.value !== 0) {
    realLeft.value = realLeft.value + (event.screenX - lastX.value)
  }
  else {
    realRight.value = realRight.value - (event.screenX - lastX.value)
  }
  realTop.value = realTop.value + (event.screenY - lastY.value)
  lastX.value = event.screenX
  lastY.value = event.screenY
}
const onExpandTop = (event:any) => {
  realTop.value = realTop.value + (event.screenY - lastY.value)
  realHeight.value = realHeight.value - (event.screenY - lastY.value)
  lastY.value = event.screenY
}
const onExpandBottom = (event:any) => {
  //bottom = bottom + (event.screenY - lastY.value)
  realHeight.value = realHeight.value + (event.screenY - lastY.value)
  lastY.value = event.screenY
}
const onExpandLeft = (event:any) => {
  if (realLeft.value) {
    realLeft.value = realLeft.value + (event.screenX - lastX.value)
    realWidth.value = realWidth.value - (event.screenX - lastX.value)
  }
  else {
    realWidth.value = realWidth.value - (event.screenX - lastX.value)
  }
  lastX.value = event.screenX
}
const onExpandRight = (event:any) => {
  realRight.value = realRight.value - (event.screenX - lastX.value)
  realWidth.value = realWidth.value - (lastX.value - event.screenX)
  lastX.value = event.screenX
}
const onExpandLeftTop = (event:any) => {
  if (realLeft.value) {
    realLeft.value = realLeft.value + (event.screenX - lastX.value)
    realWidth.value = realWidth.value - (event.screenX - lastX.value)
  }
  else {
    realWidth.value = realWidth.value - (event.screenX - lastX.value)
  }
  realTop.value = realTop.value + (event.screenY - lastY.value)
  realHeight.value = realHeight.value - (event.screenY - lastY.value)
  lastX.value = event.screenX
  lastY.value = event.screenY
}
const onExpandRightTop = (event:any) => {
  realRight.value = realRight.value - (event.screenX - lastX.value)
  realWidth.value = realWidth.value - (lastX.value - event.screenX)
  realTop.value = realTop.value + (event.screenY - lastY.value)
  realHeight.value = realHeight.value - (event.screenY - lastY.value)
  lastX.value = event.screenX
  lastY.value = event.screenY
}
const onExpandLeftBottom = (event:any) => {
  if (realLeft.value) {
    realLeft.value = realLeft.value + (event.screenX - lastX.value)
    realWidth.value = realWidth.value - (event.screenX - lastX.value)
  }
  else {
    realWidth.value = realWidth.value - (event.screenX - lastX.value)
  }
  realHeight.value = realHeight.value + (event.screenY - lastY.value)
  lastX.value = event.screenX
  lastY.value = event.screenY
}
const onExpandRightBottom = (event:any) => {
  realRight.value = realRight.value - (event.screenX - lastX.value)
  realWidth.value = realWidth.value - (lastX.value - event.screenX)
  realHeight.value = realHeight.value + (event.screenY - lastY.value)
  lastX.value = event.screenX
  lastY.value = event.screenY
}
const onMouseUp = (event:any) => {
  lastX.value = 0
  document.removeEventListener('mousemove', onMove)
  document.removeEventListener('mousemove', onExpandTop)
  document.removeEventListener('mousemove', onExpandBottom)
  document.removeEventListener('mousemove', onExpandLeft)
  document.removeEventListener('mousemove', onExpandRight)
  document.removeEventListener('mousemove', onExpandLeftTop)
  document.removeEventListener('mousemove', onExpandRightTop)
  document.removeEventListener('mousemove', onExpandLeftBottom)
  document.removeEventListener('mousemove', onExpandRightBottom)
  document.body.classList.remove('can-not-be-selected')
  console.log("close")
}
const forbidSelect = () => {
  document.body.classList.add('can-not-be-selected')
}
</script>
    
<style scoped>
.widget ::-webkit-scrollbar {
  width: 0.3rem;
  height: 0.3rem;
  background: #232323;
}

.widget ::-webkit-scrollbar-track {
  border-radius: 0;
}

.widget ::-webkit-scrollbar-thumb {
  border-radius: 0;
  background: #535353;
  transition: all .2s;
}

.widget ::-webkit-scrollbar-thumb:hover {
  background-color: #606060;
}

.widget ::-webkit-scrollbar-corner {
  background: #232323;
}

.widget {
  position: fixed;
  box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.5);
  font-size: 13px;
  display: flex;
  flex-flow: column;
  z-index: 99;
  padding: 5px;
}

.widget {
  background: #26282a;
  color: #c2c2c2;
  border-radius: 3px;
}

.mini-widget-handle {
  width: 100%;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: move;
  font-size: 16px;
  color: #999;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  border-bottom: #363636 solid 1px;
}

.mini-widget-close {
  margin-right: -2px;
  margin-top: -5px;
  cursor: pointer;
}

.realTop-handle {
  position: absolute;
  left: 5px;
  top: 0;
  width: calc(100% - 10px);
  height: 3px;
  cursor: n-resize;
}

.bottom-handle {
  position: absolute;
  left: 5px;
  bottom: 0;
  width: calc(100% - 10px);
  height: 5px;
  cursor: s-resize;
}

.left-handle {
  position: absolute;
  left: 0;
  top: 5px;
  height: calc(100% - 10px);
  width: 5px;
  cursor: w-resize;
}

.realRight-handle {
  position: absolute;
  right: 0;
  top: 5px;
  height: calc(100% - 10px);
  width: 5px;
  cursor: e-resize;
}

.left-realTop-handle {
  position: absolute;
  left: 0;
  top: 0;
  height: 6px;
  width: 6px;
  cursor: nw-resize;
}

.realRight-realTop-handle {
  position: absolute;
  right: 0;
  top: 0;
  height: 6px;
  width: 6px;
  cursor: ne-resize;
}

.left-bottom-handle {
  position: absolute;
  left: 0;
  bottom: 0;
  height: 6px;
  width: 6px;
  cursor: sw-resize;
}

.realRight-bottom-handle {
  position: absolute;
  right: 0;
  bottom: 0;
  height: 6px;
  width: 6px;
  cursor: se-resize;
}

.widget-body {
  flex: 1;
  display: flex;
}
</style>