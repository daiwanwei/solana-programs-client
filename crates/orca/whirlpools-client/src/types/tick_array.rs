use crate::types::TickArrayFacade;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TickArrays {
    One(TickArrayFacade),
    Two(TickArrayFacade, TickArrayFacade),
    Three(TickArrayFacade, TickArrayFacade, TickArrayFacade),
    Four(TickArrayFacade, TickArrayFacade, TickArrayFacade, TickArrayFacade),
    Five(TickArrayFacade, TickArrayFacade, TickArrayFacade, TickArrayFacade, TickArrayFacade),
    Six(
        TickArrayFacade,
        TickArrayFacade,
        TickArrayFacade,
        TickArrayFacade,
        TickArrayFacade,
        TickArrayFacade,
    ),
}

impl From<TickArrays> for [Option<TickArrayFacade>; 6] {
    fn from(val: TickArrays) -> Self {
        match val {
            TickArrays::One(tick_array) => [Some(tick_array), None, None, None, None, None],
            TickArrays::Two(tick_array_1, tick_array_2) => {
                [Some(tick_array_1), Some(tick_array_2), None, None, None, None]
            }
            TickArrays::Three(tick_array_1, tick_array_2, tick_array_3) => {
                [Some(tick_array_1), Some(tick_array_2), Some(tick_array_3), None, None, None]
            }
            TickArrays::Four(tick_array_1, tick_array_2, tick_array_3, tick_array_4) => [
                Some(tick_array_1),
                Some(tick_array_2),
                Some(tick_array_3),
                Some(tick_array_4),
                None,
                None,
            ],
            TickArrays::Five(
                tick_array_1,
                tick_array_2,
                tick_array_3,
                tick_array_4,
                tick_array_5,
            ) => [
                Some(tick_array_1),
                Some(tick_array_2),
                Some(tick_array_3),
                Some(tick_array_4),
                Some(tick_array_5),
                None,
            ],
            TickArrays::Six(
                tick_array_1,
                tick_array_2,
                tick_array_3,
                tick_array_4,
                tick_array_5,
                tick_array_6,
            ) => [
                Some(tick_array_1),
                Some(tick_array_2),
                Some(tick_array_3),
                Some(tick_array_4),
                Some(tick_array_5),
                Some(tick_array_6),
            ],
        }
    }
}

impl From<TickArrayFacade> for TickArrays {
    fn from(val: TickArrayFacade) -> Self { TickArrays::One(val) }
}

impl From<[TickArrayFacade; 1]> for TickArrays {
    fn from(val: [TickArrayFacade; 1]) -> Self { TickArrays::One(val[0]) }
}

impl From<[TickArrayFacade; 2]> for TickArrays {
    fn from(val: [TickArrayFacade; 2]) -> Self { TickArrays::Two(val[0], val[1]) }
}

impl From<[TickArrayFacade; 3]> for TickArrays {
    fn from(val: [TickArrayFacade; 3]) -> Self { TickArrays::Three(val[0], val[1], val[2]) }
}

impl From<[TickArrayFacade; 4]> for TickArrays {
    fn from(val: [TickArrayFacade; 4]) -> Self { TickArrays::Four(val[0], val[1], val[2], val[3]) }
}

impl From<[TickArrayFacade; 5]> for TickArrays {
    fn from(val: [TickArrayFacade; 5]) -> Self {
        TickArrays::Five(val[0], val[1], val[2], val[3], val[4])
    }
}

impl From<[TickArrayFacade; 6]> for TickArrays {
    fn from(val: [TickArrayFacade; 6]) -> Self {
        TickArrays::Six(val[0], val[1], val[2], val[3], val[4], val[5])
    }
}
