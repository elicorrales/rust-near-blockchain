use mylib::Counter;

fn main() {
    println!("Hello, I am a Counter!");
    let mut counter = Counter{value:0};
    let _value = counter.get_num();
    println!("{}", counter.value);
    counter.increment();
    println!("{}", counter.value);
    counter.decrement();
    println!("{}", counter.value);
    counter.increment();
    println!("{}", counter.value);
    counter.increment();
    println!("{}", counter.value);
    counter.reset();
}
