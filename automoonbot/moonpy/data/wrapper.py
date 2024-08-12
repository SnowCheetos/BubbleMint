from torch_geometric.data import HeteroData
from typing import Dict

from moonrs import HeteroGraph
from automoonbot.moonpy.data.api import AlphaVantage


class HeteroGraphWrapper(HeteroGraph):
    def __init__(self) -> None:
        super().__init__()

    def node_count(self) -> int:
        return super().node_count()

    def edge_count(self) -> int:
        return super().edge_count()

    def reset(self) -> None:
        self.clear()

    def to_pyg(self) -> HeteroData:
        ret = HeteroData()
        data = super().to_pyg()

    def remove_node(self, name: str) -> None:
        super().remove_node(name)

    def add_article(
        self,
        title: str,
        summary: str,
        sentiment: float,
        publisher: str,
        capacity: int,
        tickers: Dict[str, float],
    ) -> None:
        super().add_article(title, summary, sentiment, publisher, tickers)
        if len(publisher) > 0 and not super().has_node(publisher):
            super().add_publisher(publisher, capacity)

    def add_publisher(self, name: str, capacity: int) -> None:
        super().add_publisher(name, capacity)

    def add_equity(self, symbol: str, company: str, capacity: int) -> None:
        super().add_equity(symbol, company, capacity)
        if len(company) > 0 and not super().has_node(company):
            # symbols = self.av.get_symbols(company)
            # if not symbols["ok"]:
            #     return
            # super().add_company(company, [s["1. symbol"] for s in symbols["bestMatches"]])
            pass

    def add_currency(self, symbol: str, capacity: int) -> None:
        super().add_currency(symbol, capacity)

    def add_bond(
        self, symbol: str, interest_rate: float, maturity: float, capacity: int
    ) -> None:
        super().add_bond(symbol, interest_rate, maturity, capacity)

    def add_option(self, symbol: str, strike: float, capacity: int) -> None:
        super().add_option(symbol, strike, capacity)

    def update_currency(
        self,
        symbol: str,
        timestamp: float,
        duration: float,
        adjusted: bool,
        open: float,
        close: float,
        high: float,
        low: float,
        volume: float,
    ) -> None:
        super().update_currency(
            symbol, timestamp, duration, adjusted, open, close, high, low, volume
        )

    def update_equity(
        self,
        symbol: str,
        timestamp: float,
        duration: float,
        adjusted: bool,
        open: float,
        close: float,
        high: float,
        low: float,
        volume: float,
    ) -> None:
        super().update_equity(
            symbol, timestamp, duration, adjusted, open, close, high, low, volume
        )
