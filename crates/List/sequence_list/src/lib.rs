const MAXSIZE: usize = 50;
#[derive(Debug, Clone, Copy)]
pub struct SQList<T> {
    data: [T; MAXSIZE],
    length: usize,
}

impl<T> SQList<T>
where
    T: Clone + Default + Copy + PartialEq,
{
    pub fn new() -> Self {
        Self {
            data: [T::default(); MAXSIZE],
            length: 0,
        }
    }

    pub fn insert(&mut self, pos: usize, data: T) -> Result<(), &'static str> {
        if self.length == MAXSIZE {
            return Err("数组容量不够");
        } else if pos > self.length + 1 {
            return Err("位置不合法");
        } else {
            for i in ((pos - 1)..self.length).rev() {
                self.data[i + 1] = self.data[i];
            }
            self.data[pos - 1] = data;
            self.length += 1;
        }
        Ok(())
    }

    pub fn delete(&mut self, pos: usize) -> Result<(), &'static str> {
        if pos > self.length + 1 {
            return Err("位置不合法");
        } else {
            for i in (pos - 1)..self.length {
                self.data[i] = self.data[i + 1];
            }
            self.length -= 1;
        }
        Ok(())
    }

    pub fn locate(&self, data: T) -> usize {
        for i in 1..=self.length {
            if self.data[i - 1] == data {
                return i;
            }
        }
        0
    }
}
