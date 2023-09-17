<script lang="ts" setup>
import AppHeader from "./components/head/AppHeader.vue";
import AppMain from "./components/main/AppMain.vue"
import AppSide from "./components/side/AppSide.vue";
import AppFooter from "./components/footer/AppFooter.vue"
import { messageTimerStore } from "./script/store";

for (let index = 0; index < 10; index++) {
  messageTimerStore.push({
    date: new Date(),
    message: "aaa",
    groups: [1, 2],
    description: "ttt",
    sheduled: true,
  });
}

</script>

<template>
  <div class="common-layout">
    <el-container direction="vertical">
      <el-affix>
        <el-header id="header" height="25px">
          <app-header />
        </el-header>
      </el-affix>
      <el-container>
        <el-container>
          <el-main id="main">
            <app-main />
          </el-main>
          <el-aside id="aside" width="20vw">
            <app-side />
          </el-aside>
        </el-container>
        <el-footer id="footer">
          <app-footer />
        </el-footer>
      </el-container>
    </el-container>
  </div>
</template>

<style scoped>
#header {
  background-color: #1d3043;
  padding: 0;
}

#main {
  height: 70vh;
  margin: 0;
  display: flex;
}

#aside {
  padding: 0.5vw;
  height: 70vh;
  background-color: #1d1d1d;
}

#footer {
  width: 100%;
  height: 25vh;
  margin: 0;
  padding: 0.5vw;
  background-color: #303030;
  box-shadow: --el-box-shadow-dark;
}
</style>
