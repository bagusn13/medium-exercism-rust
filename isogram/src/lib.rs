use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut unique = HashSet::new();
    let generalize = candidate.to_lowercase().replace(" ", "").replace("-","");
    
    //insert semua char candidate ke unique 
    //hashset , yang sama di itung 1
    for i in generalize.chars() {
        if i.is_alphanumeric(){
            unique.insert(i);
        }else{
            continue
        }
    }
    //klo panjang dari uniqe == panjang dari generalize
    //itu berarti isogram
    unique.len() == generalize.len()
}