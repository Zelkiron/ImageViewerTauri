<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let name = ''
    let greetMsg = ''
  
    async function greet() {
      greetMsg = await invoke('greet', { name })
    }

    async function openFile() {
      await invoke('openFile')
    }
  </script>
  
  <div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button on:click="{greet}">Greet</button>
    <button on:click="{openFile}">Open File</button>
    <p>{greetMsg}</p>
  </div>