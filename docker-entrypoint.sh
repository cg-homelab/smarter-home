#!/bin/sh
# Write runtime environment variables into a JS file that the webapp loads
# before the main bundle. This allows the container to be configured at
# startup without rebuilding the static assets.

cat <<EOF > /usr/share/nginx/html/config.js
window.__ENV__ = {
  apiBaseUrl: "${API_URL:-http://localhost:3001}"
};
EOF

exec nginx -g "daemon off;"
