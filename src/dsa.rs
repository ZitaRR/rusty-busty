pub mod fibonacci{
    pub fn f(n: i32) -> Vec<i32>{
        return _f(0, 1, n, Vec::new());
    }

    fn _f(a: i32, b: i32, n: i32, mut list: Vec<i32>) -> Vec<i32>{
        if a + b >= n{
            return list;
        }

        list.push(a + b);
        return _f(b, a + b, n, list);
    }
}

pub mod arrays{
    pub fn buddle_sort(){
        
    }
}