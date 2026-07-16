<script lang="ts">
  import { runFoundationDemo, type FoundationDemoResult } from "./lib/api";

  let caseIdentity = $state("");
  let source = $state("");
  let outputDir = $state("");
  let confirmedSynthetic = $state(false);
  let running = $state(false);
  let progress = $state("");
  let result = $state<FoundationDemoResult | null>(null);
  let errorMessage = $state<string | null>(null);
  let ranAt = $state<string | null>(null);

  const canRun = $derived(
    confirmedSynthetic && source.trim().length > 0 && outputDir.trim().length > 0 && !running,
  );

  // Report status is derived only from what the Rust core actually
  // returned; the UI never invents a completion state of its own.
  const reportStatus = $derived.by(() => {
    if (result && result.status === "verified_complete") return "Verified Complete";
    if (errorMessage) return "Failed";
    return null;
  });

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
      ranAt = new Date().toISOString();
      running = false;
    }
  }
</script>

<main>
  <h1>Trareon Acquire — Foundation Demo</h1>
  <p class="banner" id="lab-use-banner" role="note">
    Engineering Alpha — Lab Use Only. Not validated for production evidence acquisition.
  </p>

  <label for="case-identity">Case identity (operator note only)</label>
  <input
    id="case-identity"
    type="text"
    bind:value={caseIdentity}
    disabled={running}
    placeholder="e.g. TRN-2026-0001"
    aria-describedby="case-identity-help"
  />
  <p id="case-identity-help" class="help">
    Recorded for the operator's own reference only. It is not sent to the core, not part of the
    audit chain, and not covered by verification below.
  </p>

  <label for="source-path">Source path</label>
  <input
    id="source-path"
    type="text"
    bind:value={source}
    disabled={running}
    placeholder="/path/to/synthetic-source.img"
  />

  <label for="output-dir">Output directory</label>
  <input
    id="output-dir"
    type="text"
    bind:value={outputDir}
    disabled={running}
    placeholder="/path/to/output"
  />

  <label class="confirmation" for="confirm-synthetic">
    <input
      id="confirm-synthetic"
      type="checkbox"
      bind:checked={confirmedSynthetic}
      disabled={running}
      aria-describedby="lab-use-banner"
    />
    I confirm the source is a synthetic or training file, not real evidence.
  </label>

  <button onclick={run} disabled={!canRun} aria-busy={running}>
    {running ? "Running…" : "Run"}
  </button>

  <div aria-live="polite">
    {#if progress}
      <p class="progress">{progress}</p>
    {/if}
  </div>

  {#if reportStatus}
    <section
      class="result"
      class:verified={reportStatus === "Verified Complete"}
      class:failed={reportStatus === "Failed"}
      aria-live="polite"
    >
      <h2>Chain of Custody Summary — {reportStatus}</h2>
      {#if caseIdentity}
        <p>Case identity (operator note): {caseIdentity}</p>
      {/if}
      <p>Run recorded at: {ranAt}</p>
      {#if result && reportStatus === "Verified Complete"}
        <p>Package: {result.packagePath}</p>
        <p>Evidence SHA-256: {result.evidenceSha256}</p>
        <p>Evidence size: {result.evidenceSize} bytes</p>
        <p class="verifier-note">
          Verified by the independent core verifier (`traeron_core::verify_fsnap`), not by this
          UI. The UI only displays the core's result.
        </p>
      {:else if errorMessage}
        <p>{errorMessage}</p>
      {/if}
      <p class="not-validated">
        Capability status: <strong>NotValidated</strong> for any real hardware, OS, or production
        evidence claim. This report only covers a synthetic, file-backed fixture acquired on this
        machine.
      </p>
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
    margin-top: 1rem;
    margin-bottom: 0.25rem;
  }

  input[type="text"] {
    display: block;
    width: 100%;
    box-sizing: border-box;
  }

  .help {
    margin-top: 0.25rem;
    font-size: 0.85rem;
    color: #555;
  }

  .confirmation {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  button {
    margin-top: 1rem;
  }

  .result.verified {
    border: 1px solid green;
    padding: 1rem;
    margin-top: 1rem;
  }

  .result.failed {
    border: 1px solid darkred;
    padding: 1rem;
    margin-top: 1rem;
  }

  .verifier-note,
  .not-validated {
    font-size: 0.85rem;
    color: #555;
  }
</style>
