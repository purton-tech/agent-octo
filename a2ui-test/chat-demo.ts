import { LitElement, css, html } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { provide } from "@lit/context";
import { v0_8 } from "@a2ui/lit";
import type * as Types from "@a2ui/web_core/types/types";
import { structuralStyles } from "@a2ui/web_core/styles/index";
import { defaultTheme } from "./src/a2ui-default-theme";
import { renderMarkdown } from "./vendor/a2ui/renderers/markdown/markdown-it/src/markdown";

const SURFACE_ID = "chat-demo";

type DemoRole = "user" | "assistant";

interface DemoMessage {
  id: string;
  role: DemoRole;
  content: string;
}

const seedMessages: DemoMessage[] = [
  {
    id: crypto.randomUUID(),
    role: "assistant",
    content:
      "Hello from local vendored A2UI.\n\nType a prompt below and this demo will stream a fake assistant response through the message processor.",
  },
];

const cannedResponses = [
  "That makes sense. I would start by modeling the smallest working state transition, then layer richer components on top.",
  "You can drive the whole transcript with `surfaceUpdate` messages.\n\n```ts\nprocessor.processMessages([{ surfaceUpdate: { surfaceId: \"chat-demo\", components } }]);\n```",
  "A2UI is strongest when it owns the rendered content tree. Keep orchestration in the host, and let the surface focus on layout and markdown rendering.",
  "A practical migration path is: move the transcript first, keep the composer native, then decide whether client event plumbing is worth it.",
];

const globalStyleId = "a2ui-structural-styles";
if (!document.getElementById(globalStyleId)) {
  const globalStyle = document.createElement("style");
  globalStyle.id = globalStyleId;
  globalStyle.textContent = structuralStyles;
  document.head.appendChild(globalStyle);
}

function roleLabel(role: DemoRole) {
  return role === "user" ? "User" : "Assistant";
}

function buildComponents(messages: DemoMessage[]) {
  const transcript = messages
    .map((message) => `### ${roleLabel(message.role)}\n\n${message.content || "_..._"}`)
    .join("\n\n---\n\n");

  return [
    {
      id: "root",
      component: {
        Text: {
          usageHint: "body",
          text: {
            literalString: transcript,
          },
        },
      },
    },
  ];
}

@customElement("a2ui-chat-demo")
class A2uiChatDemo extends LitElement {
  @provide({ context: v0_8.UI.Context.theme })
  theme: Types.Theme = defaultTheme;

  @provide({ context: v0_8.UI.Context.markdown })
  markdownRenderer: Types.MarkdownRenderer = renderMarkdown;

  @state()
  accessor draft = "";

  @state()
  accessor messages: DemoMessage[] = seedMessages;

  @state()
  accessor isStreaming = false;

  @state()
  accessor isAtBottom = true;

  @state()
  accessor surfaceState: Types.Surface | null = null;

  @query(".messages")
  accessor messagesEl!: HTMLElement;

  processor = new v0_8.Data.A2uiMessageProcessor();

  connectedCallback(): void {
    super.connectedCallback();
    this.processor.processMessages([
      {
        beginRendering: {
          surfaceId: SURFACE_ID,
          root: "root",
          styles: {
            primaryColor: "#7c93ff",
            font: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif",
          },
        },
      },
    ] as never[]);
    this.syncSurface();
  }

  get surface() {
    return this.surfaceState;
  }

  get debugState() {
    const surface = this.surface;
    return {
      draft: this.draft,
      messageCount: this.messages.length,
      isStreaming: this.isStreaming,
      hasSurface: Boolean(surface),
      rootComponentId: surface?.rootComponentId ?? null,
      hasComponentTree: Boolean(surface?.componentTree),
      componentCount: surface?.components?.size ?? 0,
      styles: surface?.styles ?? null,
      lastMessage: this.messages.at(-1) ?? null,
    };
  }

  handleScroll = () => {
    if (!this.messagesEl) {
      return;
    }

    const distance =
      this.messagesEl.scrollHeight -
      (this.messagesEl.scrollTop + this.messagesEl.clientHeight);
    this.isAtBottom = distance < 48;
  };

  updated(changed: Map<string, unknown>) {
    if (changed.has("messages") && this.isAtBottom && this.messagesEl) {
      this.messagesEl.scrollTop = this.messagesEl.scrollHeight;
    }
  }

  syncSurface() {
    const components = buildComponents(this.messages);
    this.processor.processMessages([
      {
        surfaceUpdate: {
          surfaceId: SURFACE_ID,
          components,
        },
      },
    ] as never[]);

    const currentSurface = this.processor.getSurfaces().get(SURFACE_ID) ?? null;
    this.surfaceState = currentSurface
      ? {
          ...currentSurface,
          styles: { ...(currentSurface.styles ?? {}) },
          componentTree: currentSurface.componentTree
            ? { ...currentSurface.componentTree }
            : null,
          components: new Map(currentSurface.components),
        }
      : null;
    this.requestUpdate();
  }

  setDraft = (event: Event) => {
    const target = event.currentTarget as HTMLTextAreaElement | null;
    this.draft = target?.value ?? "";
  };

  generateResponse(input: string) {
    const sample = cannedResponses[Math.floor(Math.random() * cannedResponses.length)];
    return `${sample}\n\nYou said: \`${input}\`\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit.`; 
  }

  async sendPrompt(promptText?: string) {
    const text = (promptText ?? this.draft).trim();
    if (!text || this.isStreaming) {
      return;
    }

    this.draft = "";
    this.messages = [
      ...this.messages,
      { id: crypto.randomUUID(), role: "user", content: text },
    ];

    const assistantId = crypto.randomUUID();
    this.messages = [
      ...this.messages,
      { id: assistantId, role: "assistant", content: "" },
    ];
    this.isStreaming = true;
    this.syncSurface();

    const response = this.generateResponse(text);
    let acc = "";

    for (const ch of response) {
      acc += ch;
      this.messages = this.messages.map((message) =>
        message.id === assistantId ? { ...message, content: acc } : message,
      );
      this.syncSurface();
      await new Promise((resolve) => setTimeout(resolve, 14 + Math.random() * 24));
    }

    this.isStreaming = false;
  }

  handleSubmit = async (event: Event) => {
    event.preventDefault();
    await this.sendPrompt();
  };

  handleComposerKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      void this.sendPrompt();
    }
  };

  static styles = css`
    :host {
      display: block;
      min-height: 100vh;
      box-sizing: border-box;
      color: white;
      background:
        radial-gradient(circle at top, rgba(124, 147, 255, 0.24), transparent 24rem),
        linear-gradient(180deg, #0f172a 0%, #101827 100%);
      font-family: "IBM Plex Sans", ui-sans-serif, system-ui, sans-serif;
    }

    .shell {
      min-height: 100vh;
      display: grid;
      grid-template-rows: auto 1fr auto;
      max-width: 900px;
      margin: 0 auto;
      padding: 24px;
      gap: 16px;
      box-sizing: border-box;
    }

    .header {
      display: grid;
      gap: 6px;
    }

    .eyebrow {
      font-size: 0.75rem;
      text-transform: uppercase;
      letter-spacing: 0.18em;
      color: rgba(196, 208, 255, 0.72);
    }

    h1 {
      margin: 0;
      font-size: clamp(1.8rem, 4vw, 2.6rem);
      line-height: 1;
      font-family: "IBM Plex Mono", ui-monospace, monospace;
      font-weight: 600;
      color: #e5ebff;
    }

    .subhead {
      margin: 0;
      color: rgba(216, 223, 252, 0.84);
      max-width: 44rem;
    }

    .messages {
      min-height: 0;
      overflow-y: auto;
      padding: 20px;
      border: 1px solid rgba(124, 147, 255, 0.24);
      border-radius: 24px;
      background: rgba(15, 23, 42, 0.78);
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.04);
      backdrop-filter: blur(18px);
    }

    a2ui-surface {
      display: block;
    }

    form {
      display: grid;
      gap: 10px;
      padding: 18px;
      border: 1px solid rgba(124, 147, 255, 0.24);
      border-radius: 24px;
      background: rgba(15, 23, 42, 0.78);
    }

    textarea {
      min-height: 5.5rem;
      resize: vertical;
      border: 0;
      border-radius: 16px;
      padding: 14px 16px;
      background: rgba(30, 41, 59, 0.92);
      color: #f8fafc;
      font: inherit;
      outline: none;
      box-sizing: border-box;
    }

    textarea::placeholder {
      color: rgba(203, 213, 225, 0.62);
    }

    .controls {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 12px;
    }

    .hint {
      font-size: 0.875rem;
      color: rgba(196, 208, 255, 0.72);
    }

    button {
      border: 0;
      border-radius: 999px;
      padding: 12px 18px;
      background: linear-gradient(135deg, #7c93ff 0%, #94a3ff 100%);
      color: #0b1120;
      font: inherit;
      font-weight: 600;
      cursor: pointer;
    }

    button:disabled {
      cursor: default;
      opacity: 0.55;
    }

  `;

  render() {
    return html`
      <div class="shell">
        <header class="header">
          <div class="eyebrow">Local Demo</div>
          <h1>A2UI Transcript Chat</h1>
          <p class="subhead">
            The transcript is rendered by the vendored A2UI Lit renderer. The
            composer and fake response loop stay local in the host.
          </p>
        </header>

        <section class="messages" @scroll=${this.handleScroll}>
          <a2ui-surface
            .surfaceId=${SURFACE_ID}
            .surface=${this.surface}
            .processor=${this.processor}
          ></a2ui-surface>
        </section>

        <form @submit=${this.handleSubmit}>
          <textarea
            .value=${this.draft}
            ?disabled=${this.isStreaming}
            placeholder="Type a prompt..."
            @input=${this.setDraft}
            @keydown=${this.handleComposerKeydown}
          ></textarea>
          <div class="controls">
            <div class="hint">
              Enter to send. Shift+Enter for a new line.
            </div>
            <button type="submit" ?disabled=${this.isStreaming || !this.draft.trim()}>
              ${this.isStreaming ? "Streaming..." : "Send"}
            </button>
          </div>
        </form>
      </div>
    `;
  }
}

const mount = document.querySelector("#app");

if (!(mount instanceof HTMLElement)) {
  throw new Error("Missing #app element");
}

mount.replaceChildren(document.createElement("a2ui-chat-demo"));

Object.assign(window, {
  runDemoPrompt: (prompt: string) => {
    const app = document.querySelector("a2ui-chat-demo") as A2uiChatDemo | null;
    if (!app) {
      throw new Error("Missing a2ui-chat-demo");
    }

    return app.sendPrompt(prompt);
  },
  feedA2uiMessage: (message: unknown) => {
    const app = document.querySelector("a2ui-chat-demo") as A2uiChatDemo | null;
    if (!app) {
      throw new Error("Missing a2ui-chat-demo");
    }

    app.processor.processMessages([message as never]);
    app.requestUpdate();
  },
});
