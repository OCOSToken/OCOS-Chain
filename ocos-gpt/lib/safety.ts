export function sanitizeUserText(input: string): string {
  // Minimal guard: trim and collapse bizarre control chars
  return input.replace(/[\u0000-\u001F\u007F]/g, " ").trim();
}

// Optional basic policy checks
export function isDisallowed(input: string): boolean {
  const low = input.toLowerCase();
  // Add your own filters here (PII, seeds, keys, etc.)
  if (low.includes("seed phrase") || low.includes("private key")) return true;
  return false;
}
