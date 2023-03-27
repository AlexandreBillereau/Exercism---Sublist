#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (fl,sl) if fl < sl => {
            if _second_list.windows(_first_list.len()).any( |x| x == _first_list) {
                return Comparison::Sublist;
            }
            return Comparison::Unequal;
        },
        (fl,sl) if fl > sl => {
            if _first_list.windows(_second_list.len()).any(|x| x == _second_list) {
                return Comparison::Superlist;
            }
            return Comparison::Unequal;
        }
        _ => {
            if _first_list.iter().any(|x| _second_list.contains(x)){
                return Comparison::Equal
            }
            Comparison::Unequal
        },
    }
    
}

