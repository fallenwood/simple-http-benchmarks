package main

import (
  "net/http"

  "github.com/gin-gonic/gin"
)

func main() {
  gin.SetMode(gin.ReleaseMode)
  r := gin.New()
  r.GET("/", func(c *gin.Context) {
    c.String(http.StatusOK, "Hello World!")
  })
  r.Run(":5000")
}
