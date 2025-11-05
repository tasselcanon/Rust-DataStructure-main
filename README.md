# 用 Rust 语言实现的数据结构

一个基于 Rust 语言实现的数据结构的小 demo，包含顺序表、链表、二叉树、并查集、图、查找和排序等经典结构，并在各个章节中嵌入了部分习题的函数实现，便于学习与理解。

作为数据结构学习的实践项目，旨在使用 Rust 重写经典算法，
帮助我熟悉所有权、借用、生命周期与泛型等核心特性。

## 项目概况

本项目共有 6 大部分：

-   Common => 全局可供使用的模块，如 **栈** 和 **队列**
-   List => 顺序表、链表和串
-   Tree => 二叉树、哈夫曼树和并查集
-   Graph => 图的增删改查以及遍历
-   Searching => 顺序查找、分块查找、树形查找、B+树和散列表
-   Sort => 插入排序、交换排序、归并排序、选择排序和基数排序

## 项目结构

```txt
Rust-DataStructure-main/
├── crates/
│   ├── Common/
│   │   ├── stack/
│   │   └── queue/
│   ├── List/
│   │   ├── sequence_list/
│   │   ├── linked_list/
│   │   └── string/
│   ├── Tree/
│   │   ├── bitree/
│   │   ├── union_find/
│   │   └── huffman_tree/
│   ├── Graph/
│   │   └── graph/
│   ├── Searching/
│   │   ├── block_searching/
│   │   ├── seq_searching/
│   │   ├── tree_searching/
│   │   └── hash_searching/
│   ├── Sorting/
│   │   ├── exchange_sort/
│   │   ├── insertion_sort/
│   │   ├── merge_sort/
│   │   ├── radix_sort/
│   │   └── selection_sort/
├── Cargo.toml
└── README.md

```

## 快速开始

### 1.克隆项目

```bash
git clone https://github.com/tasselcanon/Rust-DataStructure-main.git
cd Rust-DataStructure-main
```

### 2.安装依赖

```bash
cargo build
```

### 3.运行测试

我们以测试 stack 下的 main.rs 为例

```bash
cargo run -p stack
```

正常运行结果为：

```txt
8 4 2 1
length: 4
false
```

## 开发进度

-   [√] List（顺序表、链表、串）
-   [√] Tree（二叉树、哈夫曼树、并查集）
-   [√] Graph（遍历、最短路径、最小生成树）
-   [√] Searching（分块查找、B+树、哈希）
-   [ ] Sorting（冒泡、快排、堆排）
