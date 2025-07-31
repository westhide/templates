<script setup lang="ts">
import { get_username } from "t-web";
import { onMounted } from "vue";

onMounted(async () => {
    const user = await get_username("username")
    console.log(user)
})
</script>

<template>
    <div>Username</div>
</template>

<style scoped></style>
