# Flora Workspace - Master Design System

## 1. Brand Philosophy & Visual Metaphor
Flora Workspace is built on the concept of organic, self-sustained growth. The visual language must communicate lightweight agility, robust security, and natural scalability.
- **The Metaphor:** The UI should feel like fertile soil—unobtrusive and foundational. Data and interactions are the seeds and plants.
- **The Vibe:** Corporate but approachable. Clean, snappy, and distraction-free. It should look perfectly at home on a minimal Tauri desktop app or a standard web browser.
- **The Promise:** Zero lag, zero bloat, zero vendor lock-in.

## 2. Color Palette & Theming (Svelte 5 + Tailwind + shadcn/ui)
The color palette revolves around CSS variables that seamlessly switch between Light and Dark modes. The core palette is inspired by the deployment tiers.

### Primary Brand Colors
- **Flora Seed (Primary Accent):** A vibrant, energetic green that signifies new life and primary actions (e.g., "Create Workspace", "Send Message").
  - Hex: `#10b981` (Emerald 500)
- **Flora Forest (Deep Backgrounds):** A dark, rich pine green/gray used for sidebars and dark mode backgrounds, providing high contrast and reduced eye strain.
  - Hex: `#064e3b` (Emerald 900) to `#022c22` (Emerald 950)
- **Fertile Soil (Base Neutrals):** Grays with a slight warm (brownish/earthy) or cool (slate) undertone to ground the UI.
  - Light Mode Base: `#f8fafc` (Slate 50)
  - Dark Mode Base: `#0f172a` (Slate 900)

### Tailwind / shadcn/ui Mapping
```css
@layer base {
  :root {
    --background: 210 40% 98%; /* Light slate #f8fafc */
    --foreground: 222.2 84% 4.9%; /* Dark slate #0f172a */
    --primary: 152 76% 39%; /* Flora Seed Green #10b981 */
    --primary-foreground: 210 40% 98%;
    --secondary: 210 40% 96.1%;
    --muted: 210 40% 96.1%;
    --border: 214.3 31.8% 91.4%;
    --radius: 0.5rem; /* Clean, slightly rounded corners */
  }
  .dark {
    --background: 222.2 84% 4.9%; /* Deep soil #0f172a */
    --foreground: 210 40% 98%;
    --primary: 152 76% 39%; /* Flora Seed Green */
    --primary-foreground: 222.2 47.4% 11.2%;
    --secondary: 217.2 32.6% 17.5%;
    --muted: 217.2 32.6% 17.5%;
    --border: 217.2 32.6% 17.5%;
  }
}
```

## 3. Typography
Typography must prioritize readability for long-form reading (chat, docs) and dense data visualization (tasks, settings).
- **Primary Font (UI & Content):** Inter or Geist (highly legible, modern sans-serif).
- **Monospace Font (Code & Logs):** JetBrains Mono or Geist Mono (for CI/CD logs, markdown code blocks, system settings).
- **Hierarchy:**
  - `h1` to `h3`: Tracking slightly tight (`tracking-tight`), semi-bold.
  - `body`: Regular weight, relaxed line-height (`leading-relaxed`) for chat readability.

## 4. UI/UX Core Principles

### A. Progressive Disclosure (Scaling with Deployment)
The UI must adapt based on the deployment tier:
- **🌱 Seed / 🌿 Sprout UI:** Hide complex administrative panels. The UI should look as simple as a basic chat app. No overwhelming sidebars.
- **🌳 Grove / 🌲 Forest UI:** Unlock advanced settings (RustFS distributed config, Meilisearch index tweaking, Zitadel OIDC mapping). Use shadcn/ui Tabs and Accordions to hide advanced configs until explicitly requested.

### B. "Soberania Digital" (Digital Sovereignty) Indicators
- **AI Feature Flags:** When an external AI (LLM API) is invoked, the UI must show a distinct visual indicator (e.g., a subtle purple sparkle icon or a border) denoting that data is temporarily leaving the local environment.
- **Local-First Feedback:** When data is saved locally (PostgreSQL/RustFS), use instant, optimistic UI updates via Svelte 5 Runes to make the app feel native and completely local.

### C. The "VPS Test" Performance Constraints
- **WebSockets over HTTP Polling:** Rely exclusively on Valkey Pub/Sub pushed through WebSockets for real-time chat and notifications. Absolutely no polling.
- **Svelte 5 Granular Reactivity:** Use `$state` and `$derived` to update only the specific DOM nodes that change (e.g., a single task status), preventing full-page repaints.

## 5. Component Architecture (shadcn/ui + Svelte 5)

### Navigation & Layout
- **App Shell:** A collapsible left sidebar containing Workspace switching, Channels, Direct Messages, and Modules (Tasks, Files). Fixed spacing, properly floating.
- **Header:** Minimalist. Contains a global search bar (powered by Meilisearch), current context title, and user profile/settings. Account for fixed heights in layout.
- **Right Panel (Contextual):** Used for Thread replies, Task details, or Document properties, sliding in without obstructing the main view.

### The Kanban Task Board (Planka/Wekan Inspired)
- **Cards:** Use standard shadcn/ui Cards.
- **Interactions:** Implement drag-and-drop using lightweight Svelte actions. Ensure hover states do not cause layout shifts.
- **Visual Feedback:** When dragging, the card should not feel heavy. Drop zones should highlight with the Flora Seed color (`bg-primary/20`).

### Real-Time Chat & Messaging
- **Layout:** Clean, full-width message rows (Slack/Discord style) optimizing horizontal space for desktop.
- **Features:** Rich text with integrated markdown support.
- **File Uploads:** Drag-and-drop zones streaming directly to RustFS. Display sleek shadcn/ui progress bars.

## 6. Key User Journeys

### Journey 1: The "First Seed" (Initial Setup)
- **Goal:** Pass the "VPS Test" setup smoothly.
- **Flow:** Admin runs `docker compose up` -> Accesses web UI -> Sees a beautiful, minimalist setup screen (shadcn/ui Card centered).
- **Steps:** 
  1. Create Admin Account (Saved to local PostgreSQL).
  2. Name the Workspace.
- **Result:** Immediate access without AWS keys or complex DB configs.

### Journey 2: Feature-Flagged AI Interaction
- **Goal:** Use AI securely and transparently.
- **Flow:** User clicks "Summarize Thread" -> Dialog warns: "This feature uses an external API (Feature Flag: ON). Proceed?" -> User accepts.
- **Result:** UI shows a skeleton loader (`shadcn Skeleton`) while OmniRoute/external API processes. Summary is injected into chat with a specific visual tag (AI-generated indicator).

## 7. UI/UX Pro-Max Rules & Implementation Guidelines

### Icons & Visual Elements
- **DO NOT** use emojis as UI icons. Use SVG icons (Lucide is standard for shadcn/ui).
- **DO** maintain stable hover states. Use color/opacity transitions, avoiding scale transforms that shift layout.
- **DO** use consistent icon sizing with a fixed viewBox (24x24) and uniform utility classes (e.g., `w-5 h-5` or `w-6 h-6`).

### Interaction & Accessibility (a11y)
- **Cursor:** Ensure all clickable/hoverable elements explicitly have `cursor-pointer`.
- **Transitions:** Use smooth transitions (`transition-colors duration-200`) for all interactive elements.
- **Accessibility built-in:** Utilize Bits UI (underlying Svelte shadcn/ui) for ARIA attributes, keyboard navigation, and screen reader support. This is crucial for an enterprise AGPL product.

### Light/Dark Mode Contrast
- **Light Mode:** Use high contrast text (e.g., `#0F172A` `text-slate-900`) for readability. Muted text must be at least `#475569` `text-slate-600`.
- **Borders:** Ensure borders are visible in both modes (e.g., `border-gray-200` in light mode).
- **Glass Effects:** If using glassmorphism in light mode, ensure opacity is high enough (`bg-white/80+`) to be legible.

### Svelte 5 Structural Rigor
- **Zero Console Errors:** Handle all Promise rejections gracefully via Svelte 5 Error Boundaries or global toast notifications (`shadcn/ui toast`). Raw `console.log` or unhandled exceptions are strictly forbidden.
- **Error States:** Every component must have a fallback UI. Example: If the WebSocket drops, display a subtle banner (`shadcn/ui Alert`): "Connection to Flora lost. Reconnecting...".
