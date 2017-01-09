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

}