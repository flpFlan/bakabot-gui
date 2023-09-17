<script setup lang="ts">
import Terminal from "vue-web-terminal";
import { config, onExecCmd } from "../../script/terminal";
</script>

<template>
    <terminal v-bind="config" @exec-cmd="onExecCmd"></terminal>
</template>

<style scoped>
</style>