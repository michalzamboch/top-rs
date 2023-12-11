pub trait IDiskStringView {
    fn get_name(&self) -> String;
    fn get_free_space(&self) -> String;
    fn get_used_space(&self) -> String;
    fn get_total_space(&self) -> String;
}
