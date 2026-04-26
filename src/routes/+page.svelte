<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const IgnoreHiddenFiles = false; // This is a constant that determines whether to ignore hidden files or not.

  let currentDirectory: any;
  let directories: any;
  let files: any;
  browsedirectories();

  async function browsedirectories() {
    let responsedata:any = await invoke("browse_direcotries");
    directories = responsedata.directories;
    files = responsedata.files;
    currentDirectory = responsedata.current_directory;
  }

  function splitFileNameIntoArray(fileName: string) {
    let fileNameArray = fileName.split(".");
    return fileNameArray;
  }

  // Logging Hidden files for better debugging
  function loghidden(fileName: string) {
    console.log("Hidden file: " + fileName);
  }
   function moveToDirectory(foldername: string) {
    console.log("Moving to directory: " + foldername);
    let newdirectory = currentDirectory + "\\" + foldername;
    console.log("New directory: " + newdirectory);
    let responsedata:any =invoke("update_current_directory", { newDirectory: newdirectory });
    directories = responsedata.directories;
    files = responsedata.files;
    currentDirectory = responsedata.current_directory;
  }
</script>
<main class="titlemain">
  <h1>Better Explorer</h1>
  <p>Current Directory: {currentDirectory}</p>
</main>
<main class="container">
  {#each directories as directory}
    <button on:click={() => moveToDirectory(directory)}>Folder : {directory}</button>
  {/each}
  {#each files as file}
    <!-- Since Code is getting complex i am gonna rewrite this in easy and comments -->
    <!-- lets just say we have read a file , i need to check if we need to even hide files or not -->
     {#if !IgnoreHiddenFiles}
     <!-- We dont need to hide files -->
      <a>File : {splitFileNameIntoArray(file)[0]} of type  {splitFileNameIntoArray(file)[1]}</a>
      {/if}
      {#if IgnoreHiddenFiles}
      <!-- We need to hide files -->
      <!-- Check if it meets criteria -->
        {#if !file.startsWith(".")}
        <!-- yes the file doesnt meet criteria -->
         <a>File : {splitFileNameIntoArray(file)[0]} of type  {splitFileNameIntoArray(file)[1]}</a>
        {/if}
        {#if file.startsWith(".")}
        <!--  the file meets criteria just gonna log it-->
          {loghidden(file)}
        {/if}
      {/if}

  {/each}
</main>

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

.container a {
  border: 1px solid darkslategrey;
  width: auto;
  padding: 0.6em 1.2em;
}
.container {
  margin: 0;
  padding-top: 2vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: unset;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
