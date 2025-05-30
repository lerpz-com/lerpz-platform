FROM node:23-slim AS base


FROM base AS deps
WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN corepack enable pnpm && pnpm i --frozen-lockfile


FROM base AS builder
WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules
COPY . .

ENV NEXT_TELEMETRY_DISABLED=1

RUN corepack enable pnpm && pnpm run build


FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

RUN addgroup --system app && \
    adduser --system --ingroup app --shell /bin/false --disabled-password --no-create-home lerpz && \
    chown -R lerpz:app /app

COPY --from=builder /app/public ./public

COPY --from=builder --chown=lerpz:appuser /app/.next/standalone ./
COPY --from=builder --chown=lerpz:appuser /app/.next/static ./.next/static

USER lexentry

HEALTHCHECK CMD curl --fail http://localhost:3000/ || exit 1

ENV PORT=3000
ENV HOSTNAME="0.0.0.0"

EXPOSE 3000

CMD ["node", "server.js"]
