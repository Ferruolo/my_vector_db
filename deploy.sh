# Insert Secrets
# Check if GOOGLE_API_KEY is set
if [ -z "$GOOGLE_API_KEY" ]; then
    echo "Error: GOOGLE_API_KEY environment variable is not set"
    exit 1
fi

# Create base64 encoded value
GOOGLE_API_KEY_BASE64=$(echo -n "$GOOGLE_API_KEY" | base64)

# Create actual secret file from template
cat config/secret-template.yaml | \
    sed "s/\${GOOGLE_API_KEY_BASE64}/$GOOGLE_API_KEY_BASE64/g" > \
    config/secret.yaml

# Apply configurations
kubectl apply -f config/secret.yaml

# rm -f config/secret.yaml

# Build images
cd scraper
docker build -t lead-finder:latest -f Dockerfile.lead_finder .
docker build -t scraper:latest -f Dockerfile.scraper .
cd ../test_rust_backend
docker build -t tokio-server:latest -f Dockerfile .

cd ..
# Load into Kind cluster
kind load docker-image lead-finder:latest --name kind
kind load docker-image scraper:latest --name kind
kind load docker-image redis:latest --name kind
kind load docker-image tokio-server:latest --name kind

kubectl apply -f config/manifest.yaml


