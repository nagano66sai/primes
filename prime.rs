//100以下の素数を求める
fn  main(){
println!("{}",2);
let  mut  i=1;
let  mut  s=0;
while  i <100{
 s=0;
i=i+1;
for  j  in  2..i{
if  i%j!=0{

s=s+1;

}
if  s==i-2  {
println!("{}",i);
}}
}

}
