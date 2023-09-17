<script setup lang="ts">
import { ref } from "vue";
import { messageTimerStore } from "../../script/store";

const activeName = ref("1");
</script>

<template>
  <el-scrollbar max-height="65vh">
    <el-collapse v-model="activeName" accordion>
      <el-collapse-item v-for="item in messageTimerStore" :name="messageTimerStore.indexOf(item) + 1">
        <template #title>
          <el-text size="small" :type="item.sheduled ? 'success' : 'danger'" truncated>{{ item.description }} </el-text>
        </template>
        <el-switch active-text="开" inactive-text="关" inline-prompt v-model="item.sheduled" />
        <el-time-picker v-model="item.date" placeholder="select a time" />
      </el-collapse-item>
    </el-collapse>
  </el-scrollbar>
</template>

<style scoped>
el-time-picker {
  display: flex;
}

el-text {
  font-family: 'Helvetica Neue';
}
</style>
