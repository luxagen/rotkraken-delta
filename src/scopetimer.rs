pub struct ScopeTimer
{
	start: std::time::Instant,
	context: Option<&'static str>,
}

impl ScopeTimer
{
	pub fn new(enable: bool,context: &'static str) -> Self
	{
		ScopeTimer
		{
			context: if enable {Some(context)} else {None},
			start: std::time::Instant::now(),
		}
	}
}

impl Drop for ScopeTimer
{
	fn drop(&mut self)
	{
		if self.context.is_none() {return}

		let finish = self.start.elapsed();

		eprintln!(
			"{}: {:.3} ms",
			self.context.unwrap(),
			(finish.as_micros() as f32)/1000f32);
	}
}