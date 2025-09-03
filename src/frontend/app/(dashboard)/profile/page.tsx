import { auth } from "@/lib/auth";

export default async function ProfilePage() {
  const session = await auth();
  const user = session?.user;

  if (!user) {
    return (
      <div className="p-6">
        <h1 className="text-2xl font-bold">Your profile page</h1>
        <p>You're not signed in yet.</p>
      </div>
    );
  }

}
