const express = require('express');
const favicon = require('express-favicon');
const path = require('path');
const prometheus = require('express-prom-bundle')

const log4js = require("log4js");
log4js.configure({
  appenders: { server: { type: "stdout" } },
  categories: { default: { appenders: ["server"], level: "info" } }
});
const logger = log4js.getLogger("server");

// This will create the /metrics endpoint for you and expose Node default
// metrics.
const metricsMiddleware = prometheus({
  includeMethod: true,
  includePath: true,
  promClient: { collectDefaultMetrics: {} }
})

const port = 3000;

const app = express();
app.use(favicon(__dirname + '/build/favicon.ico'));

// the __dirname is the current directory from where the script is running
app.use(express.static(__dirname));
app.use(express.static(path.join(__dirname, 'build')));
app.use(metricsMiddleware);

app.get('/*', function (req, res) {
  res.sendFile(path.join(__dirname, 'build', 'index.html')); // <-- change if not using index.html
});

app.listen(port, () => {
  logger.info(`Starting server on port ${port}....`);
});