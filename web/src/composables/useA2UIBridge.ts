export function useA2UIBridge() {
  function handleCustom(value: unknown) {
    if (
      value &&
      typeof value === "object" &&
      (value as Record<string, unknown>).kind === "a2ui_render"
    ) {
      // A2UI rendering will be wired when a2ui-vue integration is stable
      // For now, the custom event is stored in agent.messages for display
    }
  }

  function buildSubmit(
    surfaceId: string,
    payload: Record<string, unknown>,
  ) {
    return {
      role: "user" as const,
      content: {
        kind: "a2ui_submit",
        surface_id: surfaceId,
        payload,
      },
    };
  }

  return { handleCustom, buildSubmit };
}
