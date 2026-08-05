import type {
    IntegrationPlatform,
    PlatformMeta,
    ScriptExample,
} from '@/types/integrations'

/**
 * Placeholder header prepended to every example. Users replace these three
 * values with their own; `HOME_ID` and `WRITE_TOKEN` come from the Homes page.
 */
const CONFIG_HEADER = `// Replace these three values. Home id and write token are on the Homes page.
const API_URL = 'https://your-smarter-home-api.example.com'
const HOME_ID = '00000000-0000-0000-0000-000000000000'
const WRITE_TOKEN = 'YOUR_HOME_WRITE_TOKEN'
`

const homeyExamples: ScriptExample[] = [
    {
        id: 'homey-power',
        title: 'Send a power snapshot',
        description:
            'Reads live price and power capabilities from your energy meter device and posts one snapshot. Run it from a Flow every 1-5 minutes.',
        method: 'POST',
        path: '/power',
        language: 'javascript',
        code: `${CONFIG_HEADER}
// Name of the device that reports live power (Tibber Pulse, P1 reader, ...).
const METER_DEVICE_NAME = 'Tibber Pulse'

// Map payload fields to the capability ids your device actually exposes.
// Check Homey Developer Tools -> Devices to see the ids for your meter.
const CAPABILITY_MAP = {
  price: 'measure_price_total',
  power: 'measure_power',
  solarPower: 'measure_power.produced',
  lastMeterConsumption: 'meter_power.consumed',
  lastMeterProduction: 'meter_power.produced',
  lastSolarTotal: 'meter_power.solar',
  consumptionSinceMidnight: 'meter_power.daily_consumed',
  productionSinceMidnight: 'meter_power.daily_produced',
  solarSinceMidnight: 'meter_power.daily_solar',
  costSinceMidnight: 'meter_cost.daily',
}

const devices = Object.values(await Homey.devices.getDevices())
const meter = devices.find((device) => device.name === METER_DEVICE_NAME)

if (!meter) {
  throw new Error('Device not found: ' + METER_DEVICE_NAME)
}

// Every field on POST /power is required, so missing capabilities fall back to 0.
const read = (capabilityId) => meter.capabilitiesObj?.[capabilityId]?.value ?? 0

const payload = { homeId: HOME_ID, ts: new Date().toISOString(), currency: 'NOK' }
for (const [field, capabilityId] of Object.entries(CAPABILITY_MAP)) {
  payload[field] = read(capabilityId)
}

const response = await fetch(API_URL + '/power', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    Authorization: 'Bearer ' + WRITE_TOKEN,
  },
  body: JSON.stringify(payload),
})

if (!response.ok) {
  throw new Error('Power ingest failed: ' + response.status + ' ' + (await response.text()))
}

log('Sent power snapshot: ' + payload.power + ' W')
return true`,
        notes: [
            'All twelve fields are required by the API. The loop above fills every one, using 0 when a capability is missing.',
            '`currency` is a plain string such as `NOK` or `EUR`, not a number.',
            'Return `true` so the Flow card reports success; throwing marks the Flow step as failed and shows up in the Homey timeline.',
        ],
    },
    {
        id: 'homey-devices',
        title: 'Register your devices',
        description:
            'Sends every Homey device to the API so metrics can be attached to them. Run once, then again whenever you add or rename devices.',
        method: 'POST',
        path: '/devices',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const devices = Object.values(await Homey.devices.getDevices())

const payload = {
  homeId: HOME_ID,
  devices: devices.map((device) => ({
    // Omit \`id\` on first run and the API assigns one. Send it back on later
    // runs to update the existing row instead of creating a duplicate.
    id: undefined,
    name: device.name,
    deviceType: device.class,
    hasPowerMeter: Boolean(device.capabilitiesObj?.measure_power),
    isActive: device.available !== false,
  })),
}

const response = await fetch(API_URL + '/devices', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    Authorization: 'Bearer ' + WRITE_TOKEN,
  },
  body: JSON.stringify(payload),
})

if (!response.ok) {
  throw new Error('Device ingest failed: ' + response.status + ' ' + (await response.text()))
}

// The response contains the stored devices, including server-assigned ids.
const stored = await response.json()

// Persist the Homey name -> API device id mapping for the metrics script.
const idsByName = {}
for (const device of stored) {
  idsByName[device.name] = device.id
}
await global.set('smarterHomeDeviceIds', JSON.stringify(idsByName))

log('Registered ' + stored.length + ' devices')
return true`,
        notes: [
            '`deviceType` is free text. Homey `device.class` values (`light`, `socket`, `sensor`, ...) map onto it directly.',
            'The response body is the stored device list with the ids the API assigned. Keep those ids: `POST /devices/metrics` references devices by `deviceId`, not by name.',
            '`global.set` stores the mapping between HomeyScript runs. If your Homey firmware does not expose `global`, write the JSON to a Homey Logic variable instead.',
        ],
    },
    {
        id: 'homey-device-metrics',
        title: 'Push per-device power readings',
        description:
            'Samples every device that has a power meter and sends the batch as one request. Pairs with the registration script above.',
        method: 'POST',
        path: '/devices/metrics',
        language: 'javascript',
        code: `${CONFIG_HEADER}
// Written by the "Register your devices" script.
const idsByName = JSON.parse((await global.get('smarterHomeDeviceIds')) ?? '{}')

const devices = Object.values(await Homey.devices.getDevices())
const ts = new Date().toISOString()

const metrics = devices
  .filter((device) => idsByName[device.name] && device.capabilitiesObj?.measure_power)
  .map((device) => ({
    deviceId: idsByName[device.name],
    ts,
    wattageW: device.capabilitiesObj.measure_power.value ?? null,
    state: device.capabilitiesObj.onoff?.value === false ? 'off' : 'on',
    // Anything extra you want to keep goes in \`metrics\` as free-form JSON.
    metrics: {
      zone: device.zoneName,
      available: device.available,
    },
    source: 'homey',
  }))

if (metrics.length === 0) {
  log('Nothing to send')
  return true
}

const response = await fetch(API_URL + '/devices/metrics', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    Authorization: 'Bearer ' + WRITE_TOKEN,
  },
  body: JSON.stringify({ homeId: HOME_ID, metrics }),
})

if (!response.ok) {
  throw new Error('Metrics ingest failed: ' + response.status + ' ' + (await response.text()))
}

const result = await response.json()
log('Stored ' + result.rowsAffected + ' metric rows')
return true`,
        notes: [
            'Batch the whole house into a single request rather than one request per device. The endpoint takes an array.',
            '`wattageW`, `state`, `metrics` and `source` are all optional; only `deviceId` and `ts` are required.',
            'A `deviceId` that does not belong to your home is rejected, so keep the mapping in sync after renaming devices.',
        ],
    },
    {
        id: 'homey-weather-observations',
        title: 'Log current weather',
        description:
            'Fetches the current conditions for your coordinates from Open-Meteo (no API key) and stores them alongside your energy data.',
        method: 'POST',
        path: '/weather/observations',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const LATITUDE = 59.9139
const LONGITUDE = 10.7522

const params = new URLSearchParams({
  latitude: String(LATITUDE),
  longitude: String(LONGITUDE),
  current: 'temperature_2m,relative_humidity_2m,cloud_cover,precipitation,wind_speed_10m,surface_pressure,weather_code,shortwave_radiation',
  wind_speed_unit: 'ms',
})

const weather = await fetch('https://api.open-meteo.com/v1/forecast?' + params).then((r) => r.json())
const current = weather.current

const payload = {
  homeId: HOME_ID,
  observations: [
    {
      ts: new Date(current.time + 'Z').toISOString(),
      provider: 'open-meteo',
      tempC: current.temperature_2m,
      cloudsPct: current.cloud_cover,
      rainMm1h: current.precipitation,
      windMs: current.wind_speed_10m,
      pressureHpa: current.surface_pressure,
      humidityPct: current.relative_humidity_2m,
      solarIrradianceWm2: current.shortwave_radiation,
      weatherCode: String(current.weather_code),
      raw: current,
    },
  ],
}

const response = await fetch(API_URL + '/weather/observations', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    Authorization: 'Bearer ' + WRITE_TOKEN,
  },
  body: JSON.stringify(payload),
})

if (!response.ok) {
  throw new Error('Weather ingest failed: ' + response.status + ' ' + (await response.text()))
}

log('Sent observation: ' + current.temperature_2m + ' degC')
return true`,
        notes: [
            'Only `ts` is required. Send whatever your source provides and leave the rest out.',
            '`raw` accepts any JSON, so you can keep the untouched provider response for later reprocessing.',
            'Hourly is plenty for observations. Running this every minute wastes both the provider quota and storage.',
        ],
    },
    {
        id: 'homey-weather-forecasts',
        title: 'Store a weather forecast run',
        description:
            'Pulls the next 48 hours from Open-Meteo and stores them as one forecast run, so you can compare predicted against actual production later.',
        method: 'POST',
        path: '/weather/forecasts',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const LATITUDE = 59.9139
const LONGITUDE = 10.7522
const HORIZON_HOURS = 48

const params = new URLSearchParams({
  latitude: String(LATITUDE),
  longitude: String(LONGITUDE),
  hourly: 'temperature_2m,cloud_cover,precipitation,wind_speed_10m,uv_index',
  forecast_days: '3',
  wind_speed_unit: 'ms',
})

const weather = await fetch('https://api.open-meteo.com/v1/forecast?' + params).then((r) => r.json())
const hourly = weather.hourly

// One run timestamp ties the whole batch together as a single forecast run.
const runTs = new Date().toISOString()
const runMs = Date.parse(runTs)

const forecasts = hourly.time
  .map((time, index) => {
    const validTs = new Date(time + 'Z')
    return {
      runTs,
      validTs: validTs.toISOString(),
      horizonHours: Math.round((validTs.getTime() - runMs) / 3_600_000),
      provider: 'open-meteo',
      tempC: hourly.temperature_2m[index],
      cloudsPct: hourly.cloud_cover[index],
      rainMm: hourly.precipitation[index],
      windMs: hourly.wind_speed_10m[index],
      uvi: hourly.uv_index[index],
    }
  })
  .filter((point) => point.horizonHours >= 0 && point.horizonHours <= HORIZON_HOURS)

const response = await fetch(API_URL + '/weather/forecasts', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    Authorization: 'Bearer ' + WRITE_TOKEN,
  },
  body: JSON.stringify({ homeId: HOME_ID, forecasts }),
})

if (!response.ok) {
  throw new Error('Forecast ingest failed: ' + response.status + ' ' + (await response.text()))
}

log('Sent ' + forecasts.length + ' forecast points')
return true`,
        notes: [
            '`runTs`, `validTs` and `horizonHours` are required on every point. `horizonHours` is an integer number of hours between the two timestamps.',
            'Keeping `runTs` identical across the batch is what makes it one run, which is how you compare successive runs later.',
            'Every provider refresh creates a new run. Four to six runs a day is a reasonable schedule.',
        ],
    },
]

const homeAssistantExamples: ScriptExample[] = [
    {
        id: 'ha-power',
        title: 'Send a power snapshot',
        description:
            'Function node that reads your energy sensors from Home Assistant state and builds the request for a downstream HTTP request node.',
        method: 'POST',
        path: '/power',
        language: 'javascript',
        code: `${CONFIG_HEADER}
// Entity ids from your Home Assistant instance.
const ENTITY_MAP = {
  price: 'sensor.electricity_price',
  power: 'sensor.power_consumption',
  solarPower: 'sensor.solar_power',
  lastMeterConsumption: 'sensor.energy_meter_consumption',
  lastMeterProduction: 'sensor.energy_meter_production',
  lastSolarTotal: 'sensor.solar_energy_total',
  consumptionSinceMidnight: 'sensor.energy_consumption_today',
  productionSinceMidnight: 'sensor.energy_production_today',
  solarSinceMidnight: 'sensor.solar_energy_today',
  costSinceMidnight: 'sensor.energy_cost_today',
}

// Requires "Enable global context store" on the Home Assistant server node.
const states = global.get('homeassistant').homeAssistant.states

// Every field on POST /power is required, so unknown states fall back to 0.
const read = (entityId) => {
  const value = Number(states[entityId]?.state)
  return Number.isFinite(value) ? value : 0
}

const payload = { homeId: HOME_ID, ts: new Date().toISOString(), currency: 'NOK' }
for (const [field, entityId] of Object.entries(ENTITY_MAP)) {
  payload[field] = read(entityId)
}

msg.method = 'POST'
msg.url = API_URL + '/power'
msg.headers = {
  'Content-Type': 'application/json',
  Authorization: 'Bearer ' + WRITE_TOKEN,
}
msg.payload = payload

return msg`,
        notes: [
            'Flow wiring: `inject` (repeat every minute) -> this `function` node -> `http request` node (method and URL taken from `msg`, return: parsed JSON).',
            'Sensors that are `unavailable` or `unknown` parse to `NaN`, which is why `read()` falls back to 0 rather than sending null.',
            '`currency` is a string such as `NOK` or `EUR`. Sending a number is rejected.',
        ],
    },
    {
        id: 'ha-devices',
        title: 'Register your devices',
        description:
            'Turns your power-measuring entities into API devices, then caches the returned ids in flow context for the metrics node.',
        method: 'POST',
        path: '/devices',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const states = global.get('homeassistant').homeAssistant.states

// Everything that measures power, plus every switch, becomes a device.
const entities = Object.values(states).filter((entity) => {
  const deviceClass = entity.attributes?.device_class
  return deviceClass === 'power' || entity.entity_id.startsWith('switch.')
})

msg.method = 'POST'
msg.url = API_URL + '/devices'
msg.headers = {
  'Content-Type': 'application/json',
  Authorization: 'Bearer ' + WRITE_TOKEN,
}
msg.payload = {
  homeId: HOME_ID,
  devices: entities.map((entity) => ({
    // Omit \`id\` on first run and the API assigns one.
    name: entity.attributes?.friendly_name ?? entity.entity_id,
    deviceType: entity.entity_id.split('.')[0],
    hasPowerMeter: entity.attributes?.device_class === 'power',
    isActive: entity.state !== 'unavailable',
  })),
}

// Remember which entity produced which device name so the response can be mapped back.
flow.set('entityIdByName', Object.fromEntries(
  entities.map((entity) => [entity.attributes?.friendly_name ?? entity.entity_id, entity.entity_id]),
))

return msg`,
        notes: [
            'Flow wiring: `inject` -> this `function` node -> `http request` node -> a second `function` node holding the snippet below.',
            'Second function node, to cache the ids the API assigned:\n`const byEntity = {}`\n`const entityIdByName = flow.get("entityIdByName")`\n`for (const device of msg.payload) byEntity[entityIdByName[device.name]] = device.id`\n`flow.set("deviceIdByEntity", byEntity)`',
            'Run this again after adding or renaming entities. Sending a device back with its `id` updates the existing row instead of creating a duplicate.',
        ],
    },
    {
        id: 'ha-device-metrics',
        title: 'Push per-device power readings',
        description:
            'Batches a reading for every registered entity into one request. Use with an inject node on a 1-5 minute interval.',
        method: 'POST',
        path: '/devices/metrics',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const states = global.get('homeassistant').homeAssistant.states

// Written by the "Register your devices" flow.
const deviceIdByEntity = flow.get('deviceIdByEntity') ?? {}
const ts = new Date().toISOString()

const metrics = Object.entries(deviceIdByEntity)
  .map(([entityId, deviceId]) => {
    const entity = states[entityId]
    if (!entity) {
      return null
    }

    const numeric = Number(entity.state)

    return {
      deviceId,
      ts,
      wattageW: Number.isFinite(numeric) ? numeric : null,
      state: Number.isFinite(numeric) ? null : entity.state,
      // Anything extra you want to keep goes in \`metrics\` as free-form JSON.
      metrics: {
        entityId,
        unit: entity.attributes?.unit_of_measurement ?? null,
      },
      source: 'home-assistant',
    }
  })
  .filter(Boolean)

if (metrics.length === 0) {
  return null
}

msg.method = 'POST'
msg.url = API_URL + '/devices/metrics'
msg.headers = {
  'Content-Type': 'application/json',
  Authorization: 'Bearer ' + WRITE_TOKEN,
}
msg.payload = { homeId: HOME_ID, metrics }

return msg`,
        notes: [
            'Returning `null` stops the message, so nothing is sent when there is nothing to report.',
            'Numeric sensors fill `wattageW`; everything else (`on`, `off`, `heating`) goes to `state`.',
            'For an event-driven variant, replace the inject node with `server-state-changed` and build a single-element `metrics` array from `msg.data.new_state`.',
        ],
    },
    {
        id: 'ha-weather-observations',
        title: 'Log current weather',
        description:
            "Reads Home Assistant's own weather entity and forwards the conditions, so no extra provider or API key is needed.",
        method: 'POST',
        path: '/weather/observations',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const WEATHER_ENTITY = 'weather.forecast_home'

const states = global.get('homeassistant').homeAssistant.states
const weather = states[WEATHER_ENTITY]

if (!weather) {
  node.error('Weather entity not found: ' + WEATHER_ENTITY, msg)
  return null
}

const attributes = weather.attributes

msg.method = 'POST'
msg.url = API_URL + '/weather/observations'
msg.headers = {
  'Content-Type': 'application/json',
  Authorization: 'Bearer ' + WRITE_TOKEN,
}
msg.payload = {
  homeId: HOME_ID,
  observations: [
    {
      ts: new Date().toISOString(),
      provider: 'home-assistant',
      tempC: attributes.temperature ?? null,
      cloudsPct: attributes.cloud_coverage ?? null,
      rainMm1h: attributes.precipitation ?? null,
      windMs: attributes.wind_speed != null ? attributes.wind_speed / 3.6 : null,
      pressureHpa: attributes.pressure ?? null,
      humidityPct: attributes.humidity ?? null,
      uvi: attributes.uv_index ?? null,
      weatherCode: weather.state,
      raw: attributes,
    },
  ],
}

return msg`,
        notes: [
            'Only `ts` is required, so omitting fields your entity does not expose is fine.',
            'Home Assistant reports wind in km/h by default while the API expects m/s, hence the `/ 3.6`. Check your unit system before trusting it.',
            '`weatherCode` is a string, which suits Home Assistant condition values such as `partlycloudy` or `rainy`.',
        ],
    },
    {
        id: 'ha-weather-forecasts',
        title: 'Store a weather forecast run',
        description:
            'Maps the response of the `weather.get_forecasts` service into one forecast run.',
        method: 'POST',
        path: '/weather/forecasts',
        language: 'javascript',
        code: `${CONFIG_HEADER}
const WEATHER_ENTITY = 'weather.forecast_home'
const HORIZON_HOURS = 48

// msg.payload comes from a "call service" node running weather.get_forecasts
// with type: hourly and "return response" enabled.
const forecast = msg.payload?.[WEATHER_ENTITY]?.forecast ?? []

// One run timestamp ties the whole batch together as a single forecast run.
const runTs = new Date().toISOString()
const runMs = Date.parse(runTs)

const forecasts = forecast
  .map((point) => {
    const validTs = new Date(point.datetime)
    return {
      runTs,
      validTs: validTs.toISOString(),
      horizonHours: Math.round((validTs.getTime() - runMs) / 3_600_000),
      provider: 'home-assistant',
      tempC: point.temperature ?? null,
      cloudsPct: point.cloud_coverage ?? null,
      rainMm: point.precipitation ?? null,
      windMs: point.wind_speed != null ? point.wind_speed / 3.6 : null,
      uvi: point.uv_index ?? null,
    }
  })
  .filter((point) => point.horizonHours >= 0 && point.horizonHours <= HORIZON_HOURS)

if (forecasts.length === 0) {
  return null
}

msg.method = 'POST'
msg.url = API_URL + '/weather/forecasts'
msg.headers = {
  'Content-Type': 'application/json',
  Authorization: 'Bearer ' + WRITE_TOKEN,
}
msg.payload = { homeId: HOME_ID, forecasts }

return msg`,
        notes: [
            'Flow wiring: `inject` -> `call service` (`weather.get_forecasts`, `type: hourly`, return response) -> this `function` node -> `http request` node.',
            '`runTs`, `validTs` and `horizonHours` are required on every point. `horizonHours` is an integer number of hours between the two timestamps.',
            'Each service call is a new run. Four to six runs a day gives you enough history to score forecast accuracy.',
        ],
    },
]

/** Copy-pasteable ingest examples, keyed by platform. */
export const scriptExamples: Record<IntegrationPlatform, ScriptExample[]> = {
    homey: homeyExamples,
    'home-assistant': homeAssistantExamples,
}

/** Tab labels and setup steps, keyed by platform. */
export const platformMeta: Record<IntegrationPlatform, PlatformMeta> = {
    homey: {
        label: 'Homey Pro',
        description:
            'HomeyScript runs JavaScript directly on your Homey Pro, with the whole device tree available through the `Homey` global.',
        setup: [
            'Open the HomeyScript app on your Homey and create a new script.',
            'Paste an example below and replace the three values at the top.',
            'Press Run once to check the output, then save the script.',
            'Create a Flow with a time trigger (for example every 5 minutes) and the "Run script" action.',
        ],
    },
    'home-assistant': {
        label: 'Home Assistant',
        description:
            'Home Assistant has no built-in JavaScript runtime, so these examples target the Node-RED add-on. Each snippet goes in a `function` node that feeds an `http request` node.',
        setup: [
            'Install the Node-RED add-on from Settings, Add-ons, and start it.',
            'Add a `function` node, paste an example, and replace the three values at the top.',
            'Wire it to an `http request` node with method and URL set to "- set by msg. -".',
            'Trigger the flow from an `inject` node on an interval, or from a `server-state-changed` node.',
        ],
    },
}

/** Platform tab order on the integrations page. */
export const platformOrder: IntegrationPlatform[] = ['homey', 'home-assistant']

/** Type guard for the `platform` search param. */
export function isIntegrationPlatform(
    value: string | undefined,
): value is IntegrationPlatform {
    return value === 'homey' || value === 'home-assistant'
}
