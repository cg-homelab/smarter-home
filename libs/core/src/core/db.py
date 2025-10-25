import core.models  # noqa: F401
from os import getenv
from sqlmodel import SQLModel, create_engine, Session

database_url = getenv("DATABASE_URL", "sqlite:///app.db")
engine = create_engine(database_url)
SQLModel.metadata.create_all(engine)


def get_session():
    with Session(engine) as session:
        yield session
