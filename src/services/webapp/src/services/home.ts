import CONFIG from "@/lib/config";
import { apiFetch, type ApiError } from "@/lib/api";

export type { ApiError };

export interface Home {
  id: string;
  name: string;
  address: string;
  writeToken: string;
}

export interface NewHome {
  name: string;
  address: string;
}

export interface UpdateHome {
  name: string;
  address: string;
}

export const homeService = {
  getHomes: (): Promise<Home[]> =>
    apiFetch<Home[]>(CONFIG.endpoints.home.getHomes),

  createHome: (payload: NewHome): Promise<Home> =>
    apiFetch<Home>(CONFIG.endpoints.home.postHome, {
      method: "POST",
      body: payload,
    }),

  updateHome: (id: string, payload: UpdateHome): Promise<Home> =>
    apiFetch<Home>(CONFIG.endpoints.home.putHome(id), {
      method: "PUT",
      body: payload,
    }),
};
