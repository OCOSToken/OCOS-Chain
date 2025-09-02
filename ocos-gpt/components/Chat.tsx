"use client";

import { useEffect, useRef, useState } from "react";
import { Message, MessageBubbles } from "./MessageBubbles";

export default function Chat() {
  const [messages, setMessages] = useState<Message[]>([
    {
      role: "assistant",
      content:
        "Salam! Mən **OCOS GPT**-yəm. Sualını yaz — DAO, tokenomics, on-chain, təhlükəsizlik və ekosistem sualları üçün buradayam."
    }
  ]);
  const [input, setInput] = useState("");
  const [streaming, setStreaming] = useState(false);
  const controllerRef = useRef<AbortController | null>(null);

  async function send() {
    const text = input.trim();
    if (!text || streaming) return;

    const next = [...messages, { role: "user" as const, content: text }];
    setMessages(next);
    setInput("");
    setStreaming(true);

    controllerRef.current = new AbortController();
    try {
      const res = await fetch("/api/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ messages: next }),
        signal: controllerRef.current.signal
      });

      if (!res.ok || !res.body) {
        setMessages([
          ...next,
          { role: "assistant", content: "Xəta baş verdi. Bir daha yoxlayın." }
        ]);
        setStreaming(false);
        return;
      }

      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      let assistant = { role: "assistant" as const, content: "" };
      setMessages([...next, assistant]);

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        const chunk = decoder.decode(value, { stream: true });
        assistant = { role: "assistant", content: assistant.content + chunk };
        setMessages([...next, assistant]);
      }
    } catch (e) {
      setMessages([
        ...messages,
        { role: "assistant", content: "Şəbəkə xətası və ya abort edildi." }
      ]);
    } finally {
      setStreaming(false);
      controllerRef.current = null;
    }
  }

  function onKeyDown(e: React.KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      send();
    }
  }

  useEffect(() => {
    // Auto-scroll behavior can be added if container is scrollable.
  }, [messages]);

  return (
    <div className="mx-auto max-w-3xl">
      <header className="mb-6">
        <div className="flex items-center gap-3">
          <img src="/logo.svg" alt="OCOS" className="h-8 w-8" />
          <h1 className="text-2xl font-semibold tracking-tight">OCOS GPT</h1>
        </div>
        <p className="text-sm text-slate-300/80">
          Web3 + AI köməkçisi • təhlükəsiz • sürətli • genişlənə bilən
        </p>
      </header>

      <div className="rounded-2xl bg-[var(--card)]/60 ring-1 ring-white/5 p-4">
        <MessageBubbles items={messages} />
      </div>

      <div className="mt-4 rounded-2xl ring-1 ring-white/5 bg-[var(--card)]/60 p-3">
        <textarea
          className="w-full bg-transparent outline-none resize-none h-28 p-3 rounded-xl ring-1 ring-white/10 focus:ring-sky-400/40"
          placeholder="Mesajını yaz… (Göndər: ⌘/Ctrl+Enter)"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={onKeyDown}
          disabled={streaming}
        />
        <div className="flex justify-end">
          <button
            onClick={send}
            disabled={streaming}
            className="mt-2 inline-flex items-center rounded-xl px-4 py-2 ring-1 ring-white/10 hover:ring-white/20 disabled:opacity-50"
          >
            {streaming ? "Göndərilir…" : "Göndər"}
          </button>
        </div>
      </div>
    </div>
  );
}
