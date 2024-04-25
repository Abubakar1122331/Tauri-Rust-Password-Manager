<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import User from "./lib/User.svelte";
</script>

<main class="container">
  <h1>Welcome to Tauri Password Manager!</h1>
  <div class="row">
    <User />
  </div>
</main>

<style>
</style>
