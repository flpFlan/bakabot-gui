import { createApp } from "vue";
import "./styles.css";
import "element-plus/theme-chalk/dark/css-vars.css";
import Terminal from "vue-web-terminal";
import App from "./App.vue";
import { loadStore } from "./script/store";

loadStore();
const app = createApp(App).use(Terminal);
app.mount("#app");
