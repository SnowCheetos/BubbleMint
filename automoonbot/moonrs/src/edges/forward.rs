use crate::edges::*;

pub trait ForwardDynEdge<S, T, Ix, X>: StaticEdge
where
    S: StaticNode,
    T: DynamicNode<Ix, X>,
    Ix: Clone + Hash + Eq + PartialOrd,
    X: Clone,
{
    fn fowrard_corr(&self, src: &S, tgt: &T) -> Option<na::DMatrix<f64>>;
    fn update(&mut self, src: &S, tgt: &T);
}

impl ForwardDynEdge<Article, Company, Instant, FinancialStatement> for Mentioned {
    fn fowrard_corr(&self, src: &Article, tgt: &Company) -> Option<na::DMatrix<f64>> {
        let tgt_mat = tgt.mat()?;
        let tgt_std = tgt_mat.row_variance().map(|var| var.sqrt());
        let sentiment = src.sentiment();
        let ret = tgt_std / sentiment;
        Some(na::DMatrix::from_rows(&vec![ret]))
    }

    fn update(&mut self, src: &Article, tgt: &Company) {
        todo!()
    }
}

impl ForwardDynEdge<Article, Equity, Instant, PriceAggregate> for Referenced {
    fn fowrard_corr(&self, src: &Article, tgt: &Equity) -> Option<na::DMatrix<f64>> {
        let tgt_mat = tgt.mat()?;
        let tgt_std = tgt_mat.row_variance().map(|var| var.sqrt());
        let sentiment = src.ticker_sentiment(tgt.name().to_string())?;
        let ret = tgt_std / sentiment;
        Some(na::DMatrix::from_rows(&vec![ret]))
    }

    fn update(&mut self, src: &Article, tgt: &Equity) {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mentioned() {
        let article = Article::new(
            "foo".to_owned(),
            "foo sumary".to_owned(),
            0.5,
            "".to_owned(),
            Some(HashMap::from([("boo".to_owned(), 0.5)])),
        );
        let mut company = Company::new("bar".to_owned(), vec!["boo".to_owned()], 10);
        let src_index = NodeIndex::new(0);
        let tgt_index = NodeIndex::new(1);
        let edge = Mentioned::try_new(src_index, tgt_index, &article, &company);
        assert!(edge.is_some());

        let edge = edge.unwrap();
        let corr = edge.fowrard_corr(&article, &company);
        assert!(corr.is_none());

        let now = Instant::now();

        company.update_balance_sheet(
            now,
            BalanceSheet::new(
                Instant::now(),
                Duration::from_secs(10),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ),
        );

        company.update_cash_flow(
            now,
            CashFlow::new(
                Instant::now(),
                Duration::from_secs(10),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ),
        );

        company.update_earnings(
            now,
            Earnings::new(
                now,
                Duration::from_secs(10),
                true,
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ),
        );

        company.update_income_statement(
            now,
            IncomeStatement::new(
                now,
                Duration::from_secs(10),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ),
        );

        let corr = edge.fowrard_corr(&article, &company);
        assert!(corr.is_some());
    }
}
