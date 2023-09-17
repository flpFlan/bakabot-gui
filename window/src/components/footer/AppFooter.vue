<script setup lang="ts">
import { reactive } from 'vue';
const userInput = reactive({ text: "" });
</script>

<template>
    <el-form>
        <el-form-item :inline="true" :model="userInput">
            <el-input v-model="userInput.text" resize="none" rows="6" type="textarea" placeholder="Enter your message..." />
        </el-form-item>
        <!-- <el-form-item>
            <el-button type="primary">发送</el-button>
        </el-form-item> -->
    </el-form>
</template>

<style scoped></style>