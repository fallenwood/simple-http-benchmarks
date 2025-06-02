from django.http import HttpResponse


def index(request):
    return HttpResponse(content=b"Hello, world!")
