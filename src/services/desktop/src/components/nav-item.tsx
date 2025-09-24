import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
import { NavLink } from "react-router-dom";

type Props = {
  to: string;
  icon: IconDefinition;
  size?: number;
  className?: string;
};

export default function NavItem({ to, icon, size = 24, className }: Props) {
  const [width, height, , , svgPathData] = icon.icon;

  return (
    <NavLink
      to={to}
      className={({ isActive }) =>
        `${className} ${isActive ? "active" : ""}`
      }
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox={`0 0 ${width} ${height}`}
        width={size}
        height={size}
        fill="currentColor"
      >
        {typeof svgPathData === "string" ? (
          <path d={svgPathData} />
        ) : (
          svgPathData.map((d, i) => <path key={i} d={d} />)
        )}
      </svg>
    </NavLink>
  );
}
