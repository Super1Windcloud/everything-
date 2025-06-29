<script setup lang="ts">
import { ref, watch } from "vue";
import SearchInput from "vue-search-input";
import "vue-search-input/dist/styles.css";
//@ts-ignore
import VueGlow from "vue-glow";
import Select from "primevue/select";
import { filetypes } from "@/utils";
import { useQueryTextStore, usesFilterType } from "@/Store";

const searchVal = ref<string>('');
const selectFilterType = ref<string>("all");
const queryStore = useQueryTextStore();
const filterTypeStore = usesFilterType();
watch(
  searchVal,
  (newVal) => {
    console.log(newVal);
  },
  {
    immediate: true,
  },
);

const updateSearchVal = (event: Event) => {
  const target = event.target as HTMLInputElement;
  searchVal.value = target.value;
  queryStore.setQueryText(target.value);
};

const updateSelectFilterType = (event: string) => {
  selectFilterType.value = event;
  filterTypeStore.setFilterType(event);
};

watch(
  selectFilterType,
  (newVal) => {
    console.log(newVal);
  },
  {
    immediate: true,
  },
);
</script>

<template>
  <div
    class="search-box"
    :style="{
      display: 'flex',
      alignItems: 'center',
      gap: '1px',
    }"
  >
    <VueGlow color="lightpink" style="flex: 9" :fade="true" elevation="12">
      <SearchInput
        :style="{
          width: '10  0%',
          backgroundColor: 'transparent',
          borderRadius: '10px',
        }"
         v-model="searchVal"
        :hideShortcutIconOnBlur="false"
        :selectOnFocus="true"
        :onchange="updateSearchVal"
        :clearOnEsc=false
        :blurOnEsc=false
      />
    </VueGlow>
    <VueGlow color="lightpink" :fade="true" elevation="12" style="flex: 1">
      <div
        class="filter-bar"
        :style="{
          display: 'flex',
          alignItems: 'center',
          backgroundColor: 'transparent',
          justifyContent: 'center',
          flex: 1, // 与VueGlow形成9:1比例
          height: '38px',
          borderRadius: '7px',
          borderStyle: 'outset',
          borderWidth: '1px',
          borderColor: 'skyblue',
          overflow: 'hidden',

        }"
      >
        <Select
          v-model="selectFilterType"
          @valueChange="updateSelectFilterType"
          :options="filetypes"
          optionLabel="label"
          highlightOnSelect
          checkmark
          optionValue="value"
          variant="outlined"
          :placeholder="filetypes[0].label"
          :labelStyle="{
            color: 'skyblue',
          }"
          :overlayStyle="{
            color: 'lightpink',
            marginTop: '10px',
            backgroundColor: '#4B4b4b',
          }"
        />
      </div>
    </VueGlow>
  </div>
</template>

<style scoped>
.search-box {
  color: #ffffff;
  position: relative;
  display: inline-block;
}

.search-box::after,
.search-box::before {
  content: "";
  position: absolute;
  width: 100%;
  height: 2px;
  background: linear-gradient(to right, #ff0000, #00ffff);
  bottom: -5px;
  left: 0;
  transform: scaleX(0);
  transform-origin: right;
  transition: transform 0.4s ease-out;
}

.search-box::before {
  top: -5px;
  transform-origin: left;
}

.search-box:hover::after,
.search-box:hover::before {
  transform: scaleX(1);
}
</style>
