// 分块查找
#[derive(Debug, Clone)]
// 索引项
pub struct IndexItem<T> {
    pub max_key: T,
    pub start_index: usize,
}

#[derive(Debug)]
// 索引表 + 分块表
pub struct BlockTable<T> {
    pub index: Vec<IndexItem<T>>,
    pub data: Vec<T>,
    pub block_size: usize,
}

impl<T> BlockTable<T>
where
    T: PartialOrd + PartialEq + Clone,
{
    pub fn new(data: Vec<T>, mut block_size: usize) -> Self {
        if block_size == 0 {
            block_size = (data.len() as f64).sqrt() as usize;
        }
        let mut index_item = Vec::new();
        let mut start = 0;
        while start < data.len() {
            let end = usize::min(start + block_size, data.len());
            let block = &data[start..end];
            let max_key = block
                .iter()
                .cloned()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            index_item.push(IndexItem {
                max_key,
                start_index: start,
            });
            start = end;
        }

        Self {
            index: index_item,
            data,
            block_size,
        }
    }

    pub fn block_search(&self, key: T) -> Option<usize> {
        let block_ldx = self.index.iter().position(|x| x.max_key >= key)?;
        let start = self.index[block_ldx].start_index;
        let end = usize::min(start + self.block_size, self.data.len());
        self.data[start..end]
            .iter()
            .position(|x| *x == key)
            .map(|pos| start + pos)
    }
}
