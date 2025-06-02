import asyncio
import tornado
import tornado.process

class MainHandler(tornado.web.RequestHandler):
    def get(self):
        self.write("Hello, world!")

def main():
    tornado.process.fork_processes(0)
    async def post_fork_main():
        app = tornado.web.Application([
            (r"/", MainHandler),
        ])
        app.listen(5000, reuse_port=True)
        await asyncio.Event().wait()
    asyncio.run(post_fork_main())

if __name__ == "__main__":
    main()
