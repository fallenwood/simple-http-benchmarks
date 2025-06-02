#include <crow.h>

int main() {
  crow::SimpleApp app;

  app.loglevel(crow::LogLevel::Warning);

  CROW_ROUTE(app, "/")([]() { return "Hello, World!"; });
  CROW_ROUTE(app, "/health")([]() { return "Healthy"; });

  app.port(5000).multithreaded().run();
}
