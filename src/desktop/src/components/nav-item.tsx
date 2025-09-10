import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";


type Props = {
  icon: IconDefinition;
  size?: number;
  className?: string;
};

export default function NavItem({ icon, size = 24, className }: Props) {
  const [width, height, , , svgPathData] = icon.icon;

  return (
      <button 
        className={className}>
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
      </button>
  );
}