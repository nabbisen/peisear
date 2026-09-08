# Handoffs — vendored front-end assets

Implementing **`DEC-051`** (RFC 011, authorised at step 4). Tailwind and DaisyUI
move from two CDNs into `static/`, so a self-hosted instance renders without
egress and a layout gate becomes possible at all.

| ID | Link | What | Release |
|---|---|---|---|
| ASSET-001 | [ASSET-001](./ASSET-001-vendor-tailwind-daisyui.md) | Purged Tailwind via the standalone binary; DaisyUI as its prebuilt stylesheet. No Node. | 0.32.0 |
