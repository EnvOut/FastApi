import json

import psycopg2
from sqlalchemy.engine import Engine
from sqlalchemy.util import namedtuple

from api_provider.base_provider import BaseProvider


class PostgresProvider(BaseProvider):

    def __init__(self, name: str = None,
                 user: str = None, password: str = None, url: str = 'localhost',
                 db: str = None, port: int = 5432, allowed: dict = None, **config):
        def __init_alchemy_connection() -> Engine:
            import sqlalchemy

            session = sqlalchemy.create_engine(f"postgresql+psycopg2://{user}:{password}@{url}:{port}/{db}")
            # sessions = scoped_session(sessionmaker(bind=engine))

            # s = Session()
            # result = s.execute('SELECT * FROM my_table WHERE my_column = :val', {'val': 5})
            return session

        def __init_psycopg2_connection():
            return psycopg2.connect(f"postgresql+psycopg2://{user}:{password}@{url}:{port}/{db}")

        self.name = name
        self.__session = __init_alchemy_connection()

    def is_valid(self):
        return True

    @staticmethod
    def __prepare_sql(base_part: str, params: dict, is_single: bool) -> str:
        # make params mutable
        params = {**params}

        offset = params.pop('offset', 0)
        limit = params.pop('limit', 10)

        where_part = None
        if len(params.keys()) > 0:
            rr = [f"{k} = '{v}'" if type(v) is str else f"{k} = {v}" for k, v in params.items()]

            where_part = " WHERE " + " and ".join(rr)

        paging_part = f'OFFSET {offset} LIMIT {limit}'

        sql = f'{base_part} {where_part} {paging_part}'
        return sql

    def execute(self, sql: str = None, is_single=True, request=None, **config):
        if not sql:
            raise ValueError(f"The sql query shouldn't be empty")

        params = request.args

        if not params:
            params = {}

        prepared_sql = self.__prepare_sql(base_part=sql, params=params, is_single=is_single)
        execute = self.__session.execute(prepared_sql)

        def to_result():
            QueryEntity = namedtuple('response', execute.keys())

            result = None
            if is_single:
                # items = execute.items()
                result = QueryEntity(*execute.fetchone())._asdict()
            else:
                # all =execute.fetchall()
                result = [QueryEntity(*i)._asdict() for i in execute.fetchall()]

            return result

        data = to_result()
        return json.dumps(data)
