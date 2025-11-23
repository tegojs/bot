# Documentation

This directory contains the VitePress documentation site for @tego/bot.

## Structure

- `/` - VitePress documentation site
  - `index.md` - Homepage with hero section
  - `api.md` - API documentation index
  - `.vitepress/` - VitePress configuration
    - `config.ts` - Site configuration
  
- `/api/` - TypeDoc generated API documentation (HTML)
  - Generated from `packages/botjs/src/index.ts`

- `/developments/` - Development notes and research documents
  - `index.md` - Development notes index
  - Various research markdown files

## Local Development

### Prerequisites

Node.js and pnpm installed.

### Install Dependencies

```bash
# From project root
pnpm install

# Or in docs directory
cd docs
pnpm install
```

### Generate API Documentation

```bash
# From project root
pnpm docs:api
```

### Serve Documentation Locally

```bash
# From project root
pnpm docs:dev

# Or manually
cd docs
pnpm dev
```

Visit http://localhost:5173

### Build Documentation

```bash
# From project root
pnpm docs:build

# Or manually
cd docs
pnpm build
```

Output: `.vitepress/dist/`

### Preview Built Site

```bash
# From project root
pnpm docs:preview

# Or manually
cd docs
pnpm preview
```

## Deployment

### GitHub Pages

1. Build the site: `pnpm docs:build`
2. The output is in `docs/.vitepress/dist/`
3. Configure GitHub Pages:
   - Settings > Pages
   - Source: GitHub Actions (recommended) or Deploy from a branch
   - Use a GitHub Action to build and deploy

### Alternative: Deploy from /docs

If deploying directly from `/docs` directory:
- Set `base: '/bot/'` in `.vitepress/config.ts` (already configured)
- Push the built site to the repository
- Configure GitHub Pages to serve from `/docs`

## Configuration

### VitePress Config

Located at `.vitepress/config.ts`:
- Site title, description, base URL
- Navigation menu
- Sidebar configuration
- Search settings
- Social links

### Theme Customization

VitePress uses Vue 3 + Vite. You can customize:
- Theme colors via CSS variables
- Custom components in `.vitepress/theme/`
- Layout overrides

## Notes

- VitePress is a Vue-powered static site generator
- TypeDoc output (`/api/`) is served as static HTML
- Markdown files are automatically converted to pages
- Built-in search functionality
- Fast HMR during development
