import CONFIG from "@/lib/config";
import { handleResponse, type ApiError } from "../lib/api";

export type { ApiError };

export interface AuthResponse {
  message: string;
  accessToken: string;
  tokenType: string;
}

export interface LoginPayload {
  email: string;
  password: string;
}

export interface RegisterPayload {
  email: string;
  password: string;
  firstName: string;
  lastName: string;
}

export const authService = {
  login: (payload: LoginPayload): Promise<AuthResponse> =>
    fetch(CONFIG.apiBaseUrl + CONFIG.endpoints.auth.postLogin, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    }).then(handleResponse<AuthResponse>),

  register: (payload: RegisterPayload): Promise<AuthResponse> =>
    fetch(CONFIG.apiBaseUrl + CONFIG.endpoints.auth.postRegister, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    }).then(handleResponse<AuthResponse>),
};
