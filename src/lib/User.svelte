<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import SQLite from "tauri-plugin-sqlite-api";
  import App from "../App.svelte";

  let username = "";
  let password = "";
  let website = "";

  export async function InitializeDatabase() {
    try {
      let database_path = "Data.db";
      // Create Database if not exists
      const db = await SQLite.open("Data.db");

      // Create Table if not exists
      await db.execute(`
    CREATE TABLE users (username TEXT, website TEXT, password TEXT);
    INSERT INTO users VALUES ('Ali', 'Googel', "SoemPasswtds");
`);
      await db.execute(
        "INSERT INTO users (username, password, website) VALUES ('man', 'AnotherMan', 'YetAnotherMAN')"
      );

      const isClosed = await db.close();
    } catch (error) {
      console.error("Error Initializing Database:", error);
    }
  }

  onMount(InitializeDatabase);
  onMount;
</script>

<div class="input-box">
  <div class="user-inputs">
    <input placeholder="Enter your Username..." bind:value={username} />
    <input placeholder="Enter your Website..." bind:value={website} />
    <input placeholder="Enter your Password..." bind:value={password} />
  </div>
  <button>Enter</button>
</div>

<style>
  .user-inputs {
    display: flex;
    gap: 20px;
  }
  .input-box {
  }
</style>
