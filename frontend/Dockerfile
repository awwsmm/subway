# Step 1: Build the app
FROM node:24.8.0-alpine AS builder

WORKDIR /app/frontend

# Copy dependencies
COPY ./frontend/package.json ./frontend/package-lock.json* ./

# Install dependencies
RUN npm ci

# Copy source files
COPY ./frontend .
COPY ./keycloak/realm-export.json ../keycloak/

RUN npm run build

# Step 2: Serve the built app
FROM node:24.8.0-alpine AS runner

# Install a static file server
RUN npm install -g serve

WORKDIR /app/frontend

# Copy built files from builder
COPY --from=builder /app/frontend/dist ./dist
COPY --from=builder /app/keycloak/realm-export.json ../keycloak/

# Serve the app
CMD ["serve", "-s", "dist", "-l", "5173"]