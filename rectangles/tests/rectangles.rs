use rectangles::count;

#[test]
#[ignore]
fn my_test() {
    #[rustfmt::skip]
    // let lines = &[
    //     "+------+----+",
    //     "|      |    |",
    //     "+---+--+    |",
    //     "|   |       |",
    //     "+---+-------+",
    // ];
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| |  ",
        "+-+  ",
    ];
    rectangles::count(lines);
    assert!(false)
}

#[test]
fn zero_area_1() {
    let lines = &[];
    assert_eq!(0, count(lines))
}

#[test]
fn zero_area_2() {
    let lines = &[""];
    assert_eq!(0, count(lines))
}

#[test]
fn empty_area() {
    let lines = &[" "];
    assert_eq!(0, count(lines))
}

#[test]
fn one_rectangle() {
    #[rustfmt::skip]
    let lines = &[
        "+-+",
        "| |",
        "+-+",
    ];
    assert_eq!(1, count(lines))
}

#[test]
fn two_rectangles_no_shared_parts() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| |  ",
        "+-+  ",
    ];
    assert_eq!(2, count(lines))
}

#[test]
fn five_rectangles_three_regions() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| | |",
        "+-+-+",
    ];
    assert_eq!(5, count(lines))
}

#[test]
fn rectangle_of_height_1() {
    #[rustfmt::skip]
    let lines = &[
        "+--+",
        "+--+",
    ];
    assert_eq!(1, count(lines))
}

#[test]
fn rectangle_of_width_1() {
    #[rustfmt::skip]
    let lines = &[
        "++",
        "||",
        "++",
    ];
    assert_eq!(1, count(lines))
}

#[test]
fn unit_square() {
    #[rustfmt::skip]
    let lines = &[
        "++",
        "++",
    ];
    assert_eq!(1, count(lines))
}

#[test]
fn incomplete_rectangles() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "    |",
        "+-+-+",
        "| | -",
        "+-+-+",
    ];
    assert_eq!(1, count(lines))
}

#[test]
fn complicated() {
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+---+--+    |",
        "|   |       |",
        "+---+-------+",
    ];
    assert_eq!(3, count(lines))
}

#[test]
fn not_so_complicated() {
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+------+    |",
        "|   |       |",
        "+---+-------+",
    ];
    assert_eq!(2, count(lines))
}

#[test]
fn large_input_with_many_rectangles() {
    let lines = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];
    assert_eq!(60, count(lines))
}

#[test]
fn three_rectangles_no_shared_parts() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+  ",
        "  | |  ",
        "+-+-+-+",
        "| | | |",
        "+-+ +-+",
    ];
    assert_eq!(3, count(lines))
}
