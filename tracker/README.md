## RoadMap
1. 降低内存使用
    - 目前对每个用户对每个种子的做种信息均记录ip信息，每个记录需要约8byte`hash`+(16+32)byte`String`+24byte`ip&port`=80bytes
    - 优化方向
        - String inline，每个记录8byte`hash`+32byte`[u8;32]`+24byte`ip&port`=64bytes
        - 所有种子共享用户的ip信息，每个记录需要8byte`hash`+32byte`[u8;32]`+8byte`Box<ip&port>`=48bytes，但是会导致每个用户只能使用一台电脑做种（peer_id替换passkey,但是难以回避用户滥用问题），也会带来额外1次的寻址开销
        - passkey硬编码进uid，将32byte的passkey降至8byte的uid
        - 由于单个种子做种量很难过万，8bytes的hash value可以直接用uint16替换
        - 完全的ipv6环境，直接削除v4，节省字节4，但是padding存在，并不能完美使用，可以结合Box优化，转成packed的字节数组
        - pt特性导致很多单人做种，这时为它专开哈希表过于得不偿失，可以考虑将个人做种信息内联。

2. 回避慢查询
    - 目前每个种子的哈希表再散列时是非渐进式的，尽管不足以导致极长阻塞，但是还是需要避免多次扩容
        - 解决方案： 添加命令HINT，对竞价置顶、新free种子等预先reserve，哈希表元素不多，扩容影响也不大