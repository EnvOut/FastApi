http_methods = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', 'OPTIONS']


class BaseProvider:
    def is_valid(self): pass

    # def call_method(self): pass

    def execute(self, **config): pass


class AbstractAllowedParametersChecker:
    def __init__(self, allowed_parameters):
        self.allowed_parameters = allowed_parameters

    def is_allowed(self, method: str) -> bool:
        raise NotImplemented()

    def is_all_allowed(self, params: list) -> bool:
        not_allowed_params = [p for p in params if not self.is_allowed(p)]
        # filtered = [p for p in params if self.is_allowed(p)]
        # filtered = filter(lambda param: not self.is_allowed(param), params)
        return len(not_allowed_params) == 0


class AnyAllowedParametersChecker(AbstractAllowedParametersChecker):
    def __init__(self):
        super().__init__(http_methods)

    def is_allowed(self, method) -> bool:
        return True


class NoAllowedParametersChecker(AbstractAllowedParametersChecker):
    def __init__(self):
        super().__init__([])

    def is_allowed(self, method) -> bool:
        return True


class AllowedParametersChecker(AbstractAllowedParametersChecker):

    def is_allowed(self, param) -> bool:
        return param in self.allowed_parameters
