
kind load docker-image scraper:latest --name kind
kubectl apply -f manifest.yaml


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
