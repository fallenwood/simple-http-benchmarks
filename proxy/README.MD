Add `127.0.0.1 pingora.example` to `/etc/hosts`
```
docker built ../dotnet/jit -t backend:dotnet
cd pingora
docker compose up
wrknet -c 125 -d 60 http://pingora.example:8080/
```