struct InnerData {
    pub field: usize
}

struct Data(InnerData);

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{Data, InnerData};

    #[test]
    pub fn it_work() {
        let x = Data(InnerData{field: 0xA});
        assert_eq!(x.0.field, 0xA);
    }
}
