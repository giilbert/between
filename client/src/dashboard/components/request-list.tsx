import { Badge } from "@/ui/components/badge";
import { Link } from "@tanstack/react-router";

type BasicRequestInfo = {
  id: string;
  path: string;
  method: string;
  status: number;
  type?: string;
  // Time it took the request to complete in milliseconds
  duration?: number;
};

const TEST_REQUESTS: BasicRequestInfo[] = [
  {
    id: "1",
    path: "/api/v1/test",
    method: "GET",
    status: 200,
    type: "application/json",
    duration: 100,
  },
  {
    id: "2",
    path: "/",
    method: "GET",
    status: 404,
    type: "text/html",
    duration: 100,
  },
  {
    id: "3",
    path: "/api/v1/test",
    method: "POST",
    status: 302,
    type: "application/json",
  },
  {
    id: "4",
    path: "/api/v1/test",
    method: "GET",
    status: 200,
    type: "application/json",
    duration: 100,
  },
  {
    id: "5",
    path: "/api/ws",
    method: "GET",
    status: 101,
  },
];

export const RequestList: React.FC = () => {
  return (
    <div className="flex gap-1 flex-col">
      {TEST_REQUESTS.map((r) => (
        <Link
          to="/$id"
          params={{
            id: r.id,
          }}
        >
          <div
            key={r.id}
            className="hover:bg-muted/50 px-2 py-1 rounded-sm transition-colors flex gap-2.5 items-center"
          >
            <Badge
              className="font-mono"
              variant={determineBadgeVariantFromStatus(r.status)}
            >
              {r.status}
            </Badge>
            <p className="min-w-10 font-mono">{r.method}</p>
            <p className="font-mono">{r.path}</p>

            {r.type && (
              <p className="ml-auto text-muted-foreground">{r.type}</p>
            )}
          </div>
        </Link>
      ))}
    </div>
  );
};

const determineBadgeVariantFromStatus = (status: number) => {
  if (status >= 200 && status < 300) {
    return "default";
  }

  if (status >= 300 && status < 400) {
    return "secondary";
  }

  if (status >= 400 && status < 500) {
    return "destructive";
  }

  if (status >= 500 && status < 600) {
    return "destructive";
  }

  return "outline";
};
