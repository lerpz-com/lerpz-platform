FROM node:23-slim AS base


FROM base AS deps
WORKDIR /app

COPY pnpm-workspace.yaml pnpm-lock.yaml package.json ./
COPY ./svc/frontend/package.json ./svc/frontend/

RUN --mount=type=cache,target=/app/.pnpm-store \
    corepack enable pnpm && pnpm i --frozen-lockfile


FROM base AS builder
WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/svc/frontend/node_modules ./svc/frontend/node_modules

COPY pnpm-workspace.yaml pnpm-lock.yaml package.json ./
COPY ./svc/frontend ./svc/frontend

ENV NEXT_TELEMETRY_DISABLED=1

RUN --mount=type=cache,target=/app/svc/frontend/.next/cache \
    corepack enable pnpm && pnpm run build


FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

RUN addgroup --system app && \
    adduser --system --ingroup app --shell /bin/false --disabled-password --no-create-home lerpz && \
    chown -R lerpz:app /app

COPY --from=builder --chown=lerpz:app /app/svc/frontend/.next/standalone ./
COPY --from=builder --chown=lerpz:app /app/svc/frontend/.next/static ./.next/static
COPY --from=builder --chown=lerpz:app /app/svc/frontend/public ./public

USER lerpz

HEALTHCHECK CMD curl --fail http://localhost:3000/ || exit 1

ENV PORT=3000
ENV HOSTNAME="0.0.0.0"

EXPOSE 3000

CMD ["node", "server.js"]
