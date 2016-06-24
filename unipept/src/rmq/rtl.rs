
use taxon::{TaxonId, Taxon};
use agg::{Aggregator, count};

pub struct RTLCalculator {
    pub ancestors: Vec<Option<TaxonId>>,
}

impl RTLCalculator {
    pub fn new(taxons: &Vec<Option<Taxon>>) -> Self {
        let mut ancestors: Vec<Option<TaxonId>> = taxons
            .iter()
            .map(|mtaxon| mtaxon.as_ref().map(|t| t.parent))
            .collect();
        ancestors[1] = None;

        RTLCalculator {
            ancestors: ancestors
        }
    }
}

impl Aggregator for RTLCalculator {
    fn aggregate(&self, taxons: &Vec<TaxonId>) -> Result<TaxonId, String> {
        let counts         = count(taxons);
        let mut rtl_counts = counts.clone();
        for (taxon, count) in rtl_counts.iter_mut() {
            let mut next = *taxon;
            while let Some(ancestor) = self.ancestors[next] {
                *count += *counts.get(&ancestor).unwrap_or(&0);
                next = ancestor;
            }
        }

        rtl_counts.iter()
                  .max_by_key(|&(_, count)| count)
                  .map(|tup| *tup.0)
                  .ok_or("Aggregation called on empty list.".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::RTLCalculator;
    use agg::Aggregator;
    use fixtures;


    #[test]
    fn test_all_on_same_path() {
        let calculator = RTLCalculator::new(&fixtures::by_id());
        assert_eq!(Ok(1), calculator.aggregate(&vec![1]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![1, 12884]));
        assert_eq!(Ok(185751), calculator.aggregate(&vec![1, 12884, 185751]));
    }

    #[test]
    fn favouring_root() {
        let calculator = RTLCalculator::new(&fixtures::by_id());
        assert_eq!(Ok(185751), calculator.aggregate(&vec![1, 1, 1, 185751, 1, 1]));
    }

    #[test]
    fn leaning_close() {
        let calculator = RTLCalculator::new(&fixtures::by_id());
        assert_eq!(Ok(185751), calculator.aggregate(&vec![1, 1, 185752, 185751, 185751, 1]));
    }

    #[test]
    fn non_deterministic() {
        let calculator = RTLCalculator::new(&fixtures::by_id());
        assert!(vec![Ok(185751), Ok(185752)].contains(&calculator.aggregate(&vec![1, 1, 185752, 185751, 1])))
    }
}
