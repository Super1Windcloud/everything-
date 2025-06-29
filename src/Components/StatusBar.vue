<script setup lang="ts">
import { everySearchResultCountStore, useQueryTextStore } from "@/Store";
import { onMounted, ref, watch } from "vue";
import { is_chinese_locale } from "@/utils";

const query = useQueryTextStore();
const showStatusBar = ref<boolean>(false);
const totalResultCount = everySearchResultCountStore();
const totalCount = ref(0);

onMounted(() => {
  totalCount.value = totalResultCount.count;
});
watch(
  () => totalResultCount.count,
  (newValue) => {
    totalCount.value = newValue;
  },
);
watch(
  () => query.queryText,
  (newValue) => {
    showStatusBar.value = newValue ? newValue.trim() !== "" : false;
  },
  { immediate: true },
);
</script>

<template>
  <div v-if="showStatusBar" class="status-bar">
    <div class="global-footer">
      {{
        is_chinese_locale()
          ? `共找到 ${totalCount} 个结果`
          : `Total : ${totalCount}`
      }}
    </div>
  </div>
</template>

<style scoped>
.global-footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  color:     lightseagreen;
  background:  #2E303D;
  text-align: left;
  z-index: 99;
  border-top: 1px solid #ccc;
}

.status-bar {
  overflow:hidden;
  position: fixed;

}
</style>
