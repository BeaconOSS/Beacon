FROM node:22-slim

WORKDIR /app

COPY package.json package-lock.json* ./
RUN npm install --omit=dev --no-audit --no-fund

COPY normalize.js server.js ./

ENV PORT=3000
EXPOSE 3000

CMD ["node", "server.js"]
