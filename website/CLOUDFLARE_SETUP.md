# Cloudflare Pages Setup Guide

This guide explains how to deploy the kodo documentation site to Cloudflare Pages.

## Prerequisites

- Cloudflare account
- GitHub repository access

## Setup Steps

### 1. Login to Cloudflare Dashboard

Go to [Cloudflare Dashboard](https://dash.cloudflare.com/) and login to your account.

### 2. Create a Pages Project

1. Navigate to **Workers & Pages** in the sidebar
2. Click **Create** button
3. Select **Pages** tab
4. Click **Connect to Git**

### 3. Connect GitHub Repository

1. Authorize Cloudflare to access your GitHub account
2. Select the `yumazak/kodo` repository
3. Click **Begin setup**

### 4. Configure Build Settings

| Setting | Value |
|---------|-------|
| Project name | `kodo-docs` (or your preference) |
| Production branch | `main` |
| Framework preset | `None` |
| Build command | `cd website && pnpm install && pnpm build` |
| Build output directory | `website/doc_build` |
| Root directory | `/` (leave empty) |

### 5. Environment Variables (Optional)

| Variable | Value |
|----------|-------|
| `NODE_VERSION` | `20` |

### 6. Deploy

Click **Save and Deploy**. Cloudflare will build and deploy your site.

## Automatic Deployments

By default, Cloudflare Pages deploys on:
- Push to the production branch (`main`)
- Pull requests (Preview deployments)

To avoid deployments for every PR (issue #30), configure the project as follows:

1. Open **Workers & Pages** → your project → **Settings** → **Builds & deployments**
2. Set **Production branch** to `main`
3. Disable **Preview deployments** (or limit preview branch patterns)
4. Optional: in **Build watch paths**, include only `website/**` so non-doc changes do not trigger deploys

This keeps production deploys tied to `main` merges while avoiding unnecessary PR preview builds.

## Custom Domain (Optional)

1. Go to your Pages project
2. Click **Custom domains** tab
3. Click **Set up a custom domain**
4. Follow the DNS configuration instructions

## Troubleshooting

### Build fails with pnpm not found

Add environment variable:
- `PNPM_VERSION`: `10`

Or use npm instead:
- Build command: `cd website && npm install && npm run build`

### Node version issues

Set environment variable:
- `NODE_VERSION`: `20`

## Local Preview

Before deploying, you can preview locally:

```bash
cd website
pnpm install
pnpm dev      # Development server at http://localhost:5173
pnpm build    # Production build
pnpm preview  # Preview production build
```
