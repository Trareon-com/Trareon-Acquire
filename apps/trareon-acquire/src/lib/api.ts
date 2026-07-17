import { invoke } from "@tauri-apps/api/core";

export interface FoundationDemoResult {
  status: string;
  packagePath: string;
  evidenceSha256: string;
  evidenceSize: number;
}

export function runFoundationDemo(
  source: string,
  outputDir: string,
): Promise<FoundationDemoResult> {
  return invoke<FoundationDemoResult>("run_foundation_demo", {
    source,
    outputDir,
  });
}

export function cancelFoundationDemo(): Promise<void> {
  return invoke<void>("cancel_foundation_demo");
}
