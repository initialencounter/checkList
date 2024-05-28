<script setup lang="ts">
import { ref } from "vue";
import { register } from '@tauri-apps/api/globalShortcut';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
let target = 0;

type file = {
  name: string,
  state: boolean
}[]

const check_list = ref<file>();
(async () => {
  const check_list_value = (await invoke('get_config')) as string;
  check_list.value = JSON.parse(check_list_value)["check_list"]
})()

function dragenterEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

async function dragoverEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

function dragleaveEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

function dropEvent(event: DragEvent) {
  event.stopPropagation();
  event.preventDefault();
  const files = event.dataTransfer!.files;
  displayChsFile(files);
}

function displayChsFile(files: FileList) {
  for (const file of files) {
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        // @ts-ignore
        check_list.value = JSON.parse(e.target.result);
      } catch (error) {
        alert("Error parsing JSON file.");
      }
    };
    reader.onerror = () => {
      alert("Error reading file.");
    };
    reader.readAsText(file);
  }
}

function switchState(name: string) {
  target = Number(name)
  // @ts-ignore
  check_list.value[name].state = check_list.value[name].state ? false : true
}
async function handleHide(){
  await appWindow.hide()
}

register('CommandOrControl+Shift+C', () => {
  // @ts-ignore
  if (!check_list.value[String(target)]) {
    target = 0
  }
  // @ts-ignore
  check_list.value[String(target)].state = check_list.value[String(target)].state ? false : true
  target += 1
});

// https://blog.csdn.net/weixin_44786530/article/details/129533160

//屏蔽右键菜单
document.oncontextmenu = function (event: any) {
  try {
    var the = event.srcElement
    if (
      !(
        (the.tagName == 'INPUT' && the.type.toLowerCase() == 'text') ||
        the.tagName == 'TEXTAREA'
      )
    ) {
      return false
    }
    return true
  } catch (e) {
    return false
  }
}
function handleClearList() {
  target = 0
  // @ts-ignore
  for (var i of check_list.value) {
    i.state = false
  }
}
</script>

<template>
  <!-- 内容区 -->
  <div class="grid-container" @dragenter="dragenterEvent" @dragover="dragoverEvent" @dragleave="dragleaveEvent"
    @drop="dropEvent">
    <button v-for="(item, index) in check_list" class="grid-item"
      :style="{ background: item.state ? '#3cb44b' : '#4363d8' }" @click="switchState(String(index))">{{ item.name
      }}</button>
    <button class="action-btn" @click="handleClearList">清除列表</button>
    <button class="action-btn-hide" @click="handleHide">隐藏</button>
  </div>
</template>

<style scoped>
@import url('./assets/css/app.css');
</style>
