<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const password = ref("");

async function generatePassword() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  password.value = await invoke("generate_password", { length: 16 });
}
</script>

<template>
  <div>
    <button type="button" @click="generatePassword()">GO</button>
  </div>

  <p>{{ password }}</p>
</template>
