<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SearchBox from "@/Components/SearchBox.vue";
import FileInfoList from "@/Components/FileInfoList.vue";
import StatusBar from "@/Components/StatusBar.vue";

onMounted(() => {
  invoke("show_window");
});

function closeMenu(event: MouseEvent) {
  event.preventDefault();
}
</script>

<template>
  <SearchBox />
  <FileInfoList @contextmenu="closeMenu" />
  <StatusBar />
</template>

<style scoped>
html,
body {
  background-color: transparent;
  color: white;
  font-family: inherit;
  font-size: inherit;
  font-style: inherit;
  font-weight: inherit;
  overflow: hidden;
}
</style>
