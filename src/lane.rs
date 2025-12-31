use git2::Oid;

#[derive(Debug, Clone)]
pub struct LaneManager {
    pub lanes: Vec<Option<Oid>>,
}

impl LaneManager {
    pub fn new() -> Self {
        Self { lanes: Vec::new() }
    }

    pub fn get_lanes(&self) -> &[Option<Oid>] {
        &self.lanes
    }

    /// assign commit to a lane and update lanes for parents
    pub fn assign_commit(&mut self, commit_oid: &Oid, parent_oids: &[Oid]) -> usize {
        let lane = match self
            .lanes
            .iter()
            .position(|slot| slot.as_ref() == Some(commit_oid))
        {
            Some(i) => i,
            None => {
                self.lanes.push(None);
                self.lanes.len() - 1
            }
        };

        self.lanes[lane] = None;

        let mut none_lane: Vec<usize> = self
            .lanes
            .iter()
            .enumerate()
            .filter_map(|(i, l)| if l.is_none() { Some(i) } else { None })
            .collect();

        for parent in parent_oids {
            if self.lanes.contains(&Some(*parent)) {
                continue;
            }
            if let Some(position) = none_lane.pop() {
                self.lanes[position] = Some(*parent);
            } else {
                self.lanes.push(Some(*parent));
            }
        }

        while matches!(self.lanes.last(), Some(None)) {
            self.lanes.pop();
        }

        lane
    }
}
