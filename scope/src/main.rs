fn main() {
    let outer_scope: u32 = 10;

    // innner scope
    {
        let inner_scope: u32 = 2;
        println!("This is inner scope variable inner_scope={}", inner_scope)

    }

    println!("This is outer scope variable outer_scope={}", outer_scope)
}
