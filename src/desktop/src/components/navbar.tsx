import { faHome, faUser, faGear, faChartLine } from "@fortawesome/free-solid-svg-icons";
import NavItem from "./nav-item";
import "../Global.css";

export default function NavBar() {
  return (
    <aside className="navbar">
      <NavItem to="/" icon={faHome} size={28} className="nav-item" />
      <NavItem to="/user" icon={faUser} size={28} className="nav-item" />
      <NavItem to="/analytics" icon={faChartLine} size={28} className="nav-item" />
      <NavItem to="/settings" icon={faGear} size={28} className="nav-item" />
    </aside>
  );
}
