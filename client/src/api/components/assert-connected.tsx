import { useConnection } from "../connection";

export const AssertConnected: React.FC<{
  fallback: React.ReactNode | null;
  children: React.ReactNode;
}> = ({ children, fallback }) => {
  const { status } = useConnection();
  if (status.type !== "connected") return <>{fallback}</>;
  return <>{children}</>;
};
