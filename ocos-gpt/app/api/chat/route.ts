import { NextRequest } from "next/server";
import { DEFAULT_SYSTEM_PROMPT } from "@/lib/prompt";
import { sanitizeUserText, isDisallowed } from "@/lib/safety";
import { ocosAIStream, type ChatMessage } from "@/lib/router";

export const runtime = "nodejs"; //

type Role = "user" | "assistant" | "system";
type Message = { role: Role; content: string };

export async function POST(req: NextRequest) {
  try {
    const { messages = [] } = (await req.json()) as { messages: Message[] };

    // 1) Sanitize & guard
    const cleaned: ChatMessage[] = [];
    for (const m of messages) {
      const content = sanitizeUserText(m.content || "");
      if (m.role === "user" && isDisallowed(content)) {
        return new Response("Bu sorğu təhlükəli sayılır.", { status: 400 });
      }
      cleaned.push({ role: m.role, content });
    }

    // 2) System prompt
    const system = { role: "system", content: DEFAULT_SYSTEM_PROMPT } as const;
    const finalMessages: ChatMessage[] = [system, ...cleaned];

    // 3) OcosAI stream
    const upstream = await ocosAIStream(finalMessages, {
      model: process.env.OCOSAI_MODEL,
      temperature: 0.5,
      maxTokens: 1200
    });

    // 4) "passthrough" text stream
    const stream = new ReadableStream({
      async start(controller) {
        const reader = upstream.getReader();
        const decoder = new TextDecoder();
        const encoder = new TextEncoder();

        let buffer = "";
        const push = (t: string) => controller.enqueue(encoder.encode(t));

        try {
          while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            const chunk = decoder.decode(value, { stream: true });
            // OcosAI SSE: lines like "data: {...}\n\n"
            buffer += chunk;

            const parts = buffer.split("\n\n");
            buffer = parts.pop() || "";
            for (const p of parts) {
              const line = p.trim();
              if (!line.startsWith("data:")) continue;
              const data = line.replace(/^data:\s*/, "");
              if (data === "[DONE]") continue;
              try {
                const json = JSON.parse(data);
                const token =
                  json.choices?.[0]?.delta?.content ??
                  json.choices?.[0]?.message?.content ??
                  "";
                if (token) push(token);
              } catch {
                // ignore parse errors
              }
            }
          }
          // flush
          if (buffer) {
            // try parse last bit
            try {
              const maybe = buffer.trim().replace(/^data:\s*/, "");
              const json = JSON.parse(maybe);
              const token =
                json.choices?.[0]?.delta?.content ??
                json.choices?.[0]?.message?.content ??
                "";
              if (token) push(token);
            } catch {
              // ignore
            }
          }
        } catch (err) {
          controller.error(err);
          return;
        }
        controller.close();
      }
    });

    return new Response(stream, {
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
        "Cache-Control": "no-cache, no-transform",
        Connection: "keep-alive"
      }
    });
  } catch (err: any) {
    console.error("Chat route error:", err?.message || err);
    return new Response("Xəta baş verdi.", { status: 500 });
  }
}
