// Deploy-time rendered. Committed values are safe local-dev defaults.
// Overwrite at container/entrypoint start with the real API origin; see .env.example.
/* eslint-disable unicorn/no-global-object-property-assignment -- the runtime config global is the deploy-time contract consumed by the app bundle. */
globalThis.__FLORA_CONFIG__ = {
	apiBaseUrl: "http://localhost:3000",
	wsBaseUrl: null
};
