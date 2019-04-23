use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..(sum/3) {
        for b in a+1..(sum/2){
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                triplets.insert([a, b, c]);
            }
        }
    }
    triplets
}

/*
sum = 12
iterasi 1
a:b:c 1:2:9
a:b:c 1:3:8
a:b:c 1:4:7
a:b:c 1:5:6

iterasi 2
a:b:c 2:3:7
a:b:c 2:4:6
a:b:c 2:5:5

iterasi 3
a:b:c 3:4:5
a:b:c 3:5:4
*/
