<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const password = ref("");
const sliderValue = ref(8);

async function generatePassword() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  password.value = await invoke("generate_password", {
    length: 16,
    numbers: true,
    lowercaseLetters: true,
    uppercaseLetters: true,
    symbols: true,
    spaces: true,
    excludeSimilarCharacters: true,
    strict: true,
  });
}
</script>

<template>
  <div class="generator-container">
    <div class="slider">
      <p>longueur</p>
      <input
        v-model="sliderValue"
        type="range"
        min="4"
        max="48"
        class="slider"
      />
      <p>{{ sliderValue }}</p>
    </div>
    <button
      type="button"
      @click="generatePassword()"
    >GO</button>
  </div>

  <p>{{ password }}</p>
</template>

<style scoped>
.generator-container {
  margin: 0 auto;
  width: 400px;
}
.slider {
  display: flex;
  justify-content: space-around;
}
</style>