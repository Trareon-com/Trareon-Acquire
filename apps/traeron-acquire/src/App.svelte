<script lang="ts">
  import { runFoundationDemo, type FoundationDemoResult } from "./lib/api";

  let source = $state("");
  let outputDir = $state("");
  let confirmedSynthetic = $state(false);
  let running = $state(false);
  let progress = $state("");
  let result = $state<FoundationDemoResult | null>(null);
  let errorMessage = $state<string | null>(null);

  const canRun = $derived(
    confirmedSynthetic && source.trim().length > 0 && outputDir.trim().length > 0 && !running,
  );

  async function run() {
    if (!canRun) {
      return;
    }
    running = true;
    progress = "Running foundation demo…";
    result = null;
    errorMessage = null;

    try {
      const demoResult = await runFoundationDemo(source, outputDir);
      result = demoResult;
      progress = "Done.";
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
      result = null;
      progress = "Failed.";
    } finally {
      running = false;
    }
  }
</script>

<main>
  <h1>Trareon Acquire — Foundation Demo</h1>
  <p class="banner">Engineering Alpha — Lab Use Only. Not validated for production evidence acquisition.</p>

  <label>
    Source path
    <input type="text" bind:value={source} disabled={running} placeholder="/path/to/synthetic-source.img" />
  </label>

  <label>
    Output directory
    <input type="text" bind:value={outputDir} disabled={running} placeholder="/path/to/output" />
  </label>

  <label class="confirmation">
    <input type="checkbox" bind:checked={confirmedSynthetic} disabled={running} />
    I confirm the source is a synthetic or training file, not real evidence.
  </label>

  <button onclick={run} disabled={!canRun}>Run</button>

  {#if progress}
    <p class="progress">{progress}</p>
  {/if}

  {#if result && result.status === "verified_complete"}
    <section class="result verified">
      <h2>Verified Complete</h2>
      <p>Package: {result.packagePath}</p>
      <p>SHA-256: {result.evidenceSha256}</p>
      <p>Size: {result.evidenceSize} bytes</p>
    </section>
  {:else if errorMessage}
    <section class="result failed">
      <h2>Failed</h2>
      <p>{errorMessage}</p>
    </section>
  {/if}
</main>

<style>
  main {
    max-width: 640px;
    margin: 0 auto;
    padding: 1.5rem;
    font-family: sans-serif;
  }

  .banner {
    font-weight: bold;
    color: #7a5b00;
  }

  label {
    display: block;
    margin-bottom: 1rem;
  }

  input[type="text"] {
    display: block;
    width: 100%;
    box-sizing: border-box;
  }

  .confirmation {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .result.verified {
    border: 1px solid green;
    padding: 1rem;
  }

  .result.failed {
    border: 1px solid darkred;
    padding: 1rem;
  }
</style>
