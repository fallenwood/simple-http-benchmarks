var builder = WebApplication.CreateSlimBuilder(args);

var app = builder.Build();

app.MapGet("/", static context => context.Response.WriteAsync("Hello world!"));

await app.RunAsync();
