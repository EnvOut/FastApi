from api_provider.base_provider import BaseProvider
from api_provider.proxy_provider import ProxyProvider
from api_provider.rdbms_provider import PostgresProvider


class ProviderFactory:
    def __init__(self, name: str, definition): pass

    def is_valid(self) -> bool: return False

    def get_provider(self) -> BaseProvider: pass


class PostgresProviderFactory(ProviderFactory):
    __type = 'postgres'

    def __init__(self, name: str, definition):
        self.__definition = definition
        self.__name = name

    def is_valid(self):
        return True if self.__definition['type'] == self.__type \
            else False

    def get_provider(self) -> BaseProvider:
        return self.__to_provider(self.__definition)

    def __to_provider(self, config: dict):
        return PostgresProvider(name=self.__name, **config)


class ProxyProviderFactory(ProviderFactory):
    __type = 'proxy'

    def __init__(self, name: str, definition):
        self.__definition = definition
        self.__name = name

    def is_valid(self):
        return True if self.__definition['type'] == self.__type \
            else False

    def get_provider(self) -> BaseProvider:
        return self.__to_provider(**self.__definition)

    def __to_provider(self, path: str = None, schema='https',
                      allowed=None, **options):
        return ProxyProvider(name=self.__name, path=path, schema=schema,
                             allowed=allowed, options=options)


class DefaultProviderFactory(ProviderFactory):
    def __init__(self, name: str, definition):
        self.__name = name
        self.__proxy_factory = ProxyProviderFactory(name, definition)
        self.__postgres_factory = PostgresProviderFactory(name, definition)

    def is_valid(self) -> bool:
        return self.__proxy_factory.is_valid() or self.__postgres_factory.is_valid()

    def get_provider(self) -> BaseProvider:
        provider: ProviderFactory
        if self.__proxy_factory.is_valid():
            provider = self.__proxy_factory
        elif self.__postgres_factory.is_valid():
            provider = self.__postgres_factory
        else:
            raise NotImplementedError(f"Not implemented factory for the provider: {self.__name}")

        return provider.get_provider()
