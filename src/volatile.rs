use core::ptr;


#[derive(Copy, Clone, Debug)]
pub struct VolatileCell<T> where T: Copy {
	value: T
}

impl<T> VolatileCell<T> where T: Copy {
	/// Create a cell with an initial value.
	#[inline]
	fn new(value: T) -> VolatileCell<T> {
		VolatileCell {
			value: value,
		}
	}

	#[inline]
	fn get(&self) -> T {
		unsafe {
			ptr::read_volatile(&self.value)
		}
	}

	#[inline]
	fn set(&self, value: T) {
		unsafe {
			let ptr = &self.value as *const T as *mut T;
			ptr::write_volatile(ptr, value);
		}
	}

	#[inline]
	fn update<F>(&mut self, f: F) 
		where F: FnOnce(&mut T) 
	{
		let mut value = self.get();
		f(&mut value);
		self.set(value);
	} 
}


#[cfg(test)]
mod tests {
	use super::VolatileCell;

	#[test]
	fn test_get() {
		let value = 1;
		let cell = VolatileCell::new(value);
		let got_value = cell.get();

		assert_eq!(value, got_value);
	}

	#[test]
	fn test_set() {
		let value = 1;
		let new_value = 2;
		let cell = VolatileCell::new(value);

		cell.set(new_value);
		
		assert_eq!(cell.get(), new_value);
	}

	#[test]
	fn test_update() {
		let mut value = 1;
		let f = |val: &mut usize| { *val = *val + 4 };
		let mut cell = VolatileCell::new(value);
		f(&mut value);
		cell.update(f);

		assert_eq!(value, cell.get());
	}
}