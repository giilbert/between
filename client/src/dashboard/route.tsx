import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/router";
import { Button } from "@/ui/components/button";
import { AssertConnected } from "@/api/components/assert-connected";

export const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: () => (
    <AssertConnected fallback={<p>Loading..</p>}>
      <Button>Button</Button>
    </AssertConnected>
  ),
});
