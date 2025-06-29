<script setup lang="ts">
import {
  everySearchResultCountStore,
  useLoadingStore,
  useQueryTextStore,
} from "@/Store";
import { Ref, ref, watch } from "vue";
import {
  executeQueryWithFilter,
  QueryResult,
} from "@/System/GetQueryResult.ts";

import DataListView from "@/Components/DataListView.vue";
import LoadingToast from "@/Components/LoadingToast.vue";

const showFileInfoList = ref(false);
const query = useQueryTextStore();
const loadingStatus = useLoadingStore();
const resultTotalCount = everySearchResultCountStore();
const showDataListView = ref(false);
const datasource = ref([]) as Ref<QueryResult[]>;

const callback = async (query: string, filter: string) => {
  await executeQueryWithFilter(
    query,
    filter,
    () => {},
    datasource,
    loadingStatus,
    resultTotalCount,
  );
};

watch(
  () => loadingStatus.loadFinish,
  (newValue: boolean) => {
    if (newValue) {
      console.log("current loading status ", newValue);
      showDataListView.value = newValue;
    }
  },
);
watch(datasource, () => {
  if (datasource.value.length > 0) {
    resultTotalCount.setCount(datasource.value.length);
    console.log("resultTotalCount", resultTotalCount.getCount);
  } else {
    resultTotalCount.setCount(0);
    console.log("resultTotalCount :" + resultTotalCount.getCount);
  }
});

watch(
  () => query.queryText,
  (newValue: string) => {
    showFileInfoList.value = newValue ? newValue.trim() !== "" : false;
    if (!newValue || newValue.length === 0) {
      resultTotalCount.clearCount();
      return;
    }
    console.log(newValue + "file info list ");
    loadingStatus.setLoading(false);
  },
);
</script>

<template>
  <div
    class="file-info-list"
    v-if="showFileInfoList"
    :style="{
      margin: '10px',
    }"
  >
    <div v-if="showDataListView">
      <DataListView :datasource="datasource" />
    </div>
    <LoadingToast :callback="callback"></LoadingToast>
  </div>
</template>

<style scoped>
.file-info-list {
  overflow: hidden;
}
</style>
