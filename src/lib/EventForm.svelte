<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { event_fields as params } from "../stores/event_fields";




  function create_event2(){
    invoke("create_event", {event: $params});
  }
</script>

<h1>Form</h1>
<div>
  <form class="content">
    <label>Nome</label>
    <input type="text" bind:value={$params.summary} />
    <label>E-mail</label>
    <input type="text" bind:value={$params.description} />
    <label>Telefone</label>
    <input type="text" bind:value={$params.location} />
    <input type="text" bind:value={$params.uid} />
    <input type="text" bind:value={$params.starts} />
    <input type="text" bind:value={$params.ends} />
  </form>
</div>

<button on:click={create_event2}>
  Create Event
</button>


