import { createRouter } from "@tanstack/react-router";
import { ThemeProvider } from "./ui/components/theme-provider";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { indexRoute } from "./dashboard/route";

export const rootRoute = createRootRoute({
  component: () => <Outlet />,
});

export const routeTree = rootRoute.addChildren([indexRoute]);

export const router = createRouter({
  routeTree,
  Wrap: ({ children }) => <ThemeProvider>{children}</ThemeProvider>,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
