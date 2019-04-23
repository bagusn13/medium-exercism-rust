# medium-exercism-rust

Ilmu Komputer
Universitas Negeri Jakarta
Esai ini disusun untuk memenuhi tugas mata kuliah sistem operasi di semester 110
saya akan menjelaskan bagaimana saya menyelesaikan triangle problem


# Triangle
problem ini meminta untuk menentukan apakah segitiga sama sisi, sama kaki, atau tak sama panjang. 
1. Equilateral (sama sisi), semua panjang sisinya sama panjang.
2. Isosceles (sama kaki), memiliki setidaknya dua sisi dengan panjang yang sama.
3. Scalene (tak sama panjang), semua panjang sisinya berbeda-beda.

# penyelesaian triangle
untuk menyelesaikan task ini, pertama kita harus memperhatikan syarat membentuk sebuah segitiga, kemudian dapat menentukan apakah segitiga yang di berikan termasuk kedalam Equilateral, Isoceles, atau Scalene dengan cara membandingkan panjang dari setiap sisinya. 

# inequlity
fungsi ini untuk mengkonfimasi apakah array yang diberikan dapat dibuat bentuk menjadi segitiga
syarat membentuk segitiga
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
## penyelesaian
1. Buat variable count untuk menghitung ada berapa panjang sisi yang bukan nol
2. Cek setiap sisi segitiga tersebut apakah panjangnya lebih dari 0 atau bukan
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
## penyelesaian
jika X + Y >= Z, X + Z >= Y dan Y + Z >= X maka segitiga tersebut memenuhi salah satu syarat pembentukan segitiga.

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
# is_equilateral
1. menentukan apakah segitiga tersebut sama sisi atau tidak.
2. jika sisi yang sama sebanyak 3 maka segitiga tersebut termasuk segitiga sama sisi.

```rust
    pub fn is_equilateral(&self) -> bool {
        Triangle::equal_sides(self) == 3
    }
```

# is_scalane
1. menentukan apakah segitiga tersebut tidak ada sisi yang sama panjang atau tidak.
2. jika tidak ada sisi yang sama maka segitiga tersebut termasuk segitiga sembarang.

```rust
    pub fn is_scalene(&self) -> bool {
        Triangle::equal_sides(self) == 0
    }
```

# is_isosceles
1. menentukan apakah segitiga tersebut sama kaki atau tidak.
2. jika sisi yang sama sebanyak minimal 2 sisi yang sama maka segitiga tersebut termasuk segitiga sama kaki.
```rust
    pub fn is_isosceles(&self) -> bool {
        Triangle::equal_sides(self) >= 1
    }
```



