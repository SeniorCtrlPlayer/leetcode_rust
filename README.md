# leetcode_rust

## String

- `` 如何多次遍历Vec，因为for循环默认使用的是.into_iter()会转移所有权，所以不能多次遍历，想要多次遍历某个Vec时，应该使用.iter()。#Vec
- `ff5977a0` 如何遍历访问String数组，这里需要通过String的nth方法来访问索引对应的字符，并且还需要使用unwrap。比c++使用[]操作符访问麻烦很多。unwrap的作用暂不明确。#String