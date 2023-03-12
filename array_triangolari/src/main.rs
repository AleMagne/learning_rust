
fn create_array_triangolare(height :u32) -> Vec<Vec<u32>> {
    let mut to_ret :Vec<Vec<u32>> = Vec::new();
    for i in 0..height{
        let mut tmp :Vec<u32> = Vec::new();
        for j in 0..i{
            tmp.push(j)
        }
        to_ret.push(tmp);
    }
    to_ret
}

fn main() {
    let triangolo :Vec<Vec<u32>> = create_array_triangolare(10); 
    println!("Il triangolo generato è:");
    for i in 0..triangolo.len(){
        println!("{:?}", &triangolo.get(i));
    }

}
