use git2::Repository;

pub struct DiffCalculator {
    pub repo: Repository,
}
impl DiffCalculator {
    pub fn new(repo: Repository) -> DiffCalculator {
        DiffCalculator { repo }
    }

    pub fn diff<'repo>(
        &'repo self,
        old_oid: &'repo git2::Oid,
        new_oid: &'repo git2::Oid,
    ) -> Result<git2::Diff<'repo>, git2::Error> {
        let old_tree = self.repo.find_tree(*old_oid)?;
        let new_tree = self.repo.find_tree(*new_oid)?;
        let dif = self
            .repo
            .diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;
        Ok(dif)
    }
}
