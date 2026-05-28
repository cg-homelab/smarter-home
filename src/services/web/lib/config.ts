import 'server-only'

// Read at runtime by the Node.js server — never bundled into the client.
export const API_URL = process.env.API_URL ?? 'http://localhost:3001'
