use partial_derive::Partial;

#[test]
fn test() {
    #[rustfmt::skip::attributes(derive)]
    #[allow(dead_code)]
    #[derive(Partial)]
    #[derive(Debug, Clone, PartialEq)]
    struct Struct {
        field: i32,
    }
    let o = Struct {
        field: 3,
    };
    let partial = PartialStruct::from(o);
    assert_eq!(partial.field, Some(3));
}
