from pprint import pprint
from unittest import TestCase
from providers import YamlProvider


class TestYamlProvider(TestCase):
    def test_read(self):

        provider = YamlProvider(r'../config.yml')
        provider.read()

        pprint(provider.endpoints)

        assert provider.endpoints != []
        assert provider.providers != []
        # self.fail()
