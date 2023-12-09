FROM node:20-slim AS base
WORKDIR /app
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

COPY package*.json ./
RUN pnpm install

EXPOSE 5173

CMD ["pnpm", "dev", "--host"]
