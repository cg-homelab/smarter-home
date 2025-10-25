from typing import Union
from core import db

from fastapi import FastAPI


app = FastAPI()


@app.on_event("startup")
def on_startup():
    db.init_db()


@app.get("/")
async def read_root():
    return {"Hello": "World"}


@app.get("/items/{item_id}")
async def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}
