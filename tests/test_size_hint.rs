use deku::prelude::*;

#[test]
fn test_struct_impl() {
    #[derive(Default, DekuRead, DekuWrite)]
    #[deku(sized)]
    struct Color1 {
        r: u8,
        g: u8,
        b: u8,
    }
    let color = Color1::default();
    assert_eq!(color.bit_size(), 24);
    assert_eq!(<Color1 as DekuWriteSized>::BIT_SIZE, 24);

    #[derive(Default, DekuRead, DekuWrite)]
    #[deku(sized)]
    struct Color2(u8, u8, u8);
    let color = Color2::default();
    assert_eq!(color.bit_size(), 24);

    #[derive(DekuRead, DekuWrite)]
    #[deku(sized)]
    struct Empty1;
    assert_eq!(<Empty1 as DekuWriteSized>::BIT_SIZE, 0);

    #[derive(DekuRead, DekuWrite)]
    #[deku(sized, magic = b"deku-sized")]
    struct Empty2 {}
    assert_eq!(<Empty2 as DekuWriteSized>::BIT_SIZE, b"deku-sized".len());
}

#[test]
fn test_enum_impl() {
    #[derive(DekuRead, DekuWrite)]
    #[deku(size_hint, id_type = "u8", magic = b"deku-sized")]
    enum Enum1 {
        #[deku(id = "0")]
        None,
        #[deku(id = "1")]
        One { a: u8 },
        #[deku(id = "2")]
        Two(u8, u8),
        #[deku(id = "3")]
        Three(u8, u8, u8),
    }
    let e = Enum1::Three(5, 6, 7);
    assert_eq!(e.bit_size(), b"deku-sized".len() + 32);

    #[derive(DekuRead, DekuWrite)]
    #[deku(sized, id_type = "u8")]
    enum Enum2 {
        #[deku(id = "3")]
        Three1(u8, u8, u8),
        #[deku(id = "33")]
        Three2(u8, u8, u8),
    }
    assert_eq!(<Enum2 as DekuWriteSized>::BIT_SIZE, 32);

    #[derive(DekuRead, DekuWrite)]
    #[deku(sized, id = "e", ctx = "e: Enum2")]
    enum Enum3 {
        #[deku(id_pat = "Enum2::Three1(..)")]
        IsFirst,
        #[deku(id_pat = "_")]
        Others,
    }
    assert_eq!(
        <Enum3 as DekuWriteSized>::BIT_SIZE,
        <Enum2 as DekuWriteSized>::BIT_SIZE
    );
}
