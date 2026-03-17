# Wireframe

This wireframe uses a shared `index.html` shell and loads page partials with `fetch()`.

It also uses a compiled Tailwind/DaisyUI stylesheet built from `input.css` into `dist/tailwind.css`.

Because of that, do not open `index.html` with `file://`. Serve this folder over HTTP and run the Tailwind watcher when changing CSS.

## Quick start

From `/workspace/wireframe` run:

```bash
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000
```

## Alternative

To rebuild CSS while editing:

```bash
tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
```

If you want both in one command flow, use:

```bash
./wireframe-tmux.sh
```

This starts a tmux session with:

- a Python HTTP server on port `8000`
- a `tailwind-extra --watch` pane

If you already use Node.js, you can also run:

```bash
npx serve .
```

Then open the URL shown in the terminal.

## Notes

- `index.html` contains the shared header and sidebar.
- `dashboard.html`, `users.html`, `teams.html`, `billing.html`, and `settings.html` are partials loaded into the shell.
- `input.css` is the Tailwind v4 entrypoint and the place to add shared `@apply` rules.
- If the page body does not load, check that you are serving the folder over HTTP rather than opening the file directly.
