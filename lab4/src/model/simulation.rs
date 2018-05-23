pub struct SimulationOptions<'a> {
  pub input: Option<&'a str>,
  pub table_size: usize,
  pub to_table_size: Option<usize>,
  pub algorithm: &'a str,
  pub should_stdout: bool,
}
