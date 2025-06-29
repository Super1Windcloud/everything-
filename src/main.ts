import { createApp } from "vue";
import App from "./App.vue";
// import  "./base.css"
import PrimeVue from "primevue/config";
import { createPinia } from "pinia";
import "primeicons/primeicons.css";

import "@imengyu/vue3-context-menu/lib/vue3-context-menu.css";
import ContextMenu from "@imengyu/vue3-context-menu";

const app = createApp(App);
app.use(PrimeVue);
app.use(createPinia());
app.use(ContextMenu);
app.mount("#app");
