from flask import Flask
from tornado.netutil import bind_sockets
from tornado.tcpserver import TCPServer
from tornado.wsgi import WSGIContainer
from tornado.httpserver import HTTPServer
from tornado.ioloop import IOLoop
from tornado.process import fork_processes
import asyncio
import tornado
import tornado.process

app = Flask(__name__)

@app.route("/")
def index():
    return "Hello, world!"

async def devmain():
    container = WSGIContainer(app)
    http_server = HTTPServer(container)
    http_server.listen(5000)
    await asyncio.Event().wait()

def main():
    sockets = bind_sockets(5000)
    tornado.process.fork_processes(0)
    async def post_fork_main():
        container = WSGIContainer(app)
        http_server = HTTPServer(container)
        http_server.add_sockets(sockets)
        await asyncio.Event().wait()
    asyncio.run(post_fork_main())

if __name__ == "__main__":
    main()
