<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import SQLite from "tauri-plugin-sqlite-api";
  import App from "../App.svelte";

  let username = "";
  let password = "";
  let website = "";

  async function InitializeDatabase() {
    try {
      let database_path = "Data.db";

      // Create Database if not exists
      const db = await SQLite.open("Data.db");

      // Create Table if not exists
      await db.execute(
        `CREATE TABLE users (username TEXT, website TEXT, password TEXT);`
      );

      const isClosed = await db.close();
    } catch (error) {
      console.error("Error Initializing Database:", error);
    }
  }

  let rows: any = [];

  async function EnterData() {
    console.log("Data entered");
    let db = await SQLite.open("Data.db");
    await db.execute("INSERT INTO users VALUES (?1, ?2, ?3)", [
      [username, password, website],
    ]);

    let retrieved_data = await db.select<
      Array<{ username: string; website: string; password: string }>
    >("SELECT username, website, password FROM users");

    rows = retrieved_data.map((row) => ({
      username: row.username,
      password: row.password,
      website: row.website,
    }));

    console.log("", rows);
  }

  EnterData();
  onMount(InitializeDatabase);
</script>

<div class="bg">
  <div class="input-box">
    <div class="user-inputs">
      <input placeholder="Enter your Username..." bind:value={username} />
      <input placeholder="Enter your Website..." bind:value={website} />
      <input placeholder="Enter your Password..." bind:value={password} />
      <button on:click={EnterData}>Enter</button>
    </div>
  </div>
  <div class="users">
    <div class="top">
      <p>Passwords</p>
    </div>
    {#each rows as row}
      <div class="user">
        <p>{row.username}</p>
        <p>{row.website}</p>
        <p>{row.password}</p>
        <!-- <button>{row.website}</button> -->
      </div>
    {/each}
  </div>
</div>

<style>
  .user-inputs {
    display: flex;
    gap: 20px;
  }
  .users {
    margin-top: 20px;
  }
  .top {
    font-size: 2em;
    font-weight: 600;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 40px;
    color: lightgray;
    background-color: rgb(76, 93, 51);
    border-radius: 10px 10px 0 0;
  }
  .user {
    display: flex;
    align-items: center;
    justify-content: space-around;
    height: 40px;
    border-radius: 0px;
    border-bottom: 2px solid black;
    background-color: gray;
    color: black;
    font-weight: 700;
  }
</style>
