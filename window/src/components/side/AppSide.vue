<script setup lang="ts">
import { Clock, Delete, Download, Open, Setting, TurnOff } from "@element-plus/icons-vue";
import MessageTimer from './MessageTimer.vue';
import { installEnv, uninstallEnv, startBot, stopBot_forcbly, openBotConfig, isBotOn } from '../../script/env'
(document.querySelectorAll(".option-item") as NodeListOf<HTMLButtonElement>).forEach(i => {
    i.addEventListener("click", () => i.blur())
})
</script>

<template>
    <el-button type="primary" class="option-item" :icon="Download" @click="installEnv">安装环境</el-button>
    <el-button type="danger" class="option-item" :icon="Delete" @click="uninstallEnv">卸载环境</el-button>
    <el-button v-if="!isBotOn" type="primary" class="option-item" :icon="TurnOff" @click="startBot">启动bot</el-button>
    <el-button v-else type="danger" class="option-item" :icon="Open" @click="stopBot_forcbly">关闭bot(强制)</el-button>
    <el-button type="primary" class="option-item" :icon="Setting" @click="openBotConfig">打开bot配置文件</el-button>
    <el-popover placement="left-start" trigger="click">
        <template #reference>
            <el-button type="primary" class="option-item" :icon="Clock">消息定时器</el-button>
        </template>
        <message-timer />
    </el-popover>
</template>

<style scoped>
.option-item {
    margin: 16px
}
</style>