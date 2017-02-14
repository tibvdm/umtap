//! Allows calculating the Lowest Common Ancestor (LCA).

use std::ops::Add;

use agg;
use taxon::{TaxonId, TaxonList};
use tree::tree::SubTree;

/// Struct capable of calculating the LCA of 2 nodes in a TaxonTree.
pub struct LCACalculator {
    /// The root of the taxon tree.
    pub root: TaxonId,
    /// Contains the ancestor for each node. Nodes are indexed by their id.
    pub parents: Vec<Option<TaxonId>>,
}

impl LCACalculator {
    /// Constructs an LCACalculator for a given taxon tree.
    ///
    /// # Arguments:
    /// * `root`     - the root of the taxon tree.
    /// * `taxonomy` - the taxons, indexed by their id.
    pub fn new(root: TaxonId, taxonomy: &TaxonList) -> Self {
        LCACalculator {
            root:    root,
            parents: taxonomy.ancestry(),
        }
    }
}

impl agg::Aggregator for LCACalculator {
    fn aggregate(&self, taxons: &Vec<TaxonId>) -> Result<TaxonId, agg::Error> {
        if taxons.len() == 0 { return Err(agg::Error::EmptyInput); }
        let counts  = agg::count(taxons);
        let subtree = try!(SubTree::new(self.root, &self.parents, counts)).collapse(&Add::add);
        Ok(subtree.root)
    }
}

#[cfg(test)]
mod tests {
    use super::LCACalculator;
    use agg::Aggregator;
    use fixtures;

    #[test]
    fn test_two_on_same_path() {
        let calculator = LCACalculator::new(fixtures::tree().root, &fixtures::by_id());
        assert_eq!(Ok(185752), calculator.aggregate(&vec![12884, 185752]));
        assert_eq!(Ok(185752), calculator.aggregate(&vec![185752, 12884]));
        assert_eq!(Ok(2), calculator.aggregate(&vec![1, 2]));
        assert_eq!(Ok(2), calculator.aggregate(&vec![2, 1]));
    }

    #[test]
    fn test_two_on_fork() {
        let calculator = LCACalculator::new(fixtures::tree().root, &fixtures::by_id());
        assert_eq!(Ok(1), calculator.aggregate(&vec![2, 10239]));
        assert_eq!(Ok(1), calculator.aggregate(&vec![10239, 2]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![185751, 185752]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![185752, 185751]));
    }

    #[test]
    fn test_three_on_triangle() {
        let calculator = LCACalculator::new(fixtures::tree().root, &fixtures::by_id());
        assert_eq!(Ok(12884), calculator.aggregate(&vec![12884, 185751, 185752]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![12884, 185752, 185751]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![185751, 12884, 185752]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![185752, 12884, 185751]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![185751, 185752, 12884]));
        assert_eq!(Ok(12884), calculator.aggregate(&vec![185752, 185751, 12884]));
    }
}
