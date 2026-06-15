import { createRouter, createWebHistory } from "vue-router";
import LandingView from "@/views/LandingView.vue";
import OidcCallbackView from "@/views/OidcCallbackView.vue";
import WorkbenchView from "@/views/WorkbenchView.vue";

export const routes = [
  {
    path: "/",
    name: "landing",
    component: LandingView,
  },
  {
    path: "/auth/callback/oidc",
    name: "oidc-callback",
    component: OidcCallbackView,
  },
  {
    path: "/workbench",
    name: "workbench",
    component: WorkbenchView,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
