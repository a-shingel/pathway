import pytest
from utils import (
    DebeziumContext,
    MongoDBContext,
    PgvectorContext,
    PostgresContext,
    QuestDBContext,
)


@pytest.fixture
def postgres():
    return PostgresContext()


@pytest.fixture
def pgvector():
    return PgvectorContext()


@pytest.fixture
def questdb():
    return QuestDBContext()


@pytest.fixture
def mongodb():
    return MongoDBContext()


@pytest.fixture
def debezium():
    return DebeziumContext()
