import { createContext, useContext } from "react";
import { Socket } from "socket.io-client";

export type ConnectionStatus =
  | {
      type: "disconnected";
    }
  | {
      type: "connecting";
    }
  | {
      type: "connected";
    };

export const ConnectionContext = createContext<{
  status: ConnectionStatus;
  io: Socket | null;
} | null>(null);

export const useConnection = () => {
  const context = useContext(ConnectionContext);
  if (context === null) {
    throw new Error("useConnection must be used within a ConnectionProvider");
  }
  return context;
};

export const useIo = () => {
  const { io } = useConnection();
  if (io === null) {
    throw new Error("useIo must be used within a ConnectionProvider");
  }
  return io;
};
