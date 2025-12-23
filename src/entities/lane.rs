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
        // 1️⃣ หา lane ที่รอ commit นี้
        let mut lane = self
            .lanes
            .iter()
            .position(|slot| slot.as_ref() == Some(commit_oid));

        // 2️⃣ ถ้าไม่เจอ → สร้าง lane ใหม่
        let lane = match lane {
            Some(i) => i,
            None => {
                self.lanes.push(None);
                self.lanes.len() - 1
            }
        };

        // 3️⃣ clear lane ปัจจุบัน (commit consume แล้ว)
        self.lanes[lane] = None;

        // 4️⃣ parent ตัวแรก → ใช้ lane เดิม
        if let Some(first_parent) = parent_oids.first() {
            self.lanes[lane] = Some(*first_parent);
        }

        // 5️⃣ parent ที่เหลือ → เปิด lane ใหม่ (merge)
        for parent in parent_oids.iter().skip(1) {
            self.lanes.push(Some(*parent));
        }

        lane
    }
}
