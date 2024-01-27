import { Outlet, createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/router";
import { AssertConnected } from "@/api/components/assert-connected";
import { RequestList } from "./components/request-list";

export const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: () => (
    <AssertConnected fallback={<p>Loading..</p>}>
      <div className="grid grid-cols-3 gap-2">
        <RequestList />
        <div className="col-span-2">
          <Outlet />
        </div>
      </div>
    </AssertConnected>
  ),
});

export const requestViewRoute = createRoute({
  getParentRoute: () => indexRoute,
  path: "/$id",
  component: () => {
    const params = requestViewRoute.useParams();

    return (
      <AssertConnected fallback={<p>Loading..</p>}>
        <p>Viewing request with id {params.id}</p>
      </AssertConnected>
    );
  },
});
