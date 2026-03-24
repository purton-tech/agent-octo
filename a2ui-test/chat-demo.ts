import { LitElement, css, html } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { provide } from "@lit/context";
import { v0_8 } from "@a2ui/lit";
import type * as Types from "@a2ui/web_core/types/types";
import { structuralStyles } from "@a2ui/web_core/styles/index";
import { defaultTheme } from "./src/a2ui-default-theme";
import { renderMarkdown } from "./vendor/a2ui/renderers/markdown/markdown-it/src/markdown";

const SURFACE_ID = "chat-demo";
const ASSISTANT_COMPONENT_SEQUENCE = [
  "Text",
  "MultipleChoice",
  "Tabs",
  "Card",
  "Button",
  "TextField",
] as const;

type DemoRole = "user" | "assistant";
type AssistantComponentKind = (typeof ASSISTANT_COMPONENT_SEQUENCE)[number];

interface DemoMessage {
  id: string;
  role: DemoRole;
  content: string;
  componentKind?: AssistantComponentKind;
  staged?: boolean;
}

const seedMessages: DemoMessage[] = [
  {
    id: crypto.randomUUID(),
    role: "assistant",
    componentKind: "Text",
    staged: false,
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

function nextAssistantComponentKind(messages: DemoMessage[]): AssistantComponentKind {
  const stagedAssistantCount = messages.filter(
    (message) => message.role === "assistant" && message.staged,
  ).length;
  return ASSISTANT_COMPONENT_SEQUENCE[
    Math.min(stagedAssistantCount, ASSISTANT_COMPONENT_SEQUENCE.length - 1)
  ];
}

function addText(
  components: any[],
  id: string,
  text: string,
  usageHint: "caption" | "body" | "h4" = "body",
) {
  components.push({
    id,
    component: {
      Text: {
        usageHint,
        text: { literalString: text || "_..._" },
      },
    },
  });
}

function addMessageBlock(
  components: any[],
  message: DemoMessage,
  messageIndex: number,
): string {
  const prefix = `message-${messageIndex}-${message.id}`;
  const blockChildren: string[] = [];

  const labelId = `${prefix}-label`;
  addText(components, labelId, roleLabel(message.role), "caption");
  blockChildren.push(labelId);

  const bodyChildren: string[] = [];
  const summaryId = `${prefix}-summary`;
  addText(components, summaryId, message.content, "body");
  bodyChildren.push(summaryId);

  if (message.role === "assistant" && message.componentKind) {
    const componentId = addAssistantComponent(
      components,
      prefix,
      message.componentKind,
      message.content,
    );
    if (componentId) {
      const panelBodyId = `${prefix}-panel-body`;
      components.push({
        id: panelBodyId,
        component: {
          Column: {
            children: {
              explicitList: [componentId],
            },
            alignment: "stretch",
          },
        },
      });

      const panelId = `${prefix}-panel`;
      components.push({
        id: panelId,
        component: {
          Card: {
            child: panelBodyId,
          },
        },
      });
      bodyChildren.push(panelId);
    }
  }

  const bodyId = `${prefix}-body`;
  components.push({
    id: bodyId,
    component: {
      Column: {
        children: {
          explicitList: bodyChildren,
        },
        alignment: "stretch",
      },
    },
  });

  const shellId = `${prefix}-${message.role === "assistant" ? "assistant" : "user"}-shell`;
  components.push({
    id: shellId,
    component: {
      Card: {
        child: bodyId,
      },
    },
  });
  blockChildren.push(shellId);

  const blockId = `${prefix}-block`;
  components.push({
    id: blockId,
    component: {
      Column: {
        children: {
          explicitList: blockChildren,
        },
        alignment: "stretch",
      },
    },
  });

  return blockId;
}

function addAssistantComponent(
  components: any[],
  prefix: string,
  kind: AssistantComponentKind,
  content: string,
): string | null {
  switch (kind) {
    case "Text":
      return null;

    case "MultipleChoice": {
      const componentId = `${prefix}-multi-choice`;
      components.push({
        id: componentId,
        component: {
          MultipleChoice: {
            selections: { literalArray: [] },
            options: [
              { label: { literalString: "Summarize it" }, value: "summarize" },
              { label: { literalString: "Show raw data" }, value: "raw" },
              { label: { literalString: "Turn into tasks" }, value: "tasks" },
            ],
            type: "checkbox",
          },
        },
      });
      return componentId;
    }

    case "Tabs": {
      const summaryId = `${prefix}-tab-summary`;
      const detailId = `${prefix}-tab-detail`;
      const rawId = `${prefix}-tab-raw`;
      addText(components, summaryId, `Summary\n\n${content || "_..._"}`, "body");
      addText(
        components,
        detailId,
        "Details\n\nThis tab can hold structured follow-up output for the same assistant turn.",
        "body",
      );
      addText(
        components,
        rawId,
        "Raw\n\n```json\n{\"status\":\"demo\",\"source\":\"a2ui-test\"}\n```",
        "body",
      );

      const componentId = `${prefix}-tabs`;
      components.push({
        id: componentId,
        component: {
          Tabs: {
            tabItems: [
              { title: { literalString: "Summary" }, child: summaryId },
              { title: { literalString: "Details" }, child: detailId },
              { title: { literalString: "Raw" }, child: rawId },
            ],
          },
        },
      });
      return componentId;
    }

    case "Card": {
      const cardBodyId = `${prefix}-card-body`;
      addText(
        components,
        cardBodyId,
        "Card payload\n\nThis assistant turn is rendered inside a card container.",
        "body",
      );
      const componentId = `${prefix}-card`;
      components.push({
        id: componentId,
        component: {
          Card: {
            child: cardBodyId,
          },
        },
      });
      return componentId;
    }

    case "Button": {
      const buttonLabelId = `${prefix}-button-label`;
      addText(components, buttonLabelId, "Continue", "body");
      const componentId = `${prefix}-button`;
      components.push({
        id: componentId,
        component: {
          Button: {
            child: buttonLabelId,
            primary: true,
            action: {
              name: "demo.continue",
              context: [
                {
                  key: "source",
                  value: { literalString: "chat-demo" },
                },
              ],
            },
          },
        },
      });
      return componentId;
    }

    case "TextField": {
      const componentId = `${prefix}-text-field`;
      components.push({
        id: componentId,
        component: {
          TextField: {
            label: { literalString: "Follow-up input" },
            text: {
              literalString: content ? `Seeded from: ${content.slice(0, 48)}` : "",
            },
            textFieldType: "longText",
          },
        },
      });
      return componentId;
    }
  }
}

function buildComponents(messages: DemoMessage[]) {
  const components: any[] = [];
  const rootChildren = messages.map((message, index) =>
    addMessageBlock(components, message, index),
  );

  components.unshift({
    id: "root",
    component: {
      Column: {
        children: {
          explicitList: rootChildren,
        },
        alignment: "stretch",
      },
    },
  });

  return components;
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
    const assistantComponentKind = nextAssistantComponentKind(this.messages);
    this.messages = [
      ...this.messages,
      {
        id: assistantId,
        role: "assistant",
        content: "",
        componentKind: assistantComponentKind,
        staged: true,
      },
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
      max-width: 760px;
      margin: 0 auto;
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
