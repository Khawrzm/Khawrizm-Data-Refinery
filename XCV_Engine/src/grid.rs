use std::collections::HashMap;

// تمثيل نوع البيانات داخل الخلية
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Formula(String),
}

// هيكل ورقة العمل (الشبكة)
pub struct XcvSheet {
    pub name: String,
    // خريطة لتخزين الخلايا باستخدام الإحداثيات (مثال: "A1")
    cells: HashMap<String, CellValue>,
}

impl XcvSheet {
    pub fn new(name: &str) -> Self {
        XcvSheet {
            name: name.to_string(),
            cells: HashMap::new(),
        }
    }

    pub fn set_cell(&mut self, ref_id: &str, value: CellValue) {
        let parsed_value = match value {
            CellValue::Text(s) => {
                if let Ok(n) = s.trim().parse::<f64>() {
                    CellValue::Number(n)
                } else {
                    CellValue::Text(s)
                }
            }
            other => other,
        };
        self.cells.insert(ref_id.to_uppercase(), parsed_value);
    }

    pub fn get_cell(&self, ref_id: &str) -> CellValue {
        self.cells.get(&ref_id.to_uppercase()).cloned().unwrap_or(CellValue::Empty)
    }

    pub fn get_cell_value(&self, ref_id: &str) -> f64 {
        match self.get_cell(ref_id) {
            CellValue::Number(n) => n,
            CellValue::Text(s) => s.trim().parse::<f64>().unwrap_or(0.0),
            CellValue::Formula(_) => 0.0,
            CellValue::Empty => 0.0,
        }
    }

    pub fn get_cell_formula(&self, ref_id: &str) -> String {
        match self.get_cell(ref_id) {
            CellValue::Formula(f) => f,
            _ => String::new(),
        }
    }
}
