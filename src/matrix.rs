pub struct Mat{
    pub width:usize,
    pub height:usize,
    pub capacity:usize,
    pub data:Vec<char>
}

impl Mat{
    pub fn new(width: usize, height: usize) -> Mat{
        let data = vec![' '; width * height];
        let capacity = width * height;
        Mat { width, height, capacity, data }
    }
    pub fn size(&self) -> u64{
        (self.width * self.height) as u64
    }
    pub fn shape(&self) -> (usize, usize){
        (self.width, self.height)
    }
    pub fn set(&mut self, x:i32, y:i32, value: char){
        let index = (y * self.width as i32 + x) as usize;
        self.data[index] = value;
    }
    
    pub fn print(&self) {
        for chunk in self.data.chunks(self.width).rev() {
            for &ch in chunk {
                print!("{}", ch);
            }
            println!();
        }
    }
}