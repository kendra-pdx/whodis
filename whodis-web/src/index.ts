import { createRouter, createWebHistory } from "vue-router";
import HomeView from "./views/HomeView.vue";
import AccountLogin from "./views/AccountLoginView.vue";

const routes = [
    { path: "/login/:accountId/:appId", component: AccountLogin, props: true },
    { path: "/", component: HomeView },
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})