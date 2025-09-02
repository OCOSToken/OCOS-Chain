type Role = "user" | "assistant" | "system";
export type Message = { role: Role; content: string };

export function MessageBubbles({ items }: { items: Message[] }) {
  return (
    <div className="space-y-3">
      {items.map((m, i) => (
        <div
          key={i}
          className={
            "whitespace-pre-wrap leading-relaxed rounded-2xl px-4 py-3 ring-1 " +
            (m.role === "user"
              ? "bg-sky-950/30 ring-sky-300/10 text-sky-100"
              : "bg-[var(--card)]/70 ring-white/5 text-slate-100")
          }
        >
          <div className="text-[10px] uppercase tracking-widest opacity-60 mb-1">
            {m.role === "user" ? "Sən" : m.role === "system" ? "Sistem" : "OCOS GPT"}
          </div>
          {m.content}
        </div>
      ))}
    </div>
  );
}
