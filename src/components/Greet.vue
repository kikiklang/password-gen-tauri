<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("generate_password", { length: 16 });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="greet()">Generate</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
