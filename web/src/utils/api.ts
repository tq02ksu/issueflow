import { createDiscreteApi } from "naive-ui";

const { message } = createDiscreteApi(["message"]);

interface ProblemDetail {
  title: string;
  detail: string;
}

export async function apiFetch(
  url: string,
  init?: RequestInit,
): Promise<Response> {
  const resp = await fetch(url, init);

  if (!resp.ok && resp.status >= 400) {
    let msg = `Request failed (${resp.status})`;
    try {
      const body: ProblemDetail = await resp.clone().json();
      if (body.detail) {
        msg = body.detail;
      } else if (body.title) {
        msg = body.title;
      }
    } catch {
      // not JSON, use status text
    }
    message.error(msg);
  }

  return resp;
}
