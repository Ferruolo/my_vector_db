# Build images
cd scraper
docker build -t lead-finder:latest -f Dockerfile.lead_finder .
docker build -t scraper:latest -f Dockerfile.scraper .

# Load into Kind cluster
kind load docker-image lead-finder:latest --name kind
kind load docker-image scraper:latest --name kind
kind load docker-image redis:latest --name kind
# Build images
docker build -t lead-finder:latest -f Dockerfile.lead_finder .
docker build -t scraper:latest -f Dockerfile.scraper .

cd ..
# Load into Kind cluster
kind load docker-image lead-finder:latest --name kind
kind load docker-image scraper:latest --name kind
kubectl apply -f manifest.yaml
