<script setup lang="ts">
import { useRoute } from 'vue-router';
import Login from '../components/Login.vue';
const props = defineProps<{
    accountId: string,
    appId: string,
}>();
</script>
<template>
    <h1>Account Login</h1>
    <div>
        Logging in to account: {{ accountId }}, app: {{ appId }}
    </div>
    <Login :accountId="props.accountId" :appId="props.appId" />
</template>
<style scoped></style>