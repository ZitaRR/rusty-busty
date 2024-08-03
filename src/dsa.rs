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

    fn print_array(list: &mut Vec<usize>){
        println!("{}", format!("{:?}", list));
    }
}