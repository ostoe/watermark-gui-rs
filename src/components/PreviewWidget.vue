<!-- <template>
         <div class="widget"
    v-if="inputValue"
    :style="{
      top : realTop + 'px',
      right : realRight + 'px',
      height : realHeight + 'px',
      width : realWidth + 'px',
      left : realLeft + 'px'
    }"
  >
    <div class="mini-widget-handle"
      @mousedown = "startMove"
    >
      <div></div>
      <span class="mini-widget-close"
        @click="close"
      >×</span>
    </div>
    <div class="widget-body">
      <slot></slot>
    </div>

    <div class="realTop-handle"
      @mousedown = "startChangeHeightAndTop"
    >
    </div>
    <div class="bottom-handle"
      @mousedown = 'startChangeHeightAndBottom'
    >
    </div>
    <div class="left-handle"
      @mousedown = "startChangeWidthAndLeft"
    >
    </div>
    <div class="realRight-handle"
    @mousedown = "startChangeWidthAndRight"
    >
    </div>
    <div class="left-realTop-handle"
      @mousedown = "startChangeLeftTop"
    >
    </div>
    <div class="realRight-realTop-handle"
      @mousedown = "startChangeRightTop"
    >
    </div>
    <div class="left-bottom-handle"
      @mousedown = "startChangeLeftBottom"
    >
    </div>
    <div class="realRight-bottom-handle"
      @mousedown = "startChangeRightBottom"
    >
    </div>
  </div>
</template>
    
<script setup lang='ts'>
    close(){
      this.inputValue = false
    },
    startMove(event){
      document.addEventListener('mousemove', this.onMove)
      //$rxbus.$on('canvasMouseMove', this.mouseMove)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    startChangeHeightAndTop(event){
      document.addEventListener('mousemove', this.onExpandTop)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    startChangeHeightAndBottom(event){
      document.addEventListener('mousemove', this.onExpandBottom)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    startChangeWidthAndLeft(event){
      document.addEventListener('mousemove', this.onExpandLeft)
      this.lastX = event.screenX
      this.forbidSelect()
    },
    startChangeWidthAndRight(event){
      document.addEventListener('mousemove', this.onExpandRight)
      this.lastX = event.screenX
      this.forbidSelect()
    },
    startChangeLeftTop(event){
      document.addEventListener('mousemove', this.onExpandLeftTop)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    startChangeRightTop(event){
      document.addEventListener('mousemove', this.onExpandRightTop)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    startChangeLeftBottom(event){
      document.addEventListener('mousemove', this.onExpandLeftBottom)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    startChangeRightBottom(event){
      document.addEventListener('mousemove', this.onExpandRightBottom)
      this.lastX = event.screenX
      this.lastY = event.screenY
      this.forbidSelect()
    },
    onMove(event){
      if(this.realLeft !== ''){
        this.realLeft = this.realLeft + (event.screenX - this.lastX)
      }
      else{
        this.realRight = this.realRight - (event.screenX - this.lastX)
      }
      this.realTop = this.realTop + (event.screenY - this.lastY)
      this.lastX = event.screenX
      this.lastY = event.screenY
    },
    onExpandTop(event){
      this.realTop = this.realTop + (event.screenY - this.lastY)
      this.realHeight = this.realHeight - (event.screenY - this.lastY)
      this.lastY = event.screenY
    },
    onExpandBottom(event){
      //this.bottom = this.bottom + (event.screenY - this.lastY)
      this.realHeight = this.realHeight + (event.screenY - this.lastY)
      this.lastY = event.screenY
    },
    onExpandLeft(event){
      if(this.realLeft){
        this.realLeft = this.realLeft + (event.screenX - this.lastX)
        this.realWidth = this.realWidth - (event.screenX - this.lastX)
      }
      else{
        this.realWidth = this.realWidth - (event.screenX - this.lastX)
      }
      this.lastX = event.screenX
    },
    onExpandRight(event){
      this.realRight = this.realRight - (event.screenX - this.lastX)
      this.realWidth = this.realWidth - (this.lastX - event.screenX)
      this.lastX = event.screenX
    },
    onExpandLeftTop(event){
      if(this.realLeft){
        this.realLeft = this.realLeft + (event.screenX - this.lastX)
        this.realWidth = this.realWidth - (event.screenX - this.lastX)
      }
      else{
        this.realWidth = this.realWidth - (event.screenX - this.lastX)
      }
      this.realTop = this.realTop + (event.screenY - this.lastY)
      this.realHeight = this.realHeight - (event.screenY - this.lastY)
      this.lastX = event.screenX
      this.lastY = event.screenY
    },
    onExpandRightTop(event){
      this.realRight = this.realRight - (event.screenX - this.lastX)
      this.realWidth = this.realWidth - (this.lastX - event.screenX)
      this.realTop = this.realTop + (event.screenY - this.lastY)
      this.realHeight = this.realHeight - (event.screenY - this.lastY)
      this.lastX = event.screenX
      this.lastY = event.screenY
    },
    onExpandLeftBottom(event){
      if(this.realLeft){
        this.realLeft = this.realLeft + (event.screenX - this.lastX)
        this.realWidth = this.realWidth - (event.screenX - this.lastX)
      }
      else{
        this.realWidth = this.realWidth - (event.screenX - this.lastX)
      }
      this.realHeight = this.realHeight + (event.screenY - this.lastY)
      this.lastX = event.screenX
      this.lastY = event.screenY
    },
    onExpandRightBottom(event){
      this.realRight = this.realRight - (event.screenX - this.lastX)
      this.realWidth = this.realWidth - (this.lastX - event.screenX)
      this.realHeight = this.realHeight + (event.screenY - this.lastY)
      this.lastX = event.screenX
      this.lastY = event.screenY
    },
    onMouseUp(event){
      this.lastX = ''
      document.removeEventListener('mousemove', this.onMove)
      document.removeEventListener('mousemove', this.onExpandTop)
      document.removeEventListener('mousemove', this.onExpandBottom)
      document.removeEventListener('mousemove', this.onExpandLeft)
      document.removeEventListener('mousemove', this.onExpandRight)
      document.removeEventListener('mousemove', this.onExpandLeftTop)
      document.removeEventListener('mousemove', this.onExpandRightTop)
      document.removeEventListener('mousemove', this.onExpandLeftBottom)
      document.removeEventListener('mousemove', this.onExpandRightBottom)
      document.body.classList.remove('can-not-be-selected')
    },
    forbidSelect(){
      document.body.classList.add('can-not-be-selected')
    }
  },
</script>
    
<style>
    
</style> -->