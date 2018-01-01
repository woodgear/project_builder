extern crate tera;

use tera::Context;
use tera::Tera;

pub fn build() {
    let mut context = Context::new();
    context.add("product", &"a".to_owned());
    context.add("vat_rate", &"b".to_owned());
    let res = Tera::one_off("{{ product }} world", &context, true);
    println!("res {}",res.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        build();
    }
}
