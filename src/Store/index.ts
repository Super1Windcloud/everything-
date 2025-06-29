import { defineStore } from "pinia";

export const useQueryTextStore = defineStore("queryText", {
  state: () => {
    return { queryText: "" };
  },
  getters: {
    getQueryText(state) {
      return state.queryText;
    },
  },
  actions: {
    setQueryText(text: string) {
      this.$state.queryText = text;
    },
  },
});

export const usesFilterType = defineStore("filterType", {
  state: () => {
    return { filterType: "" };
  },
  getters: {
    getFilterType(state) {
      return state.filterType;
    },
  },
  actions: {
    setFilterType(filter: string) {
      this.$state.filterType = filter;
    },
  },
});

export const useLoadingStore = defineStore("loadingStore", {
  state: () => {
    return { loadFinish: false };
  },
  getters: {
    getLoading(state) {
      return state.loadFinish;
    },
  },
  actions: {
    setLoading(loading: boolean) {
      this.$state.loadFinish = loading;
    },
  },
});

export const everySearchResultCountStore = defineStore(
  "everySearchResultCount",
  {
    state: () => {
      return { count: 0 };
    },
    getters: {
      getCount(state) {
        return state.count;
      },
    },
    actions: {
      setCount(count: number) {
        this.$state.count = count;
      },
      clearCount() {
        this.$state.count = 0;
      },
    },
  },
);
