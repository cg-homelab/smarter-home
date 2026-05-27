// Shared TypeScript interfaces — no runtime code, safe to import from both
// server components and client components.

export interface AuthUser {
  email: string;
  role: string;
  id: string | null;
}
