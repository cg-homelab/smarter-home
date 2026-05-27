import * as React from "react";
import { authApi, type LoginPayload, type RegisterPayload } from "@/lib/api";

const TOKEN_KEY = "smarter_home_token";

interface JwtPayload {
  sub: string; // email
  role: string;
  id: string | null;
  exp: number;
}

interface AuthUser {
  email: string;
  role: string;
  id: string | null;
}

interface AuthContextValue {
  user: AuthUser | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (payload: LoginPayload) => Promise<void>;
  register: (payload: RegisterPayload) => Promise<void>;
  logout: () => void;
}

const AuthContext = React.createContext<AuthContextValue | null>(null);

function decodeJwt(token: string): JwtPayload | null {
  try {
    const [, payload] = token.split(".");
    const decoded = atob(payload.replace(/-/g, "+").replace(/_/g, "/"));
    return JSON.parse(decoded) as JwtPayload;
  } catch {
    return null;
  }
}

function isTokenExpired(payload: JwtPayload): boolean {
  return payload.exp * 1000 < Date.now();
}

function userFromToken(token: string): AuthUser | null {
  const payload = decodeJwt(token);
  if (!payload || isTokenExpired(payload)) return null;
  return { email: payload.sub, role: payload.role, id: payload.id };
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [token, setToken] = React.useState<string | null>(() => {
    const stored = localStorage.getItem(TOKEN_KEY);
    if (!stored) return null;
    const payload = decodeJwt(stored);
    if (!payload || isTokenExpired(payload)) {
      localStorage.removeItem(TOKEN_KEY);
      return null;
    }
    return stored;
  });

  const user = React.useMemo(
    () => (token ? userFromToken(token) : null),
    [token],
  );

  const storeToken = (newToken: string) => {
    localStorage.setItem(TOKEN_KEY, newToken);
    setToken(newToken);
  };

  const login = async (payload: LoginPayload) => {
    const res = await authApi.login(payload);
    storeToken(res.accessToken);
  };

  const register = async (payload: RegisterPayload) => {
    const res = await authApi.register(payload);
    storeToken(res.accessToken);
  };

  const logout = () => {
    localStorage.removeItem(TOKEN_KEY);
    setToken(null);
  };

  return (
    <AuthContext.Provider
      value={{ user, token, isAuthenticated: !!user, login, register, logout }}
    >
      {children}
    </AuthContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useAuth(): AuthContextValue {
  const ctx = React.useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used inside <AuthProvider>");
  return ctx;
}
