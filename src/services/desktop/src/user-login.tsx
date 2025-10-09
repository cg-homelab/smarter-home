import "./global.css";
import NavItem from "./components/nav-item"
import { faSignIn } from "@fortawesome/free-solid-svg-icons";
import InputField from "./components/input";

function UserLogin() {

  return (
    <main className="container">
        <h1>Login</h1>

        <form className="login-form">
            <InputField label="Username" placeholder="Username" />
            <InputField label="Password" type="password" placeholder="Password" />
            <NavItem to="/user" icon={faSignIn} size={28} className="login-button" />
        </form>

    </main>
  );
}

export default UserLogin;
