<script setup lang="ts">
import { everythingResultInfos, is_chinese_locale } from "@/utils";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import { QueryResult } from "@/System/GetQueryResult.ts";
import { onMounted, PropType, ref, watch } from "vue";
import { usesFilterType } from "@/Store";
import ContextMenu from "@imengyu/vue3-context-menu";

const filterType = usesFilterType();
const props = defineProps({
  datasource: {
    type: Array as PropType<QueryResult[]>, // 属性传递ref会被自动解包
    default: [],
  },
  callback: {
    type: Function as PropType<() => Promise<void>>,
    default: () => {},
  },
});

const dataSource = ref<QueryResult[]>([]);
const contextMenuRef = ref<QueryResult | null>(null);
const hoveredRow = ref<QueryResult | null>(null);

onMounted(() => {
  dataSource.value = props.datasource;
});

watch(
  () => filterType.filterType,
  () => {
    console.log(filterType.filterType);
    const filter = filterType.filterType;
    switch (filter) {
      case "all": {
        const results = props.datasource;
        dataSource.value = results;
        break;
      }
      case "folder": {
        const results = props.datasource.filter(
          (f) => f.file_type === "folder",
        );
        dataSource.value = results;

        break;
      }
      case "file": {
        const results = props.datasource.filter(
          (f) => f.file_type !== "folder",
        );
        dataSource.value = results;
        break;
      }
      case "image": {
        const results = props.datasource.filter((f) => f.file_type === "image");
        dataSource.value = results;
        break;
      }
      case "video": {
        const results = props.datasource.filter((f) => f.file_type === "video");
        dataSource.value = results;
        break;
      }
      case "audio": {
        const results = props.datasource.filter((f) => f.file_type === "audio");
        dataSource.value = results;
        break;
      }
      case "document": {
        const results = props.datasource.filter(
          (f) => f.file_type === "document",
        );
        dataSource.value = results;
        break;
      }
      case "archive": {
        const results = props.datasource.filter(
          (f) => f.file_type === "archive",
        );
        dataSource.value = results;
        break;
      }
      case "exe": {
        const results = props.datasource.filter((f) => f.file_type === "exe");
        dataSource.value = results;
        break;
      }
      case "font": {
        const results = props.datasource.filter((f) => f.file_type === "font");
        dataSource.value = results;
        break;
      }
      case "text": {
        const results = props.datasource.filter((f) => f.file_type === "text");
        dataSource.value = results;
        break;
      }
      case "shortcut": {
        const results = props.datasource.filter(
          (f) => f.file_type === "shortcut",
        );
        dataSource.value = results;
        break;
      }
      default: {
        const results = props.datasource;
        dataSource.value = results;
        break;
      }
    }
  },
);

watch(
  () => props.datasource,
  (newVal) => {
    dataSource.value = newVal;
  },
  { immediate: true, deep: true },
);

import {
  copyFile,
  copyPathToClipboard,
  deleteFile,
  openPathWithExplorer,
} from "@/System/ContextMenu.ts";
import { Toaster } from "@steveyuowo/vue-hot-toast";
import {
  copyFileToClipboardToast,
  copyPathToClipboardToast,
  deleteFileToast,
  openPathFinishedToast,
} from "@/utils/FileToast.ts";

function onContextMenu(e: MouseEvent) {
  e.preventDefault();
  const target = e.target as HTMLElement;
  const tr = target.closest("tr");
  if (!tr) return;

  const classList = Array.from(tr.classList);
  const rowClass = classList.find((cls) => cls.startsWith("context-row-"));
  if (!rowClass) return;

  const indexStr = rowClass.replace("context-row-", "");
  const index = Number(indexStr);

  const matched = dataSource.value.find((item) => item.index === index);
  if (!matched) return;

  contextMenuRef.value = matched;

  ContextMenu.showContextMenu({
    theme: "mac dark",
    x: e.x,
    y: e.y,
    items: [
      {
        label: is_chinese_locale() ? "打开路径" : "Open Path",
        onClick: () => {
          openPathWithExplorer(matched);
          openPathFinishedToast();
        },
        shortcut: "Ctrl + O",
      },
      {
        label: is_chinese_locale() ? "复制路径" : "Copy Path",
        onClick: () => {
          copyPathToClipboard(matched);
          copyPathToClipboardToast();
        },
        shortcut: "Ctrl + Shift + C",
      },
      {
        label: is_chinese_locale() ? "复制文件" : "Copy File",
        onClick: () => {
          copyFile(matched);
          copyFileToClipboardToast();
        },
        shortcut: "Ctrl + C",
      },

      {
        label: is_chinese_locale() ? "删除文件" : "Delete File",
        onClick: () => {
          deleteFile(matched);
          deleteFileToast();
        },
        shortcut: "Delete",
      },
    ],
  });
}

function onMouseLeave() {
  hoveredRow.value = null;
}

function onMouseMove(e: MouseEvent) {
  const tr = (e.target as HTMLElement).closest("tr");
  if (!tr) {
    hoveredRow.value = null;
    return;
  }
  const rowClass = Array.from(tr.classList).find((cls) =>
    cls.startsWith("context-row-"),
  );
  if (!rowClass) {
    hoveredRow.value = null;
    return;
  }
  const index = Number(rowClass.replace("context-row-", ""));
  const matched = dataSource.value.find((item) => item.index === index);
  if (matched && hoveredRow.value?.index !== matched.index) {
    hoveredRow.value = matched;
  }
}

function onShortcutOperation(e: KeyboardEvent) {
  if (!hoveredRow.value) return;

  if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "c") {
    e.preventDefault();
    copyPathToClipboard(hoveredRow.value);
    copyPathToClipboardToast();
  } else if (e.ctrlKey && e.key.toLowerCase() === "c") {
    e.preventDefault();
    copyFile(hoveredRow.value);
    copyFileToClipboardToast();
  } else if (e.ctrlKey && e.key.toLowerCase() === "o") {
    e.preventDefault();
    openPathWithExplorer(hoveredRow.value);
    openPathFinishedToast();
  } else if (e.key === "Delete") {
    e.preventDefault();
    deleteFile(hoveredRow.value);
    deleteFileToast();
  }
}

function getRowClass(data: QueryResult) {
  return `context-row-${data.index}`;
}
</script>

<template>
  <div
    @contextmenu="onContextMenu"
    @mousemove="onMouseMove"
    @mouseleave="onMouseLeave"
    @keydown="onShortcutOperation"
    tabindex="0"
    ref="tableWrapper"
  >
    <DataTable
      class="datatable"
      :value="dataSource"
      :row-class="getRowClass"
      rowHover
      :column-resize-mode="'fit'"
      scrollable
      scrollDirection="both"
      size="small"
      :scroll-height="'flex'"
      :highlightOnSelect="true"
      :style="{
        color: 'lightgray',
        overflow: 'hidden',
        whiteSpace: 'nowrap',
        cursor: 'pointer',
      }"
    >
      <Column
        class="name"
        :field="everythingResultInfos[0].field"
        :header="everythingResultInfos[0].label"
        :row-editor="true"
        columnKey="index"
        :sortable="true"
        :style="{
          width: 'auto',
          height: '40px',
          maxHeight: '40px',
        }"
        :bodyStyle="{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }"
        :headerStyle="{
          color: 'cyan',
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }"
      >
      </Column>
      <Column
        class="size"
        :sortable="true"
        :field="everythingResultInfos[2].field"
        :header="everythingResultInfos[2].label"
        :style="{
          width: 'auto',
          minWidth: '100px',
          height: '40px',
          maxHeight: '40px',
        }"
        :bodyStyle="{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          padding: '0 20px',
        }"
        :headerStyle="{
          color: 'cyan',
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }"
      ></Column>
      <Column
        class="extension"
        :sortable="true"
        :headerStyle="{
          color: 'cyan',
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }"
        :field="everythingResultInfos[3].field"
        :header="everythingResultInfos[3].label"
        :style="{
          width: 'auto',
          minWidth: '100px',
          height: '40px',
          maxHeight: '40px',
        }"
        :bodyStyle="{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }"
      ></Column>
      <Column
        class="filetype"
        :sortable="true"
        :headerStyle="{
          color: 'cyan',
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }"
        :field="everythingResultInfos[4].field"
        :header="everythingResultInfos[4].label"
        :style="{
          width: 'auto',
          minWidth: '100px',
          height: '40px',
          maxHeight: '40px',
        }"
        :bodyStyle="{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          padding: '0 20px',
        }"
      >
      </Column>
      <Column
        class="modifiedTime"
        :sortable="true"
        :headerStyle="{
          color: 'cyan',
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          padding: '0 20px',
        }"
        :field="everythingResultInfos[5].field"
        :header="everythingResultInfos[5].label"
        :style="{
          width: 'auto',
          minWidth: '100px',
          height: '40px',
          maxHeight: '40px',
        }"
        :bodyStyle="{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          padding: '0 20px',
        }"
      ></Column>
      <Column
        class="path"
        :sortable="true"
        :headerStyle="{
          color: 'cyan',
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          padding: '0 10px',
        }"
        :field="everythingResultInfos[1].field"
        :header="everythingResultInfos[1].label"
        :style="{ width: 'auto', height: '40px', maxHeight: '40px' }"
        :bodyStyle="{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          padding: '0 10px',
        }"
      >
      </Column>
    </DataTable>
  </div>
  <Toaster />
</template>

<style scoped>
:deep(.p-datatable-tbody > tr:hover) {
  background-color: #4b4b4b !important;
  cursor: pointer;
}

.datatable {
  :hover {
    background-color: #4b4b4b;
  }
}

.modifiedTime,
.name,
.size,
.extension,
.filetype,
.path {
  :hover {
    background-color: #4b4b4b;
  }
}
</style>
