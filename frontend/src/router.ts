import { createMemoryHistory, createRouter } from "vue-router";

import HomeView from "./pages/HomeView.vue";
import AboutView from "./pages/AboutView.vue";
import AddView from "./pages/AddView.vue";

const routes = [
    { path: "/", component: HomeView },
    { path: "/about", component: AboutView },
    { path: "/add", component: AddView },
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

export default router;
