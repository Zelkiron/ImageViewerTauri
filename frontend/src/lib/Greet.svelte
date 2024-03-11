<script>
    import { invoke } from '@tauri-apps/api/tauri'

    async function open_file() {
      await invoke('open_file')
    }
  </script>
  
  <div>
    <button on:click="{open_file}">Open File</button>
    <h1>No Open File...
  </div>