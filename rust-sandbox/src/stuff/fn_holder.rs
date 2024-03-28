pub struct FnHolder<F>
where F: Fn(&str) -> String
{
    pub id: String,
    pub the_fn: F
}
