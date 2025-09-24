# Step 1: Build the app
FROM node:24.8.0-alpine AS builder

WORKDIR /app

# Copy dependencies
COPY package.json package-lock.json* ./

# Install dependencies
RUN npm ci

# Copy source files, including .env.local, which is used to configure 'npm run build'
COPY . .

# Build the project
RUN npm run build

# Step 2: Serve the built app
FROM node:24.8.0-alpine AS runner

# Install a static file server
RUN npm install -g serve

WORKDIR /app

# Copy built files from builder
COPY --from=builder /app/dist ./dist

# Expose port
EXPOSE 5173

# Serve the app
CMD ["serve", "-s", "dist", "-l", "5173"]
