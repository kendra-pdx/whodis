import { defineStore } from "pinia";
import { ref } from "vue";

enum LoginStatus {
    LoggedOut,
    LoggedIn,
}
type LoggedIn = {
    status: LoginStatus,
    accessToken: string
}

type LoggedOut = {
    status: LoginStatus,
}

export const oauthStore = defineStore("oauth", () => {
    const login = ref<LoggedIn | LoggedOut>({ status: LoginStatus.LoggedOut })

    function isLoggedIn(): boolean {
        return false
    }

    return { isLoggedIn, login }
})