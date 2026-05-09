import { invoke } from "@tauri-apps/api/core";

export async function getOpencodeMessages(): Promise<string> {
  return await invoke<string>("get_opencode_messages");
}

export async function getUnifiedMessages(client: string): Promise<string> {
  return await invoke<string>("get_unified_messages", { client });
}
