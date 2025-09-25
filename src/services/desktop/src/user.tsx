import "./global.css";
import NavItem from "./components/nav-item"
import { faSignIn } from "@fortawesome/free-solid-svg-icons";
import UserCard from "./components/user-card";

function User() {

  return (
    <main className="container">
        <h1>User Profile</h1>
        <section className= "login-section">
            <h2>You are logged out</h2>
            <NavItem to="/login" icon={faSignIn} size={28} className="login-button"/>
            <UserCard name="Ola Nordmann" email="ola.nordmann@example.com"/>
        </section>
    </main>
  );
}

export default User;
