import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/router";
import { Button } from "@/ui/components/button";

export const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: () => (
    <>
      <Button>Button</Button>
    </>
  ),
});
