# Build images
docker build -t lead-finder:latest -f Dockerfile.lead_finder ./scraper
docker build -t scraper:latest -f Dockerfile.scraper ./scraper

# Load into Kind cluster
kind load docker-image lead-finder:latest --name lead-scraper-cluster
kind load docker-image scraper:latest --name lead-scraper-cluster
kind load docker-image redis:latest --name lead-scraper-cluster
# Build images
docker build -t lead-finder:latest -f Dockerfile.lead_finder .
docker build -t scraper:latest -f Dockerfile.scraper .

# Load into Kind cluster
kind load docker-image lead-finder:latest --name lead-scraper-cluster
kind load docker-image scraper:latest --name lead-scraper-cluster
kubectl apply -f manifest.yaml
