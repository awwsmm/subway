# Step 1: Build the app
FROM node:24.8.0-alpine AS builder

WORKDIR /app/frontend

# Copy dependencies
COPY ./frontend/package.json ./frontend/package-lock.json* ./

# Install dependencies
RUN npm ci

# Copy source files, including .env.container, which is used to configure 'npm run build'
COPY ./frontend .
COPY ./keycloak/realm-export.json ../keycloak/

# Build the project, using .env.container, rather than .env.local
#
#   Recall that 'npm run build' runs the 'build' script defined in package.json. As of this writing, that script is:
#     tsc -b && vite build
#   Executing 'npm run build -- x y z' appends the arguments 'x y z' to the end of the script, so
#     npm run build -- --mode container
#   expands to
#     tsc -b && vite build --mode container
#   This tells vite to use the .env.container file, rather than the typical .env.local one, during the build.
#
RUN npm run build -- --mode container

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