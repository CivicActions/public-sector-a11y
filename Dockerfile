FROM node:alpine AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn config set network-timeout 300000 && yarn install --frozen-lockfile

FROM node:alpine AS builder
WORKDIR /app

ARG WEB_SOCKET_URL
ARG API

ENV API=${API:-http://localhost:3280/graphql}
ENV WEB_SOCKET_URL=${WEB_SOCKET_URL:-ws://localhost:3280/graphql}
ENV NEXT_PUBLIC_DISABLE_SEO 1
ENV DOCKER_CONTAINER 1
ENV NODE_ENV production

COPY . .
COPY --from=deps /app/node_modules ./node_modules
# remove all dev modules
RUN yarn build && rm -R ./node_modules && yarn install --production

FROM node:alpine AS runner
WORKDIR /app

RUN addgroup -g 1001 -S nodejs
RUN adduser -S nextjs -u 1001
USER nextjs

# You only need to copy next.config.js if you are NOT using the default configuration
COPY --from=builder /app/next.config.js ./
COPY --from=builder /app/next-sitemap.config.js ./
COPY --from=builder /app/tailwind.config.js ./
COPY --from=builder /app/postcss.config.js ./
COPY --from=builder /app/public ./public
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder --chown=nextjs:nodejs /app/.next ./.next

EXPOSE 3000

ENV PORT 3000

ENV API=${API:-http://localhost:3280/graphql}
ENV WEB_SOCKET_URL=${WEB_SOCKET_URL:-ws://localhost:3280/graphql}
ENV NODE_ENV production
ENV NEXT_TELEMETRY_DISABLED 1
ENV DOCKER_CONTAINER 1
ENV NEXT_PUBLIC_DISABLE_SEO 1
ENV PWA_ENABLED 1

CMD ["node_modules/.bin/next", "start"]