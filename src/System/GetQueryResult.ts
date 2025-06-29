import { invoke } from "@tauri-apps/api/core";
import { Ref } from "vue";

export interface QueryResult {
  file_name: string;
  file_path: string;
  file_size: string;
  file_extension: string;
  file_type: string;
  file_modified_time: string;
  file_created_time: string;
  total_result_count: number;
  index ?:number;
}

export async function getQueryResult(query: string) {
  const result: QueryResult[] = await invoke("request_everything_ipc", {
    query,
  });

  result.forEach((obj,index ) => {
    if (!obj.file_modified_time) {
      obj.file_modified_time = obj.file_created_time;
    }
    obj.index= index
    if (obj.file_type === "folder") {
       obj.file_name = '📁'+obj.file_name
    }else  {
       obj.file_name ='📄'+ obj.file_name
    }

  });
  return result;
}

export async function executeQueryWithFilter(
  query: string,
  filter: string,
  callback: () => void,
  datasource: Ref<QueryResult[]>,
  loadingState: any,
  resultTotalCount: any,
) {
  const results = await getQueryResult(query);
  switch (filter) {
    case "all": {
      datasource.value = results;
      break;
    }
    case "folder": {
      const result = results.filter((f) => f.file_type === "folder");
      datasource.value = result;
      break;
    }
    case "file": {
      const result = results.filter((f) => f.file_type === "file");
      datasource.value = result;
      break;
    }
    case "image": {
      const result = results.filter((f) => f.file_type === "image");
      datasource.value = result;
      break;
    }
    case "video": {
      const result = results.filter((f) => f.file_type === "video");
      datasource.value = result;
      break;
    }
    case "audio": {
      const result = results.filter((f) => f.file_type === "audio");
      datasource.value = result;
      break;
    }
    case "document": {
      const result = results.filter((f) => f.file_type === "document");
      datasource.value = result;
      break;
    }
    case "archive": {
      const result = results.filter((f) => f.file_type === "archive");
      datasource.value = result;
      break;
    }
    case "exe": {
      const result = results.filter((f) => f.file_type === "exe");
      datasource.value = result;
      break;
    }
    case "font": {
      const result = results.filter((f) => f.file_type === "font");
      datasource.value = result;
      break;
    }
    case "text": {
      const result = results.filter((f) => f.file_type === "text");
      datasource.value = result;
      break;
    }
    case "shortcut": {
      const result = results.filter((f) => f.file_type === "shortcut");
      datasource.value = result;
      break;
    }
    default: {
      datasource.value = results;
      break;
    }
  }
  if (datasource.value.length === 0) {
    resultTotalCount.clearCount();
  }
  console.log("queryText :" + query);
  console.log("filterType :" + filter);
  loadingState.setLoading(true);
  callback();
}
