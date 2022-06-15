const express = require('express');
const path = require('path');
const prometheus = require('express-prom-bundle')


const metrics = prometheus({
  includeMethod: true,
  includePath: true,
  promClient: { collectDefaultMetrics: {} },
  includeUp: false,
  httpDurationMetricName: 'nodejs_http_request_duration_seconds',
  buckets: [0.25, 0.5, 0.75, 1, 1.5, 2, 2.5, 5, 10]
})
const port = 3000;

const app = express();
app.use(metrics);
app.use(express.static(path.join(__dirname, 'build')))

app.listen(port, () => {
  console.log(`Starting server on port ${port}....`);
});