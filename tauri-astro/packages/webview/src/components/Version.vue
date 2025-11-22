<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const version = ref("");

onMounted(async () => {
	version.value = await invoke("version", {});
});
</script>

<template>
    <div>Version: {{ version }}</div>
</template>
