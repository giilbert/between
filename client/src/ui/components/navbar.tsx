import { Link } from "@tanstack/react-router";
import { Logo } from "./logo";

export const Navbar: React.FC = () => {
  return (
    <nav className="p-2.5 w-screen border-b flex items-center gap-4">
      <Logo />
      <Link to="/">Home</Link>
    </nav>
  );
};
