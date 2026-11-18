#!/usr/bin/env bash
# Compile the baseline plugin (Maven, paper-api 26.3 from repo.papermc.io).
# Requires: JDK 21 + Maven (both available via setup-java/setup-maven on CI).
#
# The baseline matrix runs up to 12 parallel jobs and every one resolves the
# same plugin set from Maven Central — central answers bursts with HTTP 429
# and a bare `mvn` treats that as fatal (the mixed-150k / tiles-100k flights
# died exactly this way on f542062). Retry the package phase a few times:
# a 429 clears in seconds, the local ~/.m2 cache absorbs everything after
# the first success, so later attempts are cheap.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$ROOT/build"

ok=0
for attempt in 1 2 3 4 5; do
    if mvn -q -B -f "$ROOT/baseline/plugin/pom.xml" package -DskipTests; then
        ok=1
        break
    fi
    echo "mvn package attempt $attempt failed (maven central 429 / flake) — retrying in $((attempt * 15))s"
    sleep $((attempt * 15))
done
if [ "$ok" != 1 ]; then
    echo "mvn package failed after 5 attempts" >&2
    exit 1
fi
cp "$ROOT/baseline/plugin/target/ce-baseline.jar" "$ROOT/build/ce-baseline.jar"
echo "built: $ROOT/build/ce-baseline.jar"
