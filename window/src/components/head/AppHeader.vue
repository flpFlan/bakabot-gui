<script setup lang="ts">
import { FullScreen } from "@element-plus/icons-vue";
import { Close } from "@element-plus/icons-vue";
import { Minus } from "@element-plus/icons-vue";
import { appWindow } from "@tauri-apps/api/window";

const handleMinimize = (e: Event) => {
    appWindow.minimize(); let target = (e.target as HTMLInputElement); target.blur();
    if (target.nodeName == "svg") (target.parentNode?.parentNode as HTMLInputElement).blur();
}
const handleMaxmize = (e: Event) => {
    appWindow.toggleMaximize(); let target = (e.target as HTMLInputElement); target.blur();
    if (target.nodeName == "svg") (target.parentNode?.parentNode as HTMLInputElement).blur();
}
const handleClose = (e: Event) => {
    appWindow.close(); let target = (e.target as HTMLInputElement); target.blur();
    if (target.nodeName == "svg") (target.parentNode?.parentNode as HTMLInputElement).blur();
}
</script>

<template>
    <div class="titlebar" data-tauri-drag-region>
        <el-button-group size="large" class="operator">
            <el-button :icon="Minus" @click="handleMinimize" class="titlebar-button" />
            <el-button :icon="FullScreen" @click="handleMaxmize" class="titlebar-button" />
            <el-button :icon="Close" @click="handleClose" class="titlebar-button close-button" />
        </el-button-group>
    </div>
</template>

<style scoped>
.titlebar{
    display: flex;
    justify-content: flex-end;
}
.menu {
    height: 5vh;
    display: flex;
    justify-content: flex-start;
}

.operator {
    margin: 0;
}

.titlebar-button {
    height: 25px;
    border: none;
    border-radius: 0;
}
.close-button:hover{
    background-color: red;
}
</style>