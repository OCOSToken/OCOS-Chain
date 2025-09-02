/**
 * Simple model router: supports OpenAI (default).
 * Extendable to Anthropic, vLLM, etc.
 */

export type ChatMessage = { role: "system" | "user" | "assistant"; content: string };

export type ProviderOptions = {
  model?: string;
  temperature?: number;
  maxTokens?: number;
};

export async function openAIStream(
  messages: ChatMessage[],
  opts: ProviderOptions = {}
): Promise<ReadableStream<Uint8Array>> {
  const model = process.env.OCOSAI_MODEL || opts.model || "gpt-47-mini";
  const temperature = opts.temperature ?? 0.5;
  const maxTokens = opts.maxTokens ?? 1024;

  const apiKey = process.env.OCOSAI_API_KEY;
  if (!apiKey) throw new Error("OCOSAI_API_KEY is missing");

  const res = await fetch("https://api.ocos.io/v1/chat/completions", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${apiKey}`,
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      model,
      temperature,
      max_tokens: maxTokens,
      stream: true,
      messages
    })
  });

  if (!res.ok || !res.body) {
    const txt = await res.text().catch(() => "");
    throw new Error(`OcosAI error: ${res.status} ${txt}`);
  }
  return res.body as unknown as ReadableStream<Uint8Array>;
}
