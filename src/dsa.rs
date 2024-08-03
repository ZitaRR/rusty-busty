pub mod fibonacci{
    pub fn f(n: usize) -> Vec<usize>{
        return _f(0, 1, n, Vec::new());
    }

    fn _f(a: usize, b: usize, n: usize, mut list: Vec<usize>) -> Vec<usize>{
        if a + b >= n{
            return list;
        }

        list.push(a + b);
        return _f(b, a + b, n, list);
    }
}

pub mod arrays{
    use std::cmp::Ordering;

    pub struct SearchData{
        pub index: isize,
        pub operations: usize
    }

    pub fn bubble_sort(list: &mut Vec<usize>){
        for i in 0..list.len(){
            for j in 0..list.len() - 1{
                if list[j] > list[j + 1]{
                    list.swap(j, j + 1);
                }
            }
            print_array(list);
        }
    }
    
    pub fn selection_sort(list: &mut Vec<usize>){
        for i in 0..list.len(){
            let mut index = i;
            for j in i..list.len(){
                if list[index] > list[j]{
                    index = j;
                }
            }
            let temp = list[index];
            list.remove(index);
            list.insert(i, temp);
            print_array(list);
        }
    }

    pub fn linear_search(list: &mut Vec<usize>, target: usize) -> SearchData{
        for (i, value) in list.iter().enumerate(){
            if *value == target{
                return SearchData{
                    index: i as isize,
                    operations: i
                }
            }
        }
        return SearchData{
            index: -1,
            operations: list.len()
        }
    }

    pub fn binary_search(list: &mut Vec<usize>, target: usize) -> SearchData{
        return search(list, target, 0, list.len(), SearchData{index: 0, operations: 0});

        fn search(list: &mut Vec<usize>, target: usize, min: usize, max: usize, data: SearchData) -> SearchData{
            let mid = (min + max) / 2;
            println!("Interval: {0}-{1}, Mid: {2}, Value: {3}, Target: {4}", min, max, mid, list[mid], target);
            let data = SearchData{
                index: mid as isize,
                operations: data.operations + 1
            };
            return match list[mid].cmp(&target){
                Ordering::Equal => data,
                Ordering::Less => search(list, target, mid, max, data),
                Ordering::Greater => search(list, target, min, mid, data)
            }
        }
    }

    fn print_array(list: &mut Vec<usize>){
        println!("{}", format!("{:?}", list));
    }
}