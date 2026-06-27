export interface UserInfo {
  user_id: number;
  sub: string;
}

export async function me(token: string): Promise<UserInfo | null> {
  const resp = await fetch("/api/auth/me", {
    headers: { Authorization: `Bearer ${token}` },
  });
  if (!resp.ok) return null;
  return resp.json();
}

export const LOGIN_URL = "/api/auth/login";
