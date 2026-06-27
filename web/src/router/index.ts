import { createRouter, createWebHistory } from "vue-router";
import LandingView from "@/views/LandingView.vue";
import OidcCallbackView from "@/views/OidcCallbackView.vue";
import WorkbenchView from "@/views/WorkbenchView.vue";

export const routes = [
  {
    path: "/",
    name: "landing",
    component: LandingView,
    meta: { public: true },
  },
  {
    path: "/auth/callback/oidc",
    name: "oidc-callback",
    component: OidcCallbackView,
    meta: { public: true },
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

const PUBLIC_PATHS = ["/", "/auth/callback/oidc"];

router.beforeEach(async (to, _from, next) => {
  if (PUBLIC_PATHS.includes(to.path)) {
    next();
    return;
  }

  const token = localStorage.getItem("issueflow_token");
  if (!token) {
    next("/");
    return;
  }

  try {
    const resp = await fetch("/api/auth/me", {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (resp.ok) {
      next();
    } else {
      localStorage.removeItem("issueflow_token");
      next("/");
    }
  } catch {
    // network error — allow through, component will retry
    next();
  }
});
