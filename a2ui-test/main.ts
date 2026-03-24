import { LitElement, css, html } from "lit";
import { customElement } from "lit/decorators.js";
import { provide } from "@lit/context";
import { v0_8 } from "@a2ui/lit";
import type * as Types from "@a2ui/web_core/types/types";
import { structuralStyles } from "@a2ui/web_core/styles/index";
import { defaultTheme } from "./src/a2ui-default-theme";
import { renderMarkdown } from "./vendor/a2ui/renderers/markdown/markdown-it/src/markdown";

const SURFACE_ID = "main";

const globalStyle = document.createElement("style");
globalStyle.textContent = structuralStyles;
document.head.appendChild(globalStyle);

const bootstrapMessages = [
  {
    beginRendering: {
      surfaceId: SURFACE_ID,
      root: "root",
      styles: {
        primaryColor: "#4f7cff",
        font: "Inter, ui-sans-serif, system-ui, sans-serif",
      },
    },
  },
  {
    surfaceUpdate: {
      surfaceId: SURFACE_ID,
      components: [
        {
          id: "root",
          component: {
            Text: {
              usageHint: "body",
              text: {
                literalString: "Hello from local vendored A2UI.",
              },
            },
          },
        },
      ],
    },
  },
];

@customElement("a2ui-demo-app")
class A2uiDemoApp extends LitElement {
  @provide({ context: v0_8.UI.Context.theme })
  theme: Types.Theme = defaultTheme;

  @provide({ context: v0_8.UI.Context.markdown })
  markdownRenderer: Types.MarkdownRenderer = renderMarkdown;

  processor = new v0_8.Data.A2uiMessageProcessor();

  connectedCallback(): void {
    super.connectedCallback();
    this.processor.processMessages(bootstrapMessages as never[]);
  }

  get surface() {
    return this.processor.getSurfaces().get(SURFACE_ID) ?? null;
  }

  static styles = css`
    :host {
      display: block;
      min-height: 100vh;
      padding: 32px;
      background:
        radial-gradient(circle at top, rgba(79, 124, 255, 0.18), transparent 26rem),
        #0f172a;
      color: white;
      box-sizing: border-box;
    }

    a2ui-surface {
      max-width: 720px;
      margin: 0 auto;
    }
  `;

  render() {
    return html`
      <a2ui-surface
        .surfaceId=${SURFACE_ID}
        .surface=${this.surface}
        .processor=${this.processor}
      ></a2ui-surface>
    `;
  }
}

const mount = document.querySelector("#app");

if (!(mount instanceof HTMLElement)) {
  throw new Error("Missing #app element");
}

mount.replaceChildren(document.createElement("a2ui-demo-app"));

Object.assign(window, {
  feedA2uiMessage: (message: unknown) => {
    const app = document.querySelector("a2ui-demo-app") as A2uiDemoApp | null;
    if (!app) {
      throw new Error("Missing a2ui-demo-app");
    }

    app.processor.processMessages([message as never]);
    app.requestUpdate();
  },
});
