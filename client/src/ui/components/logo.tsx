import { forwardRef } from "react";

export const Logo = forwardRef<
  SVGSVGElement,
  {
    size?: number;
  }
>(({ size = 24 }, ref) => {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className="text-foreground"
      ref={ref}
    >
      <path d="M75 25L100 0V100H75V25Z" fill="currentColor" />
      <rect x="38" width="24" height="100" fill="currentColor" />
      <path
        d="M25 75L9.53674e-06 100L7.94466e-07 2.18557e-06L25 0L25 75Z"
        fill="currentColor"
      />
    </svg>
  );
});
