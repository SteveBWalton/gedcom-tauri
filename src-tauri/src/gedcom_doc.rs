

// Member variables for the 'GedComDoc' class.
pub struct GedcomDoc {
    // This is a count variable just for testing.
    pub count: i32,

    // The file name of the gedcom file.
    pub file_name: String,
}


impl GedcomDoc {
    pub fn new() -> GedcomDoc {
        let file_name = "".to_string();
        return GedcomDoc{count: 0, file_name: file_name};
    }
}
