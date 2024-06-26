// import { devtools } from "@vue/devtools";
import { createApp } from "vue";
import App from "./App.vue";

import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import { md3 } from "vuetify/blueprints";
import eventBus from "vue3-eventbus";

import "./assets/main.postcss";

import router from "./router";

if (process.env.NODE_ENV === "development") {
    // devtools.connect('http://localhost', 8098)
}

const vuetify = createVuetify({
    components,
    directives,
    blueprint: md3,
});

createApp(App).use(router).use(vuetify).use(eventBus).mount("#app");
