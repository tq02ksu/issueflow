export function authHeaders(): Record<string, string> {
  const token = localStorage.getItem("issueflow_token");
  if (!token) return {};
  return { Authorization: `Bearer ${token}` };
}
