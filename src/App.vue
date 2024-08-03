<script setup lang="ts">
import { ref } from "vue";
import { register, ShortcutEvent } from "@tauri-apps/plugin-global-shortcut";
import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  getCurrentWebviewWindow,
  WebviewWindow,
} from "@tauri-apps/api/webviewWindow";
import { listen, Event } from "@tauri-apps/api/event";
let appWindow: WebviewWindow;

let target = 0;

type file = {
  name: string;
  state: boolean;
}[];

interface Link {
    link: string;
}

const check_list = ref<file>([]);
if (isTauri()) {
  appWindow = getCurrentWebviewWindow();
  (async () => {
    const check_list_value = (await invoke("get_config")) as string;
    check_list.value = JSON.parse(check_list_value)["check_list"];
  })();
  listen('open_link', (data: Event<Link>): void => {
    window.open(data.payload.link)
  })
}

function switchState(name: number) {
  target = Number(name);
  check_list.value[name].state = check_list.value[name].state ? false : true;
}
async function handleHide() {
  await appWindow.hide();
}

if (isTauri()) {
  register("CommandOrControl+Shift+C", (event: ShortcutEvent) => {
    if (event.state === "Pressed") return;
    if (!check_list.value[target]) {
      target = 0;
    }
    check_list.value[target].state = check_list.value[target]
      .state
      ? false
      : true;
    target += 1;
  });
}

document.oncontextmenu = function () {
  return false;
};

function handleClearList() {
  target = 0;
  for (var i of check_list.value) {
    i.state = false;
  }
}
</script>

<template>
  <div class="grid-container" data-tauri-drag-region>
    <button
      v-for="(item, index) in check_list"
      class="grid-item"
      :key="index"
      :style="{ 
        background: item.state ? '#3cb44b' : '#4363d8',
        borderRadius: index == 0? '10px 0 0 10px' : '0'
      }"
      @click="switchState(index)"
    >
      {{ item.name }}
    </button>
    <button class="action-btn" @click="handleClearList">清除列表</button>
    <button class="action-btn-hide" @click="handleHide" :style="{borderRadius:'0 10px 10px 0'}">隐藏</button>
  </div>
</template>

<style scoped>
@import url("./assets/css/app.css");
</style>
