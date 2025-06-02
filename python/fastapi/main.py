from fastapi import FastAPI
import logging

logging.getLogger("uvicorn").propagate = False

app = FastAPI()

@app.get("/")
def index():
    return "Hello World"
