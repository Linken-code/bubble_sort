fn main() {
    let mut list = vec![1, 22, 50, 100, 30, 51, 25, 120, 45];
    list = bubble_sort(list);
    println!("{:?}", list);
}

pub fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 0..list.len() {
        for x in 0..list.len() - 1 {
            if list[x] > list[x + 1] {
                list.swap(x, x + 1); //元素交换位置
            }
        }
    }
    list
}
