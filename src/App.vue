<script setup lang="ts">
import { ref } from "vue";
import { register, ShortcutEvent } from "@tauri-apps/plugin-global-shortcut";
import { invoke } from "@tauri-apps/api/core";
import {
  getCurrentWebviewWindow,
  WebviewWindow,
} from "@tauri-apps/api/webviewWindow";
let appWindow: WebviewWindow = getCurrentWebviewWindow();
let target = 0;

type file = {
  name: string;
  state: boolean;
}[];
const info = async (info: string) => {
  invoke("log_info", { info: info });
};
const check_list = ref<file>([]);
(async () => {
  const check_list_value = (await invoke("get_config")) as string;
  check_list.value = JSON.parse(check_list_value)["check_list"];
})();
function switchState(name: number) {
  info(String(name))
  target = name;
  check_list.value[name].state = check_list.value[name].state ? false : true;
}
async function handleHide() {
  await appWindow.hide();
}

register("CommandOrControl+Shift+V", (event: ShortcutEvent) => {
  if (event.state === "Pressed") return;
  if (!check_list.value[target]) {
    target = 0;
  }
  check_list.value[target].state = !check_list.value[target].state;
  let item = document.getElementById("item" + target);
  if (item) {
    item.scrollIntoView();
  }
  target += 1;
});

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
  <!-- 内容区 -->
  <div class="grid-container" data-tauri-drag-region>
    <button :style="{
        background: check_list[target-1].state ? '#3cb44b' : '#4363d8',
        border: '2px solid #ff0000',
      }"
      @click="switchState(target-1)"
      >
      {{ check_list[target-1].name }}
    </button>
    <button :style="{
        background: check_list[target].state ? '#3cb44b' : '#4363d8',
        border: '2px solid #ff0000',
      }"
      @click="switchState(target)"
      >
      {{ check_list[target].name }}
    </button>
    <button :style="{
        background: check_list[target+1].state ? '#3cb44b' : '#4363d8',
        border: '2px solid #ff0000',
      }"
      @click="switchState(target+1)"
      >
      {{ check_list[target+1].name }}
    </button>

    <button class="action-btn-hide" @click="handleHide">隐藏</button>
  </div>
  <button class="action-btn" @click="handleClearList">清除列表</button>
</template>

<style scoped>
@import url("./assets/css/app.css");
</style>
