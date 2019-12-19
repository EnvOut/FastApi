from functools import reduce

http_methods = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE']

class BaseProvider:
    def is_valid(self): pass

    # def call_method(self): pass

    def execute(self, params, **kwargs): pass


class AbstractAllowedParametersChecker:
    def __init__(self, allowed_parameters):
        self.allowed_parameters = allowed_parameters

    def is_allowed(self, method: str) -> bool:
        pass

    def is_all_allowed(self, params: list) -> bool:
        return reduce(lambda x, y: True if x is False or y is False else False,
                      filter(lambda param: not self.is_allowed(param), params))


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
