import ctypes
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from . import settings

app = FastAPI()

origins = [
    "http://127.0.0.1:8080",
]

app.add_middleware(
    CORSMiddleware,
    **settings.MIDDLEWARE_SETTINGS
)

main_so_file = "../Spyrals-c/main.so"
clibrary = ctypes.CDLL(main_so_file)

@app.get("/")
async def root():
    var1 = "Hello"
    var2 = "World"

    hello_world = clibrary.hello_world
    hello_world.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
    hello_world.restype = ctypes.c_char_p
    msg = hello_world(var1.encode(), var2.encode()).decode()
    print(msg)
    return [{"message": msg}]