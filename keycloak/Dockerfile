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

# COPY the certificate and key from localhost
COPY ./keycloak/certs /opt/keycloak/certs

# COPY the healthcheck script
COPY ./keycloak/HealthCheck.java /opt/keycloak/

# entrypoint to run Keycloak
# Note that this requires a cert.pem and a key.pem file. Create these with
# mkdir -p keycloak/certs && openssl req -x509 -newkey rsa:4096 -keyout keycloak/certs/key.pem -out keycloak/certs/cert.pem -sha256 -days 3650 -nodes -subj '/CN=localhost'
ENTRYPOINT ["/opt/keycloak/bin/kc.sh", "start", "--health-enabled=true", "--https-certificate-file=/opt/keycloak/certs/cert.pem", "--https-certificate-key-file=/opt/keycloak/certs/key.pem", "--hostname=localhost"]