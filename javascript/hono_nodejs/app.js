const { serve } = require('@hono/node-server');
const { Hono } = require('hono');

const app = new Hono();

app.get('/', (c) => {
  return c.text('Hello World!')
});

const port = 5000;

serve({
  fetch: app.fetch,
  port
});
