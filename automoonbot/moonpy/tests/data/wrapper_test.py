import pytest
from automoonbot.moonpy.data import HeteroGraphWrapper


def test_basics():
    wrapper = HeteroGraphWrapper()
    assert wrapper is not None

    wrapper.add_article("title", "summary", 1.0, "publisher", 1, {})
    assert wrapper.node_count() == 2
    assert wrapper.edge_count() == 1

    wrapper.add_equity("symbol", "", 1)
    assert wrapper.node_count() == 3
    assert wrapper.edge_count() == 1

    wrapper.add_currency("foo", 1)
    assert wrapper.node_count() == 4
    assert wrapper.edge_count() == 1


def test_update_equity():
    wrapper = HeteroGraphWrapper()

    wrapper.add_equity("foo", "", 5)
    wrapper.update_equity("foo", 0, 10, True, 100, 99, 101, 98, 10)
    assert wrapper.node_count() == 1
    assert wrapper.edge_count() == 0

    wrapper.add_equity("bar", "", 5)
    assert wrapper.node_count() == 2
    assert wrapper.edge_count() == 0

    wrapper.update_equity("bar", 0, 10, True, 100, 99, 101, 98, 10)
    assert wrapper.node_count() == 2
    assert wrapper.edge_count() == 2


def test_update_currency():
    wrapper = HeteroGraphWrapper()

    wrapper.add_currency("foo", 1)
    wrapper.update_currency("foo", 0, 10, True, 100, 99, 101, 98, 10)
    assert wrapper.node_count() == 1
    assert wrapper.edge_count() == 0
