# medium-exercism-rust

Esai ini disusun untuk memenuhi tugas mata kuliah sistem operasi di semester 110. 
saya akan menjelaskan bagaimana saya menyelesaikan triangle. 


# Triangle (segitiga)
problem ini meminta untuk menentukan apakah segitiga sama sisi, sama kaki, atau tak sama panjang. 
1. Equilateral (sama sisi), semua panjang sisinya sama panjang.
2. Isosceles (sama kaki), memiliki setidaknya dua sisi dengan panjang yang sama.
3. Scalene (sembarang), semua panjang sisinya berbeda-beda.

# Solusi dari masalah segitiga
Untuk menyelesaikan masalah ini, pertama, kita harus memperhatikan syarat membentuk sebuah segitiga, setelah itu barulah kita dapat menentukan apakah segitiga yang di berikan termasuk kedalam Equilateral, Isoceles, atau Scalene dengan cara membandingkan panjang dari setiap sisinya. jika semua panjang sisinya sama panjang maka termasuk segitiga sama sisi, jika memiliki setidaknya dua sisi dengan panjang yang sama maka termasuk segitiga sama kaki, dan jika semua panjang sisinya berbeda-beda maka termasuk kedalam segitiga sembarang.

# Triangle inequality
Fungsi ini untuk mengkonfimasi apakah array yang diberikan dapat dibuat bentuk menjadi segitiga atau tidak. 
Adapun syarat-syarat untuk membentuk segitiga :
1. Semua sisinya harus lebih besar dari nol.
2. Jumlah panjang kedua sisi harus lebih besar dari atau sama dengan panjang sisi ketiga.

# All sides are not zero
kita harus menentukan apakah segitiga tersebut / array yang di berikan , tidak mempunyai sisi dengan panjang nol.
```rust
    fn all_sides_no_zero (sides: [u64; 3]) ->bool{
        let mut count = 0;
        for i in &sides{
            if i > &0{
                count += 1;
            }
        }
        return count == 3;
    }

```
## Penjelasan fungsi all_sides_no_zero
1. Membuat variable count untuk menghitung ada berapa panjang sisi yang bukan nol
2. Cek setiap sisi segitiga tersebut apakah panjangnya lebih dari 0 atau bukan. menggunakan [array reference's](https://doc.rust-lang.org/std/primitive.array.html) untuk bisa di iterasi.
3. Jika iya, maka count akan bertambah 1
4. Karena function ini untuk mengecek apakah sisinya ada yang 0 , maka keluarannya adalah boolean
5. Terakhir kita cek apakah count samadengan 3 atau tidak

# Sum of two sides >= third side
kita harus menentukan apakah jumlah dari 2 sisi lebih dari atau sama dengan sisi ketiga segitiga tersebut  
```rust
  fn sum_of_length(sides: [u64; 3]) -> bool{
        sides[0] + sides[1] >= sides[2] && sides[0] + sides[2] >= sides[1] && sides[1] + sides[2] >= 
        sides[0]
    }
```
## Penjelasan fungsi sum_of_length
jika X + Y >= Z, X + Z >= Y dan Y + Z >= X fungsi tersebut akan menghasilkan sebuah boolean true dan sebaliknya akan menghasilkan boolean false. Fungsi ini adalah untuk menentukan apakah segitiga tersebut memenuhi salah satu syarat pembentukan segitiga. sides[0] , berarti saya mengambil value dari index ke 0 pada array sides ini biasanya disebut [slicing array](https://doc.rust-lang.org/rust-by-example/primitives/array.html) . 

# Equal side
tugas dari fungsi ini untuk menghitung ada berapa sisi yang sama. Dengan perbanding, 0 dengan 1 , 0 dengan 2, dan 1 dengan 2. jika pembanding sama panjang, maka akan dihitung 1 dan seterusnya.
```rust
    fn equal_sides(&self) -> u64{
        let mut count = 0;
        for x in 0..(self.sides).len() {
            for y in x+1..(self.sides).len(){
                if self.sides[x] == self.sides[y]{
                    count+=1;
                }
            }
        }
```
## Penjelasan fungsi equal_side
1. Membuat variable count untuk menyimpan ada berapakah sisi yang sama panjang.
2. for x in 0..(self.sides).len() Melakukan iterasi  dari 0 sampai panjang dari array (3) , jadi iterasi tersebut berajalan dari 0 sampai 2.
3. for y in x+1..(self.sides).len() Melakukan iterasi dari 1 sampai panjang dari array (3), jadi iterasi tersebut berjalan dari 1 sampai 2.
4. Poin 2 dan 3 di atas itu untuk membandingkan 1 per satu panjang dari sisi-sisinya, dalam hal ini berarti, 0 dengan 1 , 0 dengan 2, 1 dengan 2.
5. Jika sisi sisi yang dibandingkan tersebut memiliki panjang yang sama, maka count akan bertambah 1.

# is_equilateral
1. menentukan apakah segitiga tersebut sama sisi atau tidak.
2. jika sisi yang sama sebanyak 3 maka segitiga tersebut termasuk segitiga sama sisi.

```rust
    pub fn is_equilateral(&self) -> bool {
        Triangle::equal_sides(self) == 3
    }
```
## Penjelasan fungsi is_equilateral
Jika pada equal_side menghasilkan 3 maka fungsi ini menghasilkan true.

# is_scalane
1. menentukan apakah segitiga tersebut tidak ada sisi yang sama panjang atau tidak.
2. jika tidak ada sisi yang sama maka segitiga tersebut termasuk segitiga sembarang.

```rust
    pub fn is_scalene(&self) -> bool {
        Triangle::equal_sides(self) == 0
    }
```
## Penjelasan fungsi is_scelene
Jika pada equal_side menghasilkan 0 maka fungsi ini menghasilkan true.

# is_isosceles
1. menentukan apakah segitiga tersebut sama kaki atau tidak.
2. jika sisi yang sama sebanyak minimal 2 sisi yang sama maka segitiga tersebut termasuk segitiga sama kaki.
```rust
    pub fn is_isosceles(&self) -> bool {
        Triangle::equal_sides(self) >= 1
    }
```
## Penjelasan fungsi is_isosceles
Jika pada equal_side menghasilkan minimal 1 maka fungsi ini menghasilkan true.

# Full Code
```rust
pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Triangle::inequality(sides){
            Some(Triangle{sides: sides})
        }else{
            None
        }  
    }

    fn inequality(sides: [u64; 3]) -> bool {
        Triangle::sum_of_length(sides) && Triangle::all_sides_no_zero(sides)
    }

    fn sum_of_length(sides: [u64; 3]) -> bool{
        sides[0] + sides[1] >= sides[2] && sides[0] + sides[2] >= sides[1] && sides[1] + sides[2] >= sides[0]
    }

    fn all_sides_no_zero (sides: [u64; 3]) ->bool{
        let mut count = 0;
        for i in &sides{
            if i > &0{
                count += 1;
            }
        }
        return count == 3;
    }

    fn equal_sides(&self) -> u64{
        let mut count = 0;
        for x in 0..(self.sides).len() {
            for y in x+1..(self.sides).len(){
                if self.sides[x] == self.sides[y]{
                    count+=1;
                }
            }
        }
        count
    }

    pub fn is_equilateral(&self) -> bool {
        Triangle::equal_sides(self) == 3
    }

    pub fn is_scalene(&self) -> bool {
        Triangle::equal_sides(self) == 0
    }

    pub fn is_isosceles(&self) -> bool {
        Triangle::equal_sides(self) >= 1
    }
}
```

