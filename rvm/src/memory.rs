static mut MEM: [u16; u16::MAX as usize] = [0; u16::MAX as usize];

pub fn read(address: u16) -> u16 {
  unsafe { MEM[address as usize] }
}

pub fn write(address: u16, value: u16) {
  unsafe { MEM[address as usize] = value }
}

pub fn load(buffer: Vec<u8>, start: u16) {
  assert!(start + (buffer.len() as u16) < u16::MAX);

  unsafe {
    core::ptr::copy_nonoverlapping(
      buffer.as_ptr(),
      MEM.as_mut_ptr().offset(start as isize) as *mut u8,
      buffer.len(),
    );
  }
}
