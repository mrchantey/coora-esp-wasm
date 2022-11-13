use crate::SketchBuilder;
use std::time::SystemTime;




//TODO import trait with now functionality
pub struct Core;
impl Core {
	pub fn append_imports(builder: &mut SketchBuilder) {
		let start_time = SystemTime::now();
		builder.add_import_0("core", "millis", move |_| {
			let now = SystemTime::now();
			let elapsed = now.duration_since(start_time).unwrap();
			elapsed.as_millis() as u64
		});
	}
}
