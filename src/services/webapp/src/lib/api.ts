const API_BASE_URL = import.meta.env.VITE_API_URL ?? "http://localhost:3001";

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

export interface ApiError {
  message: string;
  status: number;
}

async function handleResponse<T>(res: Response): Promise<T> {
  if (!res.ok) {
    let message = res.statusText;
    try {
      const body = await res.text();
      if (body) message = body;
    } catch {
      // ignore parse error
    }
    throw { message, status: res.status } satisfies ApiError;
  }
  return res.json() as Promise<T>;
}

export const authApi = {
  login: (payload: LoginPayload): Promise<AuthResponse> =>
    fetch(`${API_BASE_URL}/auth/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
      },
      body: JSON.stringify(payload),
    }).then(handleResponse<AuthResponse>),

  register: (payload: RegisterPayload): Promise<AuthResponse> =>
    fetch(`${API_BASE_URL}/auth/signup`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
      },
      body: JSON.stringify(payload),
    }).then(handleResponse<AuthResponse>),
};
