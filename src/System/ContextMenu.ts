import { QueryResult } from "@/System/GetQueryResult.ts";
import { invoke } from "@tauri-apps/api/core";

export function openPathWithExplorer(currentRow: QueryResult | null) {
  console.log(currentRow);
  invoke("open_path_with_explorer", { filePath: currentRow?.file_path }).then(
    (_r) => {},
  );
}

export function copyPathToClipboard(currentRow: QueryResult | null) {
  invoke("copy_to_clipboard", { filePath: currentRow?.file_path });
}

export function copyFile(currentRow: QueryResult | null) {
  invoke("copy_to_file", { filePath: currentRow?.file_path });
}

export function deleteFile(currentRow: QueryResult | null) {
  invoke("delete_to_file", { filePath: currentRow?.file_path });
}
