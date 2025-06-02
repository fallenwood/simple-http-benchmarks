# from gevent import monkey; monkey.patch_all()
import logging
from flask import Flask

app = Flask(__name__)
app.logger.setLevel(logging.ERROR)

@app.route("/")
def index():
    return "Hello, world!"

if __name__ == '__main__':
   from gevent.pywsgi import WSGIServer
   http_server = WSGIServer(("0.0.0.0", 5000), app, log=None)
   http_server.serve_forever()
