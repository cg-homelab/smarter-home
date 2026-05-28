import { cookies } from "next/headers";
import { NextResponse } from "next/server";

function decodeTokenPayload(
  token: string,
): { sub: string; role: string; id: string | null; exp: number } | null {
  try {
    const base64 = token.split(".")[1].replace(/-/g, "+").replace(/_/g, "/");
    return JSON.parse(atob(base64));
  } catch {
    return null;
  }
}

export async function GET() {
  const cookieStore = await cookies();
  const token = cookieStore.get("__session")?.value;

  if (!token) {
    return NextResponse.json({ user: null });
  }

  const payload = decodeTokenPayload(token);

  if (!payload || payload.exp * 1000 < Date.now()) {
    const response = NextResponse.json({ user: null });
    response.cookies.delete("__session");
    return response;
  }

  return NextResponse.json({
    user: { email: payload.sub, role: payload.role, id: payload.id },
  });
}
