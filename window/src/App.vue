<script lang="ts" setup>
import { reactive, ref } from "vue";
import Terminal from "vue-web-terminal";
import { config as terminalConfig, onExecCmd } from "./script/terminal";
import { config as drawerConfig } from "./script/drawer";
import MessageTimer from "./components/MessageTimer.vue";
import { messageTimerStore } from "./script/store";

messageTimerStore.push({
  date: new Date(),
  message: "aaa",
  groups: [1, 2],
  sheduled: true,
});

const dialog = ref(true);
const userInput = reactive({ text: "" });
</script>

<template>
  <div class="common-layout">
    <el-drawer v-model="dialog" v-bind="drawerConfig">
      <el-text type="info">there are no current news yet</el-text>
    </el-drawer>
    <el-container>
      <el-container>
        <el-main class="main"
          ><terminal v-bind="terminalConfig" @exec-cmd="onExecCmd"></terminal>
        </el-main>
        <el-aside class="aside" width="20vw">
          <message-timer />
        </el-aside>
      </el-container>
      <el-footer class="footer">
        <el-form>
          <el-form-item :inline="true" :model="userInput">
            <el-input
              v-model="userInput.text"
              resize="none"
              rows="7"
              type="textarea"
              placeholder="Enter your message..."
            />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="">发送</el-button>
          </el-form-item>
        </el-form>
      </el-footer>
    </el-container>
  </div>
</template>

<style scoped>
.main {
  height: 70vh;
  margin: 0;
  display: flex;
}
.aside {
  padding: 0.5vw;
  height: 70vh;
  background-color: #1d1d1d;
}
.footer {
  width: 100%;
  height: 30vh;
  margin: 0;
  padding: 0.5vw;
  background-color: #303030;
}
</style>
