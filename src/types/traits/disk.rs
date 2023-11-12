pub trait IDiskStringView {
    fn get_name(&self) -> String;
    fn get_free_space(&self) -> String;
    fn get_used_space(&self) -> String;
    fn get_total_space(&self) -> String;

    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.get_name(),
            self.get_free_space(),
            self.get_used_space(),
            self.get_total_space(),
        )
    }
}
