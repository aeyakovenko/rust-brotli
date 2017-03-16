#[derive(Clone)]
pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}


fn NewHuffmanTree(    count : u32,
    left : i16,
    right : i16) -> HuffmanTree {
    return HuffmanTree{total_count_ : count,
           index_left_:left,
           index_right_or_value_:right,
           };
}
fn InitHuffmanTree(
    xself : &mut HuffmanTree,
    count : u32,
    left : i16,
    right : i16
) {
    *xself = NewHuffmanTree(count, left, right);
}


fn BrotliSetDepth(
    p0: i32, pool: &mut[HuffmanTree], depth: &mut [u8], max_depth: i32)-> bool {
  let mut stack: [i16;16] = [0;16];
  let mut level = 0i32;
  let mut p = p0;
  assert!(max_depth <= 15);
  stack[0] = -1;
  loop {
    if (pool[p as usize].index_left_ >= 0) {
      level+=1;
      if (level > max_depth) {return false};
      stack[level as usize] = pool[p as usize].index_right_or_value_;
      p = pool[p as usize].index_left_ as i32;
      continue;
    } else {
      depth[pool[p as usize].index_right_or_value_ as usize] = level as u8;
    }
    while (level >= 0 && stack[level as usize] == -1){ level-=1;}
    if (level < 0){ return true;}
    p = stack[level as usize] as i32;
    stack[level as usize] = -1;
  }
}
fn brotli_max_uint32_t(
    a : u32, b : u32
) -> u32 {
    if a > b { a } else { b }
}


trait HuffmanComparator {
   fn Cmp(self:&Self, a: &HuffmanTree, b: &HuffmanTree) -> bool;
}
pub struct SortHuffmanTree {
}
impl HuffmanComparator for SortHuffmanTree {
     fn Cmp(self :&Self, v0: &HuffmanTree, v1: &HuffmanTree) -> bool {
            if (*v0).total_count_ != (*v1).total_count_ {
        if !!((*v0).total_count_ < (*v1).total_count_) {
            true
        } else {
            false
        }
    } else if !!((*v0).index_right_or_value_ as (i32) > (*v1).index_right_or_value_ as (i32)) {
        true
    } else {
        false
    }
     }
}
fn SortHuffmanTreeItems<Comparator:HuffmanComparator>(
    mut items : &mut [HuffmanTree],
    n : usize,
    mut
    comparator
    :
    Comparator) {

    static gaps
        : [usize; 6]
        = [   132i32 as (usize),
              57i32 as (usize),
              23i32 as (usize),
              10i32 as (usize),
              4i32 as (usize),
              1i32 as (usize)
          ];
    if n < 13 {
        let mut i : usize;
        i = 1;
        'loop14: loop {
            if i < n {
                let tmp : HuffmanTree = items[i as usize].clone();
                let mut k : usize = i;
                let mut j : usize = i.wrapping_sub(1);
                'loop17: loop {
                    if comparator.Cmp(
                           &tmp,
                           &items[j as usize]
                       ) {
                        items[k as usize] = items[j as usize].clone();
                        k = j;
                        if {
                               let _old = j;
                               j = j.wrapping_sub(1);
                               _old
                           } == 0 {
                            break 'loop17;
                        } else {
                            continue 'loop17;
                        }
                    } else {
                        break 'loop17;
                    }
                }
                items[k as usize] = tmp;
                i = i.wrapping_add(1);
                continue 'loop14;
            } else {
                break 'loop14;
            }
        }
    } else {
        let mut g : i32 = if n < 57 { 2i32 } else { 0i32 };
        'loop2: loop {
            if g < 6i32 {
                let gap : usize = gaps[g as usize];
                let mut i : usize;
                i = gap;
                'loop5: loop {
                    if i < n {
                        let mut j : usize = i;
                        let tmp : HuffmanTree = items[i as usize].clone();
                        'loop8: loop {
                            if j >= gap && (comparator.Cmp(
                                                &tmp,
                                                &items[(j - gap) as usize]
                                            )) {
                                items[j as usize] = items[(j - gap) as usize].clone();
                                j = j.wrapping_sub(gap);
                                continue 'loop8;
                            } else {
                                break 'loop8;
                            }
                        }
                        items[j as usize] = tmp;
                        i = i.wrapping_add(1);
                        continue 'loop5;
                    } else {
                        break 'loop5;
                    }
                }
                g = g + 1;
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
    }
}



pub fn BrotliCreateHuffmanTree(
    mut data : &[u32],
    length : usize,
    tree_limit : i32,
    mut tree : &mut [HuffmanTree],
    mut depth : &mut[u8]
) {
    let mut count_limit : u32;
    let mut sentinel : HuffmanTree = 
    NewHuffmanTree(
        !(0i32 as (u32)),
        -1i32 as (i16),
        -1i32 as (i16)
    );
    count_limit = 1i32 as (u32);
    'loop1: loop {
        let mut n : usize = 0i32 as (usize);
        let mut i : usize;
        let mut j : usize;
        let mut k : usize;
        i = length;
        'loop2: loop {
            if i != 0i32 as (usize) {
                i = i.wrapping_sub(1 as (usize));
                if data[i as (usize)] != 0 {
                    let count
                        : u32
                        = brotli_max_uint32_t(data[i as (usize)],count_limit);
                    InitHuffmanTree(
                        &mut tree[
                                  {
                                      let _old = n;
                                      n = n.wrapping_add(1 as (usize));
                                      _old
                                  } as (usize)]
                              ,
                        count,
                        -1i32 as (i16),
                        i as (i16)
                    );
                    continue 'loop2;
                } else {
                    continue 'loop2;
                }
            } else {
                break 'loop2;
            }
        }
        if n == 1i32 as (usize) {
            depth[
                 (tree[0]).index_right_or_value_ as (usize)
             ] = 1;
        } else {
            SortHuffmanTreeItems(tree,n,SortHuffmanTree{});
            tree[n as (usize)] = sentinel.clone();
            tree[
                 n.wrapping_add(1i32 as (usize)) as (usize)
             ] = sentinel.clone();
            i = 0usize;
            j = n.wrapping_add(1i32 as (usize));
            k = n.wrapping_sub(1i32 as (usize));
            'loop5: loop {
                if k != 0i32 as (usize) {
                    let mut left : usize;
                    let mut right : usize;
                    if (tree[i as (usize)]).total_count_ <= (tree[j as usize]).total_count_ {
                        left = i;
                        i = i.wrapping_add(1 as (usize));
                    } else {
                        left = j;
                        j = j.wrapping_add(1 as (usize));
                    }
                    if (tree[i as (usize)]).total_count_ <= (tree[
                                                                          j as (usize)
                                                                      ]).total_count_ {
                        right = i;
                        i = i.wrapping_add(1 as (usize));
                    } else {
                        right = j;
                        j = j.wrapping_add(1 as (usize));
                    }
                    let mut j_end
                        : usize
                        = (2i32 as (usize)).wrapping_mul(n).wrapping_sub(k);
                    (tree[j_end as usize]).total_count_ = (tree[
                                                                          left as (usize)
                                                                      ]).total_count_.wrapping_add(
                                                                        (tree[
                                                                              right as (usize)
                                                                          ]).total_count_
                                                                    );
                    (tree[j_end as (usize)]).index_left_ = left as (i16);
                    (tree[
                          j_end as (usize)
                      ]).index_right_or_value_ = right as (i16);
                    tree[
                         j_end.wrapping_add(1i32 as (usize)) as (usize)
                     ] = sentinel.clone();
                    k = k.wrapping_sub(1 as (usize));
                    continue 'loop5;
                } else {
                    break 'loop5;
                }
            }
            if BrotliSetDepth(
                   (2i32 as (usize)).wrapping_mul(n).wrapping_sub(
                       1i32 as (usize)
                   ) as (i32),
                   tree,
                   depth,
                   tree_limit
               ) {
                break 'loop1;
            } else {
                count_limit = count_limit.wrapping_mul(2i32 as (u32));
                continue 'loop1;
            }
        }
    }
}
pub fn BrotliOptimizeHuffmanCountsForRle(
    mut length : usize,
    mut counts : &mut[u32],
    mut good_for_rle : &mut[u8]
) {
    let mut nonzero_count : usize = 0i32 as (usize);
    let mut stride : usize;
    let mut limit : usize;
    let mut sum : usize;
    let streak_limit : usize = 1240i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            if counts[i as usize] != 0 {
                nonzero_count = nonzero_count.wrapping_add(1 as (usize));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    if nonzero_count < 16i32 as (usize) {
    } else {
        'loop3: loop {
            if length != 0i32 as (usize) && (counts[
                                                  length.wrapping_sub(1i32 as (usize)) as (usize)
                                              ] == 0i32 as (u32)) {
                length = length.wrapping_sub(1 as (usize));
                continue 'loop3;
            } else {
                break 'loop3;
            }
        }
        if length == 0i32 as (usize) {
        } else {
            let mut nonzeros : usize = 0i32 as (usize);
            let mut smallest_nonzero : u32 = (1i32 << 30i32) as (u32);
            i = 0i32 as (usize);
            'loop6: loop {
                if i < length {
                    if counts[i as usize] != 0i32 as (u32) {
                        nonzeros = nonzeros.wrapping_add(1 as (usize));
                        if smallest_nonzero > counts[i as usize] {
                            smallest_nonzero = counts[i as usize];
                        }
                    }
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop6;
                } else {
                    break 'loop6;
                }
            }
            if nonzeros < 5i32 as (usize) {
            } else {
                if smallest_nonzero < 4i32 as (u32) {
                    let mut zeros : usize = length.wrapping_sub(nonzeros);
                    if zeros < 6i32 as (usize) {
                        i = 1i32 as (usize);
                        'loop11: loop {
                            if i < length.wrapping_sub(1i32 as (usize)) {
                                if counts[
                                        i.wrapping_sub(1i32 as (usize)) as (usize)
                                    ] != 0i32 as (u32) && (counts[
                                                                i as (usize)
                                                            ] == 0i32 as (u32)) && (counts[
                                                                                         i.wrapping_add(
                                                                                             1i32 as (usize)
                                                                                         ) as (usize)
                                                                                     ] != 0i32 as (u32)) {
                                    counts[i as usize] = 1i32 as (u32);
                                }
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop11;
                            } else {
                                break 'loop11;
                            }
                        }
                    }
                }
                if nonzeros < 28i32 as (usize) {
                } else {
                    for rle_elem in good_for_rle[..length].iter_mut() {
                        *rle_elem = 0; // memset
                    }
                    let mut symbol : u32 = counts[0i32 as usize];
                    let mut step : usize = 0i32 as (usize);
                    i = 0i32 as (usize);
                    'loop14: loop {
                        if i <= length {
                            if i == length || counts[i as usize] != symbol {
                                if symbol == 0i32 as (u32) && (step >= 5i32 as (usize)) || symbol != 0i32 as (u32) && (step >= 7i32 as (usize)) {
                                    let mut k : usize;
                                    k = 0i32 as (usize);
                                    'loop43: loop {
                                        if k < step {
                                            good_for_rle[
                                                 i.wrapping_sub(k).wrapping_sub(
                                                     1i32 as (usize)
                                                 ) as (usize)
                                             ] = 1i32 as (u8);
                                            k = k.wrapping_add(1 as (usize));
                                            continue 'loop43;
                                        } else {
                                            break 'loop43;
                                        }
                                    }
                                }
                                step = 1i32 as (usize);
                                if i != length {
                                    symbol = counts[i as usize];
                                }
                            } else {
                                step = step.wrapping_add(1 as (usize));
                            }
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop14;
                        } else {
                            break 'loop14;
                        }
                    }
                    stride = 0i32 as (usize);
                    limit = (256i32 as (u32)).wrapping_mul(
                                (counts[0i32 as usize]).wrapping_add(
                                    counts[1i32 as usize]
                                ).wrapping_add(
                                    counts[2i32 as usize]
                                )
                            ).wrapping_div(
                                3i32 as (u32)
                            ).wrapping_add(
                                420i32 as (u32)
                            ) as (usize);
                    sum = 0i32 as (usize);
                    i = 0i32 as (usize);
                    'loop16: loop {
                        if i <= length {
                            if i == length || good_for_rle[
                                                   i as (usize)
                                               ] != 0 || i != 0i32 as (usize) && (good_for_rle[
                                                                                       i.wrapping_sub(
                                                                                           1i32 as (usize)
                                                                                       ) as (usize)
                                                                                   ] != 0) || ((256i32 as (u32)).wrapping_mul(
                                                                                                   counts[
                                                                                                        i as (usize)
                                                                                                    ]
                                                                                               ) as (usize)).wrapping_sub(
                                                                                                  limit
                                                                                              ).wrapping_add(
                                                                                                  streak_limit
                                                                                              ) >= (2i32 as (usize)).wrapping_mul(
                                                                                                       streak_limit
                                                                                                   ) {
                                if stride >= 4i32 as (usize) || stride >= 3i32 as (usize) && (sum == 0i32 as (usize)) {
                                    let mut k : usize;
                                    let mut count
                                        : usize
                                        = sum.wrapping_add(
                                              stride.wrapping_div(2i32 as (usize))
                                          ).wrapping_div(
                                              stride
                                          );
                                    if count == 0i32 as (usize) {
                                        count = 1i32 as (usize);
                                    }
                                    if sum == 0i32 as (usize) {
                                        count = 0i32 as (usize);
                                    }
                                    k = 0i32 as (usize);
                                    'loop25: loop {
                                        if k < stride {
                                            counts[
                                                 i.wrapping_sub(k).wrapping_sub(
                                                     1i32 as (usize)
                                                 ) as (usize)
                                             ] = count as (u32);
                                            k = k.wrapping_add(1 as (usize));
                                            continue 'loop25;
                                        } else {
                                            break 'loop25;
                                        }
                                    }
                                }
                                stride = 0i32 as (usize);
                                sum = 0i32 as (usize);
                                if i < length.wrapping_sub(2i32 as (usize)) {
                                    limit = (256i32 as (u32)).wrapping_mul(
                                                (counts[i as usize]).wrapping_add(
                                                    counts[
                                                         i.wrapping_add(1i32 as (usize)) as (usize)
                                                     ]
                                                ).wrapping_add(
                                                    counts[
                                                         i.wrapping_add(2i32 as (usize)) as (usize)
                                                     ]
                                                )
                                            ).wrapping_div(
                                                3i32 as (u32)
                                            ).wrapping_add(
                                                420i32 as (u32)
                                            ) as (usize);
                                } else if i < length {
                                    limit = (256i32 as (u32)).wrapping_mul(
                                                counts[i as usize]
                                            ) as (usize);
                                } else {
                                    limit = 0i32 as (usize);
                                }
                            }
                            stride = stride.wrapping_add(1 as (usize));
                            if i != length {
                                sum = sum.wrapping_add(counts[i as usize] as (usize));
                                if stride >= 4i32 as (usize) {
                                    limit = (256i32 as (usize)).wrapping_mul(sum).wrapping_add(
                                                stride.wrapping_div(2i32 as (usize))
                                            ).wrapping_div(
                                                stride
                                            );
                                }
                                if stride == 4i32 as (usize) {
                                    limit = limit.wrapping_add(120i32 as (usize));
                                }
                            }
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop16;
                        } else {
                            break 'loop16;
                        }
                    }
                }
            }
        }
    }
}
