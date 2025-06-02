from litestar import Litestar, get
from litestar.logging import LoggingConfig

@get("/")
async def index() -> str:
    return "Hello, World!"

logging_config = LoggingConfig(
    disable_existing_loggers = True
)

app = Litestar(route_handlers=[index], logging_config=logging_config)
