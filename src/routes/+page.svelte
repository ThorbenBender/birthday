<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let vec: number[][] = []

  onMount(async () => {
    vec = await invoke('init_sodoku')
  })
</script>

<div class="container">
  {#each vec as section}
    <div class="section">
      {#each section as block}
        <input class="section-input" value={block} type="number" />
      {/each}
    </div>
  {/each}
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    display: grid;
    grid-template-rows: 1fr 1fr 1fr;
    grid-template-columns: 1fr 1fr 1fr;
    width: 270px;
    height: 270px;
    box-sizing: border-box;
    border: 2px solid black;
  }

  .section {
    margin: 0;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr;
    border: 1px solid black;
  }

  .section-input {
    color: white;
    border: none;
    box-sizing: border-box;
    display: flex;
    justify-content: center;
    align-items: center;
    background: lightgrey;
  }
  .section-input::-webkit-inner-spin-button {
    display: none;
  }
</style>
