# Publish all crates in dependency order
[group('publish')]
publish_all: publish_pocket_ic_harness_macro publish_pocket_ic_harness

# Publish `pocket-ic-harness-macro`
[group('publish')]
publish_pocket_ic_harness_macro: (_publish "pocket-ic-harness-macro")

# Publish `pocket-ic-harness`
[group('publish')]
publish_pocket_ic_harness: (_publish "pocket-ic-harness")

# Publish a crate with retry logic to handle registry lag
[private]
_publish crate:
  #!/usr/bin/env bash
  set -euo pipefail
  max_retries=5
  delay=15
  for attempt in $(seq 1 $max_retries); do
    echo "Publishing {{crate}} (attempt $attempt/$max_retries)..."
    if cargo publish -p {{crate}}; then
      echo "Successfully published {{crate}}"
      exit 0
    fi
    if [ "$attempt" -lt "$max_retries" ]; then
      echo "Publish failed, retrying in ${delay}s..."
      sleep "$delay"
    fi
  done
  echo "Failed to publish {{crate}} after $max_retries attempts"
  exit 1
