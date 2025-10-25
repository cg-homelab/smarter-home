from sqlmodel import SQLModel, Field, Relationship


class User(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    username: str
    email: str
    created_at: str
    modified_at: str
    homes: list["Home"] = Relationship(
        back_populates="users", link_model="UserHomeLink"
    )


class Home(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    address: str
    created_at: str
    modified_at: str
    users: list[User] = Relationship(back_populates="homes", link_model="UserHomeLink")


class UserHomeLink(SQLModel, table=True):
    user_id: int = Field(foreign_key="user.id", primary_key=True)
    home_id: int = Field(foreign_key="home.id", primary_key=True)


class PowerMetric(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    home_id: int = Field(foreign_key="home.id")
    timestamp: str
    power_usage: float
