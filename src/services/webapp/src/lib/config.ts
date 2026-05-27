const CONFIG = {
  apiBaseUrl: import.meta.env.VITE_API_URL || "http://localhost:3001",
  tokenKey: "smarter_home_token",
  timeout: 5000,
  endpoints: {
    user: {
      getProfile: "/user/me",
    },
    auth: {
      postLogin: "/auth/login",
      postRegister: "/auth/signup",
    },
    home: {
      getHomes: "/home",
      postHome: "/home",
      putHome: (id: string) => `/home/${id}`,
      deleteHome: (id: string) => `/home/${id}`,
    },
    power: {
      getPowerUsage: (homeId: string, startDate: string, endDate: string) =>
        `/power/metrics?home_id=${homeId}&start_date=${startDate}&end_date=${endDate}`,
    },
  },
};

export default CONFIG;
