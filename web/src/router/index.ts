import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router";
import { me } from "@/api/auth.api";
import { isMockMode } from "@/app-mode";
import LandingView from "@/views/LandingView.vue";

const defaultRoutes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "landing",
    component: LandingView,
    meta: { public: true },
  },
  {
    path: "/auth/callback/oidc",
    name: "oidc-callback",
    component: () => import("@/views/OidcCallbackView.vue"),
    meta: { public: true },
  },
  {
    path: "/workbench",
    name: "workbench",
    component: () => import("@/views/WorkbenchView.vue"),
  },
  {
    path: "/workbench/issues",
    name: "workbench-issues",
    component: () => import("@/views/IssuesView.vue"),
  },
  {
    path: "/workbench/pending-actions",
    name: "workbench-pending-actions",
    component: () => import("@/views/PendingActionsView.vue"),
  },
];

const mockRoutes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "mock-landing",
    component: LandingView,
    meta: { public: true },
  },
  {
    path: "/workbench",
    name: "mock-workbench-overview",
    component: () => import("@/views/WorkbenchView.vue"),
    meta: { public: true },
  },
  {
    path: "/workbench/issues",
    name: "mock-workbench-issues",
    component: () => import("@/views/IssuesView.vue"),
    meta: { public: true },
  },
  {
    path: "/workbench/mrs",
    name: "mock-workbench-mrs",
    component: () => import("@/views/MrsView.vue"),
    meta: { public: true },
  },
  {
    path: "/workbench/milestones",
    name: "mock-workbench-milestones",
    component: () => import("@/views/MilestonesView.vue"),
    meta: { public: true },
  },
  {
    path: "/settings",
    name: "mock-user-settings",
    component: () => import("@/views/UserSettingsView.vue"),
    meta: { public: true },
  },
];

export const routes = isMockMode ? mockRoutes : defaultRoutes;

export const router = createRouter({
  history: createWebHistory(),
  routes,
});

const PUBLIC_PATHS = isMockMode
  ? routes.map((route) => route.path)
  : ["/", "/auth/callback/oidc"];

router.beforeEach(async (to, _from, next) => {
  if (isMockMode) {
    next();
    return;
  }

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
