import { invoke } from "@tauri-apps/api/core";
import { parse, stringify } from "smol-toml";

enum SortType {
  Name = "Name",
  Path = "Path",
  Size = "Size",
  Extension = "Extension",
  FileType = "FileType",
  ModifiedDate = "ModifiedDate",
}

enum FileTypes {
  All = "All",
  Exe = "Exe",
  Folder = "Folder",
  Audio = "Audio",
  Video = "Video",
  Image = "Image",
  Document = "Document",
  Archive = "Archive",
}

export interface ViewConfig {
  filter: boolean;
  preview: boolean;
  statusBar: boolean;
  fontSize: number;
  sortBy: SortType;
}

export interface SearchConfig {
  CaseSensitive: boolean;
  WholeWord: boolean;
  RegexSearch: boolean;
  MatchPath: boolean;
  ShowFileType: FileTypes;
}

export async function readViewConfig(): Promise<ViewConfig> {
  const configStr: string = await invoke("read_config_content");
  const parsed = parse(configStr);
  let view = parsed.view;
  view = stringify(view);
  return JSON.parse(view) as ViewConfig;
}

export async  function readSearchConfig(): Promise<SearchConfig> {
  const configStr: string =  await invoke("read_config_content");
  const parsed = parse(configStr);
  let search = parsed.search;
  search = stringify(search);
  return JSON.parse(search) as SearchConfig;
}
