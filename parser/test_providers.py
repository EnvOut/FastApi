from pprint import pprint
from unittest import TestCase

from parser.providers import YamlConfigProvider


class TestYamlProvider(TestCase):
    def test_read(self):

        provider = YamlConfigProvider(r'config.yml')
        provider.read()

        pprint(provider.endpoints)

        assert provider.endpoints != []
        assert provider.providers != []
        # self.fail()
