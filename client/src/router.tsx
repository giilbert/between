import { createRouter } from "@tanstack/react-router";
import { ThemeProvider } from "./ui/components/theme-provider";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { indexRoute, requestViewRoute } from "./dashboard/route";
import { Navbar } from "./ui/components/navbar";
import { ConnectionWrapper } from "./api/components/connection-wrapper";

export const rootRoute = createRootRoute({
  component: () => (
    <ConnectionWrapper>
      <Navbar />
      <main className="m-2.5">
        <Outlet />
      </main>
    </ConnectionWrapper>
  ),
});

export const routeTree = rootRoute.addChildren([indexRoute, requestViewRoute]);

export const router = createRouter({
  routeTree,
  Wrap: ({ children }) => <ThemeProvider>{children}</ThemeProvider>,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
