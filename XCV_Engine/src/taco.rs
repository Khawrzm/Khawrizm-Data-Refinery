// taco.rs
// TACO (Tabular Locality-based Compression) architecture for formula dependency DAGs.
// Represents ranges and cell dependencies using 2D Bounding Boxes and spatial indexing.

use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellCoord {
    pub col: u32,
    pub row: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellCoord3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox3D {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
}

impl BoundingBox3D {
    pub fn new(c1: CellCoord3D, c2: CellCoord3D) -> Self {
        BoundingBox3D {
            min_x: std::cmp::min(c1.x, c2.x),
            min_y: std::cmp::min(c1.y, c2.y),
            min_z: std::cmp::min(c1.z, c2.z),
            max_x: std::cmp::max(c1.x, c2.x),
            max_y: std::cmp::max(c1.y, c2.y),
            max_z: std::cmp::max(c1.z, c2.z),
        }
    }

    pub fn contains(&self, p: CellCoord3D) -> bool {
        p.x >= self.min_x && p.x <= self.max_x &&
        p.y >= self.min_y && p.y <= self.max_y &&
        p.z >= self.min_z && p.z <= self.max_z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox2D {
    pub min_col: u32,
    pub min_row: u32,
    pub max_col: u32,
    pub max_row: u32,
}

impl BoundingBox2D {
    pub fn new(c1: CellCoord, c2: CellCoord) -> Self {
        BoundingBox2D {
            min_col: min(c1.col, c2.col),
            min_row: min(c1.row, c2.row),
            max_col: max(c1.col, c2.col),
            max_row: max(c1.row, c2.row),
        }
    }

    pub fn contains(&self, cell: CellCoord) -> bool {
        cell.col >= self.min_col && cell.col <= self.max_col &&
        cell.row >= self.min_row && cell.row <= self.max_row
    }

    pub fn intersects(&self, other: &BoundingBox2D) -> bool {
        self.min_col <= other.max_col && self.max_col >= other.min_col &&
        self.min_row <= other.max_row && self.max_row >= other.min_row
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalityType {
    FF, // Fixed-Fixed
    RR { col_offset: i32, row_offset: i32 }, // Relative-Relative
    RF { target_col: u32, target_row: u32 }, // Relative-Fixed
    FR { source_col: u32, source_row: u32 }, // Fixed-Relative
}

#[derive(Debug, Clone)]
pub struct TacoDependency {
    pub source_range: BoundingBox2D,
    pub target_range: BoundingBox2D,
    pub locality: LocalityType,
}

impl TacoDependency {
    pub fn new(source: BoundingBox2D, target: BoundingBox2D, locality: LocalityType) -> Self {
        TacoDependency {
            source_range: source,
            target_range: target,
            locality,
        }
    }

    pub fn resolve_dependencies(&self, source: CellCoord) -> Option<Vec<CellCoord>> {
        if !self.source_range.contains(source) {
            return None;
        }

        match self.locality {
            LocalityType::FF => {
                let mut targets = Vec::new();
                for c in self.target_range.min_col..=self.target_range.max_col {
                    for r in self.target_range.min_row..=self.target_range.max_row {
                        targets.push(CellCoord { col: c, row: r });
                    }
                }
                Some(targets)
            }
            LocalityType::RR { col_offset, row_offset } => {
                let target_col = (source.col as i32 + col_offset) as u32;
                let target_row = (source.row as i32 + row_offset) as u32;
                let target = CellCoord { col: target_col, row: target_row };
                if self.target_range.contains(target) {
                    Some(vec![target])
                } else {
                    None
                }
            }
            LocalityType::RF { target_col, target_row } => {
                Some(vec![CellCoord { col: target_col, row: target_row }])
            }
            LocalityType::FR { source_col: _, source_row: _ } => {
                let mut targets = Vec::new();
                for c in self.target_range.min_col..=self.target_range.max_col {
                    for r in self.target_range.min_row..=self.target_range.max_row {
                        targets.push(CellCoord { col: c, row: r });
                    }
                }
                Some(targets)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct TacoSpatialIndex {
    pub dependencies: Vec<TacoDependency>,
}

impl TacoSpatialIndex {
    pub fn insert(&mut self, dep: TacoDependency) {
        self.dependencies.push(dep);
    }

    pub fn query(&self, cell: CellCoord) -> Vec<CellCoord> {
        let mut results = Vec::new();
        for dep in &self.dependencies {
            if let Some(mut resolved) = dep.resolve_dependencies(cell) {
                results.append(&mut resolved);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taco_locality() {
        let source_box = BoundingBox2D::new(CellCoord { col: 0, row: 0 }, CellCoord { col: 0, row: 10 });
        let target_box = BoundingBox2D::new(CellCoord { col: 1, row: 0 }, CellCoord { col: 1, row: 10 });
        
        // 1. RR: Slide next to each other
        let rr_dep = TacoDependency::new(source_box, target_box, LocalityType::RR { col_offset: 1, row_offset: 0 });
        let res = rr_dep.resolve_dependencies(CellCoord { col: 0, row: 5 }).unwrap();
        assert_eq!(res, vec![CellCoord { col: 1, row: 5 }]);

        // 2. RF: Fixed target cell
        let rf_dep = TacoDependency::new(source_box, target_box, LocalityType::RF { target_col: 9, target_row: 9 });
        let res_rf = rf_dep.resolve_dependencies(CellCoord { col: 0, row: 2 }).unwrap();
        assert_eq!(res_rf, vec![CellCoord { col: 9, row: 9 }]);

        // 3. FF: Fixed range
        let ff_dep = TacoDependency::new(source_box, target_box, LocalityType::FF);
        let res_ff = ff_dep.resolve_dependencies(CellCoord { col: 0, row: 0 }).unwrap();
        assert_eq!(res_ff.len(), 11); // Target range size is 11 cells

        // 4. FR: Fixed source, target slides
        let fr_dep = TacoDependency::new(source_box, target_box, LocalityType::FR { source_col: 0, source_row: 0 });
        let res_fr = fr_dep.resolve_dependencies(CellCoord { col: 0, row: 0 }).unwrap();
        assert_eq!(res_fr.len(), 11);
    }

    #[test]
    fn test_spatial_index() {
        let mut index = TacoSpatialIndex::default();
        let source_box = BoundingBox2D::new(CellCoord { col: 0, row: 0 }, CellCoord { col: 0, row: 5 });
        let target_box = BoundingBox2D::new(CellCoord { col: 1, row: 0 }, CellCoord { col: 1, row: 5 });
        let dep = TacoDependency::new(source_box, target_box, LocalityType::RR { col_offset: 1, row_offset: 0 });
        index.insert(dep);

        let query_res = index.query(CellCoord { col: 0, row: 3 });
        assert_eq!(query_res, vec![CellCoord { col: 1, row: 3 }]);
    }

    #[test]
    fn test_3d_spatial_bounds() {
        let c1 = CellCoord3D { x: 0, y: 0, z: 0 };
        let c2 = CellCoord3D { x: 10, y: 10, z: 10 };
        let bbox = BoundingBox3D::new(c1, c2);
        
        let p_inside = CellCoord3D { x: 5, y: 5, z: 5 };
        let p_outside = CellCoord3D { x: 15, y: 5, z: 5 };
        
        assert!(bbox.contains(p_inside));
        assert!(!bbox.contains(p_outside));
    }
}
