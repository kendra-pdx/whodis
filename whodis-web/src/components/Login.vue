<script setup lang="ts">
import { ref } from 'vue';

const { accountId, appId } = defineProps<{
    accountId: string,
    appId: string,
}>()

const username = ref<String>("");
const password = ref<String>("");

function doLogin() {
    console.log("doLogin:", "accountId", accountId, "appId", appId, "username", username.value, "password", "***")
}
</script>
<template>
    <div>
        <label>Username</label>
        <input type="text" placeholder="Username" v-model="username" />
    </div>

    <div>
        <label>Password</label>
        <input type="password" placeholder="Password" v-model="password" />
    </div>
    <div>
        <button @click="doLogin">Login</button>
        <a href="#register">Register</a>|<a href="#recover">Lost Account</a>
    </div>
</template>
<style scoped></style>