import {invoke} from "@tauri-apps/api/core";
import type {Port} from "$lib/types/Port";

export async function usePorts(): Promise<Port[]> {
        return await invoke<Port[]>('get_ports_info');
}
