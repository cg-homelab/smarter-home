import "./App.css";
import { faHome, faUser, faGear, faChartLine } from "@fortawesome/free-solid-svg-icons";
import NavItem from "./components/nav-item";


function App() {


  return (
    <main className="container">

      <h1 className="title">Smarter Home</h1>
      <aside className="navbar">
        <NavItem icon={faHome} size={28} className="nav-item" />
        <NavItem icon={faUser} size={28} className="nav-item" />
        <NavItem icon={faChartLine} size={28} className="nav-item" />
        <NavItem icon={faGear} size={28} className="nav-item" />
      </aside>
    </main>
  );
}

export default App;
