use crate::nodes::*;

#[derive(Debug)]
pub enum NodeType {
    TestNode(TestNode),
    Article(Article),
    Publisher(Publisher),
    Company(Company),
    Currency(Currency),
    Equity(Equity),
    Indices(Indices),
    ETFs(ETFs),
    Bonds(Bonds),
    Options(Options),
}

impl From<TestNode> for NodeType {
    fn from(test_node: TestNode) -> Self {
        NodeType::TestNode(test_node)
    }
}

impl From<Article> for NodeType {
    fn from(article: Article) -> Self {
        NodeType::Article(article)
    }
}

impl From<Publisher> for NodeType {
    fn from(publisher: Publisher) -> Self {
        NodeType::Publisher(publisher)
    }
}

impl From<Company> for NodeType {
    fn from(company: Company) -> Self {
        NodeType::Company(company)
    }
}

impl From<Currency> for NodeType {
    fn from(currency: Currency) -> Self {
        NodeType::Currency(currency)
    }
}

impl From<Equity> for NodeType {
    fn from(equity: Equity) -> Self {
        NodeType::Equity(equity)
    }
}

impl From<Indices> for NodeType {
    fn from(indices: Indices) -> Self {
        NodeType::Indices(indices)
    }
}

impl From<ETFs> for NodeType {
    fn from(etfs: ETFs) -> Self {
        NodeType::ETFs(etfs)
    }
}

impl From<Bonds> for NodeType {
    fn from(bonds: Bonds) -> Self {
        NodeType::Bonds(bonds)
    }
}

impl From<Options> for NodeType {
    fn from(options: Options) -> Self {
        NodeType::Options(options)
    }
}

#[derive(Debug)]
pub struct Article {
    pub(super) title: String,
    pub(super) summary: String,
    pub(super) sentiment: f64,
    pub(super) publisher: String,
    pub(super) tickers: Option<HashMap<String, f64>>,
}

#[derive(Debug)]
pub struct Publisher {
    pub(super) name: String,
    pub(super) sentiments: TimeSeries<f64>,
}

#[derive(Debug)]
pub struct Company {
    pub(super) name: String,
    pub(super) symbols: HashSet<String>,
    pub(super) income_statement: TimeSeries<IncomeStatement>,
    pub(super) balance_sheet: TimeSeries<BalanceSheet>,
    pub(super) cash_flow: TimeSeries<CashFlow>,
    pub(super) earnings: TimeSeries<Earnings>,
}

#[derive(Debug)]
pub struct Currency {
    pub(super) symbol: String,
    pub(super) history: TimeSeries<PriceAggregate>,
}

#[derive(Debug)]
pub struct Equity {
    pub(super) symbol: String,
    pub(super) history: TimeSeries<PriceAggregate>,
}

#[derive(Debug)]
pub struct Indices {
    pub(super) symbol: String,
    pub(super) history: TimeSeries<PriceAggregate>,
}

#[derive(Debug)]
pub struct ETFs {
    pub(super) symbol: String,
    pub(super) indice: String,
    pub(super) history: TimeSeries<PriceAggregate>,
}

#[derive(Debug)]
pub struct Bonds {
    pub(super) symbol: String,
    pub(super) interest_rate: f64,
    pub(super) maturity: Instant,
    pub(super) history: TimeSeries<PriceAggregate>,
}

#[derive(Debug)]
pub struct Options {
    pub(super) contract_id: String,
    pub(super) direction: String,
    pub(super) underlying: String,
    pub(super) strike: f64,
    pub(super) expiration: Instant,
    pub(super) history: TimeSeries<OptionsAggregate>,
}

impl Article {
    pub fn new(
        title: String,
        summary: String,
        sentiment: f64,
        publisher: String,
        tickers: Option<HashMap<String, f64>>,
    ) -> Self {
        Article {
            title,
            summary,
            sentiment,
            publisher,
            tickers,
        }
    }

    pub fn publisher(&self) -> &String {
        &self.publisher
    }

    pub fn ticker_sentiment(&self, symbol: String) -> Option<f64> {
        if let Some(tickers) = &self.tickers {
            tickers.get(&symbol).copied()
        } else {
            None
        }
    }

    pub fn ticker_intersect(&self, symbols: HashSet<String>) -> Option<HashSet<String>> {
        let tickers = self.tickers.as_ref()?; // Use `as_ref` to get a reference to the inner HashMap
        let ticker_keys: HashSet<String> = tickers.keys().cloned().collect();
        let intersection: HashSet<String> = symbols.intersection(&ticker_keys).cloned().collect();
        if intersection.is_empty() {
            None
        } else {
            Some(intersection)
        }
    }
}

impl Publisher {
    pub fn new(name: String, capacity: usize) -> Self {
        Publisher {
            name,
            sentiments: TimeSeries::new(capacity),
        }
    }
}

impl Currency {
    pub fn new(symbol: String, capacity: usize) -> Self {
        Currency {
            symbol,
            history: TimeSeries::new(capacity),
        }
    }

    pub fn mat(&self) -> Option<na::DMatrix<f64>> {
        self.history.mat()
    }
}

impl Equity {
    pub fn new(symbol: String, capacity: usize) -> Self {
        Equity {
            symbol,
            history: TimeSeries::new(capacity),
        }
    }

    pub fn mat(&self) -> Option<na::DMatrix<f64>> {
        self.history.mat()
    }
}

impl Indices {
    pub fn new(symbol: String, capacity: usize) -> Self {
        Indices {
            symbol,
            history: TimeSeries::new(capacity),
        }
    }

    pub fn mat(&self) -> Option<na::DMatrix<f64>> {
        self.history.mat()
    }
}

impl ETFs {
    pub fn new(symbol: String, indice: String, capacity: usize) -> Self {
        ETFs {
            symbol,
            indice,
            history: TimeSeries::new(capacity),
        }
    }

    pub fn indice(&self) -> &String {
        &self.indice
    }

    pub fn mat(&self) -> Option<na::DMatrix<f64>> {
        self.history.mat()
    }
}

impl Bonds {
    pub fn new(symbol: String, interest_rate: f64, maturity: Instant, capacity: usize) -> Self {
        Bonds {
            symbol,
            interest_rate,
            maturity,
            history: TimeSeries::new(capacity),
        }
    }

    pub fn interest_rate(&self) -> f64 {
        self.interest_rate
    }

    pub fn maturity(&self) -> &Instant {
        &self.maturity
    }

    pub fn mat(&self) -> Option<na::DMatrix<f64>> {
        self.history.mat()
    }
}

impl Options {
    pub fn new(
        contract_id: String,
        direction: String,
        underlying: String,
        strike: f64,
        expiration: Instant,
        capacity: usize,
    ) -> Self {
        Options {
            contract_id,
            direction,
            underlying,
            strike,
            expiration,
            history: TimeSeries::new(capacity),
        }
    }

    pub fn underlying(&self) -> &String {
        &self.underlying
    }

    pub fn direction(&self) -> &String {
        &self.direction
    }

    pub fn strike(&self) -> f64 {
        self.strike
    }

    pub fn expiration(&self) -> &Instant {
        &self.expiration
    }

    pub fn mat(&self) -> Option<na::DMatrix<f64>> {
        self.history.mat()
    }
}

impl Company {
    pub fn new(name: String, symbols: HashSet<String>, capacity: usize) -> Self {
        Company {
            name,
            symbols,
            income_statement: TimeSeries::new(capacity),
            balance_sheet: TimeSeries::new(capacity),
            cash_flow: TimeSeries::new(capacity),
            earnings: TimeSeries::new(capacity),
        }
    }

    pub fn symbols(&self) -> HashSet<String> {
        self.symbols.clone()
    }

    pub fn update_income_statement(&mut self, index: Instant, item: IncomeStatement) {
        DynamicNode::<Instant, IncomeStatement>::update(self, index, item);
    }
}
