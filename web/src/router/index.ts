import { createRouter, createWebHistory } from "vue-router";
import LandingView from "@/views/LandingView.vue";
import OAuthCallbackView from "@/views/OAuthCallbackView.vue";
import WorkbenchView from "@/views/WorkbenchView.vue";

export const routes = [
  {
    path: "/",
    name: "landing",
    component: LandingView,
  },
  {
    path: "/auth/callback/:provider",
    name: "oauth-callback",
    component: OAuthCallbackView,
    props: true,
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
