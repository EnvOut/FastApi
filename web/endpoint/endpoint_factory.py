from api_provider.base_provider import http_methods
from web.api_generator import Endpoint


class EndpointFactory:
    def is_valid(self): pass

    def get_endpoint(self): pass


class CommonEndpointFactory(EndpointFactory):
    def __init__(self, providers: dict, name: str = None, definition: dict = None):
        self.__name = name
        self.__definition = definition
        self.__providers = providers

    def __is_name_valid(self) -> bool:
        return bool(self.__name)

    def __is_definition_valid(self) -> bool:
        def is_valid_properties(provider_map: dict = None, provider=None,
                                path: str = None,
                                method: str = 'GET') -> bool:
            is_valid_method = method in http_methods
            is_valid_path = bool(path) and path.startswith('/')

            def is_valid_common_provider(ref) -> bool:
                return bool(ref) and provider_map[ref]

            def is_valid_configured_provider(ref=None, **config) -> bool:
                """base implementation of the configured provider validation.
                In the future should be available to match config to provider supported methods"""
                return bool(ref) and provider_map[ref]

            is_valid_provider = is_valid_common_provider(provider) if type(provider) is str \
                else is_valid_configured_provider(**provider)

            return is_valid_method or is_valid_path and is_valid_provider

        return bool(self.__definition) and is_valid_properties(**self.__definition, provider_map=self.__providers)

    def is_valid(self) -> bool:
        return self.__is_name_valid() and self.__is_definition_valid()

    def create_endpoint(self) -> Endpoint:
        provider_definition = self.__definition['provider']
        path = self.__definition['path']
        method = self.__definition.get('method', 'GET')

        def create(provider=None) -> Endpoint:
            handler = None
            if type(provider) == str:
                provider_ref = provider
                provider = self.__providers[provider_ref]

                def handle_request(req):
                    return provider.execute(config=req)

                handler = handle_request
            elif type(provider) == dict:
                provider_ref = provider['ref']
                provider = self.__providers[provider_ref]

                def handle_request(req):
                    return provider.execute(request=req, **provider_definition)
                    # return provider.execute(config=**{**req.__dict__, **provider_definition})

                # handler = lambda req: self.__providers[provider_ref].execute({**req, **provider_definition})
                handler = handle_request

            return Endpoint(name=self.__name, path=path, handler=handler, method=method)

        endpoint = create(provider=provider_definition)
        return endpoint
