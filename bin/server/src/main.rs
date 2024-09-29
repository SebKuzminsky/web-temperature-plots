fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = yew_hello_world::Stats {
        temperatures: [ 0.1, 1.0, 2.0, 3.0, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.10 ],
    };
    println!("s = {:?}", s);

    let js = serde_json::to_string(&s).unwrap();
    println!("js = {}", js);

    let new_s = serde_json::from_str::<yew_hello_world::Stats>(&js).unwrap();
    println!("back to s: {:?}", new_s);

    Ok(())
}
