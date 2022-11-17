pub struct CooraPluginBindings {
	pub name: &'static str,
	pub typescript_bindings: &'static str,
	pub rust_bindings: &'static str,
}

impl CooraPluginBindings {}

inventory::collect!(CooraPluginBindings);
