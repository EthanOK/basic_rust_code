pub fn greet_world() {
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [chinese, english];
    println!("{:?}", regions);
    for region in regions {
        println!("{}", region);
    }
}
