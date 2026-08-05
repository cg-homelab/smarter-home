# Integrations

How data gets from a smart home hub into the Smarter Home API, and where the example scripts live.

## The /integrations page

`src/services/webapp/app/(app)/integrations/page.tsx` renders copy-pasteable ingest scripts for two platforms:

- Homey Pro: HomeyScript (JavaScript) run from a Flow.
- Home Assistant: Node-RED `function` nodes feeding an `http request` node. Home Assistant has no built-in JavaScript runtime.

The page is a server component. Platform selection lives in the URL (`/integrations?platform=home-assistant`), so tabs are shareable and the page never needs client state. The tab triggers are `next/link` anchors rendered through `TabsTrigger asChild`.

Supporting files:

| File | Purpose |
| --- | --- |
| `src/services/webapp/lib/integrations/scripts.ts` | The script catalog and per-platform setup steps. All content lives here. |
| `src/services/webapp/types/integrations.ts` | `IntegrationPlatform`, `ScriptExample`, `PlatformMeta`. |
| `src/services/webapp/lib/highlight.ts` | Shiki wrapper, server-only, cached highlighter. |
| `src/services/webapp/components/integrations/code-block.tsx` | Server-rendered highlighted block. |
| `src/services/webapp/components/integrations/copy-button.tsx` | The only client component on the page. |

To add an example, append a `ScriptExample` to the relevant array in `scripts.ts`. Nothing else needs to change.

## Ingest authentication

Ingest scripts authenticate with the **home write token**, not a user session.

- The token is minted on home creation in `src/lib/lib-db/src/home/mod.rs` and returned as `writeToken` from `POST /home` and `GET /home`.
- It is a non-expiring HS256 JWT with `role: Home` and `id` set to the home id.
- Every ingest route requires the JWT `id` claim to equal the request body `homeId`, so a user access token can never post ingest data.
- Users find the value on the Homes page, behind the Show toggle.

See `docs/auth.md` for the shared Claims extraction path.

## Ingest endpoints

All bodies are camelCase and timestamps are RFC3339.

| Endpoint | Body |
| --- | --- |
| `POST /power` | Flat object. All twelve fields are required: `homeId`, `ts`, `price`, `power`, `solarPower`, `lastMeterConsumption`, `lastMeterProduction`, `lastSolarTotal`, `consumptionSinceMidnight`, `productionSinceMidnight`, `solarSinceMidnight`, `costSinceMidnight`, `currency`. |
| `POST /devices` | `{ homeId, devices: [{ id?, name, deviceType, hasPowerMeter, isActive? }] }`. Responds with the stored devices, including server-assigned ids. |
| `POST /devices/metrics` | `{ homeId, metrics: [{ deviceId, ts, wattageW?, state?, metrics?, source? }] }`. Responds `{ message, rowsAffected }`. |
| `POST /weather/observations` | `{ homeId, observations: [{ ts, provider?, tempC?, cloudsPct?, rainMm1h?, windMs?, pressureHpa?, humidityPct?, uvi?, solarIrradianceWm2?, weatherCode?, raw? }] }`. |
| `POST /weather/forecasts` | `{ homeId, forecasts: [{ runTs, validTs, horizonHours, provider?, tempC?, cloudsPct?, rainMm?, windMs?, uvi?, raw? }] }`. |

Payload definitions: `src/lib/lib-models/src/domain/{power,device,weather}.rs`. Routes: `src/services/api/src/routes/mod.rs`. The live contract is at `/openapi.json`, rendered at `/docs`.

A `homeId` that does not match the token returns `401 {"message":"Unauthorized"}`. CORS allows any origin, so a hub can call the API directly.

## Keeping the examples honest

The scripts on the page are the user-facing copy of the ingest contract. When a payload struct in `src/lib/lib-models/src/domain/` changes, update `src/services/webapp/lib/integrations/scripts.ts` and the table above in the same pull request.
