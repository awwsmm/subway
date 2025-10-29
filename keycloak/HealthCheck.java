import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.*;
import java.security.cert.X509Certificate;
import javax.net.ssl.*;

// Why is the Keycloak healthcheck written in Java?
//
// - The keycloak base image does not contain the following, for security reasons: curl, wget, ps, top, htop, php, python...
// - You can use /dev/tcp to do an HTTP request to localhost, but not HTTPS...
// - kcadm.sh is available, but requires a truststore...
//
// But Java is available on the Keycloak base image, and it's pretty easy to write an HTTPS-ready cURL replacement...

public class HttpsClient {

    public static void main(String[] args) {
        String urlString = "https://localhost:9000/health/ready";

        // TODO: Trust all certificates (WARNING: DO NOT USE IN PRODUCTION)
        try {
            TrustManager[] trustAllCerts = new TrustManager[] {
                new X509TrustManager() {
                    public X509Certificate[] getAcceptedIssuers() { return null; }
                    public void checkClientTrusted(X509Certificate[] certs, String authType) {}
                    public void checkServerTrusted(X509Certificate[] certs, String authType) {}
                }
            };
            SSLContext sslContext = SSLContext.getInstance("TLS");
            sslContext.init(null, trustAllCerts, new java.security.SecureRandom());
            HttpsURLConnection.setDefaultSSLSocketFactory(sslContext.getSocketFactory());
            HttpsURLConnection.setDefaultHostnameVerifier((hostname, session) -> true);
        } catch (Exception e) {
            e.printStackTrace();
        }

        try {
            URL url = new URI(urlString).toURL();
            HttpsURLConnection conn = (HttpsURLConnection) url.openConnection();
            conn.setRequestMethod("GET");
            conn.setRequestProperty("Accept", "application/json");

            if (conn.getResponseCode() != 200) {
                System.out.println("Failed to connect. HTTP error code: " + conn.getResponseCode());
                return;
            }

            // Read the response
            BufferedReader br = new BufferedReader(new InputStreamReader(conn.getInputStream()));
            StringBuilder response = new StringBuilder();
            String output;
            while ((output = br.readLine()) != null) {
                response.append(output);
            }
            conn.disconnect();

            if (response.toString().contains("\"status\": \"UP\"")) {
                System.out.println("healthcheck successful");
                System.exit(0);
            } else {
                System.out.println("healthcheck failed");
                System.exit(1);
            }

        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}