import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello World!')
})

export default {
  port: 5000,
  fetch: app.fetch,
};
