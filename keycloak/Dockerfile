# build with: docker build -t subway-keycloak -f keycloak/Dockerfile .
# run with:   docker run -p 8080:8080 -e KEYCLOAK_ADMIN=admin -e KEYCLOAK_ADMIN_PASSWORD=changeme -d subway-keycloak

# -------- STAGE 1: Build --------
FROM quay.io/keycloak/keycloak:26.4.1-0 AS builder

# Run build (enables optimizations, caches dependencies)
RUN /opt/keycloak/bin/kc.sh build --health-enabled=true

# -------- STAGE 2: Runtime --------
FROM quay.io/keycloak/keycloak:26.4.1-0

# Copy the build artifacts from the builder image
COPY --from=builder /opt/keycloak/ /opt/keycloak/

# Expose default HTTP port
EXPOSE 8080

# How the healthcheck works, inside the container
# exec 3<>/dev/tcp/127.0.0.1/9000 -- open a TCP connection using bash's built-in support for TCP sockets
# echo -e "GET /health/ready ...  -- send a GET request to the /health/ready endpoint. The result will look like
#
#   HTTP/1.1 200 OK
#   content-type: application/json; charset=UTF-8
#   cache-control: no-store
#   connection: close
#   content-length: 145
#
#   {
#       "status": "UP",
#       "checks": [
#           {
#               "name": "Keycloak cluster health check",
#               "status": "UP"
#           }
#       ]
#   }
#
# response=$(cat <&3)             -- save the output of file descriptor 3 to a variable called 'response'
# exec 3>&-                       -- closes the file descriptor
# echo "$response" | grep -q ...  -- looks for the string '"status": "UP"' in the output, returns 1 (fails) if not found
HEALTHCHECK --interval=1s --retries=120 CMD bash -c '\
  exec 3<>/dev/tcp/127.0.0.1/9000 && \
  echo -e "GET /health/ready HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n" >&3 && \
  response=$(cat <&3); \
  exec 3>&-; \
  echo "$response" | grep -q "\"status\": \"UP\"" || exit 1'

# entrypoint to run Keycloak
ENTRYPOINT ["/opt/keycloak/bin/kc.sh", "start", "--health-enabled=true", "--http-enabled=true", "--hostname-strict=false"]