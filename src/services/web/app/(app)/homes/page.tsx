import { apiFetch } from "@/lib/api";
import { Home, getHomes } from "./actions";
import { HomesClient } from "@/components/homes/homes-client";

export default async function HomesPage() {
  let homes: Home[] = [];
  let fetchError: string | null = null;

  try {
    homes = await getHomes();
  } catch {
    fetchError = "Failed to load homes. Please refresh the page.";
  }

  return <HomesClient initialHomes={homes} initialError={fetchError} />;
}
