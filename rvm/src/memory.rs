static mut MEM: [u16; u16::MAX as usize] = [0; u16::MAX as usize];

pub fn read(address: u16) -> u16 {
  unsafe { MEM[address as usize] }
}

pub fn write(address: u16, value: u16) {
  unsafe { MEM[address as usize] = value }
}

pub fn load(buffer: &[u8], start: u16) {
  assert!(start + (buffer.len() as u16) < u16::MAX);
  assert!(buffer.len() % 2 == 0);

  // unsafe {
  //   core::ptr::copy_nonoverlapping(
  //     buffer.as_ptr(),
  //     MEM.as_mut_ptr().offset(start as isize) as *mut u8,
  //     buffer.len(),
  //   );
  // }

  for i in 0..buffer.len() / 2 {
    let bytes = [buffer[i * 2], buffer[i * 2 + 1]];
    let value = u16::from_be_bytes(bytes);

    write(start + (i as u16), value);
  }
}
