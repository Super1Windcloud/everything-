<script setup lang="ts">
import { ref, watch } from "vue";
import { Toaster, toast } from "@steveyuowo/vue-hot-toast";
import "@steveyuowo/vue-hot-toast/vue-hot-toast.css";
import { useQueryTextStore, usesFilterType } from "@/Store";

const props = defineProps<{
  callback: (query: string, filter: string) => Promise<void>;
}>();
const queryText = useQueryTextStore();
const startSearch = ref(false);
const filterType = usesFilterType();

const changeFinishStatus = () => {
  startSearch.value = true;
};

const finishSearchToast = async () => {
  await toast.promise(
    props.callback(queryText.queryText, filterType.filterType),
    {
      success: "Search Complete!",
      error: "Error!",
      loading: "Loading!",
      position: "bottom-right",
    },
  );
};

watch(
  () => queryText.queryText,
  () => {
    if (queryText.queryText) {
      console.log("start search query");
      changeFinishStatus();
    }
  },
  {
    immediate: true, // 刚挂载组件
  },
);

watch(
  startSearch,
  async () => {
    if (startSearch.value) {
      await finishSearchToast();
      startSearch.value = false;
      console.log("startSearch :" + startSearch.value);
    }
  },
  {
    immediate: true,
  },
);
</script>

<template>
  <Toaster />
</template>

<style scoped></style>
