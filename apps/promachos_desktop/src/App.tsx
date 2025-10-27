
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [text, setText] = useState("");
  const [messages, setMessages] = useState<{role:"user"|"assistant";content:string}[]>([]);

  async function send() {
    if (!text.trim()) return;
    const user = { role: "user" as const, content: text };
    setMessages((m) => [...m, user]);
    setText("");
    try {
      const reply = await invoke<string>("chat", { text: user.content });
      setMessages((m) => [...m, { role: "assistant", content: reply }]);
    } catch (e: any) {
      setMessages((m) => [...m, { role: "assistant", content: "⚠ " + String(e) }]);
    }
  }

  return (
    <div style={{ display:"grid", gridTemplateRows:"auto 1fr auto", height:"100vh" }}>
      <header style={{ padding:12, borderBottom:"1px solid #222" }}>Promachos</header>
      <main style={{ padding:12, overflow:"auto" }}>
        {messages.map((m,i) => (
          <div key={i} style={{ textAlign: m.role==="user" ? "right":"left", margin:"6px 0" }}>
            <span style={{ background:"#111", padding:"8px 12px", borderRadius:12 }}>{m.content}</span>
          </div>
        ))}
      </main>
      <form onSubmit={(e)=>{e.preventDefault(); send();}} style={{ padding:12, borderTop:"1px solid #222" }}>
        <input value={text} onChange={(e)=>setText(e.target.value)} placeholder="Ask anything…" style={{ width:"100%", padding:10, borderRadius:8 }} />
      </form>
    </div>
  );
}

