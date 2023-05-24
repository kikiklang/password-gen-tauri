<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Generator from "./components/Generator.vue";
</script>

<template>
  <div class="container">
    <img class="logo" src="./assets/app-icon.png" alt="" srcset="">
    <h1>PassGen</h1>

    <p>Click on the button to generate a password.</p>

    <Generator />
  </div>
</template>
