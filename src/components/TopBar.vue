<template>
  <!-- jindutiao -->
  <el-row class="row">
    <el-col :span="18" class="left">
      <div class="photoSelector">
        <el-button class="btn">选择图片</el-button>
        <el-progress id="progress-bar" :percentage="100" status="success"></el-progress>
      </div>
    </el-col>
    <el-col :span="6" class="right">
      <div class="previewer">
        <el-button class="btn">预览图片</el-button>
      </div>
      <div class="options">
        <el-icon>
          <i-ep-arrow-right />
        </el-icon>
      </div>
    </el-col>
  </el-row>
</template>

<script setup lang="ts"></script>

<style scoped>
.left {
  display: flex;
}

.left .photoSelector {
  margin-left: 30px;
}

.row {
  padding-top: 15px;
}

.previewer {
  margin: 2px 15px 2px 15px;
}

.photoSelector,
.right {
  display: flex;
  justify-content: center;
  flex-direction: row;
}

.el-progress--line {
  margin-left: 20px;
  margin-bottom: 8px;
  width: 350px;
}

.options {
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-left: 60px;
}
</style>
