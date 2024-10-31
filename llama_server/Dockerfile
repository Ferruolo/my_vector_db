# Start with Ubuntu minimal base image
FROM ubuntu


# Update package lists and install minimal essential packages
RUN apt-get update && apt-get install -y \
    ca-certificates \
    wget \
    sudo \
    libcap2-bin \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Set working directory
WORKDIR /app

# Define environment variable without trailing spaces
ENV LLAMAFILE="TinyLlama-1.1B-Chat-v1.0.Q5_K_M.llamafile"

# Download and prepare llamafile in a single RUN command to reduce layers
RUN wget https://huggingface.co/jartine/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/TinyLlama-1.1B-Chat-v1.0.Q5_K_M.llamafile \
    && chmod +x $LLAMAFILE \
    && setcap cap_sys_resource=+ep /app/$LLAMAFILE

# Expose 8080 port.
EXPOSE 8080

# Set entrypoint.
ENTRYPOINT ["/bin/sh", "/app/TinyLlama-1.1B-Chat-v1.0.Q5_K_M.llamafile"]
CMD ["--server", "--nobrowser", "--host", "0.0.0.0","--port", "8080"]
