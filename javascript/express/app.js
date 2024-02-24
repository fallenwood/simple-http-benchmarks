const express = require("express");

const app = express();

app.get("/", (req, res) => {
  res.end("Hello World");
});

app.listen(5000, "0.0.0.0", () => {
    console.log("Started");
});