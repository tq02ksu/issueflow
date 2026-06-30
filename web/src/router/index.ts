import { createRouter, createWebHistory } from "vue-router";
import { me } from "@/api/auth.api";
import LandingView from "@/views/LandingView.vue";
import OidcCallbackView from "@/views/OidcCallbackView.vue";
import WorkbenchView from "@/views/WorkbenchView.vue";
import IssuesView from "@/views/IssuesView.vue";
import PendingActionsView from "@/views/PendingActionsView.vue";

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
  {
    path: "/workbench/issues",
    name: "workbench-issues",
    component: IssuesView,
  },
  {
    path: "/workbench/pending-actions",
    name: "workbench-pending-actions",
    component: PendingActionsView,
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
    const info = await me(token);
    if (info) {
      next();
    } else {
      localStorage.removeItem("issueflow_token");
      next("/");
    }
  } catch {
    // backend unreachable — redirect to landing
    next("/");
  }
});
