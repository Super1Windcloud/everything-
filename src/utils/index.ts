export function is_chinese_locale() {
  const locale = navigator.language;
  return locale === "zh-CN";
}

export const filetypes = is_chinese_locale()
  ? [
      { label: "所有类型", value: "all" },
      { label: "文件夹", value: "folder" },
      { label: "文件", value: "file" },
      { label: "图片文件", value: "image" },
      { label: "视频文件", value: "video" },
      { label: "音频文件", value: "audio" },
      { label: "文档文件", value: "document" },
      { label: "压缩文件", value: "archive" },
      { label: "可执行文件", value: "exe" },
      { label: "字体文件", value: "font" },
      { label: "文本文件", value: "text" },
    ]
  : [
      { label: "All Types", value: "all" },
      { label: "Folder", value: "folder" },
      { label: "File", value: "file" },
      { label: "Image", value: "image" },
      { label: "Video", value: "video" },
      { label: "Audio", value: "audio" },
      { label: "Document", value: "document" },
      { label: "Compressed file", value: "archive" },
      { label: "Executable file", value: "exe" },
      { label: "Font file", value: "font" },
      { label: "Text file", value: "text" },
    ];

if (typeof it !== "undefined") {
  it("is_chinese_locale", () => {
    console.log(is_chinese_locale());
  });
}

export const everythingResultInfos = is_chinese_locale()
  ? [
      { label: "名称", field: "file_name" },
      { label: "路径", field: "file_path" },
      { label: "大小", field: "file_size" },
      { label: "扩展名", field: "file_extension" },
      { label: "文件类型", field: "file_type" },
      { label: "修改时间", field: "file_modified_time" },
      { label: "查询结果数量", field: "total_result_count" },
    ]
  : [
      { label: "Name", field: "file_name" },
      { label: "Path", field: "file_path" },
      { label: "Size", field: "file_size" },
      { label: "Extension", field: "file_extension" },
      { label: "FileType", field: "file_type" },
      { label: "Modified Time", field: "file_modified_time" },
      { label: "Result Count", field: "total_result_count" },
    ];

export function zip<T, U>(arr1: T[], arr2: U[]): [T, U][] {
  const length = Math.min(arr1.length, arr2.length);
  const result: [T, U][] = [];
  for (let i = 0; i < length; i++) {
    result.push([arr1[i], arr2[i]]);
  }
  return result;
}
