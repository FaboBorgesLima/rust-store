import { RouteRecordRaw, createMemoryHistory, createRouter } from "vue-router";

import HomeView from "../pages/HomeView.vue";

const routes: RouteRecordRaw[] = [{ path: "/", component: HomeView }];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

export default router;
