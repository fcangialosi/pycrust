extern crate libc;
extern crate libloading as lib;

fn dynamic_load() -> lib::Result<u32> {
	let lib = lib::Library::new("/home/ubuntu/cpy/libmultiply.so")?;
	unsafe {
		let func: lib::Symbol<unsafe extern fn(u32) -> u32> = lib.get(b"runpython")?;
		Ok(func(6))
	}
}

fn main() {
	println!("BEFORE");
	dynamic_load().unwrap();
	println!("AFTER");
}
