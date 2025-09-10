import { faHome, faUser, faGear, faChartLine } from "@fortawesome/free-solid-svg-icons";
import NavItem from "./nav-item";

export default function NavBar(){
    return(
              
            <aside className="navbar">
                <NavItem icon={faHome} size={28} className="nav-item" />
                <NavItem icon={faUser} size={28} className="nav-item" />
                <NavItem icon={faChartLine} size={28} className="nav-item" />
                <NavItem icon={faGear} size={28} className="nav-item" />
            </aside>
    );
}