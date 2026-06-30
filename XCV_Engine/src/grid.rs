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
        self.cells.insert(ref_id.to_uppercase(), value);
    }

    pub fn get_cell(&self, ref_id: &str) -> CellValue {
        self.cells.get(&ref_id.to_uppercase()).cloned().unwrap_or(CellValue::Empty)
    }
}
